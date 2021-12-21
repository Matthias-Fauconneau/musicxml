use {xy::xy, fehler::throws, crate::Error,
	crate::music_xml::{Font, PrintStyle, Direction, DirectionType, DirectionTypeData, Dynamics, Metronome, Wedge, OctaveShift}, 
	crate::{measure::MeasureLayoutContext, staff::Staff}};
impl MeasureLayoutContext<'_> { 
	#[throws] pub fn direction(&mut self, _staves: &mut [Staff], Direction{direction_type, ..}: &Direction) {
		for DirectionType{content} in direction_type {
			for direction in content { match direction {
				DirectionTypeData::Dynamics(Dynamics{text, print_style: PrintStyle{font: Font{font_family: Some(face), ..}, ..}}) => {
					let text = format!("{text:?}");
					let font = self.measure.sheet.font;
					let face = if let Some((_,face)) = font.iter().find(|(key,_)| key==face) { face } else {
						/*use std::{path::Path,env::var}
						font.push((face.clone(), ui::font::open(Path::new(&(var("HOME")?+"/.local/share/fonts/BravuraText.otf")))?));
						&font.iter().find(|(key,_)| key==face).unwrap().1*/
						ui::text::default_font()[0]
					};
					use ui::{graphic, text::{Plain, View, layout, Glyph, unicode_segmentation::UnicodeSegmentation}};
					let text = View::new_with_face(&face, Plain(text));
					for Glyph{x: dx, id, ..} in layout(&text.font, text.data.0.graphemes(true).enumerate()) {
						self.measure.graphic.glyphs.push(graphic::Glyph{top_left: xy{
							x: (self.x+dx) as i32 + face.glyph_hor_side_bearing(id).unwrap() as i32,
							y: (self.sheet.staff_height+self.staff_distance/2) as i32 - face.glyph_bounding_box(id).unwrap().y_max as i32,
						}, face, id});
					}
				},
				DirectionTypeData::Metronome(Metronome{..}) => {

				},
				DirectionTypeData::Wedge(Wedge{..}) => {
				},
				DirectionTypeData::OctaveShift(OctaveShift{..}) => {
				},
				_ => panic!("{direction:?}")
			}}	
		}
	}
}
