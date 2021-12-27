use {derive_more::{Deref, DerefMut}, vector::MinMax, crate::{music_xml::{self, Clef, Sign, Pitch, Stem, Note}}};

#[derive(Default, Debug)] pub struct Staff {
	pub clef: Option<Clef>,
	pub octave: i8,
	pub octave_start_x: Option<u32>
}

impl From<&music_xml::Staff> for usize { fn from(staff: &music_xml::Staff) -> Self { (2 - staff.0) as usize } } // 1..2 -> 1: treble .. 0: bass

#[derive(Deref)] pub struct StaffRef<'t> { pub index: usize, #[deref] pub staff: &'t Staff }
pub trait Index { fn index(&self, index: &music_xml::Staff) -> StaffRef; }
impl Index for [Staff] {
	fn index(&self, index: &music_xml::Staff) -> StaffRef { let index = index.into(); StaffRef{index, staff: &self[index]} }
}

#[derive(Deref, DerefMut)] pub struct StaffMut<'t> { index: usize, #[deref]#[deref_mut] staff: &'t mut Staff }
pub trait IndexMut { fn index_mut(&mut self, index: &music_xml::Staff) -> StaffMut; }
impl IndexMut for [Staff] {
	fn index_mut(&mut self, index: &music_xml::Staff) -> StaffMut { let index = index.into(); StaffMut{index, staff: &mut self[index]} }
}
impl StaffMut<'_> { pub fn as_ref(&self) -> StaffRef { StaffRef{index: self.index, staff: &self.staff} } }

impl Staff {
	#[allow(non_snake_case)]
	fn C4(&self) -> i8 { use Sign::*; (match self.clef.as_ref().unwrap().sign { G=> -2, F=> 10 }) - self.octave*7 }
	pub fn step(&self, pitch: &Pitch) -> i8 { self.C4() + i8::from(pitch) }
}

impl Note {
    fn step(&self, staves: &[Staff]) -> Option<i8> { self.pitch.map(|pitch| staves.index(&self.staff.unwrap()).step(&pitch)) }
}

pub trait Chord {
	fn staff(&self) -> usize;
    fn bounds(&self, staves: &[Staff]) -> MinMax<i8>;
    fn stem_step(&self, staves: &[Staff], stem: Stem) -> i8;
}
impl Chord for Vec<&Note> {
	fn staff(&self) -> usize { (&self.first().unwrap().staff.unwrap()).into() }
    #[track_caller] fn bounds(&self, staves: &[Staff]) -> MinMax<i8> {
        self.iter().filter(|x| x.has_stem()).filter_map(|note| note.step(staves)).map(|e|MinMax{min: e, max: e}).reduce(MinMax::minmax).unwrap()
    }
    fn stem_step(&self, staves: &[Staff], stem: Stem) -> i8 {
	    let bounds = self.bounds(staves);
	    if let Stem::Down = stem { bounds.min - 5 } else { bounds.max + 5 }
    }
}
