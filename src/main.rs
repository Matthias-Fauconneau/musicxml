mod xml;
mod music_xml;

#[fehler::throws(anyhow::Error)] fn main() {
    let score: music_xml::ScorePartwise = xml::from_document(&xml::parse(&std::fs::read("../test.xml")?)?)?;
    println!("{:#?}", score);
}
