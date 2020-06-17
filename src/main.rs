use try_extend::TryExtend;
use serde::{Deserialize, de::{self, Deserializer, Visitor, MapAccess}};

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
enum MusicData {
	Print,
	Attributes,
	Direction,
	Note,
}

struct SeqVisitor<T>(Vec<T>);
impl<'de, T:de::Deserialize<'de>> Visitor<'de> for SeqVisitor<T> {
    type Value = Vec<T>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result { formatter.write_str("sequence") }
    fn visit_map<M:MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
		let mut vec = Vec::with_capacity(access.size_hint().unwrap_or(0));
		vec.try_extend(std::iter::from_fn(|| access.next_entry().map(|o:Option<(String,_)>| o.map(|(_,v)| v)).transpose() ))?;
		Ok(vec)
    }
}
fn deserialize<'de, D:Deserializer<'de>>(deserializer: D) -> Result<Vec<MusicData>, D::Error> { deserializer.deserialize_map(SeqVisitor(Vec::new())) }

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
struct Measure {
	number: u32,
	width: f32,
	#[serde(flatten,deserialize_with="deserialize")]
	data: Vec<MusicData>,
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
struct Part {
	id: String,
	measure: Vec<Measure>
}

#[derive(Debug, Deserialize)]#[serde(rename_all="kebab-case")]
struct ScorePartwise {
    part : Part
}

#[fehler::throws(anyhow::Error)] fn main() {
    let score: ScorePartwise = serde_xml_rs::from_reader(std::fs::File::open("../test.xml")?)?;
    println!("{:#?}", score);
}
