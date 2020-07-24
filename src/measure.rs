use {derive_more::{Deref, DerefMut}, framework::{vector::xy, graphic::{Graphic, Glyph}}};
use crate::{sheet::Sheet, music_xml::Pitch, staff::StaffRef, music::BeamedMusicData};

#[derive(Deref)] pub struct Measure<'t> { #[deref] pub sheet: &'t Sheet<'t>, pub graphic: Graphic<'t> }
impl<'t> Measure<'t> {
	fn new(sheet: &'t Sheet) -> Self { Self{sheet: &sheet, graphic: Graphic::new(Default::default(), sheet.font)} }
	fn last_advance(&self) -> i32 { self.graphic.glyphs.last().map(|g:&Glyph| g.top_left.x+self.sheet.font.glyph_hor_advance(g.id).unwrap() as i32).unwrap_or(0) }

	pub fn push_glyph_id(&mut self, x: i32, staff_index: usize, step: i8, dy: i32, id: ttf_parser::GlyphId) {
		self.graphic.glyphs.push(Glyph{top_left: xy{
			x: x + self.sheet.font.glyph_hor_side_bearing(id).unwrap() as i32,
			y: self.sheet.y(staff_index, step) - self.sheet.font.glyph_bounding_box(id).unwrap().y_max as i32 + dy,
		}, id})
	}
	pub fn push_glyph(&mut self, x:  i32, staff_index: usize, step: i8, dy: i32, id: char) {
		self.push_glyph_id(x, staff_index, step, dy, self.sheet.font.glyph_index(id).unwrap())
	}
	pub fn push_glyph_at_pitch(&mut self, x:  i32, staff: StaffRef, pitch: &Pitch, id: char) {
		self.push_glyph(x, staff.index, staff.step(pitch), 0, id)
	}
}

#[derive(Deref, DerefMut)] pub struct MeasureLayoutContext<'t> { #[deref]#[deref_mut] pub measure: Measure<'t>, t: u32, pub x: u32}
impl<'t> MeasureLayoutContext<'t> {
	pub fn new(sheet: &'t Sheet) -> Self { Self{measure: Measure::new(sheet), t: 0, x: 0} }
	pub fn space(&self) -> u32 { self.measure.sheet.staff_height / 4 }
	pub fn advance(&mut self, space: u32) { self.x = self.measure.last_advance() as u32 + space; }
}

#[derive(Deref, DerefMut)] pub struct MusicLayoutContext<'t, I> { pub music_data: I, #[deref]#[deref_mut] pub layout_context: MeasureLayoutContext<'t> }
//impl<'t, I> MusicLayoutContext<'t, I> { pub fn new(music_data: I, layout_context: MeasureLayoutContext<'t>) -> Self { Self{music_data, layout_context} } }
impl<'t, I:Iterator<Item=(u32, BeamedMusicData<'t>)>> Iterator for MusicLayoutContext<'_, I> {
	type Item = (u32, u32, BeamedMusicData<'t>);
	fn next(&mut self) -> Option<Self::Item> {
		self.music_data.next().map(|(t, e)| { // Advances horizonal position as measure is constructed
			if t > self.t { let space = self.space(); self.advance(space); }
			self.t = t;
			(t, self.x, e)
		})
	}
}
