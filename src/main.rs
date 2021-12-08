mod xml;
mod music_xml;
mod music;
mod font;
mod sheet;
mod staff;
mod measure;
mod beam;
mod attributes;
mod layout; use layout::layout;
fn main() -> Result<(), impl std::fmt::Debug> { 
	ui::app::run(ui::graphic::Widget(|size| Ok(layout(&xml::from_document(&xml::parse(&std::fs::read("../Documents/Scores/sheet.xml")?)?)?, size))))
}
