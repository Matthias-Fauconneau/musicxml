use {vector::xy, fehler::throws,
	crate::music_xml::{Direction, DirectionType, UpDownStopContinue},
	crate::{font::SMuFL::EngravingDefaults, staff::{Staff, IndexMut},measure::MeasureLayoutContext}};
impl MeasureLayoutContext<'_,'_> {
	#[throws(as Option)] pub fn direction(&mut self, staves: &mut [Staff], Direction{direction, staff, ..}: &Direction) { use DirectionType::*; match direction.as_ref()? {
		Dynamics(dynamic) => {
			let face = ui::text::default_font()[0]; // TODO: italic
			use ui::{graphic, text::{Plain, View, layout, Glyph}};
			let text = View::with_face(&face, Plain(dynamic.to_string()));
			for Glyph{x: dx, id, ..} in layout(&text.font, text.data.as_ref()) {
				let scale = num::Ratio{num: 1, div: 3};
				assert!(face.glyph_bounding_box(id).is_some(), "{dynamic:?}");
				self.measure.graphic.glyphs.push(graphic::Glyph{top_left: xy{
					x: (self.x+scale*dx) as i32 + face.glyph_hor_side_bearing(id).unwrap() as i32,
					y: self.y(0, 12),
				}, face, id, scale, style: 1.});
			}
		},
		OctaveShift{r#type, size, ..} => {
			let mut staff = staves.index_mut(staff.unwrap());
			match r#type {
				direction@(UpDownStopContinue::Down|UpDownStopContinue::Up) => {
					staff.octave = match direction {
						UpDownStopContinue::Down => -1,
						UpDownStopContinue::Up => 1,
						_ => unreachable!(),
					} * ((size-1)/7) as i8;
					staff.octave_start_x = Some(self.x);
				},
				UpDownStopContinue::Stop => {
					let EngravingDefaults{staff_line_thickness, ..} = self.engraving_defaults;
					self.measure.graphic.horizontal(-((self.staff_height+self.staff_distance/3) as i32), staff_line_thickness, staff.octave_start_x.unwrap() as i32, self.x as i32, 1.);
				}
				_ => unimplemented!("{direction:?}")
			}
		},
		Metronome{..} => {},
		Wedge(_) => {},
		Words(_) => {},
	}
}}
