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

#[derive(Debug, Deserialize)]#[serde(rename="note",rename_all="kebab-case")]
pub struct Note {
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
	measure: Vec<Measure>
}

#[derive(Debug, Deserialize)]#[serde(rename="score-partwise")]
pub struct ScorePartwise {
    part : Part
}
