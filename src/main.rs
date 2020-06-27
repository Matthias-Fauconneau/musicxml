mod xml;
mod music_xml; use music_xml::MusicXML;
//struct MusicXML;
use framework::*;

impl Widget for MusicXML{
    fn size(&mut self, size : size2) -> size2 { xy{x: size.x, y: 720} }
    fn paint(&mut self, target : &mut Target) {
		let fg = bgra8{b:0xFF,g:0xFF,r:0xFF,a:0xFF};

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
						MusicData::Attributes(Attributes{clefs, ..}) => for clef in clefs {
							todo!("{:?}", clef);
						},
						//todo => todo!("{:?}", todo),
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
