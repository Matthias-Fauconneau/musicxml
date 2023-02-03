use crate::music_xml::{Attributes, Clef, Sign, Step, Pitch, Key, Time};
use crate::font::SMuFL::{clef, accidental, time_signature};
use crate::{measure::MeasureLayoutContext, staff::{Staff, IndexMut, StaffRef}};
impl MeasureLayoutContext<'_,'_> {
	pub fn attributes(&mut self, staves: &mut [Staff], Attributes{clefs, key, time, ..}: &Attributes) {
		for &clef@Clef{staff, sign, ..} in clefs.iter() {
			let ref mut staff = staves.index_mut(staff);
			staff.clef = Some(clef);
			let (glyph, step) = {use Sign::*; match sign { G=>(clef::G, Step::G), F=>(clef::F, Step::F) }};
			let x = self.x;
			self.push_glyph_at_pitch(x, staff.as_ref(), &Pitch::new(&clef, &step), glyph, 1.);
		}
		self.advance(0);
		if let &Some(Key{fifths,..}) = key {
			let mut key = |fifths:i8, symbol| {
				let mut sign = |steps: &mut dyn Iterator<Item=&Step>| {
					for step in steps {
						for (index, Staff{clef, ..}) in staves.iter().enumerate() {
							let x = self.x;
							self.push_glyph_at_pitch(x, StaffRef{index, staff: &Staff{clef: *clef, ..Staff::default()}}, &Pitch::new(clef.as_ref().unwrap(), step), symbol, 1.);
						}
					}
				};
				let steps = {use Step::*; [B,E,A,D,G,C,F].iter()};
				if fifths>0 { sign(&mut steps.rev().take(fifths as usize)) } else { sign(&mut steps.take((-fifths) as usize)) }
			};
			//if fifths == 0 { key(system.fifths, accidental::natural) } else
			key(fifths, if fifths<0 { accidental::flat } else { accidental::sharp });
			self.advance(0);
		}
		if let Some(Time{beats, beat_type,..}) = time {
			let texts : [String; 2] = [beats, beat_type].map(|number| number.to_string().chars().map(time_signature::from).collect::<String>());
			use ui::text::{Plain, View, layout, Glyph};
			let mut texts : [_; 2] = texts.map(|text| View::with_face(self.measure.sheet.face, Plain(text)));
			let width = texts.iter_mut().map(|text| text.size().x).max().unwrap();
			for (text, step) in texts.iter_mut().zip([6,2]) {
				let x = self.x + (width-text.size().x)/2;
				for Glyph{x: dx, id, ..} in layout(&text.font, text.data.as_ref()) {
					for index in 0..staves.len() {
						self.push_glyph_id(x + dx, index, step, 0, id, 1., num::unit);
					}
				}
			}
			self.advance(0);
		}
	}
}
