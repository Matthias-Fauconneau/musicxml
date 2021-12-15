#![feature(once_cell,let_else)]
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
fn main() -> ui::Result { ui::run(ui::graphic::Widget(|size| Ok(layout(&xml::from_document(&xml::parse(&std::fs::read("../Documents/Scores/sheet.xml")?)?)?, size)))) }
