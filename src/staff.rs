use {fehler::throws, derive_more::{Deref, DerefMut}, vector::{minmax,MinMax}, crate::{music_xml::{self, Clef, Sign, Pitch, Stem, Note}}};

#[derive(Default, Debug)] pub struct Staff {
	pub clef: Option<Clef>,
	pub octave: i8,
	pub octave_start_x: Option<u32>
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
 	pub fn step(&self, staves: &[Staff]) -> i8 { self.pitch.map(|pitch| staves.index(self.staff.unwrap()).step(&pitch)).unwrap() }
}

pub fn bounds(iter: impl IntoIterator<Item=&Note>, staves: &[Staff]) -> Option<MinMax<i8>> { minmax(iter.into_iter()./*filter_*/map(|note| note.step(staves))) }

pub trait Chord {
	fn staff(&self) -> usize;
	fn stem_step(&self, staves: &[Staff], stem: Stem) -> Option<i8>;
}
impl Chord for Vec<&Note> {
	fn staff(&self) -> usize { self.into_iter().next().unwrap().staff.unwrap().into() }
    #[throws(as Option)] fn stem_step(&self, staves: &[Staff], stem: Stem) -> i8 {
	    let bounds = bounds(self.into_iter().filter(|x| x.has_stem()).copied(), staves)?;
		if let Stem::Down = stem { bounds.min - 5 } else { bounds.max + 5 }
    }
}