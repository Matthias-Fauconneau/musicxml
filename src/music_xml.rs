/// MusicXML
use serde::Deserialize;

#[derive(Debug, Deserialize)]#[serde(rename="print",rename_all="kebab-case")]
pub struct Print {
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum MusicData {
	Print(Print),
	Attributes,
	Direction,
	Note,
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
