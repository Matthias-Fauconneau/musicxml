use {std::fmt::{Formatter,Result,Debug,Display}, crate::music_xml::*};//, itertools::Itertools};

impl Display for Attributes { fn fmt(&self, f: &mut Formatter) -> Result {
    if let Some(divisions) = self.divisions { write!(f, "{divisions} divisions, ")?; }
    assert_eq!(self.key.as_ref().unwrap().fifths, 0); //if let Some(key) = self.key { write!(f, "{} fifths, ", key.fifths); }
    if let Some(ref time) = self.time { write!(f, "{}/{}, ", time.beats, time.beat_type)?; }
	assert_eq!(self.staves, Some(2));
	for (has, expect) in self.clefs.iter().zip(&[Sign::G,Sign::F]) { assert_eq!(&has.sign, expect); }
    Ok(())
}}

impl Display for Direction { fn fmt(&self, f: &mut Formatter) -> Result {
    use DirectionType::*; match self.direction.as_ref().unwrap() {
        Dynamics(s)|Words(s) => Display::fmt(s, f),
        /*OctaveShift{r#type: UpDownStopContinue, size: u8,},
        Metronome{beat_unit: NoteType, per_minute: u16,},
        Wedge(Wedge),*/
        _ => Debug::fmt(self, f)
    }
}}

impl Display for Pitch { fn fmt(&self, f: &mut Formatter) -> Result {
    assert!(self.alter.is_none());
    write!(f, "{:?}{}", self.step, self.octave.unwrap())
}}

impl Display for Note { fn fmt(&self, f: &mut Formatter) -> Result {
    //assert_eq!(self.duration, Some(4));
    //assert_eq!(self.voice, Some(1));
    //assert_eq!(self.r#type, Some(NoteType::Half));
    {use NoteType::*; write!(f, "{}", match self.r#type.unwrap() { Quarter=>".", Half=>"o", Whole=>"O", t=>unimplemented!("{t:?}")})?}
	assert!(self.accidental.is_none());
    assert!(self.time_modification.is_none());
    assert!(self.dot == 0);
    if !self.ties.is_empty() { write!(f, "-")?; }
	assert!(self.beams.is_empty());
	//self.notations
	//assert_eq!(self.staff, Some(Staff(1)));
    //assert_eq!(self.stem, Some(Stem::Down));
    if self.chord { write!(f, "+")?; }
    assert!(!self.grace);
    Display::fmt(self.pitch.as_ref().unwrap(), f)
}}

impl Display for MusicData { fn fmt(&self, f: &mut Formatter) -> Result {use MusicData::*; match self {
    Attributes(s) => Display::fmt(s, f),
    Direction(s) => Display::fmt(s, f),
	Note(s) => Display::fmt(s, f),
	Backup(_) => write!(f, "|\t-\t|"),
    _ => panic!("{self:?}")
}}}

//impl Display for Measure { fn fmt(&self, f: &mut Formatter) -> Result { write!(f, "{}", self.iter().format("\n")) } }
