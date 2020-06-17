/// MusicXML
use serde::Deserialize;

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub enum MusicData {
	Print,
	Attributes,
	Direction,
	Note,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub struct Measure {
	number: u32,
	width: f32,
	#[serde(flatten)]
	content: Vec<MusicData>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub struct Part {
	id: String,
	measure: Vec<Measure>
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
pub struct ScorePartwise {
    part : Part
}
