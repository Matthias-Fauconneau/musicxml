/// MusicXML
use serde::Deserialize;

#[derive(Debug, Deserialize)]#[serde(rename="print",rename_all="kebab-case")]
pub struct Print {
}

#[derive(Debug, Deserialize)]#[serde(rename="attributes",rename_all="kebab-case")]
pub struct Attributes {
}

#[derive(Debug, Deserialize)]#[serde(rename="direction",rename_all="kebab-case")]
pub struct Direction {
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
	default_x: f32, default_y: f32,
	chord: Option<()>,
	#[serde(rename="$content")]
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
	#[serde(rename="$content")]
	content: Vec<MusicData>,
}

#[derive(Debug, Deserialize)]#[serde(rename="part",rename_all="kebab-case")]
pub struct Part {
	id: String,
	#[serde(rename="$content")]
	content: Vec<Measure>
}

#[derive(Debug, Deserialize)]#[serde(rename="score-partwise")]
pub struct ScorePartwise {
    part : Part
}

#[derive(Debug, Deserialize)]#[serde(rename="",rename_all="kebab-case")]
pub struct MusicXML {
    score_partwise: ScorePartwise
}
