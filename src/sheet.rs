#![allow(non_upper_case_globals)]
use {derive_more::Deref, ttf_parser::Face, crate::{Font, font::{SMuFont, SMuFL::EngravingDefaults}}, ::xy::{xy,Rect}};

#[derive(Deref)] 
pub struct Sheet<'f, 'r, 't> {
	pub font: &'f Font,
	#[deref] pub face: &'r Face<'t>,
	pub engraving_defaults: EngravingDefaults, 
	pub staff_height: u32, 
	pub staff_distance: u32 
}
impl<'f, 'r, 't> Sheet<'f, 'r, 't> {
	pub fn new_with_face(font: &'f Font, face: &'r Face<'t>) -> Self {
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
	pub fn new(font: &'f Font) -> Self {
		static face: std::lazy::SyncLazy<ui::font::File<'static>> = std::lazy::SyncLazy::new(|| ui::font::open(std::path::Path::new(&(std::env::var("HOME").unwrap()+"/.local/share/fonts/Bravura.otf"))).unwrap());
	    Self::new_with_face(font, &face)
    }
	// staff: 0: bass .. 1: treble; step: -8: bottom .. 0: top
	pub fn y(&self, staff: usize, step: i8) -> i32 { - ((staff as u32 * self.staff_distance) as i32) - step as i32 * (self.staff_height/8) as i32 }
	pub fn raster(&'t self, staves: impl Iterator + 't) -> impl Iterator<Item=Rect> + 't {
		pub fn horizontal(y: i32, dy: u8, x0: i32, x1: i32) -> Rect { Rect{ min: xy{ y: y-(dy/2) as i32, x: x0 }, max: xy{ y: y+(dy/2) as i32, x: x1 } } }
		staves.enumerate().map(move |(staff, _)|
			(0..=8).step_by(2).map(move |step| horizontal(self.y(staff, step), self.engraving_defaults.staff_line_thickness, 0, i32::MAX))
		).flatten()
	}
}