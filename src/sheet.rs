#![allow(non_upper_case_globals)]
use {derive_more::Deref, ttf_parser::Face, crate::font::{SMuFont, SMuFL::EngravingDefaults}, ::xy::{xy,Rect}};

#[derive(Deref)] pub struct Sheet<'t> { #[deref] pub font: &'t Face<'t>, pub engraving_defaults: EngravingDefaults, pub staff_height: u32, pub staff_distance: u32 }
impl<'t> Sheet<'t> {
	fn new(font: &'t Face<'t>) -> Self {
		let staff_height = font.units_per_em() as u32;
		let interval = staff_height / 4;
		Sheet{
			font,
			engraving_defaults: Face::engraving_defaults(),
			staff_height,
			staff_distance: 7*interval
		}
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

impl Default for Sheet<'_> {
    fn default() -> Self {
	    lazy_static::lazy_static! {
		    static ref font: ui::font::File<'static> = ui::font::open(std::path::Path::new("/usr/local/share/fonts/bravura/Bravura.otf")).unwrap();
	    }
	    Self::new(&font)
    }
}
