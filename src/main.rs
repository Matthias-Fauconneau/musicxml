#![feature(once_cell,let_else,crate_visibility_modifier)]
pub use fehler::throws;
crate type Error = Box<dyn std::error::Error>;
mod xml;
mod music_xml;
mod music;
mod font;
mod sheet;
mod staff;
mod measure;
mod beam;
mod attributes;
crate type Font = appendlist::AppendList<(String, ui::font::File<'static>)>;
mod direction;
mod layout; use layout::layout;
fn main() -> ui::Result { 
    let font = Default::default();
    ui::run(ui::graphic::Widget(|size| Ok(layout(&font, &xml::from_document(&xml::parse(&std::fs::read("../Documents/Scores/sheet.xml")?)?)?, size)))) 
}
