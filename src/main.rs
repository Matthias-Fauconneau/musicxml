//#![feature(associated_type_bounds)]
#![allow(non_upper_case_globals)]
mod xml;
mod music_xml; use music_xml::MusicXML;
use framework::{*, assert};

lazy_static::lazy_static! {
	static ref font: framework::font::File<'static> = framework::font::open(std::path::Path::new("/usr/local/share/fonts/bravura/Bravura.otf")).unwrap();
}

fn layout(music: MusicXML, width: u32) -> Graphic<'static> {
	let scale = Ratio{num: 360, div: font.units_per_em().unwrap() as u32};
	let staff_height = scale.div;
	let interval = staff_height / 4; // 90
	let staff_distance = 9*interval;

	let y = |staff:u8, step:i32| -> i32 { - ((staff as u32 * staff_distance) as i32) - step * (interval/2) as i32 }; // staff: 0: bass .. 1: treble; step: -8: bottom .. 0: top

	let staves = 2;
	let mut fill = Vec::new();
	for staff in 0..staves {
		for step in (-8..=0).step_by(2) {
			fill.push(Rect{top_left: scale * xy{x:0, y: y(staff, step)}, size: xy{x: width, y: 1}});
		}
	}

	let mut glyph = Vec::new();
	for part in &music.score_partwise.parts {
		for measure in &part.measures {
			for music_data in &measure.music_data {
				use music_xml::*;
				match music_data {
					MusicData::Attributes(Attributes{clefs, ..}) => for clef in clefs {
						#[allow(non_snake_case)] mod SMuFL {
							#![allow(non_camel_case_types)]
							pub const gClef : char = '\u{E050}'; // G clef
							pub const fClef : char = '\u{E062}'; // F clef
						}
						use {music_xml::ClefSign::*, SMuFL::*};
						let &(_, (char, step)) = [(G, (gClef, -6)), (F, (fClef, -2))].iter().find(|(k,_)| k==&clef.sign).unwrap();
						let id = font.glyph_index(char).unwrap();
						let bbox = font.glyph_bounding_box(id).unwrap();
						//println!("{} {} {} {}", font.glyph_hor_side_bearing(id).unwrap(), step, y(step), bbox.y_max);
						assert!(staves==2); impl Clef { fn staff(&self) -> u8 { 2-self.number } } // 1..2 -> 1..0
						glyph.push(Glyph{top_left: scale * xy{
							x: font.glyph_hor_side_bearing(id).unwrap() as i32,
							y: y(clef.staff(), step) - bbox.y_max as i32,
						}, id});
					},
					_ => (),
				}
			}
		}
	}
	Graphic{fill, font: &font, scale, glyph}
}

#[throws] fn main() { window::run(&mut GraphicView::new(layout(xml::from_document(&xml::parse(&std::fs::read("../test.xml")?)?)?, u32::MAX)))? }
