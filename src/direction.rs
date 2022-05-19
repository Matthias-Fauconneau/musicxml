use {vector::xy, fehler::throws, crate::Error,
	crate::music_xml::{Direction, DirectionType, UpDownStopContinue},
	crate::{font::SMuFL::EngravingDefaults, staff::{Staff, IndexMut},measure::MeasureLayoutContext}};
impl MeasureLayoutContext<'_> {
	#[throws] pub fn direction(&mut self, staves: &mut [Staff], Direction{direction, staff, ..}: &Direction) { match direction {
		DirectionType::Dynamics(dynamic) => {
			let face = ui::text::default_font()[0]; // TODO: italic
			use ui::{graphic, text::{Plain, View, layout, Glyph, unicode_segmentation::UnicodeSegmentation}};
			let text = View::new_with_face(&face, Plain(dynamic.to_string()));
			for Glyph{x: dx, id, ..} in layout(&text.font, text.data.0.graphemes(true).enumerate()) {
				let scale = num::Ratio{num: 1, div: 3};
				assert!(face.glyph_bounding_box(id).is_some(), "{dynamic:?}");
				self.measure.graphic.glyphs.push(graphic::Glyph{top_left: xy{
					x: (self.x+dx) as i32 + face.glyph_hor_side_bearing(id).unwrap() as i32,
					y: -((self.sheet.staff_height/*+self.staff_distance/2*/) as i32),
				}, face, id, scale: scale.into()});
			}
		},
		DirectionType::Metronome{..} => {

		},
		DirectionType::Wedge(_) => {
		},
		DirectionType::OctaveShift{r#type, size, ..} => {
			let mut staff = staves.index_mut(&staff.unwrap());
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
					let line = ui::graphic::horizontal(-((self.sheet.staff_height+self.staff_distance/3) as i32), staff_line_thickness, staff.octave_start_x.unwrap() as i32, self.x as i32);
					self.graphic.rects.push(line);
				}
				_ => unimplemented!("{direction:?}")
			}
		},
		_ => panic!("{direction:?}")
	}
}}
