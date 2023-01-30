use {vector::{xy, size}, ui::{Ratio, Graphic}};
use crate::{music_xml::{self, MusicData::*}, sheet::Sheet, staff::Staff, music::*, measure::Measure};
use crate::{measure::{MeasureLayoutContext,MusicLayoutContext}, font::SMuFL::EngravingDefaults};
pub fn layout<'t, 'g>(measures: &'t [music_xml::Measure], size: size) -> Graphic<'g> {
	let sheet = Sheet::new();
	let EngravingDefaults{thin_barline_thickness, ..} = sheet.engraving_defaults;
	let scale = Ratio{num: 240, div: sheet.staff_height};
	let mut staves = <[Staff; 2]>::default();
	let mut graphic = Graphic::new(scale);
	graphic.rects.extend(sheet.raster(staves.len(), size.x/scale));
	let mut position = xy{x:0,y:0};
	for measure in &measures[..2] {
		fn debug<T>(t: T, f: impl Fn(&T, &dyn Fn(&dyn std::fmt::Display))) -> T { f(&t, &|u| println!("{}", u)); t } use itertools::Itertools;
		let music_data = sort_by_start_time(measure.iter());
		//let music_data = debug(sort_by_start_time(measure.iter()), |t,f| {  f(&t.iter().map(|(_,d)| d).format(" ")) });
		let music_data = beam(music_data.iter().copied());
		let mut measure = MusicLayoutContext{music_data, layout_context: MeasureLayoutContext::new(&sheet)};
		while let Some((_, _, music_data)) = measure.next() {
			use BeamedMusicData::{Beam, MusicData};
			match music_data {
				//Beam(beam) => measure.beam(&staves, &beam),
				Beam(beam) => measure.beam(&mut staves, &debug(beam, |t,f| f(&t.iter().format_with("|", |e,f| f(&e.iter().format(" ")))))),
				MusicData(music_data) => match music_data {
					//Note(note) => measure.beam(&staves, &[vec![note]]),
					Backup(_) => {},
					Attributes(attributes) => measure.attributes(&mut staves, attributes),
					Direction(direction) => { measure.direction(&mut staves, direction); },
					_ => unimplemented!(),
				}
			}
		}
		let mut measure = measure.layout_context;
		let space = measure.space();
		if scale*(position.x + measure.x) > size.x {
			position.x = 0;
			position.y += 2*sheet.staff_distance;
			graphic.rects.extend(sheet.raster(staves.len(), size.x/scale).map(|(mut x, style)| { x.translate(xy{x:0, y: position.y as i32}); (x, style) }));
			graphic.vertical(size.x as i32, thin_barline_thickness, position.y as i32+sheet.y(staves.len()-1, 8), position.y as i32+sheet.y(0, 0), 1.);
		} else {
			measure.advance(space / 2);
		};
		let MeasureLayoutContext{measure: Measure{graphic: measure_graphic, ..}, x: measure_x, ..} = measure;
		graphic.extend(measure_graphic, position.signed());
		if position.x > 0 { graphic.vertical((position.x - (space / 2)) as i32, thin_barline_thickness, position.y as i32+sheet.y(staves.len()-1, 8), position.y as i32+sheet.y(0, 0), 1.); }
		position.x += measure_x + (space / 2);
	}
	graphic
}
