use {::xy::{size, xy}, ui::graphic::{Ratio, Rect, Graphic}, crate::music_xml::{self, MusicXML}};
pub fn layout(music: &MusicXML, size: size) -> Graphic<'static> {
	use crate::{sheet::Sheet, staff::Staff, music::*, measure::{MeasureLayoutContext,MusicLayoutContext}};
	let sheet = Sheet::default();
	let scale = Ratio{num: 240, div: sheet.staff_height};
	let size = size / scale;
	let mut staves = <[Staff; 2]>::default(); // : [Staff; 2] = /*[Staff::default; 2]*/array::Iterator::collect(std::iter::from_fn(|| Some(Staff::default())));
	let mut graphic = Graphic::new(scale, &sheet.font);
	for part in &music.score_partwise.parts {
		let mut system = xy{x: 0, y: 0};
		graphic.rects.extend(sheet.raster(staves.iter()));
		for measure in &part.measures {
			let music_data = sort_by_start_time(&measure.music_data);
			let music_data = batch_beamed_group_of_notes(music_data);
			let mut measure = MusicLayoutContext{music_data, layout_context: MeasureLayoutContext::new(&sheet)}; //::new(music_data, measure);
			while let Some((t, x, music_data)) = measure.next() {
				println!("{} {} {:?}", t, x, music_data);
				use {BeamedMusicData::{Beam, MusicData}, music_xml::MusicData::*};
			    match music_data {
				    Beam(beam) => measure.beam(&staves, &beam),
				    MusicData(music_data) => match music_data {
					    Backup(_) => {},
					    Attributes(attributes) => measure.attributes(&mut staves, attributes),
					    _ => {},
				    }
			    }
			}
			let mut measure = measure.layout_context;
			let space = measure.space();
			measure.advance(space / 2);
			if system.x + measure.x > size.x {
				system.x = 0;
				system.y += 2*sheet.staff_distance;
				graphic.rects.extend(sheet.raster(staves.iter()).map(|mut x| { x.translate(xy{x:0, y: system.y as i32}); x }));
			}
			graphic.rects.extend(measure.graphic.rects.drain(..).map(|mut x| { x.translate(system.signed()); x }));
			graphic.parallelograms.extend(measure.graphic.parallelograms.drain(..).map(|mut x| { x.translate(system.signed()); x }));
			graphic.glyphs.extend(measure.graphic.glyphs.drain(..).map(|mut x| { x.translate(system.signed()); x }));
			if system.x > 0 {
				pub fn vertical(x: i32, dx: u8, y0: i32, y1: i32) -> Rect { Rect{ min: xy{ x: x-(dx/2) as i32, y: y0 }, max: xy{ x: x+(dx/2) as i32, y: y1 } } }
				graphic.rects.push(vertical(
					(system.x - (space / 2)) as i32,
					sheet.engraving_defaults.thin_barline_thickness,
					system.y as i32+sheet.y(staves.len()-1, 8),
					system.y as i32+sheet.y(0, 0)
				));
			}
			system.x += measure.x + (space / 2);
		}
	}
	graphic
}
