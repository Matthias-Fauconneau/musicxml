#![allow(non_upper_case_globals)]
mod xml;
mod music_xml; use music_xml::MusicXML;
use framework::*;

lazy_static::lazy_static! {
	static ref font: framework::font::File<'static> = framework::font::open(std::path::Path::new("/usr/local/share/fonts/bravura/Bravura.otf")).unwrap();
}

impl Widget for MusicXML{
    fn size(&mut self, size : size2) -> size2 { xy{x: size.x, y: 720} }
    #[throws] fn paint(&mut self, target : &mut Target) {
		let staff_height = 360;
        let margin = staff_height / 2; // 180
        let interval_height = staff_height / 4; // 90

		for line in 0..5 {
				target.slice_mut(xy{x:0, y:margin+line*interval_height}, xy{x: target.size.x, y: 1}).set(|_| fg);
		}
		for part in &self.score_partwise.parts {
			for measure in &part.measures {
				for music_data in &measure.music_data {
					use music_xml::*;
					match music_data {
						MusicData::Attributes(Attributes{clefs, ..}) => for _clef in clefs {
							#[allow(non_snake_case)] mod SMuFL {
								#![allow(non_camel_case_types)]
								pub const gClef : char = '\u{E050}'; // G clef
							}
							let id = font.glyph_index(SMuFL::gClef).unwrap();
							let bbox = font.glyph_bounding_box(id).unwrap();
							let scale = Ratio{num: staff_height, div: font.units_per_em().unwrap() as u32}; // SMuFL em = staff height
							let coverage = font.rasterize(scale, id, bbox);
							let top_left = scale * xy{
								x: font.glyph_hor_side_bearing(id).unwrap() as u32,
								y: 0,//-bbox.y_max as u32
							};
							target.slice_mut(top_left, coverage.size).set_map(coverage, |_,coverage| bgra8{a : 0xFF, ..sRGB(coverage).into()});
						},
						_ => (),
					}
				}
			}
		}
	}
}

#[throws] fn main() {
	let mut score : MusicXML = xml::from_document(&xml::parse(&std::fs::read("../test.xml")?)?)?;
	//let mut score = MusicXML;
	window::run(&mut score)?
}
