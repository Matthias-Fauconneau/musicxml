use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Note {
}

#[derive(Debug, Deserialize)]
struct Measure {
    #[serde(rename="note", default)]
    pub notes: Vec<Note>
}

#[derive(Debug, Deserialize)]
struct Part {
    #[serde(rename="measure", default)]
    pub measures: Vec<Measure>
}

#[derive(Debug, Deserialize)]
struct ScorePartwise {
    pub part : Part
}

#[fehler::throws(anyhow::Error)] fn main() {
    let score: ScorePartwise = serde_xml_rs::from_reader(std::fs::File::open("../test.xml")?)?;
    println!("{:#?}", score);
}
