use {derive_more::Deref, vector::Rect, ui::font::Face, crate::font::{SMuFont, SMuFL::EngravingDefaults}};

#[derive(Deref)]
pub struct Sheet {
	#[deref] pub face: &'static Face<'static>,
	pub engraving_defaults: EngravingDefaults,
	pub staff_height: u32,
	pub staff_distance: u32
}
impl Sheet {
	pub fn new_with_face(face: &'static Face) -> Self {
		let staff_height = face.units_per_em() as u32;
		let interval = staff_height / 4;
		Sheet{
			face,
			engraving_defaults: Face::engraving_defaults(),
			staff_height,
			staff_distance: 7*interval
		}
	}
	pub fn new() -> Self {
		#[allow(non_upper_case_globals)] static face: std::sync::LazyLock<ui::font::File<'static>> = std::sync::LazyLock::new(|| ui::font::open(std::path::Path::new(&"/usr/share/fonts/OTF/Bravura.otf")).unwrap());
		Self::new_with_face(&face)
    }
	// staff: 0: bass .. 1: treble; step: 0: bottom .. 8: top
	pub fn y(&self, staff: usize, step: i8) -> i32 { - ((staff as u32 * self.staff_distance) as i32) - step as i32 * (self.staff_height/8) as i32 }
	pub fn raster(&self, staves: usize, x1: u32) -> impl Iterator<Item=(Rect,f32)>+'_ {
		(0..staves).map(move |staff|
			(0..=8).step_by(2).map(move |step| (ui::graphic::horizontal(self.y(staff, step), self.engraving_defaults.staff_line_thickness, 0, x1 as i32), 1.))
		).flatten()
	}
}