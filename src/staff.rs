use {fehler::throws, derive_more::{Deref, DerefMut}, vector::{MinMax}, crate::{music_xml::{self, Clef, Sign, Pitch, Note, Stem}}};

#[derive(Default, Debug)] pub struct Staff {
	pub clef: Option<Clef>,
	pub octave: i8,
	pub octave_start_x: Option<u32>,
	pub ties: Vec<Pitch>,
}

impl From<music_xml::Staff> for usize { fn from(staff: music_xml::Staff) -> Self { (2 - staff.0) as usize } } // 1..2 -> 1: treble .. 0: bass

#[derive(Deref)] pub struct StaffRef<'t> { pub index: usize, #[deref] pub staff: &'t Staff }
pub trait Index { fn index(&self, index: music_xml::Staff) -> StaffRef; }
impl Index for [Staff] {
	fn index(&self, index: music_xml::Staff) -> StaffRef { let index = index.into(); StaffRef{index, staff: &self[index]} }
}

#[derive(Deref, DerefMut)] pub struct StaffMut<'t> { index: usize, #[deref]#[deref_mut] staff: &'t mut Staff }
pub trait IndexMut { fn index_mut(&mut self, index: music_xml::Staff) -> StaffMut; }
impl IndexMut for [Staff] {
	fn index_mut(&mut self, index: music_xml::Staff) -> StaffMut { let index = index.into(); StaffMut{index, staff: &mut self[index]} }
}
impl StaffMut<'_> { pub fn as_ref(&self) -> StaffRef { StaffRef{index: self.index, staff: &self.staff} } }

impl Staff {
	#[allow(non_snake_case)]
	fn C4(&self) -> i8 { use Sign::*; (match self.clef.as_ref().unwrap().sign { G=> -2, F=> 10 }) - self.octave*7 }
	pub fn step(&self, pitch: &Pitch) -> i8 { self.C4() + i8::from(pitch) }
}

impl Note {
	pub fn staff(&self) -> usize { self.staff.unwrap().into() }
 	#[throws(as Option)] pub fn step(&self, staves: &[Staff]) -> i8 { staves[self.staff()].step(&self.pitch?) }
}

#[track_caller] pub fn staff(chord: impl IntoIterator<Item=&Note>) -> Option<usize> { chord.into_iter().map(|n| n.staff()).try_reduce(|a,b| (a == b).then(|| a)).flatten() }
pub fn minmax(chord: impl IntoIterator<Item=&Note>, staves: &[Staff]) -> Option<MinMax<i8>> { vector::minmax(chord.into_iter().filter_map(|note| note.step(staves))) }
#[throws(as Option)] pub fn stem_step(chord: impl IntoIterator<Item=&Note>, staves: &[Staff], stem: Stem) -> i8 {
	let bounds = minmax(chord.into_iter().filter(|x| x.has_stem()), staves)?;
	if let Stem::Down = stem { bounds.min - 5 } else { bounds.max + 5 }
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