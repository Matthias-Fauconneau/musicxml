use {derive_more::{Deref, DerefMut}, vector::xy, ui::graphic::{GlyphId, Glyph, Graphic}};
use crate::{sheet::Sheet, music_xml::Pitch, staff::StaffRef, music::BeamedMusicData};

#[derive(Deref)] pub struct Measure<'s,'g> { #[deref] pub sheet: &'s Sheet, pub graphic: Graphic<'g> }
impl<'s> Measure<'s,'_> {
	fn new(sheet: &'s Sheet) -> Self { Self{sheet, graphic: Graphic::new(num::Ratio{num:0,div:0})} }
	fn last_advance(&self) -> i32 { self.graphic.glyphs.iter().map(|g:&Glyph| g.top_left.x + g.face.glyph_hor_advance(g.id).unwrap() as i32).max().unwrap_or(0) }
	#[track_caller] pub fn push_glyph_id(&mut self, x: u32, staff_index: usize, step: i8, dy: i32, id: GlyphId, style: f32) {
		self.graphic.glyphs.push(Glyph{top_left: xy{
			x: x as i32 + self.sheet.face.glyph_hor_side_bearing(id).unwrap() as i32,
			y: self.sheet.y(staff_index, step) - self.sheet.face.glyph_bounding_box(id).unwrap().y_max as i32 + dy,
		}, face: self.sheet.face, id, scale: num::unit, style})
	}
	pub fn push_glyph(&mut self, x: u32, staff_index: usize, step: i8, dy: i32, id: char, style: f32) {
		self.push_glyph_id(x, staff_index, step, dy, self.sheet.face.glyph_index(id).unwrap(), style)
	}
	pub fn push_glyph_at_pitch(&mut self, x: u32, staff: StaffRef, pitch: &Pitch, id: char, style: f32) {
		self.push_glyph(x, staff.index, staff.step(pitch), 0, id, style)
	}
}

#[derive(Deref, DerefMut)] pub struct MeasureLayoutContext<'s,'g> { #[deref]#[deref_mut] pub measure: Measure<'s,'g>, t: u32, pub x: u32}
impl<'t> MeasureLayoutContext<'t,'_> {
	pub fn new(sheet: &'t Sheet) -> Self { Self{measure: Measure::new(sheet), t: 0, x: 0} }
}
impl MeasureLayoutContext<'_,'_> {
	pub fn space(&self) -> u32 { self.measure.sheet.staff_height / 2 }
	pub fn advance(&mut self, space: u32) { self.x = self.measure.last_advance() as u32 + space; }
}

#[derive(Deref, DerefMut)] pub struct MusicLayoutContext<'t, 'g, I> { pub music_data: I, #[deref]#[deref_mut] pub layout_context: MeasureLayoutContext<'t,'g> }
impl<'t, I:Iterator<Item=(u32, BeamedMusicData<'t>)>> Iterator for MusicLayoutContext<'_, '_, I> {
	type Item = (u32, u32, BeamedMusicData<'t>);
	fn next(&mut self) -> Option<Self::Item> {
		self.music_data.next().map(|(t, e)| { // Advances horizonal position as measure is constructed
			if t > self.t { let space = self.space(); self.advance(space); }
			self.t = t;
			(t, self.x, e)
		})
	}
}
