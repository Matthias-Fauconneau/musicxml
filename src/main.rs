//mod xml;
//mod music_xml; use music_xml::MusicXML;
struct MusicXML;
use framework::*;

impl Widget for MusicXML{
    fn paint(&mut self, target : &mut Target) {
		let fg = bgra8{b:0xFF,g:0xFF,r:0xFF,a:0xFF};

		let staff_height = 360;
        let margin = staff_height / 2; // 180
        //let interval_height = staff_height / 4.; // 90

        target.slice_mut(xy{x:0, y:margin}, xy{x: target.size.x, y: 1}).set(|_| fg);
	}
}

#[throws] fn main() {
	//let mut score : MusicXML = xml::from_document(&xml::parse(&std::fs::read("../test.xml")?)?)?;
	let mut score = MusicXML;
	window::run(&mut score)?
}
