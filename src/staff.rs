use {vector::MinMax, crate::{music_xml::{self, Clef, Sign, Pitch, Note, Stem}}};

#[derive(Default, Debug)] pub struct Staff {
	pub clef: Option<Clef>,
	pub octave: i8,
	pub octave_start_x: Option<u32>,
	pub ties: Vec<Pitch>,
}

impl From<music_xml::Staff> for usize { fn from(staff: music_xml::Staff) -> Self { (2 - staff.0) as usize } } // 1..2 -> 1: treble .. 0: bass

pub struct StaffRef<'t> { pub index: usize, pub staff: &'t Staff }
impl<'t> std::ops::Deref for StaffRef<'t> { type Target = &'t Staff; fn deref(&self) -> &Self::Target { &self.staff } }
pub trait Index { fn index(&'_ self, index: music_xml::Staff) -> StaffRef<'_>; }
impl Index for [Staff] {
	fn index(&'_ self, index: music_xml::Staff) -> StaffRef<'_> { let index = index.into(); StaffRef{index, staff: &self[index]} }
}

pub struct StaffMut<'t> { index: usize, staff: &'t mut Staff }
impl<'t> std::ops::Deref for StaffMut<'t> { type Target = &'t mut Staff; fn deref(&self) -> &Self::Target { &self.staff } }
impl<'t> std::ops::DerefMut for StaffMut<'t> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.staff } }
pub trait IndexMut { fn index_mut(&'_ mut self, index: music_xml::Staff) -> StaffMut<'_>; }
impl IndexMut for [Staff] {
	fn index_mut(&'_ mut self, index: music_xml::Staff) -> StaffMut<'_> { let index = index.into(); StaffMut{index, staff: &mut self[index]} }
}
impl StaffMut<'_> { pub fn as_ref(&'_ self) -> StaffRef<'_> { StaffRef{index: self.index, staff: &self.staff} } }

impl Staff {
	#[allow(non_snake_case)]
	fn C4(&self) -> i8 { use Sign::*; (match self.clef.as_ref().unwrap().sign { G=> -2, F=> 10 }) - self.octave*7 }
	pub fn step(&self, pitch: &Pitch) -> i8 { self.C4() + i8::from(pitch) }
}

impl Note {
	pub fn staff(&self) -> usize { self.staff.unwrap().into() }
 	pub fn step(&self, staves: &[Staff]) -> Option<i8> { Some(staves[self.staff()].step(&self.pitch?)) }
}

#[track_caller] pub fn staff(chord: impl IntoIterator<Item=&Note>) -> Option<usize> { chord.into_iter().map(|n| n.staff()).try_reduce(|a,b| (a == b).then(|| a)).flatten() }
pub fn minmax(chord: impl IntoIterator<Item=&Note>, staves: &[Staff]) -> Option<MinMax<i8>> { vector::minmax(chord.into_iter().filter_map(|note| note.step(staves))) }
pub fn stem_step(chord: impl IntoIterator<Item=&Note>, staves: &[Staff], stem: Stem) -> Option<i8> {
	let bounds = minmax(chord.into_iter().filter(|x| x.has_stem()), staves)?;
	Some(if let Stem::Down = stem { bounds.min - 5 } else { bounds.max + 5 })
}

pub trait Chord {
	fn staff(&self) -> Option<usize>;
	fn minmax(&self, staves: &[Staff]) -> Option<MinMax<i8>>;
	fn stem_step(&self, staves: &[Staff], stem: Stem) -> Option<i8>;
}
impl Chord for Box<[&Note]> {
	fn staff(&self) -> Option<usize> { staff(self.into_iter().copied()) }
	fn minmax(&self, staves: &[Staff]) -> Option<MinMax<i8>> { minmax(self.into_iter().copied(), staves) }
	fn stem_step(&self, staves: &[Staff], stem: Stem) -> Option<i8> { stem_step(self.into_iter().copied(), staves, stem) }
}