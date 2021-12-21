#![feature(once_cell,let_else,crate_visibility_modifier)]
pub use fehler::throws;
crate type Error = Box<dyn std::error::Error>;
mod xml;
mod music_xml;
mod music;
mod font;
crate type Font = &'static appendlist::AppendList<(String, ui::font::File<'static>)>;
mod sheet;
mod staff;
mod measure;
mod beam;
mod attributes;
mod direction;
mod layout; use layout::layout;
fn main() -> ui::Result { 
    let font = &*Box::leak::<'static>(Default::default());
    ui::run(ui::graphic::Widget(|size| Ok(layout(font, &xml::from_document(&xml::parse(&std::fs::read("../Documents/Scores/sheet.xml")?)?)?, size)))) 
}
