// Opiniated features for MusicXML
use crate::music_xml::{Step, ClefSign, Clef, Pitch, Note, NoteData, NoteTypeValue, Backup, Forward, MusicData};

impl From<&Step> for i8 { fn from(step: &Step) -> Self { use Step::*; match step { C=>0, D=>1, E=>2, F=>3, G=>4, A=>5, B=>6 } } }

impl Pitch {
	pub fn new(clef: &Clef, step: &Step) -> Self {
		use Step::*;
		match clef {
			Clef{sign: ClefSign::G,..} => Pitch{step: *step, octave: Some(match step { G|A|B => 4, C|D|E|F => 5 }), alter: None},
			Clef{sign: ClefSign::F,..} => Pitch{step: *step, octave: Some(match step { A|B => 2, C|D|E|F|G => 3 }), alter: None},
		}
	}
}
impl From<&Pitch> for i8 { fn from(pitch: &Pitch) -> Self { (pitch.octave.unwrap_or(4) as i8 - 4)*7 + i8::from(&pitch.step) } }

impl Note {
    pub fn pitch(&self) -> Option<&Pitch> { if let NoteData::Pitch(pitch) = &self.content { Some(pitch) } else { None } }
    pub fn has_stem(&self) -> bool { self.r#type.as_ref().unwrap().value <= NoteTypeValue::Half }
}

impl std::fmt::Display for MusicData { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
	write!(f, "{}", match self {
		MusicData::Note(_) => "Note",
		MusicData::Backup(_) => "Backup",
		_ => "_",
	})
}}

pub fn sort_by_start_time<'t, I: IntoIterator<Item=&'t MusicData>>(it: I) -> impl Iterator<Item=(u32, &'t MusicData)> {
	use itertools::Itertools;
	it.into_iter().scan((0,0), {fn f<'t>( (t, next_t) : &mut (u32, u32), music_data: &'t MusicData) -> Option<(u32, &'t MusicData)> {
		if !matches!(music_data, MusicData::Note(Note{chord: Some(()), ..})) { *t = *next_t; } // Normal progress
		// else chord inhibits preceding note progress, i.e starts at the preceding note time
		let t = *t;
		match music_data {
			MusicData::Note(Note{duration: Some(duration), ..}) => { *next_t = std::cmp::max(*next_t, t + duration); /*duration from first (longest)*/},
			MusicData::Backup(Backup{duration}) => { assert!(t >= *duration); *next_t = t - duration; },
			MusicData::Forward(Forward{duration}) => { *next_t = t + duration; },
			MusicData::Note(Note{duration: None, ..})|MusicData::Print(_)|MusicData::Attributes(_)|MusicData::Direction(_)|MusicData::Barline(_) => {}
		}
		Some((t, music_data))
	} f}).sorted_by_key(|&(t,_)| t)
}

#[derive(Debug)] pub enum BeamedMusicData<'t> { Beam(Vec::<Vec<&'t Note>>), MusicData(&'t MusicData) }
pub fn batch_beamed_group_of_notes<'t, I: IntoIterator<Item=(u32,&'t MusicData)>>(it: I) -> impl Iterator<Item=(u32,BeamedMusicData<'t>)> {
	use itertools::Itertools;
    it.into_iter().peekable().batching({
	    let mut beam = None;
	    let mut chord = None;
	    move |it| loop {
			if let Some((_, MusicData::Note(Note{stem: Some(_),..}))) = it.peek() {
				let Some((t, MusicData::Note(note))) = it.next() else { unreachable!() };
				if let Note{chord: None, ..} = note { // Next chord
					if let Some((t,chord)) = chord.take() { // Commit any pending chord
						let (_, beam) = beam.get_or_insert((t, Vec::new()));
						beam.push(chord);
					}
				}
				let (_, chord) = chord.get_or_insert((t, Vec::new()));
				chord.push(note);
			}
			else if let Some((t, beam)) = beam.take() { return Some((t, BeamedMusicData::Beam(beam))); }
		    else { let (t, music_data) = it.next()?; return Some((t, BeamedMusicData::MusicData(music_data))); }
	    }
    })
}
