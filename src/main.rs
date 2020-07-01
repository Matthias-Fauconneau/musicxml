#![feature(bindings_after_at)]
#![allow(non_upper_case_globals)]
mod xml;
mod music_xml; use music_xml::MusicXML;
#[allow(non_snake_case)] mod SMuFL {
	//#![allow(non_camel_case_types)]
	pub mod clef {
		pub const G : char = '\u{E050}';
		pub const F : char = '\u{E062}';
	}
	pub mod note_head {
		pub const breve : char = '\u{E0A1}';
		pub const whole : char = '\u{E0A2}';
		pub const half : char = '\u{E0A3}';
		pub const black : char = '\u{E0A4}';
	}
}
use framework::*;

lazy_static::lazy_static! {
	static ref font: framework::font::File<'static> = framework::font::open(std::path::Path::new("/usr/local/share/fonts/bravura/Bravura.otf")).unwrap();
}

fn layout(music: &MusicXML, width: i32) -> Graphic<'static> {
	let staff_height = font.units_per_em().unwrap() as u32;
	let interval = staff_height / 4; // 90
	let staff_distance = 9*interval;

	let y = |staff:usize, step:i32| -> i32 { - ((staff as u32 * staff_distance) as i32) - step * (interval/2) as i32 }; // staff: 0: bass .. 1: treble; step: -8: bottom .. 0: top

	use music_xml::*;
	#[derive(Default)] struct Staff { clef: Option<Clef>, octave: i8 };
	let mut staves : [Staff; 2] = array::Iterator::collect(std::iter::from_fn(|| Some(Staff::default()))); //[Staff::default; 2];
	let mut fill = Vec::new();
	for (staff, _) in staves.iter().enumerate() {
		for step in (-8..=0).step_by(2) {
			let y = y(staff, step);
			fill.push(Rect{top_left: xy{x:0, y}, bottom_right: xy{x: width, y: y+1}});
		}
	}

	let mut glyph = Vec::new();
	for part in &music.score_partwise.parts {
		for measure in &part.measures {
			for music_data in &measure.music_data {
				let x = glyph.last().map(|g:&Glyph| (g.top_left+font.size(g.id).into()).x).unwrap_or(0);
				let mut glyph = |staff, step, id| {
					let id = font.glyph_index(id).unwrap();
					glyph.push(Glyph{top_left: xy{
						x: x + font.glyph_hor_side_bearing(id).unwrap() as i32,
						y: y(staff, step) - font.glyph_bounding_box(id).unwrap().y_max as i32,
					}, id})
				};
				use SMuFL::*;
				impl From<&music_xml::Staff> for usize { fn from(staff: &music_xml::Staff) -> Self { (2 - staff.0) as usize } } // 1..2 -> 1: treble .. 0: bass
				match music_data {
					MusicData::Note(Note{staff: Some(staff), r#type: Some(NoteType{value}), content:NoteData::Pitch(Pitch{step, ..}), ..}) => {
						let staff = staff.into();
						let Staff{clef, octave} = &staves[staff];
						let step = {use Step::*; match step { C=>0, D=>1, E=>2, F=>3, G=>4, A=>5, B=>6 }};
						let step = step - {use ClefSign::*; match clef.as_ref().unwrap().sign { G=>10, F=> -2 }} - (*octave as i32)*7;
						glyph(staff, step, {use {NoteTypeValue::*, note_head::*}; match value { Breve=>breve, Whole=>whole, Half=>half, _=>black }});
					},
					MusicData::Attributes(Attributes{clefs, ..}) => for clef@Clef{staff, sign, ..} in clefs {
						let staff : usize = staff.into();
						staves[staff].clef = Some(*clef);
						let (id, step) = {use ClefSign::*; match sign { G=>(clef::G, -6), F=>(clef::F, -2) }};
						glyph(staff, step, id);
					},
					_ => (),
				}
			}
		}
	}
	Graphic{scale: Ratio{num: 360, div: staff_height}, fill, font: &font, glyph}
}

#[throws] fn main() {
	framework::rstack_self()?; framework::sigint_trace();
	window::run(&mut GraphicView::new(layout(&xml::from_document(&xml::parse(&std::fs::read("../test.xml")?)?)?, i32::MAX)))?
}
