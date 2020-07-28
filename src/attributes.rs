use crate::{music_xml::{Attributes, Clef, ClefSign, Step, Pitch, Key, Time}, font::SMuFL::{clef, accidental, time_signature}, measure::MeasureLayoutContext, staff::{Staff, IndexMut, StaffRef}};
impl MeasureLayoutContext<'_> { pub fn attributes(&mut self, staves: &mut [Staff], Attributes{clefs, key, time, ..}: &Attributes) {
	for &clef@Clef{staff, sign, ..} in clefs {
		let mut staff = staves.index_mut(&staff);
		staff.clef = Some(clef);
		let (glyph, step) = {use ClefSign::*; match sign { G=>(clef::G, Step::G), F=>(clef::F, Step::F) }};
		let x = self.x;
		self.push_glyph_at_pitch(x as i32, staff.as_ref(), &Pitch::new(&clef, &step), glyph);
	}
	self.advance(0);
	if let &Some(Key{fifths,..}) = key {
		let mut key = |fifths:i8, symbol| {
			let mut sign = |steps: &mut dyn Iterator<Item=&Step>| {
				for step in steps {
					for (index, Staff{clef, ..}) in staves.iter().enumerate() {
						let x = self.x;
						self.push_glyph_at_pitch(x as i32, StaffRef{index, staff: &Staff{clef: *clef, octave: 0}}, &Pitch::new(clef.as_ref().unwrap(), step), symbol);
					}
				}
			};
			let steps = {use Step::*; [B,E,A,D,G,C,F].iter()};
			if fifths>0 { sign(&mut steps.rev().take(fifths as usize)) } else { sign(&mut steps.take((-fifths) as usize)) }
		};
		//if fifths == 0 { key(system.fifths, accidental::natural) } else
		key(fifths, if fifths<0 { accidental::flat } else { accidental::sharp });
	}
	self.advance(0);
	if let Some(Time{beats, beat_type}) = time {
		let texts : [String; 2] = core::array::Iterator::collect(
			[beats, beat_type].iter().map(|number| number.to_string().chars().map(time_signature::from).collect::<String>())
		);
		use ui::text::{TextView, Buffer, layout, Glyph};
		let texts : [TextView; 2] = core::array::Iterator::collect(
			texts.as_ref().iter().map(|text| TextView::new(&self.measure.sheet.font, Buffer::new(text)))
		);
		let width = texts.iter().map(|text| text.size().x).max().unwrap();
		use core::array::IntoIterator;
		for (text, step) in texts.iter().zip([6,2].into_iter()) {
			let x = self.x as i32 + ((width-text.size().x)/2) as i32;
			for Glyph{x: dx, id, ..} in layout(text.font, text.buffer.text.char_indices()) {
				for index in 0..staves.len() {
					self.push_glyph_id(x + dx, index, step, 0, id);
				}
			}
		}
	}
}}
