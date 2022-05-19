#![allow(non_upper_case_globals)]
use {derive_more::Deref, ttf_parser::Face, crate::{Font, font::{SMuFont, SMuFL::EngravingDefaults}}, vector::Rect};

#[derive(Deref)]
pub struct Sheet {
	pub font: Font,
	#[deref] pub face: &'static Face<'static>,
	pub engraving_defaults: EngravingDefaults,
	pub staff_height: u32,
	pub staff_distance: u32
}
impl Sheet {
	pub fn new_with_face(font: Font, face: &'static Face) -> Self {
		let staff_height = face.units_per_em() as u32;
		let interval = staff_height / 4;
		Sheet{
			font,
			face,
			engraving_defaults: Face::engraving_defaults(),
			staff_height,
			staff_distance: 7*interval
		}
	}
	pub fn new(font: Font) -> Self {
		static face: std::lazy::SyncLazy<ui::font::File<'static>> = std::lazy::SyncLazy::new(|| ui::font::open(std::path::Path::new(&(std::env::var("HOME").unwrap()+"/.local/share/fonts/Bravura.otf"))).unwrap());
	    Self::new_with_face(font, &face)
    }
	// staff: 0: bass .. 1: treble; step: -8: bottom .. 0: top
	pub fn y(&self, staff: usize, step: i8) -> i32 { - ((staff as u32 * self.staff_distance) as i32) - step as i32 * (self.staff_height/8) as i32 }
	pub fn raster<'t>(&'t self, staves: impl Iterator + 't) -> impl Iterator<Item=Rect> + 't {
		staves.enumerate().map(move |(staff, _)|
			(0..=8).step_by(2).map(move |step| ui::graphic::horizontal(self.y(staff, step), self.engraving_defaults.staff_line_thickness, 0, i32::MAX))
		).flatten()
	}
}