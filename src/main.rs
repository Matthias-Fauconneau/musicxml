#![feature(once_cell, closure_track_caller)]
pub(crate) type Result<T=(),E=Box<dyn std::error::Error>> = std::result::Result<T,E>;
mod parse;
mod music_xml;
mod parse_music_xml; use parse_music_xml::parse_utf8; // impl FromElement for MusicXML
mod display_music_xml; // impl Display for MusicXML
mod music;
pub(crate) mod font;
mod sheet;
mod staff;
mod measure;
mod beam;
mod attributes;
mod direction;
mod layout; use layout::layout;

fn main() -> Result {
    let music = parse_utf8(&std::fs::read("../Scores/sheet.xml")?)?;
    println!("{}", itertools::Itertools::format(music.part[0].iter(), "\n"));
    ui::run(&music.work.title, &mut ui::graphic::Widget(|size| Ok(layout(&music.part, size))))
}