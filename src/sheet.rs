#![allow(non_upper_case_globals)]
use {derive_more::Deref, ttf_parser::Font, crate::font::{SMuFont, SMuFL::EngravingDefaults}, framework::graphic::Rect};

#[derive(Deref)] pub struct Sheet<'t> { #[deref] pub font: &'t Font<'t>, pub engraving_defaults: EngravingDefaults, pub staff_height: u32, pub staff_distance: u32 }
impl<'t> Sheet<'t> {
	fn new(font: &'t Font<'t>) -> Self {
		let staff_height = font.units_per_em().unwrap() as u32;
		let interval = staff_height / 4;
		Sheet{
			font,
			engraving_defaults: Font::engraving_defaults(),
			staff_height,
			staff_distance: 7*interval
		}
	}
	// staff: 0: bass .. 1: treble; step: -8: bottom .. 0: top
	pub fn y(&self, staff: usize, step: i8) -> i32 { - ((staff as u32 * self.staff_distance) as i32) - step as i32 * (self.staff_height/8) as i32 }
	pub fn raster(&'t self, staves: impl Iterator + 't) -> impl Iterator<Item=Rect> + 't {
		staves.enumerate().map(move |(staff, _)|
			(0..=8).step_by(2).map(move |step| Rect::horizontal(self.y(staff, step), self.engraving_defaults.staff_line_thickness, 0, i32::MAX))
		).flatten()
	}
}

impl Default for Sheet<'_> {
    fn default() -> Self {
	    lazy_static::lazy_static! {
		    static ref font: framework::font::File<'static> = framework::font::open(std::path::Path::new("/usr/local/share/fonts/bravura/Bravura.otf")).unwrap();
	    }
	    Self::new(&font)
    }
}
