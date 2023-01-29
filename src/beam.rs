//use iter::Single;
pub trait Single: Iterator+Sized { fn single(mut self) -> Option<Self::Item> { self.next().filter(|_| self.next().is_none()) } }
impl<I:Iterator> Single for I {}

use crate::{music_xml::Note, staff::Staff, measure::MeasureLayoutContext};
impl MeasureLayoutContext<'_,'_> { pub fn beam(&mut self, staves: &[Staff], beam: &[Vec<&Note>]) {
	use crate::{music_xml::{NoteType, Stem}, font::{SMuFont, SMuFL::{Anchor, note_head, flag}}, staff::{Index, Chord}};
	use {vector::{MinMax, xy}, ui::graphic::{Rect, Parallelogram}};
	let Some(bounds) = beam.iter().filter_map(|chord| chord.bounds(staves)).reduce(MinMax::minmax) else { // Unstemmed notes
		// Heads
		for (i, chord) in beam.iter().enumerate() {
			let x = self.x + i as u32 * self.space();
			for note in chord.iter() {
				if let Note{staff: Some(staff), r#type: Some(r#type), pitch: Some(pitch), ..} = note {
					self.push_glyph_at_pitch(x, staves.index(&staff), &pitch, {use {NoteType::*, note_head::*}; match r#type { Breve=>breve, Whole=>whole, Half=>half, _=>black }});
				} else { unreachable!() }
			}
		}
		return;
	};
	let MinMax{min: bottom, max: top} = bounds;
	let direction = if top-4 > 4-bottom { Stem::Down } else { Stem::Up };
	let stem_anchor = if let Stem::Down = direction { Anchor::StemDownNW } else { Anchor::StemUpSE };
	let stem_anchor = self.sheet.face.anchor(note_head::black, stem_anchor);

	let beam = beam.iter().scan(self.x, |x, chord| {
		let stem = *x + stem_anchor.x as u32;
		*x += self.space();
		Some((stem, chord))
	}).collect::<Vec<_>>();

	// Heads
	for &(x, chord) in beam.iter() {
		for note in chord.iter() {
			if let Note{staff: Some(staff), r#type: Some(r#type), pitch: Some(pitch), ..} = note {
				self.push_glyph_at_pitch(x, staves.index(&staff), &pitch, {use {NoteType::*, note_head::*}; match r#type { Breve=>breve, Whole=>whole, Half=>half, _=>black }});
			} else { unreachable!() }
		}
	}

	let stem_thickness = self.sheet.engraving_defaults.stem_thickness;

	if let &[(left, first), .., (right, last)] = beam.as_slice() { // Beam (fixme: >2)
		let right = right + stem_thickness as u32;
		self.measure.graphic.parallelograms.push(Parallelogram{
			top_left: xy{x: left as i32, y: self.y(first.staff(), first.stem_step(staves, direction))},
			bottom_right: xy{x: right as i32, y: self.y(last.staff(), last.stem_step(staves, direction))},
			vertical_thickness: self.sheet.engraving_defaults.beam_thickness
		});
	}

	//float opacity = allTied(beam[0]) ? 1./2 : 1;
	for (x, chord) in beam.iter() { // Stem
		//if chord.r#type() >= Note::Whole
		let x = x + stem_anchor.x as u32;
		let staff = chord.staff();
		let stem_step = chord.stem_step(staves, direction);
		if let Stem::Down = direction { // Bottom Left
			self.measure.graphic.rects.push(Rect{min: xy{x: x as i32, y: self.y(staff, top)+stem_anchor.y}, max: xy{x: x as i32 + stem_thickness as i32, y: self.y(staff, stem_step)}});
		} else { // Top Right
			self.measure.graphic.rects.push(Rect{min: xy{x: x as i32 - stem_thickness as i32, y: self.y(staff, stem_step)}, max: xy{x: x as i32, y: self.y(staff, bottom)+stem_anchor.y}});
		}
	}

	// Flag
	if let Some(&(x, chord)) = beam.iter().single() {
		let stem_step = chord.stem_step(staves, direction);
		let staff = chord.staff();
		let value = if let Stem::Down = direction { chord.first() } else { chord.last() }.unwrap();
		let flag = if let Stem::Down = direction { flag::down } else { flag::up };
		let flag_anchor = if let Stem::Down = direction { Anchor::StemDownSW } else { Anchor::StemUpNW };
		let value = value.r#type.unwrap();
		if value <= NoteType::Eighth {
			let xy{x, y: dy} = xy{x: x as i32, y: 0} + self.sheet.face.anchor(flag, flag_anchor);
			self.push_glyph(x as u32, staff, stem_step, dy, flag::from(flag, NoteType::Eighth as u32 - value as u32));
		}
	}
}}
