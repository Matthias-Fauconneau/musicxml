// Opiniated features for MusicXML
use {crate::list, itertools::Itertools, crate::music_xml::{Step, Sign, Clef, Pitch, Note, NoteType, MusicData}};

impl From<&Step> for i8 { fn from(step: &Step) -> Self { use Step::*; match step { C=>0, D=>1, E=>2, F=>3, G=>4, A=>5, B=>6 } } }

impl Pitch {
	pub fn new(clef: &Clef, step: &Step) -> Self {
		use Step::*;
		match clef {
			Clef{sign: Sign::G,..} => Pitch{step: *step, octave: Some(match step { G|A|B => 4, C|D|E|F => 5 }), alter: None},
			Clef{sign: Sign::F,..} => Pitch{step: *step, octave: Some(match step { A|B => 2, C|D|E|F|G => 3 }), alter: None},
		}
	}
}
impl From<&Pitch> for i8 { fn from(pitch: &Pitch) -> Self { (pitch.octave.unwrap_or(4) as i8 - 4)*7 + i8::from(&pitch.step) } }

impl Note {
    pub fn has_stem(&self) -> bool { self.r#type.unwrap() <= NoteType::Half }
}
pub fn sort_by_key<T, K:Ord, F: Fn(&T) -> K>(iter: impl std::iter::IntoIterator<Item=T>, f: F) -> Box<[T]> { let mut list = list(iter);  list.sort_by_key(f); list }
pub fn sort_by_start_time<'t, I: IntoIterator<Item=&'t MusicData>>(it: I) -> Box<[(u32, &'t MusicData)]> {
	sort_by_key(it.into_iter().scan((0,0), {fn f<'t>( (t, next_t) : &mut (u32, u32), music_data: &'t MusicData) -> Option<(u32, &'t MusicData)> {
		if !matches!(music_data, MusicData::Note(Note{chord: true, ..})) { *t = *next_t; } // Normal progress
		// else chord inhibits preceding note progress, i.e starts at the preceding note time
		let t = *t;
		match music_data {
			MusicData::Note(Note{duration: Some(duration), ..}) => { *next_t = std::cmp::max(*next_t, t + duration); /*duration from first (longest)*/},
			MusicData::Backup(duration) => { assert!(t >= *duration); *next_t = t - duration; },
			MusicData::Forward(duration) => { *next_t = t + duration; },
			MusicData::Note(Note{duration: None, ..})|MusicData::Attributes(_)|MusicData::Direction(_)|MusicData::Barline{..}/*|MusicData::Print*/ => {}
		}
		Some((t, music_data))
	} f}), |&(t,_)| t)
}

#[derive(Debug)] pub enum BeamedMusicData<'t> { Beam(Box::<[Box<[&'t Note]>]>), MusicData(&'t MusicData) }
pub fn beam<'t, I: IntoIterator<Item=(u32,&'t MusicData)>>(it: I) -> impl Iterator<Item=(u32,BeamedMusicData<'t>)> {
    it.into_iter().peekable().batching({
	    let mut beam = None; // All consecutive groups of notes (cut by non-note music data (and TODO: beats))
	    let mut chord = None; // All notes starting at the same time (across staves)
	    move |it| {
			let commit_any_pending_chord_to_beam = |beam: &mut Option<_>, chord: &mut Option<(_,Vec<_>)>| {
				if let Some((t,chord)) = chord.take() { let (_, beam) = beam.get_or_insert((t, Vec::new())); beam.push(chord.into_boxed_slice()); }
			};
			while let Some((_, MusicData::Note(_))) = it.peek() {
				let Some((t, MusicData::Note(note))) = it.next() else { unreachable!() };
				if let Some((chord_t,_)) = chord && chord_t != t { // Next chord
					assert!(chord_t < t);
					commit_any_pending_chord_to_beam(&mut beam, &mut chord);
				}
				let (_, chord) = chord.get_or_insert((t, Vec::new()));
				chord.push(note);
			}
			commit_any_pending_chord_to_beam(&mut beam, &mut chord);
			if let Some((t, beam)) = beam.take() { Some((t, BeamedMusicData::Beam(beam.into_boxed_slice()))) }
			else {
				assert!(beam.is_none());
				if let Some((t, music_data)) = it.next() { assert!(!matches!(music_data, MusicData::Note(_))); Some((t, BeamedMusicData::MusicData(music_data))) }
				else { None }
			}
	    }
    })
}
