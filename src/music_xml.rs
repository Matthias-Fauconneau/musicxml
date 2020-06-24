/// MusicXML
use serde::Deserialize;

#[derive(Debug, Deserialize)]#[serde(rename="work",rename_all="kebab-case")]
pub struct Work {
	work_title: Option<String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="supports",rename_all="kebab-case")]
pub struct Supports {
	element: String,
	r#type: bool,
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
	creator: Vec<String>,
	rights: Vec<String>,
	encoding: Option<Encoding>,
}

#[derive(Debug, Deserialize)]#[serde(rename="scaling",rename_all="kebab-case")]
pub struct Scaling {
	millimeters: f32,
	tenths: u32,
}

#[derive(Debug, Deserialize)]#[serde(rename="page-margins",rename_all="kebab-case")]
pub struct PageMargins {
	r#type: /*odd,even,both*/String,
	left_margin: f32,
	right_margin: f32,
	top_margin: f32,
	bottom_margin: f32,
}

#[derive(Debug, Deserialize)]#[serde(rename="page-layout",rename_all="kebab-case")]
pub struct PageLayout {
	page_height: u32,
	page_width: u32,
	#[serde(rename="page-margins{0,2}")]
	page_margins: Vec<PageMargins>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub struct Font {
	font_family: Option<String>,
	//font_style: Option<String>,
	font_size: Option<u8>,
	//font_weight: Option</*normal,bold*/String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="defaults",rename_all="kebab-case")]
pub struct Defaults {
	scaling: Option<Scaling>,
	page_layout: PageLayout,
	//appearance: Option<Appearance>,
	//music_font: Option<Font>
	word_font: Option<Font>,
	lyric_font: Vec<Font>
	//lyric_language: Vec<lyric-language>
}

#[derive(Debug, Deserialize)]#[serde(rename="print-style",rename_all="kebab-case")]
pub struct PrintStyle {
	default_x: Option<f32>,
	default_y: Option<f32>,
	relative_x: Option<f32>,
	relative_y: Option<f32>,
}

/*#[derive(Debug, Deserialize)]#[serde(rename="print-style-align",rename_all="kebab-case")]
pub struct PrintStyleAlign {
	#[serde(rename="")]
	print_style: PrintStyle,
	valign: /*top,middle,bottom,baseline*/String,
}*/

#[derive(Debug, Deserialize)]#[serde(rename="formatted-text",rename_all="kebab-case")]
pub struct FormattedText {
	//print_style_align: PrintStyleAlign,
	#[serde(rename="")]
	print_style: PrintStyle,
	valign: /*top,middle,bottom,baseline*/Option<String>,
	justify: /*left,center,right*/Option<String>,
	font_size: u8,
	font_weight: Option</*normal,bold*/String>,
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
	top_system_distance: Option<f32>,
}

#[derive(Debug, Deserialize)]#[serde(rename="staff-layout",rename_all="kebab-case")]
pub struct StaffLayout {
	number: Option<u8>,
	staff_distance: Option<f32>,
}

#[derive(Debug, Deserialize)]#[serde(rename="print",rename_all="kebab-case")]
pub struct Print {
	system_layout: Option<SystemLayout>,
	#[serde(rename="*")]
	staff_layout: Vec<StaffLayout>,
}

#[derive(Debug, Deserialize)]#[serde(rename="key",rename_all="kebab-case")]
pub struct Key {
	fifths: i8,
}

#[derive(Debug, Deserialize)]#[serde(rename="time",rename_all="kebab-case")]
pub struct Time {
	beats: u8,
	beat_type: u8,
}

#[derive(Debug, Deserialize)]#[serde(rename="time",rename_all="kebab-case")]
pub struct Clef {
	number: /*staff*/u8,
	sign: /*F,G*/String,
	line: /*2-5*/Option<u8>,
}

#[derive(Debug, Deserialize)]#[serde(rename="attributes",rename_all="kebab-case")]
pub struct Attributes {
	divisions: Option<u16>,
	key: Option<Key>,
	time: Option<Time>,
	staves: Option<u8>,
	#[serde(rename="clef*")]
	clefs: Vec<Clef>,
}

#[derive(Debug, Deserialize)]#[serde(rename="metronome",rename_all="kebab-case")]
pub struct Metronome {
	parentheses: bool,
	#[serde(rename="")] print_style: PrintStyle,
	beat_unit: /*quarter*/String,
	per_minute: u16,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]#[allow(non_camel_case_types)]
pub enum DynamicText { pppppp,ppppp,pppp,ppp,pp,p,mp,mf,f,ff,fff,ffff,fffff,ffffff, sf,sfp,sfpp,fp,rf,rfz,sfz,sffz,fz,n,pf,sfzp }

#[derive(Debug, Deserialize)]#[serde(rename="dynamics",rename_all="kebab-case")]
pub struct Dynamics {
	#[serde(rename="0")] print_style: PrintStyle,
	#[serde(rename="1")] text: DynamicText,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum DirectionTypeData {
	Metronome(Metronome),
	Words(FormattedText),
	Dynamics(Dynamics),
}

#[derive(Debug, Deserialize)]#[serde(rename="direction-type",rename_all="kebab-case")]
pub struct DirectionType {
	#[serde(rename="")]
	content: DirectionTypeData,
}

#[derive(Debug, Deserialize)]#[serde(rename="direction",rename_all="kebab-case")]
pub struct Direction {
	placement: /*above*/String,
	#[serde(rename="direction-type+")]
	direction_type: Vec<DirectionType>,
}

#[derive(Debug, Deserialize)]#[serde(rename="step")]
pub enum Step { A,B,C,D,E,F,G }

#[derive(Debug, Deserialize)]#[serde(rename="pitch",rename_all="kebab-case")]
pub struct Pitch {
	step: Step
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

#[derive(Debug, Deserialize)]#[serde(rename="note",rename_all="kebab-case")]
pub struct Note {
	default_x: Option<f32>,
	default_y: Option<f32>,
	chord: Option<()>,
	#[serde(rename="")]
	content: NoteData,
}

#[derive(Debug, Deserialize)]#[serde(rename="backup",rename_all="kebab-case")]
pub struct Backup {
}

#[derive(Debug, Deserialize)]#[serde(rename="barline",rename_all="kebab-case")]
pub struct Barline {
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
	music_data: Vec<MusicData>,
}

#[derive(Debug, Deserialize)]#[serde(rename="part",rename_all="kebab-case")]
pub struct Part {
	id: String,
	#[serde(rename="measure+")]
	measures: Vec<Measure>
}

#[derive(Debug, Deserialize)]#[serde(rename="score-partwise",rename_all="kebab-case")]
pub struct ScorePartwise {
	version: String,
	work: Option<Work>,
	identification: Identification,
	defaults: Defaults,
	#[serde(rename="credit*")]
	credits: Vec<Credit>,
	part_list: PartList,
    #[serde(rename="part+")]
	parts : Vec<Part>
}

#[derive(Debug, Deserialize)]#[serde(rename="",rename_all="kebab-case")]
pub struct MusicXML {
    score_partwise: ScorePartwise
}
