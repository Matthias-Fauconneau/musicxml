use vector::xy;
use music::{Harmony, staff::Staff};
use crate::measure::MeasureLayoutContext;

impl MeasureLayoutContext<'_,'_> {
	pub fn harmony(&mut self, _: &mut [Staff], harmony/*Harmony{step, alter}*/: &Harmony) {
		let face = ui::text::default_font()[0]; // TODO: italic
		use ui::{graphic, text::{Plain, View, layout, Glyph}};
		let text = View::with_face(&face, Plain(harmony.to_string()));
		for Glyph{x: dx, id, ..} in layout(&text.font, text.data.as_ref()) {
			let scale = num::Ratio{num: 1, div: 3};
			self.measure.graphic.glyphs.push(graphic::Glyph{top_left: xy{
				x: (self.x+scale*dx) as i32 + face.glyph_hor_side_bearing(id).unwrap() as i32,
				y: self.y(1, 12)/*+ face.glyph_hor_side_bearing(id).unwrap() as i32*/,
			}, face, id, scale, style: 1.});
		}
	}
}