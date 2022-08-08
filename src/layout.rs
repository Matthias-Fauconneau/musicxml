use {vector::{xy, size}, ui::{Ratio, Graphic, graphic::vertical}};
use crate::{music_xml::{Measure, MusicData::*}, sheet::Sheet, staff::Staff, music::*};
use crate::{measure::{MeasureLayoutContext,MusicLayoutContext}, font::SMuFL::EngravingDefaults};
pub fn layout<'g>(measures: &[Measure], size: size) -> Graphic<'g> {
	let sheet = Sheet::new();
	let EngravingDefaults{thin_barline_thickness, ..} = sheet.engraving_defaults;
	let scale = Ratio{num: 240, div: sheet.staff_height};
	let mut staves = <[Staff; 2]>::default();
	let mut graphic = Graphic::new(scale);
	graphic.rects.extend(sheet.raster(staves.len(), size.x/scale));
	let mut position = xy{x:0,y:0};
	for measure in measures {
		let music_data = sort_by_start_time(measure.iter());
		let music_data = batch_beamed_group_of_notes(music_data);
		let mut measure = MusicLayoutContext{music_data, layout_context: MeasureLayoutContext::new(&sheet)};
		while let Some((_, _, music_data)) = measure.next() {
			use BeamedMusicData::{Beam, MusicData};
			match music_data {
				Beam(beam) => measure.beam(&staves, &beam),
				MusicData(music_data) => match music_data {
					Backup(_) => {},
					Attributes(attributes) => measure.attributes(&mut staves, attributes),
					Direction(direction) => measure.direction(&mut staves, direction).unwrap(),
					_ => {},
				}
			}
		}
		let mut measure = measure.layout_context;
		let space = measure.space();
		if scale*(position.x + measure.x) > size.x {
			position.x = 0;
			position.y += 2*sheet.staff_distance;
			graphic.rects.extend(sheet.raster(staves.len(), size.x/scale).map(|mut x| { x.translate(xy{x:0, y: position.y as i32}); x }));
			graphic.rects.push(vertical(
				size.x as i32,
				thin_barline_thickness,
				position.y as i32+sheet.y(staves.len()-1, 8),
				position.y as i32+sheet.y(0, 0)
			));
		} else {
			measure.advance(space / 2);
		};
		graphic.rects.extend(measure.graphic.rects.drain(..).map(|mut x| { x.translate(position.signed()); x }));
		graphic.parallelograms.extend(measure.graphic.parallelograms.drain(..).map(|mut x| { x.translate(position.signed()); x }));
		graphic.glyphs.extend(measure.graphic.glyphs.drain(..).map(|mut x| { x.translate(position.signed()); x }));
		if position.x > 0 {
			graphic.rects.push(vertical(
				(position.x - (space / 2)) as i32,
				thin_barline_thickness,
				position.y as i32+sheet.y(staves.len()-1, 8),
				position.y as i32+sheet.y(0, 0)
			));
		}
		position.x += measure.x + (space / 2);
		break;
	}
	graphic
}
