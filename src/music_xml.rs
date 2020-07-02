/// MusicXML
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]#[serde(rename="work",rename_all="kebab-case")]
pub struct Work {
	work_title: Option<String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="supports",rename_all="kebab-case")]
pub struct Supports {
	element: String,
	//r#type: bool, // watt: "r#type" is not a valid Ident
	#[serde(rename="type")] r_type: bool,
	attribute: Option<String>,
	value: Option<String>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum EncodingElement {
	EncodingDate(/*yyyy-mm-dd*/String),
	Encoder(String),
	Software(String),
	EncodingDescription(String),
	Supports(Supports),
}

#[derive(Debug, Deserialize)]#[serde(rename="encoding",rename_all="kebab-case")]
pub struct Encoding {
	#[serde(rename="")]
	encoding_elements: Vec<EncodingElement>
}

#[derive(Debug, Deserialize)]#[serde(rename="identification",rename_all="kebab-case")]
pub struct Identification {
	#[serde(rename="creator*")]
	creator: Vec<String>,
	#[serde(rename="rights*")]
	rights: Vec<String>,
	encoding: Option<Encoding>,
	source: Option<String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="scaling",rename_all="kebab-case")]
pub struct Scaling {
	millimeters: f32,
	tenths: u32,
}

#[derive(Debug, Deserialize)]#[serde(rename="page-margins",rename_all="kebab-case")]
pub struct PageMargins {
	//r#type: /*odd,even,both*/String,
	#[serde(rename="type")] r_type: /*odd,even,both*/String,
	left_margin: f32,
	right_margin: f32,
	top_margin: f32,
	bottom_margin: f32,
}

#[derive(Debug, Deserialize)]#[serde(rename="page-layout",rename_all="kebab-case")]
pub struct PageLayout {
	page_height: f32,
	page_width: f32,
	#[serde(rename="page-margins{0,2}")]
	page_margins: Vec<PageMargins>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub struct Font {
	#[serde(rename="font-family@")]
	font_family: Option<String>,
	//font_style: Option<String>,
	#[serde(rename="font-size@")]
	font_size: Option<u8>,
	#[serde(rename="font-weight@")]
	font_weight: Option</*normal,bold*/String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="defaults",rename_all="kebab-case")]
pub struct Defaults {
	scaling: Option<Scaling>,
	page_layout: PageLayout,
	//appearance: Option<Appearance>,
	//music_font: Option<Font>
	word_font: Option<Font>,
	#[serde(rename="lyric-font*")]
	lyric_font: Vec<Font>
	//lyric_language: Vec<lyric-language>
}

#[derive(Debug, Deserialize)]#[serde(rename="position",rename_all="kebab-case")]
pub struct Position {
	default_x: Option<f32>,
	default_y: Option<f32>,
	relative_x: Option<f32>,
	relative_y: Option<f32>,
}

#[derive(Debug, Deserialize)]#[serde(rename="print-style",rename_all="kebab-case")]
pub struct PrintStyle {
	#[serde(rename="?")] position: Position,
	//#[serde(rename="1?")] font: Font,
	//#[serde(rename="2?")] color: Color,
}

#[derive(Debug, Deserialize)]#[serde(rename="print-style-align",rename_all="kebab-case")]
pub struct PrintStyleAlign {
	#[serde(rename="?")]
	print_style: PrintStyle,
	//valign: /*top,middle,bottom,baseline*/Option<String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="formatted-text",rename_all="kebab-case")]
pub struct FormattedText {
	justify: /*left,center,right*/Option<String>,
	#[serde(rename="?")]
	print_style_align: PrintStyleAlign,
		#[serde(rename="0?")] font: Font,
		valign: /*top,middle,bottom,baseline*/Option<String>,
	#[serde(rename="$")]
	content: String,
}

#[derive(Debug, Deserialize)]#[serde(rename="credit",rename_all="kebab-case")]
pub struct Credit {
	page: u16,
	credit_words: FormattedText,
}

#[derive(Debug, Deserialize)]#[serde(rename="score-instrument",rename_all="kebab-case")]
pub struct ScoreInstrument {
	id: String,
	instrument_name: String,
	instrument_abbreviation: Option<String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="midi-device",rename_all="kebab-case")]
pub struct MidiDevice {
	id: Option<String>,
	port: u8,
}

#[derive(Debug, Deserialize)]#[serde(rename="midi-device",rename_all="kebab-case")]
pub struct MidiInstrument {
	id: Option<String>,
	midi_channel: Option<u8>,
	midi_program: Option<u8>,
	volume: Option<f32>,
	pan: Option<f32>,
}

#[derive(Debug, Deserialize)]#[serde(rename="part-group",rename_all="kebab-case")]
pub struct PartGroup {
	r#type: /*group-name*/Option<String>,
	group_symbol: Option<String>,
	number: Option<u32>,
}

#[derive(Debug, Deserialize)]#[serde(rename="score-part",rename_all="kebab-case")]
pub struct ScorePart {
	id: String,
	part_name: String,
	part_abbreviation: Option<String>,
	score_instrument: ScoreInstrument,
	midi_device: Option<MidiDevice>,
	midi_instrument: Option<MidiInstrument>,
}

#[derive(Debug, Deserialize)]#[serde(rename="part-list",rename_all="kebab-case")]
pub struct PartList {
	part_group: PartGroup,
	score_part: ScorePart,
}

#[derive(Debug, Deserialize)]#[serde(rename="system-margins",rename_all="kebab-case")]
pub struct SystemMargins {
	left_margin: f32,
	right_margin: f32,
}

#[derive(Debug, Deserialize)]#[serde(rename="system-layout",rename_all="kebab-case")]
pub struct SystemLayout {
	system_margins: SystemMargins,
	system_distance: Option<f32>,
	top_system_distance: Option<f32>,
}

#[derive(Debug, Deserialize)]#[serde(rename="staff-layout",rename_all="kebab-case")]
pub struct StaffLayout {
	#[serde(rename="number")]
	staff: Option<Staff>,
	staff_distance: Option<f32>,
}

#[derive(Debug, Deserialize)]#[serde(rename="print",rename_all="kebab-case")]
pub struct Print {
	// print-attributes
	new_system: Option<bool>,
	new_page: Option<bool>,
	system_layout: Option<SystemLayout>,
	#[serde(rename="*")]
	staff_layout: Vec<StaffLayout>,
}

#[derive(Debug, Deserialize)]#[serde(rename="key",rename_all="kebab-case")]
pub struct Key {
	fifths: i8,
	mode: Option<String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="time",rename_all="kebab-case")]
pub struct Time {
	beats: u8,
	beat_type: u8,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]#[serde(rename=/*"clef-sign"*/"sign")]
pub enum ClefSign { G, F }

#[derive(Debug, Deserialize, Clone, Copy)]#[serde(rename="clef",rename_all="kebab-case")]
pub struct Clef {
	#[serde(rename="number")]
	pub(crate) staff: Staff,
	pub sign: ClefSign,
	line: /*2-5*/Option<u8>,
}

#[derive(Debug, Deserialize)]#[serde(rename="attributes",rename_all="kebab-case")]
pub struct Attributes {
	divisions: Option<u16>,
	key: Option<Key>,
	time: Option<Time>,
	staves: Option<u8>,
	#[serde(rename="clef*")]
	pub clefs: Vec<Clef>,
}

#[derive(Debug, Deserialize)]#[serde(rename="metronome",rename_all="kebab-case")]
pub struct Metronome {
	parentheses: bool,
	#[serde(rename="?")] print_style: PrintStyle,
	beat_unit: /*quarter*/String,
	per_minute: u16,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]#[allow(non_camel_case_types)]
pub enum DynamicText { pppppp,ppppp,pppp,ppp,pp,p,mp,mf,f,ff,fff,ffff,fffff,ffffff, sf,sfp,sfpp,fp,rf,rfz,sfz,sffz,fz,n,pf,sfzp }

#[derive(Debug, Deserialize)]#[serde(rename="dynamics",rename_all="kebab-case")]
pub struct Dynamics {
	#[serde(rename="?")] print_style: PrintStyle,
	#[serde(rename="")] text: DynamicText,
}

#[derive(Debug, Deserialize)]#[serde(rename="type",rename_all="kebab-case")]
enum WedgeType { Crescendo, Diminuendo, Stop, Continue }

#[derive(Debug, Deserialize)]#[serde(rename="wedge",rename_all="kebab-case")]
pub struct Wedge {
	//r#type: WedgeType,
	#[serde(rename="type")] r_type: WedgeType,
	number: Option<u8>,
	#[serde(rename="?")] position: Option<Position>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum DirectionTypeData {
	Metronome(Metronome),
	Words(FormattedText),
	Dynamics(Dynamics),
	Wedge(Wedge),
}

#[derive(Debug, Deserialize)]#[serde(rename="direction-type",rename_all="kebab-case")]
pub struct DirectionType {
	#[serde(rename="")]
	content: DirectionTypeData,
}

//type Staff = /*1-*/u8;
#[derive(Debug, Deserialize, Clone, Copy)]#[serde(transparent)] pub struct Staff(pub /*1-*/u8);
#[allow(non_camel_case_types)] type uf32 = /*0-*/f32;

#[derive(Debug, Deserialize)]#[serde(rename="sound",rename_all="kebab-case")]
pub struct Sound {
	dynamics: Option<uf32>,
	tempo: Option<uf32>,
}

#[derive(Debug, Deserialize)]#[serde(rename="direction",rename_all="kebab-case")]
pub struct Direction {
	staff: Option<Staff>,
	sound: Option<Sound>,
	placement: /*above*/String,
	#[serde(rename="direction-type+")]
	direction_type: Vec<DirectionType>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum NoteTypeValue {
	#[serde(rename="1024th")] _1024th,
	#[serde(rename="512th")] _512th,
	#[serde(rename="256th")] _256th,
	#[serde(rename="128th")] _128th,
	#[serde(rename="64th")] _64th,
	#[serde(rename="32th")] _32th,
	#[serde(rename="16th")] _16th,
	Eighth, Quarter, Half, Whole, Breve, Long, Maxima
}

#[derive(Debug, Deserialize)]#[serde(rename="note-type",rename_all="kebab-case")]
pub struct NoteType {
	#[serde(rename="$")]
	pub value: NoteTypeValue,
	//size: SymbolSize,
}

#[derive(Debug, Deserialize)]#[serde(rename="tie",rename_all="kebab-case")]
pub struct Tie {
	//r#type: /*start,stop*/String,
	#[serde(rename="type")] r_type: /*start,stop*/String,
	//time-only: Option
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum StemValue {
	Down, Up, Double, None
}

#[derive(Debug, Deserialize)]#[serde(rename="stem",rename_all="kebab-case")]
pub struct Stem {
	#[serde(rename="$")]
	r#value: StemValue,
	//y-position
	//color
}

#[derive(Debug, Deserialize)]#[serde(rename="step")]
pub enum Step { C,D,E,F,G,A,B }

#[derive(Debug, Deserialize)]#[serde(rename="pitch",rename_all="kebab-case")]
pub struct Pitch {
	pub step: Step,
	alter: /*-1..1*/Option<f32>,
	octave: Option</*0-9=4*/u8>,
}

#[derive(Debug, Deserialize)]#[serde(rename="rest",rename_all="kebab-case")]
pub struct Rest {
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum NoteData {
	Pitch(Pitch),
	Rest(Rest),
	//Unpitched(Unpitched),
}

#[derive(Debug, Deserialize)]#[serde(rename="beam",rename_all=/*space lowercase*/"kebab-case")]
enum BeamValue { Begin, Continue, End, #[serde(rename="forward hook")] ForwardHook, #[serde(rename="backward hook")] BackwardHook }

#[derive(Debug, Deserialize)]#[serde(rename="beam",rename_all="kebab-case")]
pub struct Beam {
	#[serde(rename="$")]
	value: BeamValue,
	number: /*BeamLevel=1*/Option<u8>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum TiedType { Start, Stop, Continue, LetRing }

#[derive(Debug, Deserialize)]#[serde(rename="tied",rename_all="kebab-case")]
pub struct Tied {
	//r#type: TiedType,
	#[serde(rename="type")] r_type: TiedType,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum ArticulationData { Accent, StrongAccent, Staccato, Tenuto, DetachedLegato, Staccatissimo, Spiccato, Scoop, Plop, Doit, Falloff,
	BreathMark, Caesura, Stress, Unstress, SoftAccent }

#[derive(Debug, Deserialize)]#[serde(rename="articulations",rename_all="kebab-case")]
pub struct Articulations {
	#[serde(rename="")]
	content: Vec<ArticulationData>,
}

#[derive(Debug, Deserialize)]#[serde(rename="fingering",rename_all="kebab-case")]
pub struct Fingering {
	#[serde(rename="$")]
	finger: /*1-5*/u8,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum TechnicalData { Fingering(Fingering) }

#[derive(Debug, Deserialize)]#[serde(rename="technical",rename_all="kebab-case")]
pub struct Technical {
	#[serde(rename="")]
	content: Vec<TechnicalData>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum Notation {
	Tied(Tied),
	Articulations(Articulations),
		/*slur
		tuplet
		glissando
		slide
		ornaments*/
	Technical(Technical),
		/*dynamics
		fermata
		arpeggiate
		non-arpeggiate
		accidental-mark
		other-notation*/

}

#[derive(Debug, Deserialize)]#[serde(rename="notations",rename_all="kebab-case")]
pub struct Notations {
	#[serde(rename="*")]
	content: Vec<Notation>
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub struct EmptyPlacement {
	#[serde(rename="?"/*0*/)]
	print_style: PrintStyle,
	//placement
}

#[derive(Debug, Deserialize)]#[serde(rename="note",rename_all="kebab-case")]
pub struct Note {
	#[serde(rename="?")]
	print_style: PrintStyle,
	duration: u32,
	chord: Option<()>,
	voice: Option<u8>,
	pub r#type: Option<NoteType>,
	//#[serde(rename="type")] r_type: Option<NoteType>,
	#[serde(rename="dot*")]
	dot: Vec<EmptyPlacement>,
	#[serde(rename="tie{0,2}")]
	ties: Vec<Tie>,
	#[serde(rename="beam{0,8}")]
	beams: Vec<Beam>,
	#[serde(rename="notations*")]
	notations: Vec<Notations>,
	pub staff: Option<Staff>,
	stem: Option<Stem>,
	#[serde(rename="")]
	pub content: NoteData,
}

#[derive(Debug, Deserialize)]#[serde(rename="backup",rename_all="kebab-case")]
pub struct Backup {
	duration: u32,
}

#[derive(Debug, Deserialize)]#[serde(rename="ending",rename_all="kebab-case")]
pub struct Ending {
	number: u8,
	//r#type: /*start,stop,discontinue*/String,
	#[serde(rename="type")] r_type: /*start,stop,discontinue*/String,
	#[serde(rename="?")]
	print_style: PrintStyle,
}

#[derive(Debug, Deserialize)]#[serde(rename="repeat",rename_all="kebab-case")]
pub struct Repeat {
	direction: /*backward,forward*/String,
}

#[derive(Debug, Deserialize)]#[serde(rename="barline",rename_all="kebab-case")]
pub struct Barline {
	repeat: Option<Repeat>,
	location: /*right,left,middle*/String,
	bar_style: /*enum*/Option<String>,
	ending: Option<Ending>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum MusicData {
	Print(Print),
	Attributes(Attributes),
	Direction(Direction),
	Note(Note),
	Backup(Backup),
	Barline(Barline)
}

#[derive(Debug, Deserialize)]#[serde(rename="measure",rename_all="kebab-case")]
pub struct Measure {
	number: u32,
	width: f32,
	#[serde(rename="*")]
	pub music_data: Vec<MusicData>,
}

#[derive(Debug, Deserialize)]#[serde(rename="part",rename_all="kebab-case")]
pub struct Part {
	id: String,
	#[serde(rename="measure+")]
	pub measures: Vec<Measure>
}

#[derive(Debug, Deserialize)]#[serde(rename="score-partwise",rename_all="kebab-case")]
pub struct ScorePartwise {
	version: Option<String>,
	work: Option<Work>,
	identification: Identification,
	defaults: Defaults,
	#[serde(rename="credit*")]
	credits: Vec<Credit>,
	part_list: PartList,
    #[serde(rename="part+")]
	pub parts : Vec<Part>
}

#[derive(Debug, Deserialize)]#[serde(rename="",rename_all="kebab-case")]
pub struct MusicXML {
    pub score_partwise: ScorePartwise
}
