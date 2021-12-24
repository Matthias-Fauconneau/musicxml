use {xy::xy, fehler::throws, crate::Error,
	crate::music_xml::{Font, PrintStyle, Direction, DirectionType, DirectionTypeData, Dynamics, Metronome, Wedge, OctaveShift, UpDownStopContinue}, 
	crate::{font::SMuFL::EngravingDefaults, staff::{Staff, IndexMut},measure::MeasureLayoutContext}};
impl MeasureLayoutContext<'_> { 
	#[throws] pub fn direction(&mut self, staves: &mut [Staff], Direction{direction_type, staff, ..}: &Direction) {
		for DirectionType{content} in direction_type {
			for direction in content { match direction {
				DirectionTypeData::Dynamics(Dynamics{text, print_style: PrintStyle{font: Font{font_family: Some(face), ..}, ..}}) => {
					let text = format!("{text:?}");
					let font = self.measure.sheet.font;
					let face = if let Some((_,face)) = font.iter().find(|(key,_)| key==face) { face } else {
						/*use std::{path::Path,env::var}
						font.push((face.clone(), ui::font::open(Path::new(&(var("HOME")?+"/.local/share/fonts/BravuraText.otf")))?));
						&font.iter().find(|(key,_)| key==face).unwrap().1*/
						ui::text::default_font()[0] // TODO: italic
					};
					use ui::{graphic, text::{Plain, View, layout, Glyph, unicode_segmentation::UnicodeSegmentation}};
					let text = View::new_with_face(&face, Plain(text));
					for Glyph{x: dx, id, ..} in layout(&text.font, text.data.0.graphemes(true).enumerate()) {
						let scale = num::Ratio{num: 1, div: 3};
						self.measure.graphic.glyphs.push(graphic::Glyph{top_left: xy{
							x: (self.x+dx) as i32 + face.glyph_hor_side_bearing(id).unwrap() as i32,
							y: -((self.sheet.staff_height/*+self.staff_distance/2*/) as i32),
						}, face, id, scale: scale.into()});
					}
				},
				DirectionTypeData::Metronome(Metronome{..}) => {

				},
				DirectionTypeData::Wedge(Wedge{..}) => {
				},
				DirectionTypeData::OctaveShift(OctaveShift{r#type, size, ..}) => {
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
			}}	
		}
	}
}
