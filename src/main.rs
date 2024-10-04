#![feature(let_chains, anonymous_lifetime_in_impl_trait, lazy_cell, iterator_try_reduce, try_blocks, generic_arg_infer)]
pub(crate) type Result<T=(),E=Box<dyn std::error::Error>> = std::result::Result<T,E>;
#[allow(dead_code)] mod music_xml; use music_xml::*;
//mod parse; mod parse_music_xml; use parse_music_xml::parse_utf8; // impl FromElement for MusicXML
mod display_music_xml; // impl Display for MusicXML
pub fn list<T>(iter: impl std::iter::IntoIterator<Item=T>) -> Box<[T]> { iter.into_iter().collect() }
mod music;
pub(crate) mod font;
mod sheet;
mod staff;
mod measure;
mod beam;
mod attributes;
mod direction;
mod harmony;
mod layout; use layout::layout;

impl Default for Note {
    fn default() -> Self {
        Self{
            pitch: None, r#type: None, duration: None, voice: None, accidental: None, time_modification: None, staff: None, stem: None,
            ties: [].into(), beams: [].into(), notations: [].into(),
            chord: false, grace: false,
            dot: 0,
        }
    }
}

fn main() -> Result {
    //let music = parse_utf8(&std::fs::read("../Scores/sheet.musicxml")?)?;
    let music = Root{work: Work{title: String::new()}, part: [[
        MusicData::Attributes(Attributes{
            divisions: Some(2), // per quarter
            key: Some(Key{cancel: None, fifths: -1, mode: Some(Mode::Minor)}),
            time: Some(Time{beats: 4, beat_type: 4}),
            staves: Some(2),
            clefs: Box::new([Clef{staff: Staff(1), sign: Sign::G, line: None}, Clef{staff: Staff(2), sign: Sign::F, line: None}])
        }),
        MusicData::Harmony(Harmony{step: Step::D, alter: Some(-1)}),
        MusicData::Note(Note{pitch: Some(Pitch{step: Step::D, alter: None, octave: 4}), r#type: Some(NoteType::Eighth), duration: Some(1), chord: false, staff: Some(Staff(1)), ..Note::default()}),
        MusicData::Note(Note{pitch: Some(Pitch{step: Step::A, alter: None, octave: 4}), r#type: Some(NoteType::Eighth), duration: Some(1), chord: true, staff: Some(Staff(1)), ..Note::default()}),
        MusicData::Note(Note{pitch: Some(Pitch{step: Step::F, alter: None, octave: 5}), r#type: Some(NoteType::Eighth), duration: Some(1), chord: true, staff: Some(Staff(1)), ..Note::default()}),

        MusicData::Note(Note{pitch: Some(Pitch{step: Step::A, alter: None, octave: 4}), r#type: Some(NoteType::_16th), duration: Some(1), chord: false, staff: Some(Staff(1)), ..Note::default()}),
        MusicData::Note(Note{pitch: Some(Pitch{step: Step::F, alter: None, octave: 5}), r#type: Some(NoteType::_16th), duration: Some(1), chord: true, staff: Some(Staff(1)), ..Note::default()}),

        MusicData::Note(Note{pitch: Some(Pitch{step: Step::A, alter: None, octave: 4}), r#type: Some(NoteType::_16th), duration: Some(1), chord: false, staff: Some(Staff(1)), ..Note::default()}),
        MusicData::Note(Note{pitch: Some(Pitch{step: Step::E, alter: None, octave: 5}), r#type: Some(NoteType::_16th), duration: Some(1), chord: true, staff: Some(Staff(1)), ..Note::default()}),
    ].into()].into()};
    use itertools::Itertools; println!("|{}|", music.part[..1].iter().format_with("|\n|",|e,f| f(&e.iter().format("\t"))));
    layout(&music.part[0..1], vector::xy{x: 3840, y: 2400});
    ui::run(&music.work.title, &mut ui::graphic::Widget(|size| Ok(layout(&music.part[0..1], size))))
}
