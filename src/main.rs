#![feature(bindings_after_at)]
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

use core::{throws, Error};
#[throws] fn main() {
	//rstack_self()?; sigint_trace();
	ui::window::run(&mut ui::graphic::Widget(|size| Ok(layout(&xml::from_document(&xml::parse(&std::fs::read("../test.xml")?)?)?, size))))?
}
