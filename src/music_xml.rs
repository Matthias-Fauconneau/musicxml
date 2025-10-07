#[derive(Debug)]
pub struct Work {
	pub title: String
}
#[derive(Debug, PartialEq, Clone, Copy)] pub enum Mode { Major, Minor }
#[derive(Debug, PartialEq, Clone, Copy)] pub enum Location { Left, Right, BeforeBarline }
#[derive(Debug)] pub struct Cancel {
	pub fifths: i8,
	pub location: Option<Location>,
}
#[derive(Debug)] pub struct Key {
	pub cancel: Option<Cancel>,
	pub fifths: i8,
	pub mode: Option<Mode>,
}
#[derive(Debug)] pub struct Time {
	pub beats: u8,
	pub beat_type: u8,
}
#[derive(Debug, PartialEq, Clone, Copy)] pub enum Sign { G, F }
#[derive(Debug, Clone, Copy)] pub struct Clef {
	pub staff: Staff, // number
	pub sign: Sign,
	pub line: /*2-5*/Option<u8>,
}
#[derive(Debug)] pub enum UpDownStopContinue { Up, Down, Stop, Continue }
//#[derive(Debug)] #[allow(non_camel_case_types)] pub enum Dynamics { /*pppppp,ppppp,pppp,ppp,*/pp,p,mp,mf,f,ff/*,fff,ffff,fffff,ffffff, sf,sfp,sfpp,fp,rf,rfz,sfz,sffz,fz,n,pf,sfzp*/ }
pub type Dynamics = String;
#[derive(Debug)] pub enum Wedge { Crescendo, Diminuendo, Stop, Continue }
#[derive(Debug)] pub enum DirectionType {
	Dynamics(Dynamics),
	OctaveShift{
		r#type: UpDownStopContinue,
		size: u8,
	},
	Metronome{
		beat_unit: NoteType,
		per_minute: u16,
	},
	Wedge(Wedge),
	Words(String),
	Pedal,
}
#[derive(Debug, Clone, Copy, derive_more::FromStr, PartialEq)] pub struct Staff(pub /*1-*/u8);
#[derive(Debug,PartialEq,PartialOrd,Clone,Copy)] pub enum NoteType { _1024th, _512th, _256th, _128th, _64th, _32nd, _16th, Eighth, Quarter, Half, Whole, Breve, Long, Maxima}
#[derive(Debug,Clone,Copy,PartialEq)] pub enum Tie { Start, Stop }
#[derive(Debug,Clone,Copy,PartialEq)] pub enum Stem { Down, Up }
#[derive(Debug,Clone,Copy,PartialEq)] pub enum Step { C,D,E,F,G,A,B }
#[derive(Debug,Clone,Copy,PartialEq)] pub struct Pitch {
	pub step: Step,
	pub alter: /*-1..1*/Option<i8>,
	pub octave: /*Option<*//*0-9=4*/u8,//>,
}
#[derive(Debug)] pub enum BeamTag { Begin, Continue, End, ForwardHook, BackwardHook }
#[derive(Debug)] pub struct Beam {
	pub tag: BeamTag,
	pub number: Option<u8>,
}
#[derive(Debug)] pub enum StartStopContinue { Start, Stop, Continue }
#[derive(Debug)] pub enum Orientation { Over, Under }
#[derive(Debug)] pub struct Slur {
	pub r#type: StartStopContinue,
	pub orientation: Option<Orientation>,
}
#[derive(Debug)] pub enum Articulation { Accent, StrongAccent, Staccato, Tenuto, DetachedLegato, Staccatissimo, Spiccato, Scoop, Plop, Doit, Falloff, BreathMark, Caesura, Stress, Unstress, SoftAccent }
#[derive(Debug,Default)] pub enum TremoloType { #[default] Single, Start, Stop }
#[derive(Debug)]
pub enum Ornament {
	Tremolo{
		r#type: TremoloType,
		marks: u8,
	},
	Mordent
}
#[derive(Debug)] pub enum Notation {
	Tied(StartStopContinue),
	Articulations(Box<[Articulation]>),
	Slur(Slur),
	Ornaments(Box<[Ornament]>),
	Arpeggiate,
	Fermata,
	Tuplet(StartStopContinue),
	Glissando(StartStopContinue),
}
#[derive(Debug)] pub struct TimeModification {
	pub actual_notes: u8,
	pub normal_notes: u8,
}
#[derive(Debug)] pub enum Accidental { Flat, Natural, Sharp }
#[derive(Debug)] pub struct Note {
	pub duration: Option<u32>,
	pub voice: Option<u8>,
	pub r#type: Option<NoteType>,
	pub accidental: Option<Accidental>,
	pub time_modification: Option<TimeModification>,
	pub dot: u8,
	pub ties: Box<[Tie]>,
	pub beams: Box<[Beam]>,
	pub notations: Box<[Notation]>,
	pub staff: Option<Staff>,
	pub stem: Option<Stem>,
	pub chord: bool,
	pub grace: bool,
	pub pitch: Option<Pitch>
}
#[derive(Debug)] pub struct Attributes {
	pub divisions: Option<u16>,
	pub key: Option<Key>,
	pub time: Option<Time>,
	pub staves: Option<u8>,
	pub clefs: Box<[Clef]>,
}
#[derive(Debug)] pub struct Direction {
	pub direction: Option<DirectionType>,
	pub offset: Option<i16>,
	pub voice: Option<u8>,
	pub staff: Option<Staff>,
}
#[derive(Debug)] pub enum BarStyle { Regular, Dotted, Dashed, Heavy, LightLight, LightHeavy, HeavyHeavy, Tick, Short, None }
#[derive(Debug)] pub struct Harmony {
	pub step: Step,
	pub alter: Option<i8>,
}
#[derive(Debug)]
pub enum MusicData {
	Note(Note),
	Backup(u32),
	Forward(u32),
	Attributes(Attributes),
	Direction(Direction),
	Barline(Option<BarStyle>),
	Harmony(Harmony)
	//Print,
}
pub type Measure = Box<[MusicData]>;
pub type Part = Box<[Measure]>;
#[derive(Debug)]
pub struct Root {
	pub work: Work,
	pub part: Part,
}