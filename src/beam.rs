//use iter::Single;
pub trait Single: Iterator+Sized { fn single(mut self) -> Option<Self::Item> { self.next().filter(|_| self.next().is_none()) } }
impl<I:Iterator> Single for I {}

use {std::cmp::{min,max}, crate::{music_xml::Note, staff::Staff, measure::MeasureLayoutContext, font::SMuFL::EngravingDefaults}};
impl MeasureLayoutContext<'_,'_> { pub fn beam(&mut self, staves: &mut [Staff], beam: &[Box<[&Note]>]) {
	use crate::{music_xml::{NoteType, Stem, Tie}, font::{SMuFont, SMuFL::{Anchor, note_head, flag}}};
	let head = |note:&Note| {use {NoteType::*, note_head::*}; match note.r#type.unwrap() { Breve=>breve, Whole=>whole, Half=>half, _=>black }};
	use {crate::list, vector::{reduce_minmax, MinMax, xy}, ui::graphic::{Rect, Parallelogram}, crate::staff::{Index, IndexMut, minmax, Chord}};

	let beam = list(beam.iter().scan(self.x, |x, chord| {
		let note = *x;
		*x += self.space();
		Some((note, chord))
	}));

	#[allow(non_upper_case_globals)] const tie: f32 = 1./16.;
	let style = |staves:&[_], Note{staff, pitch, ties, ..}:&Note| if ties.contains(&Tie::Stop) && staves.index(staff.unwrap()).ties.iter().any(|x| x == &pitch.unwrap()) { tie } else { 1. };

	for (x, chord) in beam.iter() {
		for note in chord.iter() { // Heads
			let note@Note{staff: Some(staff), pitch: Some(ref pitch), ..} = note else { continue;/*pause*/ };
			self.push_glyph_at_pitch(*x, staves.index(*staff), pitch, head(note), style(staves, note));
			if note.ties.contains(&Tie::Start) { staves.index_mut(*staff).ties.push(note.pitch.unwrap()) }
		}
	}

	assert!(!beam.is_empty()); for (_,chord) in beam.iter() { assert!(!chord.is_empty()); }
	let Some(MinMax{min: bottom, max: top}) = reduce_minmax(beam.iter().filter_map(|(_,chord)| minmax(chord.iter().filter(|x| x.has_stem()).copied(), staves))) else {return;};
	let direction = if top-4 > 4-bottom { Stem::Down } else { Stem::Up };
	let stem_anchor = if let Stem::Down = direction { Anchor::StemDownNW } else { Anchor::StemUpSE };
	let stem_anchor = self.sheet.face.anchor(note_head::black, stem_anchor);
	let stem_thickness = self.sheet.engraving_defaults.stem_thickness;
	for (_, chord) in beam.iter() { for note in chord.iter() { assert!(note.time_modification.is_none() /*&& note.dot==0*/); } }
	let beam = list(beam.iter().map(|(note, chord)| (note + stem_anchor.x as u32, *chord)));

	for (x, chord) in beam.iter() {
		let x = *x;
		#[allow(non_upper_case_globals)] const tie: f32 = 1./16.;
		let style = |staves:&[_], Note{staff, pitch, ties, ..}:&Note| if ties.contains(&Tie::Stop) && staves.index(staff.unwrap()).ties.iter().any(|x| x == &pitch.unwrap()) { tie } else { 1. };
		for note in chord.iter() { // Heads
			let note@Note{staff: Some(staff), pitch: Some(ref pitch), ..} = note else { continue;/*pause*/ };
			self.push_glyph_at_pitch(x - stem_anchor.x as u32, staves.index(*staff), pitch, head(note), style(staves, note));
			if note.ties.contains(&Tie::Start) { staves.index_mut(*staff).ties.push(note.pitch.unwrap()) }
		}
		for (staff, _) in staves.iter().enumerate() {
			let chord = list(chord.into_iter().filter(|x| x.staff() ==staff).copied());
			// Legers
			let mut leger = |step| {
				let EngravingDefaults{leger_line_thickness, leger_line_extension,..} = self.engraving_defaults;
				let note = chord[0];
				let note_size = self.face.bbox(self.face.glyph_index(head(note)).unwrap()).unwrap().size();
				let x = x - stem_anchor.x as u32;
				self.measure.graphic.horizontal(self.y(staff, step), leger_line_thickness, x as i32 - leger_line_extension as i32, (x+note_size.x+leger_line_extension) as i32, style(staves, note))
			};
			if let Some(MinMax{min,max}) = chord.minmax(staves) {
				for step in (10..=max).step_by(2) { leger(step) }
				for step in (min..=-2).step_by(2) { leger(step) }
			}
			// Stem
			let Some(MinMax{min: bottom, max: top}) = minmax(chord.into_iter().filter(|x| x.has_stem()).copied(), staves) else {continue;};
			let note = chord[0];
			let style = if note.ties.contains(&Tie::Stop) { tie } else { 1. };
			if let Stem::Down = direction { // Bottom Left
				self.measure.graphic.rect(Rect{min: xy{x: x as i32, y: self.y(staff, top)+stem_anchor.y}, max: xy{x: x as i32 + stem_thickness as i32, y: self.y(staff, bottom-5)}}, style);
			} else { // Top Right
				self.measure.graphic.rect(Rect{min: xy{x: x as i32 - stem_thickness as i32, y: self.y(staff, top+5)}, max: xy{x: x as i32, y: self.y(staff, bottom)+stem_anchor.y}}, style);
			}
			// Flag
			if let Some(&(x, _)) = beam.iter().single() && let Some(stem_step) = chord.stem_step(staves, direction) {
				let staff = chord.staff().unwrap();
				let r#type = if let Stem::Down = direction { chord.first() } else { chord.last() }.unwrap().r#type.unwrap();
				let flag = if let Stem::Down = direction { flag::down } else { flag::up };
				let flag_anchor = if let Stem::Down = direction { Anchor::StemDownSW } else { Anchor::StemUpNW };
				if r#type <= NoteType::Eighth {
					let xy{x, y: dy} = xy{x: x as i32, y: 0} + self.sheet.face.anchor(flag, flag_anchor);
					self.push_glyph(x as u32, staff, stem_step, dy, flag::from(flag, NoteType::Eighth as u32 - r#type as u32), style);
				}
			}
		}
		for note in chord.iter() { if note.ties.contains(&Tie::Stop) { staves[note.staff()].ties.retain(|&x| x!=note.pitch.unwrap()); } }
	}

	//float opacity = allTied(beam[0]) ? 1./2 : 1;
	let format = |beam:&Box<[(_,&Box<[&Note]>)]>| {use itertools::Itertools; format!("{}", beam.iter().format_with("|", |(_,e),f| f(&e.iter().format(" "))))};
	if let &[(left, first), .., (right, last)] = &*beam && first[0].r#type.unwrap() <= NoteType::Eighth { // Beam (fixme: >2)
		let right = right + stem_thickness as u32;
		if !(first.staff().is_some() && last.staff().is_some() && first.staff() == last.staff()) { return; }
		assert!(first.staff().is_some() && last.staff().is_some() && first.staff() == last.staff(), "{}", format(&beam));
		let staff = first.staff().unwrap();
		if let [Some(first),Some(last)] = [first, last].map(|chord| chord.stem_step(staves, direction)) {
			let [bottom, top] = [min(first,last),max(first,last)];
			//unimplemented!();
			self.measure.graphic.parallelogram(Parallelogram{
				top_left: xy{x: left as i32, y: self.y(staff, top)},
				bottom_right: xy{x: right as i32, y: self.y(staff, bottom)},
				vertical_thickness: self.sheet.engraving_defaults.beam_thickness
			});
		}
	} /*else {
		for (_, chord) in beam.iter() { for note in chord.iter() {
			assert!(note.r#type.unwrap() >= NoteType::Quarter, "{}", format(&beam));
		}}
	}*/
}}
