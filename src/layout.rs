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
		//fn debug<T: std::fmt::Display>(t: T) -> T { println!("{t}"); t }
		fn debug<T>(t: T, f: impl Fn(&T, &dyn Fn(&dyn std::fmt::Display))) -> T { f(&t, &|u| println!("{}", u)); t }
		use itertools::Itertools;
		//fn debug<T: std::fmt::Display>(t: Box<[T]>) -> Box<[T]> { f(t.iter().format(" ")) }
		//fn debug<T: std::fmt::Display>(t: Vec<Vec<T>>) -> Vec<Vec<T>> { f(t.iter().format_with("|", |e,f| f(&e.iter().format(" ")))) }
		let music_data = debug(sort_by_start_time(measure.iter()), |t,f| {  f(&t.iter().map(|(_,d)| d).format(" ")) });
		let music_data = batch_beamed_group_of_notes(music_data.iter().copied());
		let mut measure = MusicLayoutContext{music_data, layout_context: MeasureLayoutContext::new(&sheet)};
		while let Some((_, _, music_data)) = measure.next() {
			use BeamedMusicData::{Beam, MusicData};
			match music_data {
				Beam(beam) => measure.beam(&staves, &debug(beam, |t,f| f(&t.iter().format_with("|", |e,f| f(&e.iter().format(" ")))))),
				MusicData(music_data) => match music_data {
					Backup(_) => {},
					Attributes(attributes) => measure.attributes(&mut staves, attributes),
					Direction(direction) => { measure.direction(&mut staves, direction); },
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
