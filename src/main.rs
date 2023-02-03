#![feature(let_chains, anonymous_lifetime_in_impl_trait, once_cell, iterator_try_reduce, try_blocks)]
pub(crate) type Result<T=(),E=Box<dyn std::error::Error>> = std::result::Result<T,E>;
mod parse;
mod music_xml;
mod parse_music_xml; use parse_music_xml::parse_utf8; // impl FromElement for MusicXML
mod display_music_xml; // impl Display for MusicXML
pub fn list<T>(iter: impl std::iter::IntoIterator<Item=T>) -> Box<[T]> { iter.into_iter().collect() }
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
    use itertools::Itertools; println!("|{}|", music.part[..8].iter().format_with("|\n|",|e,f| f(&e.iter().format("\t"))));
    layout(&music.part, vector::xy{x: 3840, y: 2400});
    ui::run(&music.work.title, &mut ui::graphic::Widget(|size| Ok(layout(&music.part, size))))
}