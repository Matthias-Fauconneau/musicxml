#![feature(bindings_after_at)]
#![allow(non_upper_case_globals)]
#![allow(incomplete_features)]#![feature(const_generics)]
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
	pub mod accidental {
		pub const flat : char = '\u{E260}';
		//pub const natural : char = '\u{E261}';
		pub const sharp : char = '\u{E262}';
	}
	pub const time_signature : char = '\u{E080}';
}
use framework::*;

lazy_static::lazy_static! {
	static ref font: framework::font::File<'static> = framework::font::open(std::path::Path::new("/usr/local/share/fonts/bravura/Bravura.otf")).unwrap();
}

fn layout(music: &MusicXML, width: i32) -> Graphic<'static> {
	struct Sheet { staff_height: u32, staff_distance: u32 }
	impl Sheet {
		// staff: 0: bass .. 1: treble; step: -8: bottom .. 0: top
		fn y(&self, staff: usize, step: i8) -> i32 { - ((staff as u32 * self.staff_distance) as i32) - step as i32 * (self.staff_height/8) as i32 }
	}

	let sheet = {
		let staff_height = font.units_per_em().unwrap() as u32;
		let interval = staff_height / 4; // 90
		let staff_distance = 9*interval;
		Sheet{staff_height, staff_distance}
	};

	use music_xml::*;
	#[derive(Default)] struct Staff { clef: Option<Clef>, octave: i8 };
	let mut staves : [Staff; 2] = array::Iterator::collect(std::iter::from_fn(|| Some(Staff::default()))); //[Staff::default; 2];
	let mut fill = Vec::new();
	for (staff, _) in staves.iter().enumerate() {
		for step in (-8..=0).step_by(2) {
			let y = sheet.y(staff, step);
			fill.push(Rect{top_left: xy{x:0, y}, bottom_right: xy{x: width, y: y+1}});
		}
	}

	use derive_more::{Deref, DerefMut};
	#[derive(Deref)] struct Score { #[deref] sheet: Sheet, glyph: Vec<Glyph> }
	impl Score { fn new(sheet: Sheet) -> Self { Self{sheet, glyph: Vec::new()} } }
	let mut score = Score::new(sheet);
	for part in &music.score_partwise.parts {
		for measure in &part.measures {
			for music_data in &measure.music_data {

				impl From<&music_xml::Staff> for usize { fn from(staff: &music_xml::Staff) -> Self { (2 - staff.0) as usize } } // 1..2 -> 1: treble .. 0: bass
				#[derive(Deref)] struct StaffRef<'t> { index: usize, #[deref] staff: &'t Staff }
				trait Index { fn index(&self, index: &music_xml::Staff) -> StaffRef; }
				impl Index for [Staff] {
					fn index(&self, index: &music_xml::Staff) -> StaffRef { let index = index.into(); StaffRef{index, staff: &self[index]} }
				}
				#[derive(Deref, DerefMut)] struct StaffMut<'t> { index: usize, #[deref]#[deref_mut] staff: &'t mut Staff }
				trait IndexMut { fn index_mut(&mut self, index: &music_xml::Staff) -> StaffMut; }
				//impl<N: usize> IndexMut for &mut [Staff; N] {
				impl IndexMut for [Staff] {
					fn index_mut(&mut self, index: &music_xml::Staff) -> StaffMut { let index = index.into(); StaffMut{index, staff: &mut self[index]} }
				}
				impl StaffMut<'_> { fn as_ref(&self) -> StaffRef { StaffRef{index: self.index, staff: &self.staff} } }

				impl Pitch {
					fn new(clef: &Clef, step: &Step) -> Self {
						use Step::*;
						match clef {
							Clef{sign: ClefSign::G,..} => Pitch{step: *step, octave: Some(match step { G|A|B => 4, C|D|E|F => 5 }), alter: None},
							Clef{sign: ClefSign::F,..} => Pitch{step: *step, octave: Some(match step { A|B => 2, C|D|E|F|G => 3 }), alter: None},
						}
					}
				}

				impl Score {
					fn x(&self) -> i32 { self.glyph.last().map(|g:&Glyph| g.top_left.x+font.glyph_hor_advance(g.id).unwrap() as i32).unwrap_or(0) }
					fn push(&mut self, x: i32, staff_index: usize, step: i8, id: ttf_parser::GlyphId) {
						self.glyph.push(Glyph{top_left: xy{
							x: x + font.glyph_hor_side_bearing(id).unwrap() as i32,
							y: self.y(staff_index, step) - font.glyph_bounding_box(id).unwrap().y_max as i32,
						}, id})
					}
					fn pitch(&mut self, x:  i32, staff: StaffRef, pitch: &Pitch, id: char) {
						impl Staff { #[allow(non_snake_case)] fn C4(&self) -> i8 {
							use ClefSign::*; (match self.clef.as_ref().unwrap().sign { G=> -10, F=> 2 }) - self.octave*7 } } // -8: bottom .. 0: top
						impl From<&Step> for i8 { fn from(step: &Step) -> Self { use Step::*; match step { C=>0, D=>1, E=>2, F=>3, G=>4, A=>5, B=>6 } } }
						let step = Staff::C4(&staff) + (pitch.octave.unwrap_or(4) as i8 - 4)*7 + i8::from(&pitch.step);
						let id = font.glyph_index(id).unwrap();
						self.push(x, staff.index, step, id)
					}
				}

				use SMuFL::*;
				use MusicData::*;
				match music_data {
					Note(music_xml::Note{staff: Some(staff), r#type: Some(NoteType{value}), content:NoteData::Pitch(pitch), ..}) => {
						score.pitch(score.x(), staves.index(staff), pitch, {use {NoteTypeValue::*, note_head::*}; match value { Breve=>breve, Whole=>whole, Half=>half, _=>black }});
					},
					Attributes(music_xml::Attributes{clefs, key, time, ..}) => {
						let x = score.x();
						for clef@Clef{staff, sign, ..} in clefs {
							let mut staff = staves.index_mut(staff);
							staff.clef = Some(*clef);
							let (id, step) = {use ClefSign::*; match sign { G=>(clef::G, Step::G), F=>(clef::F, Step::F) }};
							score.pitch(x, staff.as_ref(), &Pitch::new(clef, &step), id);
						}
						let x = score.x();
						if let Some(Key{fifths,..}) = key {
							let mut key = |fifths:i8, symbol| {
								let mut sign = |steps: &mut dyn Iterator<Item=&Step>| {
									for step in steps {
										for (index, Staff{clef, ..}) in staves.iter().enumerate() {
											score.pitch(x, StaffRef{index, staff: &Staff{clef: *clef, octave: 0}}, &Pitch::new(clef.as_ref().unwrap(), step), symbol);
										}
									}
								};
								let steps = {use Step::*; [B,E,A,D,G,C,F].iter()};
								if fifths>0 { sign(&mut steps.rev().take(fifths as usize)) } else { sign(&mut steps.take((-fifths) as usize)) }
							};
							//if fifths == 0 { key(system.fifths, accidental::natural) } else
							key(*fifths, if *fifths<0 { accidental::flat } else { accidental::sharp });
						}
						let x = score.x();
						if let Some(Time{beats, beat_type}) = time {
							fn time_signature_digit(digit: char) -> char { use std::convert::TryInto; u32::try_into(u32::from(time_signature)+digit.to_digit(10).unwrap()).unwrap() }
							let texts : [String; 2] = framework::array::Iterator::collect(
								[beats, beat_type].iter().map(|number| number.to_string().chars().map(time_signature_digit).collect::<String>())
							);
							let texts : [Text; 2] = framework::array::Iterator::collect(
								texts.as_ref().iter().map(|text| Text::new(&font, text, &*default_style))
							);
							let width = texts.iter().map(|text| text.size().x).max().unwrap();
							use framework::array::IntoIterator;
							for (text, step) in texts.iter().zip([-2,-6].into_iter()) {
								let x = x + ((width-text.size().x)/2) as i32;
								for text::Layout{x: dx, glyph: (_, id), ..} in text.font.glyphs(text.text.char_indices()).layout() {
									for index in 0..staves.len() {
										score.push(x + dx, index, step, id);
									}
								}
							}
						}
					},
					_ => (),
				}
			}
		}
	}
	Graphic{scale: Ratio{num: 360, div: score.staff_height}, fill, font: &font, glyph: score.glyph}
}

#[throws] fn main() {
	framework::rstack_self()?; framework::sigint_trace();
	window::run(&mut GraphicView::new(layout(&xml::from_document(&xml::parse(&std::fs::read("../test.xml")?)?)?, i32::MAX)))?
}
