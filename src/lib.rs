#![feature(let_chains, generic_arg_infer, anonymous_lifetime_in_impl_trait, iterator_try_reduce)]
mod music_xml; pub use music_xml::{Measure, Pitch, Note, NoteType, Stem, Tie, Attributes, Clef, Sign, Step, Key, Time, Direction, DirectionType, UpDownStopContinue, Harmony, MusicData};
mod parse; mod parse_music_xml; pub use parse_music_xml::parse_utf8; // impl FromElement for MusicXML
mod display_music_xml; // impl Display for MusicXML
mod music; pub use music::*;
pub mod font;
mod sheet; pub use sheet::Sheet;
pub mod staff; pub use staff::{Staff, StaffRef};

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
