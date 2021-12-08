#![allow(dead_code)]
use serde_derive::Deserialize;

#[derive(Clone, Copy, Debug, Default)] struct Color { a: u8, r: u8, g: u8, b: u8 }
impl<'de> serde::Deserialize<'de> for Color {
	fn deserialize<D:serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct Visitor;
		impl<'de> serde::de::Visitor<'de> for Visitor {
			type Value = Color;
			fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "a color") }
			fn visit_str<E:serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
				let ("#", argb) = v.split_at(1) else { panic!(); };
                let v = argb.as_bytes().chunks(2).map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap()).collect::<Box<_>>();
				if v.len() == 4 { Ok(Color{a: v[0], r: v[1], g: v[2], b: v[3]}) } else { Ok(Color{a: 0xFF, r: v[0], g: v[1], b: v[2]}) }
			}
		}
		deserializer.deserialize_str(Visitor)
	}
}
#[derive(Debug, Deserialize)]#[serde(rename="work",rename_all="kebab-case")]
pub struct Work {
	work_title: Option<String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="creator",rename_all="kebab-case")]
pub struct Creator {
	r#type: String,
	#[serde(rename="$")] creator: String,
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

#[derive(Debug, Deserialize)]#[serde(rename="miscellaneous",rename_all="kebab-case")]
pub struct MiscellaneousField {
	#[serde(rename="$")]
	content: String,
	name: String,
}

#[derive(Debug, Deserialize)]#[serde(rename="miscellaneous",rename_all="kebab-case")]
pub struct Miscellaneous {
	miscellaneous_field: Vec<MiscellaneousField>
}

#[derive(Debug, Deserialize)]#[serde(rename="identification",rename_all="kebab-case")]
pub struct Identification {
	#[serde(rename="creator*")]
	creator: Vec<Creator>,
	#[serde(rename="rights*")]
	rights: Vec<String>,
	encoding: Option<Encoding>,
	source: Option<String>,
	miscellaneous: Option<Miscellaneous>,
}

#[derive(Debug, Deserialize)]#[serde(rename="scaling",rename_all="kebab-case")]
pub struct Scaling {
	millimeters: uf32,
	tenths: u32,
}

#[derive(Debug, Deserialize)]#[serde(rename="page-margins",rename_all="kebab-case")]
pub struct PageMargins {
	r#type: /*odd,even,both*/String,
	left_margin: uf32,
	right_margin: uf32,
	top_margin: uf32,
	bottom_margin: uf32,
}

#[derive(Debug, Deserialize)]#[serde(rename="page-layout",rename_all="kebab-case")]
pub struct PageLayout {
	page_height: uf32,
	page_width: uf32,
	#[serde(rename="page-margins{0,2}")]
	page_margins: Vec<PageMargins>,
}

#[derive(Debug, Deserialize)]#[serde(rename="type",rename_all="lowercase")]
enum LineWidthType { Beam, Bracket, Dashes, Enclosure, Ending, Extend, #[serde(rename="heavy barline")] HeavyBarline, Leger, #[serde(rename="light barline")] LightBarline, #[serde(rename="octave shift")] OctaveShift, Pedal, #[serde(rename="slur middle")] SlurMiddle, #[serde(rename="slur tip")] SlurTip, Staff, Stem, #[serde(rename="tie middle")] TieMiddle, #[serde(rename="tie tip")] TieTip, #[serde(rename="tuplet bracket")] TupletBracket, Wedge }

#[derive(Debug, Deserialize)]#[serde(rename="line-width",rename_all="kebab-case")]
pub struct LineWidth {
	r#type: LineWidthType,
	#[serde(rename="$")]
	tenths: uf32
}

#[derive(Debug, Deserialize)]#[serde(rename="type",rename_all="kebab-case")]
enum NoteSizeType { Cue, Grace, GraceCue, Large }

#[derive(Debug, Deserialize)]#[serde(rename="note-size",rename_all="kebab-case")]
pub struct NoteSize {
	r#type: NoteSizeType,
	#[serde(rename="$")]
	percents: uf32
}

#[derive(Debug, Deserialize)]#[serde(rename="type",rename_all="kebab-case")]
enum DistanceType { Beam, Hyphen }

#[derive(Debug, Deserialize)]#[serde(rename="distance",rename_all="kebab-case")]
pub struct Distance {
	r#type: DistanceType,
	#[serde(rename="$")]
	tenths: uf32
}

#[derive(Debug, Deserialize)]#[serde(rename="appearance",rename_all="kebab-case")]
pub struct Appearance {
	#[serde(rename="line-width*")]
	line_width: Vec<LineWidth>,
	#[serde(rename="note-size*")]
	note_size: Vec<NoteSize>,
	#[serde(rename="distance*")]
	distance: Vec<Distance>
}

#[derive(Debug, Deserialize)]#[serde(rename="lyric-language",rename_all="kebab-case")]
pub struct LyricLanguage {
	#[serde(rename="xml:lang")]
	xml_lang: String,
	name: Option<String>,
	number: Option<u8>
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub struct Font {
	#[serde(rename="font-family@")]
	font_family: Option<String>,
	#[serde(rename="font-style@")]
	font_style: Option<String>,
	#[serde(rename="font-size@")]
	font_size: Option<uf32>,
	#[serde(rename="font-weight@")]
	font_weight: Option</*normal,bold*/String>
}

#[derive(Debug, Deserialize)]#[serde(rename="defaults",rename_all="kebab-case")]
pub struct Defaults {
	scaling: Option<Scaling>,
	page_layout: Option<PageLayout>,
	system_layout: Option<SystemLayout>,
	#[serde(rename="staff-layout*")]
	staff_layout: Vec<StaffLayout>,
	appearance: Option<Appearance>,
	music_font: Option<Font>,
	word_font: Option<Font>,
	#[serde(rename="lyric-font*")]
	lyric_font: Vec<Font>,
	lyric_language: Vec<LyricLanguage>
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
	#[serde(rename="1?")] font: Font,
	color: Option<Color>,
}

#[derive(Debug, Deserialize)]#[serde(rename="print-style-align",rename_all="kebab-case")]
pub struct PrintStyleAlign {
	#[serde(rename="?")]
	print_style: PrintStyle,
	valign: Option<VAlign>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="lowercase")]
enum Justify { Left, Center, Right }

#[derive(Debug, Deserialize)]#[serde(rename_all="lowercase")]
enum VAlign { Top, Middle, Bottom, Baseline }

#[derive(Debug, Deserialize)]#[serde(rename="formatted-text",rename_all="kebab-case")]
pub struct FormattedText {
	justify: Option<Justify>,
	#[serde(rename="?")]
	print_style_align: PrintStyleAlign,
	#[serde(rename="$")]
	content: String,
}

#[derive(Debug, Deserialize)]#[serde(rename="credit",rename_all="kebab-case")]
pub struct Credit {
	page: u16,
	credit_words: FormattedText,
}

#[derive(Debug, Deserialize)]#[serde(rename="virtual-instrument",rename_all="kebab-case")]
pub struct VirtualInstrument {
	virtual_library: Option<String>,
	virtual_name: Option<String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="score-instrument",rename_all="kebab-case")]
pub struct ScoreInstrument {
	id: String,
	instrument_name: String,
	instrument_abbreviation: Option<String>,
	instrument_sound: Option<String>,
	solo: Option<()>,
	virtual_instrument: Option<VirtualInstrument>,
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


#[derive(Debug, Deserialize)]#[serde(rename="group-symbol",rename_all="kebab-case")]
enum GroupSymbolValue { Brace, Bracket, Line, None, Square }

#[derive(Debug, Deserialize)]#[serde(rename="part-group",rename_all="kebab-case")]
pub struct PartGroup {
	r#type: StartStop,
	number: Option<u32>,
	group_name: Option<String>,
	group_symbol: Option<GroupSymbolValue>,
}

#[derive(Debug, Deserialize)]#[serde(rename="instrument-link",rename_all="kebab-case")]
pub struct InstrumentLink {
	id: String
}

#[derive(Debug, Deserialize)]#[serde(rename="group-link",rename_all="kebab-case")]
pub struct GroupLink {
	#[serde(rename="$")]
	id: String
}

#[derive(Debug, Deserialize)]#[serde(rename="part-link",rename_all="kebab-case")]
pub struct PartLink {
	#[serde(rename="xlink:href")]
	xlink_href: String,
	#[serde(rename="instrument-link*")]
	instrument_link: Vec<InstrumentLink>,
	#[serde(rename="group-link*")]
	group_link: Vec<GroupLink>,
}

type DisplayText = FormattedText;
type AccidentalText = DisplayText;

#[derive(Debug, Deserialize)]#[serde(rename="part-name-display",rename_all="kebab-case")]
pub struct PartDisplay {
	print_object: Option<bool>,
	#[serde(rename="display-text*")]
	display_text: Vec<DisplayText>,
	#[serde(rename="accidental-text*")]
	accidental_text: Vec<AccidentalText>,
}

#[derive(Debug, Deserialize)]#[serde(rename="score-part",rename_all="kebab-case")]
pub struct ScorePart {
	id: String,
	identification: Option<Identification>,
	#[serde(rename="part-link*")]
	part_link: Vec<PartLink>,
	part_name: String,
	part_name_display: Option<PartDisplay>,
	part_abbreviation: Option<String>,
	part_abbreviation_display: Option<PartDisplay>,
	#[serde(rename="score-instrument*")]
	score_instrument: Vec<ScoreInstrument>,
	midi_device: Option<MidiDevice>,
	midi_instrument: Option<MidiInstrument>,
}

#[derive(Debug, Deserialize)]#[serde(rename="part-list",rename_all="kebab-case")]
enum PartGroupOrScorePart {
	PartGroup(PartGroup),
	ScorePart(ScorePart),
}

#[derive(Debug, Deserialize)]#[serde(rename="part-list",rename_all="kebab-case")]
pub struct PartList {
	#[serde(rename="part-group*")]
	start: Vec<PartGroup>,
	score_part: ScorePart,
	#[serde(rename="*")]
	part_group_score_part: Vec<PartGroupOrScorePart>,
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

#[derive(Debug, Deserialize)]#[serde(rename="measure-distance")]
pub struct MeasureDistance {
	#[serde(rename="$")]
	tenths: uf32
}

#[derive(Debug, Deserialize)]#[serde(rename="measure-layout",rename_all="kebab-case")]
pub struct MeasureLayout {
	measure_distance: Option<MeasureDistance>,
}

#[derive(Debug, Deserialize)]#[serde(rename="print",rename_all="kebab-case")]
pub struct Print {
	// print-attributes
	new_system: Option<bool>,
	new_page: Option<bool>,
	system_layout: Option<SystemLayout>,
	#[serde(rename="staff-layout*")]
	staff_layout: Vec<StaffLayout>,
	measure_layout: Option<MeasureLayout>,
}

#[derive(Debug, Deserialize)]#[serde(rename="cancel",rename_all="kebab-case")]
pub struct Cancel {
	pub fifths: i8,
	location: /*left,right,before-barline*/Option<String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="key",rename_all="kebab-case")]
pub struct Key {
	color: Option<Color>,
	cancel: Option<Cancel>,
	pub fifths: i8,
	mode: Option<String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="time",rename_all="kebab-case")]
pub struct Time {
	color: Option<Color>,
	pub beats: u8,
	pub beat_type: u8,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]#[serde(rename=/*"clef-sign"*/"sign")]
pub enum ClefSign { G, F }

#[derive(Debug, Deserialize, Clone, Copy)]#[serde(rename="clef",rename_all="kebab-case")]
pub struct Clef {
	color: Option<Color>,
	#[serde(rename="number")]
	pub(crate) staff: Staff,
	pub sign: ClefSign,
	line: /*2-5*/Option<u8>,
}

#[derive(Debug, Deserialize)]#[serde(rename="staff-details",rename_all="kebab-case")]
pub struct StaffDetails {
	#[serde(rename="number")]
	pub(crate) staff: Staff,
	print_object: bool,
}

#[derive(Debug, Deserialize)]#[serde(rename="attributes",rename_all="kebab-case")]
pub struct Attributes {
	divisions: Option<u16>,
	pub key: Option<Key>,
	pub time: Option<Time>,
	staves: Option<u8>,
	#[serde(rename="clef*")]
	pub clefs: Vec<Clef>,
	#[serde(rename="staff-details*")]
	pub staff_details: Vec<StaffDetails>,
}

#[derive(Debug, Deserialize)]#[serde(rename="metronome",rename_all="kebab-case")]
pub struct Metronome {
	beat_unit: /*quarter*/String,
	per_minute: u16,
	#[serde(rename="?")] print_style: PrintStyle,
	parentheses: Option<bool>,
}

fn eight() -> u8 { 8 }
#[derive(Debug, Deserialize)]#[serde(rename="octave-shift",rename_all="kebab-case")]
pub struct OctaveShift {
	r#type: /*up,down,stop,continue*/String,
	number: Option<u8>,
	#[serde(default="eight")] size: u8,
	#[serde(rename="?")] print_style: PrintStyle,
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
	r#type: WedgeType,
	number: Option<u8>,
	#[serde(rename="?")] position: Option<Position>,
	color: Option<Color>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum DirectionTypeData {
	Metronome(Metronome),
	OctaveShift(OctaveShift),
	Words(FormattedText),
	Dynamics(Dynamics),
	Wedge(Wedge),
}

#[derive(Debug, Deserialize)]#[serde(rename="direction-type",rename_all="kebab-case")]
pub struct DirectionType {
	#[serde(rename="+")]
	content: Vec<DirectionTypeData>,
}

//type Staff = /*1-*/u8;
#[derive(Debug, Deserialize, Clone, Copy)]#[serde(transparent)] pub struct Staff(pub /*1-*/u8);
#[allow(non_camel_case_types)] type uf32 = /*0-*/f32;

#[derive(Debug, Deserialize)]#[serde(rename="sound",rename_all="kebab-case")]
pub struct Sound {
	dynamics: Option<uf32>,
	tempo: Option<uf32>,
}

#[derive(Debug, Deserialize)]#[serde(rename="offset",rename_all="kebab-case")]
pub struct Offset {
	#[serde(rename="$")]
	divisions: u16,
	sound: bool,
}

#[derive(Debug, Deserialize)]#[serde(rename="direction",rename_all="kebab-case")]
pub struct Direction {
	#[serde(rename="direction-type+")]
	direction_type: Vec<DirectionType>,
	offset: Option<Offset>,
	voice: Option<String>,
	staff: Option<Staff>,
	sound: Option<Sound>,
	placement: /*above,below*/Option<String>,	
}

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Clone, Copy)]#[serde(rename_all="kebab-case")]
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

#[derive(Debug, Deserialize, Clone, Copy)]#[serde(rename_all="lowercase")]
pub enum StartStop { Start, Stop }

#[derive(Debug, Deserialize)]#[serde(rename="tie",rename_all="kebab-case")]
pub struct Tie {
	r#type: StartStop,
	//time-only: Option
}

#[derive(Debug, Deserialize, Clone, Copy)]#[serde(rename_all="kebab-case")]
pub enum StemDirection { Down, Up, Double, None }

#[derive(Debug, Deserialize)]#[serde(rename="stem",rename_all="kebab-case")]
pub struct Stem {
	#[serde(rename="$")]
	value: StemDirection,
	//y-position
	//color
}

#[derive(Debug, Deserialize,Clone,Copy)]#[serde(rename="step")]
pub enum Step { C,D,E,F,G,A,B }

#[derive(Debug, Deserialize)]#[serde(rename="pitch",rename_all="kebab-case")]
pub struct Pitch {
	pub step: Step,
	pub alter: /*-1..1*/Option<f32>,
	pub octave: Option</*0-9=4*/u8>,
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
	r#type: TiedType,
}

#[derive(Debug, Deserialize)]#[serde(rename="slur",rename_all="kebab-case")]
pub struct Slur {
	color: Option<Color>,
	r#type: /*start*/String,
	orientation: /*over*/String,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum ArticulationData { Accent, StrongAccent, Staccato, Tenuto, DetachedLegato, Staccatissimo, Spiccato, Scoop, Plop, Doit, Falloff,
	BreathMark, Caesura, Stress, Unstress, SoftAccent }

#[derive(Debug, Deserialize)]#[serde(rename="articulations",rename_all="kebab-case")]
pub struct Articulations {
	#[serde(rename="")]
	content: Vec<ArticulationData>,
}

#[derive(Debug, Deserialize)]#[serde(rename="tremolo",rename_all="kebab-case")]
pub struct Tremolo {
	r#type: /*single,start*/Option<String>,
	#[serde(rename="$")]
	marks: /*0-8*/u8,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum OrnamentData { Tremolo(Tremolo) }

#[derive(Debug, Deserialize)]#[serde(rename="ornaments",rename_all="kebab-case")]
pub struct Ornaments {
	#[serde(rename="")]
	content: Vec<OrnamentData>,
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
	Slur(Slur),
		/*slur
		tuplet
		glissando
		slide*/
	Ornaments(Ornaments),
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

#[derive(Debug, Deserialize)]#[serde(rename="time-modification",rename_all="kebab-case")]
pub struct TimeModification {
	pub actual_notes: u8,
	pub normal_notes: u8,
}

#[derive(Debug, Deserialize)]#[serde(rename="accidental",rename_all="lowercase")]
pub enum Accidental { Flat, Natural, Sharp }

#[derive(Debug, Deserialize)]#[serde(rename="grace",rename_all="lowercase")]
pub struct Grace {}

#[derive(Debug, Deserialize)]#[serde(rename="note",rename_all="kebab-case")]
pub struct Note {
	#[serde(rename="?")] position: Position,
	#[serde(rename="1?")] font: Font,
	color: Option<Color>,
	pub duration: Option<u32>,
	#[serde(rename="instrument*")]
	instruments: Vec<String>,
	voice: Option<u8>,
	pub r#type: Option<NoteType>,
	accidental: Option<Accidental>,
	time_modification: Option<TimeModification>,
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
	pub chord: Option<()>,
	pub grace: Option<Grace>,
	#[serde(rename="")]
	pub content: NoteData,
	id: Option<String>,
}

#[derive(Debug, Deserialize)]#[serde(rename="backup",rename_all="kebab-case")]
pub struct Backup {
	pub duration: u32,
}

#[derive(Debug, Deserialize)]#[serde(rename="forward",rename_all="kebab-case")]
pub struct Forward {
	pub duration: u32,
}

#[derive(Debug, Deserialize)]#[serde(rename="ending",rename_all="kebab-case")]
pub struct Ending {
	number: u8,
	r#type: /*start,stop,discontinue*/String,
	#[serde(rename="?")]
	print_style: PrintStyle,
}

#[derive(Debug, Deserialize)]#[serde(rename="repeat",rename_all="kebab-case")]
pub struct Repeat {
	direction: /*backward,forward*/String,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="lowercase")]
enum RightLeftMiddle { Right, Left, Middle }
fn right() -> RightLeftMiddle { RightLeftMiddle::Right }

#[derive(Debug, Deserialize)]#[serde(rename="barline",rename_all="kebab-case")]
pub struct Barline {
	repeat: Option<Repeat>,
	#[serde(default="right")]
	location: RightLeftMiddle,
	bar_style: /*enum*/Option<String>,
	ending: Option<Ending>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum MusicData {
	Note(Note),
	Backup(Backup),
	Forward(Forward),
	Print(Print),
	Attributes(Attributes),
	Direction(Direction),
	Barline(Barline)
}

#[derive(Debug, Deserialize)]#[serde(rename="measure",rename_all="kebab-case")]
pub struct Measure {
	number: u32,
	width: uf32,
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
