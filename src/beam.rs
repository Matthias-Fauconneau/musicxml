//use iter::Single;
pub trait Single: Iterator+Sized { fn single(mut self) -> Option<Self::Item> { self.next().filter(|_| self.next().is_none()) } }
impl<I:Iterator> Single for I {}

use {std::cmp::{min,max}, crate::{music_xml::Note, staff::Staff, measure::{Measure, MeasureLayoutContext}, font::SMuFL::EngravingDefaults}};
impl MeasureLayoutContext<'_,'_> { pub fn beam(&mut self, staves: &[Staff], beam: &[Vec<&Note>]) {
	use crate::{music_xml::{NoteType, Stem}, font::{SMuFont, SMuFL::{Anchor, note_head, flag}}, staff::{Index, bounds, Chord}};
	let head = |note:&Note| {use {NoteType::*, note_head::*}; match note.r#type.unwrap() { Breve=>breve, Whole=>whole, Half=>half, _=>black }};
	use {vector::{reduce_minmax, MinMax, xy}, ui::graphic::{Rect, Parallelogram}};

   	let heads = |m:&mut Measure, x: u32, chord:&[&Note]| {
		for (staff, _) in staves.iter().enumerate() {
			let mut leger = |step| {
				let EngravingDefaults{leger_line_thickness, leger_line_extension,..} = m.engraving_defaults;
				m.graphic.horizontal(m.y(staff, step), leger_line_thickness, x as i32 - leger_line_extension as i32, (x+m.sheet.face.advance(head(chord[0]))+leger_line_extension) as i32)
			};
			if let Some(bounds) = bounds(chord.into_iter().filter(|note| usize::from(note.staff.unwrap()) == staff).copied(), staves) {
				for step in (10..=bounds.max).step_by(2) { leger(step) }
				for step in (bounds.min..=-2).step_by(2) { leger(step) }
			}
		}
		for note in chord.iter() {
			let note@Note{staff: Some(staff), pitch: Some(ref pitch), ..} = note else { unreachable!() };
			m.push_glyph_at_pitch(x, staves.index(*staff), pitch, head(note));
		}
	};

	if beam.iter().all(|chord| !chord.into_iter().any(|x| x.has_stem())) { // Unstemmed heads
		for (i, chord) in beam.iter().enumerate() {
			let x = self.x + i as u32 * self.space();
			heads(self, x, chord);
		}
		return;
	};

	let MinMax{min: bottom, max: top} = reduce_minmax(beam.iter().filter_map(|chord| bounds(chord.into_iter().filter(|x| x.has_stem()).copied(), staves))).unwrap();
	let direction = if top-4 > 4-bottom { Stem::Down } else { Stem::Up };
	let stem_anchor = if let Stem::Down = direction { Anchor::StemDownNW } else { Anchor::StemUpSE };
	let stem_anchor = self.sheet.face.anchor(note_head::black, stem_anchor);

	let beam = beam.iter().scan(self.x, |x, chord| {
		let stem = *x + stem_anchor.x as u32;
		*x += self.space();
		Some((stem, chord))
	}).collect::<Vec<_>>();

	for &(x, chord) in beam.iter() { heads(self, x, chord); } // Stemmed heads

	let stem_thickness = self.sheet.engraving_defaults.stem_thickness;

	if let &[(left, first), .., (right, last)] = beam.as_slice() && first[0].r#type.unwrap() <= NoteType::Eighth { // Beam (fixme: >2)
		let right = right + stem_thickness as u32;
		assert!(first.staff() == last.staff());
		let staff = first.staff();
		if let [Some(first),Some(last)] = [first, last].map(|chord| chord.stem_step(staves, direction)) {
			let [bottom, top] = [min(first,last),max(first,last)];
			self.measure.graphic.parallelogram(Parallelogram{
				top_left: xy{x: left as i32, y: self.y(staff, top)},
				bottom_right: xy{x: right as i32, y: self.y(staff, bottom)},
				vertical_thickness: self.sheet.engraving_defaults.beam_thickness
			});
		}
	}

	//float opacity = allTied(beam[0]) ? 1./2 : 1;
	for (x, chord) in beam.iter() { // Stem
		//if chord.r#type() >= Note::Whole
		let x = x + stem_anchor.x as u32;
		let staff = chord.staff();
		let Some(stem_step) = chord.stem_step(staves, direction) else {continue;};
		if let Stem::Down = direction { // Bottom Left
			self.measure.graphic.rect(Rect{min: xy{x: x as i32, y: self.y(staff, top)+stem_anchor.y}, max: xy{x: x as i32 + stem_thickness as i32, y: self.y(staff, stem_step)}});
		} else { // Top Right
			self.measure.graphic.rect(Rect{min: xy{x: x as i32 - stem_thickness as i32, y: self.y(staff, stem_step)}, max: xy{x: x as i32, y: self.y(staff, bottom)+stem_anchor.y}});
		}
	}

	// Flag
	if let Some(&(x, chord)) = beam.iter().single() && let Some(stem_step) = chord.stem_step(staves, direction) {
		let staff = chord.staff();
		let r#type = if let Stem::Down = direction { chord.first() } else { chord.last() }.unwrap().r#type.unwrap();
		let flag = if let Stem::Down = direction { flag::down } else { flag::up };
		let flag_anchor = if let Stem::Down = direction { Anchor::StemDownSW } else { Anchor::StemUpNW };
		if r#type <= NoteType::Eighth {
			let xy{x, y: dy} = xy{x: x as i32, y: 0} + self.sheet.face.anchor(flag, flag_anchor);
			self.push_glyph(x as u32, staff, stem_step, dy, flag::from(flag, NoteType::Eighth as u32 - r#type as u32));
		}
	}
}}
