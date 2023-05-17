#[allow(non_snake_case)] pub mod SMuFL {
    #![allow(non_upper_case_globals)]
    pub struct EngravingDefaults {
		pub staff_line_thickness: u32,
		pub stem_thickness: u32,
		pub thin_barline_thickness: u32,
		pub beam_thickness: u32,
		pub leger_line_extension : u32,
		pub leger_line_thickness : u32,
	}
    #[derive(PartialEq)] pub enum Anchor { StemUpNW, StemDownNW, StemUpSE, StemDownSW }
    pub mod clef {
	    pub const G : char = '\u{E050}';
	    pub const F : char = '\u{E062}';
    }
    pub mod note_head {
	    pub const breve : char = '\u{E0A1}';
	    pub const whole : char = '\u{E0A2}';
	    pub const half : char = '\u{E0A3}';
	    pub const black : char = '\u{E0A4}';
    }

    pub mod flag {
	    pub const up : char = '\u{E240}';
	    pub const down : char = '\u{E241}';
	    pub fn from(flag: char, value: u32) -> char { u32::try_into(u32::from(flag)+value*2).unwrap() }
    }
    pub mod accidental {
	    pub const flat : char = '\u{E260}';
	    //pub const natural : char = '\u{E261}';
	    pub const sharp : char = '\u{E262}';
    }
    pub mod time_signature {
	    pub const zero : char = '\u{E080}';
	    pub fn from(digit: char) -> char { u32::try_into(u32::from(zero)+digit.to_digit(10).unwrap()).unwrap() }
    }
}

use vector::{xy, int2};

pub trait SMuFont {
    fn engraving_defaults() -> SMuFL::EngravingDefaults;
    fn anchor(&self, glyph: char, anchor: SMuFL::Anchor) -> int2;
}

pub(crate) mod bravura {
    use super::{SMuFont, SMuFL, int2, xy};

	impl SMuFont for ui::font::Face<'_> {
		fn engraving_defaults() -> SMuFL::EngravingDefaults { SMuFL::EngravingDefaults{
			staff_line_thickness: 32,
			stem_thickness: 30,
			thin_barline_thickness: 40,
			beam_thickness: 250,
			leger_line_extension: 100,
			leger_line_thickness: 40,
		}}
		fn anchor(&self, glyph: char, anchor: SMuFL::Anchor) -> int2 {
			assert_eq!(self.units_per_em(), 1000);
			use SMuFL::*;
			use Anchor::*;
			let anchors : [(_, Box<[_]>); _] = [
				(note_head::black, [
					(StemDownNW, xy{x: 0, y: 42}),
					(StemUpSE, xy{x: self.bbox(self.glyph_index(SMuFL::note_head::black).unwrap()).unwrap().max.x-1, y: -42})
				].into()),
				(flag::up, [(StemUpNW, xy{x: 0, y:10})].into()),
				(flag::down, [(StemDownSW, xy{x: 0, y:-33})].into())
			];
			let glyph_anchors = |glyph| &anchors.iter().find(|(id,_)| id == &glyph).unwrap().1;
			glyph_anchors(glyph).iter().find(|(id,_)| id == &anchor).unwrap().1
		}
	}
}

