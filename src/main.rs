#![feature(bindings_after_at)]
#![allow(non_upper_case_globals)]
#![allow(incomplete_features)]#![feature(const_generics)]
mod xml;
mod music_xml; use music_xml::MusicXML;
#[allow(non_snake_case)] mod SMuFL {
	pub struct EngravingDefaults {pub staff_line_thickness: u8, pub stem_thickness: u8, pub thin_barline_thickness: u8}
	#[derive(PartialEq)] pub enum Anchor { StemDownNW, StemUpSE }
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
	pub mod accidental {
		pub const flat : char = '\u{E260}';
		//pub const natural : char = '\u{E261}';
		pub const sharp : char = '\u{E262}';
	}
	pub const time_signature : char = '\u{E080}';
}
use framework::*;

lazy_static::lazy_static! {
	static ref font: framework::font::File<'static> = framework::font::open(std::path::Path::new("/usr/local/share/fonts/bravura/Bravura.otf")).unwrap();
}

fn layout(music: &MusicXML, size: size2) -> Graphic<'static> {
	// Bravura
	assert_eq!(font.units_per_em(), Some(1000));
	use SMuFL::*;
	//stemLength = 7*halfLineInterval
	//shortStemLength = 5*halfLineInterval;
	use Anchor::*;
	let glyphs_with_anchors = [(note_head::black, [
		(StemDownNW, xy{x: 0, y: 42}),
		(StemUpSE, xy{x: font.glyph_bounding_box(font.glyph_index(note_head::black).unwrap()).unwrap().x_max as i32-1, y: -42})
	])];

	struct Sheet { engraving_defaults: EngravingDefaults, staff_height: u32, staff_distance: u32 }
	impl Sheet {
		// staff: 0: bass .. 1: treble; step: -8: bottom .. 0: top
		fn y(&self, staff: usize, step: i8) -> i32 { - ((staff as u32 * self.staff_distance) as i32) - step as i32 * (self.staff_height/8) as i32 }
	}

	let sheet = {
		let staff_height = font.units_per_em().unwrap() as u32;
		let interval = staff_height / 4;
		Sheet{
			engraving_defaults: EngravingDefaults{staff_line_thickness: 32, stem_thickness: 30, thin_barline_thickness: 40},
			staff_height,
			staff_distance: 7*interval
		}
	};

	let scale = Ratio{num: 270, div: sheet.staff_height};
	let size = size / scale;

	use music_xml::*;
	#[derive(Default)] struct Staff { clef: Option<Clef>, octave: i8 };
	let mut staves : [Staff; 2] = array::Iterator::collect(std::iter::from_fn(|| Some(Staff::default()))); //[Staff::default; 2];

	let mut fill = Vec::new();

	let mut glyph = Vec::new();
	for part in &music.score_partwise.parts {
		let mut system = xy{x: 0, y: 0};
		impl Sheet {
			fn raster<'t>(&'t self, staves: impl Iterator<Item=&'t Staff> + 't) -> impl Iterator<Item=Rect> + 't {
				staves.enumerate().map(move |(staff, _)|
					(0..=8).step_by(2).map(move |step| Rect::horizontal(self.y(staff, step), self.engraving_defaults.staff_line_thickness, 0, i32::MAX))
				).flatten()
			}
		}
		fill.extend(sheet.raster(staves.iter()));

		for measure in &part.measures {
			use MusicData::*;
			let music_data = {
				let mut buffer = measure.music_data.iter().scan((0,0), |(t, next_t), music_data| {
					if let Note(music_xml::Note{chord: Some(_), ..}) = music_data {/*Chord inhibits preceding note progress, i.e starts at the preceding note time*/}
					else { *t = *next_t; } // Normal progress
					let start = *t;
					match music_data {
						Backup(music_xml::Backup{duration}) => { *next_t = *t - duration; },
						Note(music_xml::Note{duration, ..}) => { *next_t = *t + duration; },
						_ => {},
					}
					Some((start, music_data))
				}).collect::<Vec<_>>();
				buffer.sort_by_key(|&(t,_)| t);
				buffer
			};

			use derive_more::{Deref, DerefMut};
			#[derive(Deref)] struct Measure<'t> { #[deref] sheet: &'t Sheet, fill: Vec<Rect>, glyph: Vec<Glyph> }
			impl<'t> Measure<'t> { fn new(sheet: &'t Sheet) -> Self { Self{sheet: &sheet, fill: Vec::new(), glyph: Vec::new()} } }
			let mut measure = Measure::new(&sheet);

			let mut music_data = music_data.iter().peekable();
			let (mut t, mut x) = (0, 0);
			while let Some((next_t, music_data_element)) = music_data.next() {
				if *next_t > t { x = measure.x(); }
				t = *next_t;

				impl std::fmt::Display for MusicData { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
					write!(f, "{}", match self {
						Note(_) => "Note",
						Backup(_) => "Backup",
						_ => "_",
					})
				}}
				println!("{} {} {}", t, x, music_data_element);

				impl From<&Step> for i8 { fn from(step: &Step) -> Self { use Step::*; match step { C=>0, D=>1, E=>2, F=>3, G=>4, A=>5, B=>6 } } }

				impl From<&music_xml::Staff> for usize { fn from(staff: &music_xml::Staff) -> Self { (2 - staff.0) as usize } } // 1..2 -> 1: treble .. 0: bass
				#[derive(Deref)] struct StaffRef<'t> { index: usize, #[deref] staff: &'t Staff }
				trait Index { fn index(&self, index: &music_xml::Staff) -> StaffRef; }
				impl Index for [Staff] {
					fn index(&self, index: &music_xml::Staff) -> StaffRef { let index = index.into(); StaffRef{index, staff: &self[index]} }
				}
				#[derive(Deref, DerefMut)] struct StaffMut<'t> { index: usize, #[deref]#[deref_mut] staff: &'t mut Staff }
				trait IndexMut { fn index_mut(&mut self, index: &music_xml::Staff) -> StaffMut; }
				impl IndexMut for [Staff] {
					fn index_mut(&mut self, index: &music_xml::Staff) -> StaffMut { let index = index.into(); StaffMut{index, staff: &mut self[index]} }
				}
				impl StaffMut<'_> { fn as_ref(&self) -> StaffRef { StaffRef{index: self.index, staff: &self.staff} } }

				impl Pitch {
					fn new(clef: &Clef, step: &Step) -> Self {
						use Step::*;
						match clef {
							Clef{sign: ClefSign::G,..} => Pitch{step: *step, octave: Some(match step { G|A|B => 4, C|D|E|F => 5 }), alter: None},
							Clef{sign: ClefSign::F,..} => Pitch{step: *step, octave: Some(match step { A|B => 2, C|D|E|F|G => 3 }), alter: None},
						}
					}
				}
				impl From<&Pitch> for i8 { fn from(pitch: &Pitch) -> Self { (pitch.octave.unwrap_or(4) as i8 - 4)*7 + i8::from(&pitch.step) } }

				impl Staff {
					#[allow(non_snake_case)]
					fn C4(&self) -> i8 { use ClefSign::*; (match self.clef.as_ref().unwrap().sign { G=> -2, F=> 10 }) - self.octave*7 }
					fn step(&self, pitch: &Pitch) -> i8 { self.C4() + i8::from(pitch) }
				}

				impl Measure<'_> {
					fn x(&self) -> i32 { self.glyph.last().map(|g:&Glyph| g.top_left.x+font.glyph_hor_advance(g.id).unwrap() as i32).unwrap_or(0) }
					fn push(&mut self, x: i32, staff_index: usize, step: i8, id: ttf_parser::GlyphId) {
						self.glyph.push(Glyph{top_left: xy{
							x: x + font.glyph_hor_side_bearing(id).unwrap() as i32,
							y: self.y(staff_index, step) - font.glyph_bounding_box(id).unwrap().y_max as i32,
						}, id})
					}
					fn pitch(&mut self, x:  i32, staff: StaffRef, pitch: &Pitch, id: char) {
						self.push(x, staff.index, staff.step(pitch), font.glyph_index(id).unwrap())
					}
				}

				match music_data_element {
					Backup(_) => {},
					Note(note@music_xml::Note{staff: Some(staff),..}) => {
						let mut chord = Vec::<&music_xml::Note>::new();
						chord.push(note);
						while let Some((_, Note(music_xml::Note{staff: Some(_), chord: Some(_),..}))) = music_data.peek() {
							if let Some((_, Note(note))) = music_data.next() { chord.push(note) } else { unreachable!(); }
						}
						//float opacity = allTied(beam[0]) ? 1./2 : 1;
						{ // Stem
							impl music_xml::Note {
								fn pitch(&self) -> Option<&Pitch> { if let NoteData::Pitch(pitch) = &self.content { Some(pitch) } else { None } }
								fn has_stem(&self) -> bool { self.r#type.as_ref().unwrap().value <= NoteTypeValue::Half }
							}
							let step = |note:&&music_xml::Note| note.pitch().map(|pitch| staves.index(&note.staff.unwrap()).step(&pitch));

							use framework::graphic::Bounds;
							let (bottom, top) = chord.iter().filter(|x| music_xml::Note::has_stem(x)).filter_map(step).map(|e|(e,e)).bounds().unwrap();

							let stem_thickness = sheet.engraving_defaults.stem_thickness as i32;
							let anchors = &glyphs_with_anchors.iter().find(|(id,_)| id == &note_head::black).unwrap().1;
							let staff = staves.index(staff);
							let (anchor, top, bottom) = if top-4 > 4-bottom { (StemDownNW, top, bottom-7) } else { (StemUpSE, top+7, bottom) };
							let xy{x, y: dy} = xy{x, y: 0} + anchors.iter().find(|(id,_)| id == &anchor).unwrap().1;
							if anchor == StemDownNW { // Left align
								measure.fill.push(Rect{top_left: xy{x, y: measure.y(staff.index, top)+dy}, bottom_right: xy{x: x+stem_thickness, y: measure.y(staff.index, bottom)+dy}});
							} else { // Right align
								measure.fill.push(Rect{top_left: xy{x: x-stem_thickness, y: measure.y(staff.index, top)+dy}, bottom_right: xy{x, y: measure.y(staff.index, bottom)+dy}});
							}
						}
						//if(sign.note.value>=Eighth) glyph(vec2(x, yStem), (int(sign.note.value)-Eighth)*2 + (stemUp ? SMuFL::Flag::Above : SMuFL::Flag::Below), opacity, 7);
						// Heads
						for note in chord { if let music_xml::Note{staff: Some(staff), r#type: Some(NoteType{value}), content:NoteData::Pitch(pitch), ..} = note {
							measure.pitch(x, staves.index(staff), pitch, {use {NoteTypeValue::*, note_head::*}; match value { Breve=>breve, Whole=>whole, Half=>half, _=>black }});
						} else { unreachable!() }}
					},
					Attributes(music_xml::Attributes{clefs, key, time, ..}) => {
						x = measure.x();
						for clef@Clef{staff, sign, ..} in clefs {
							let mut staff = staves.index_mut(staff);
							staff.clef = Some(*clef);
							let (id, step) = {use ClefSign::*; match sign { G=>(clef::G, Step::G), F=>(clef::F, Step::F) }};
							measure.pitch(x, staff.as_ref(), &Pitch::new(clef, &step), id);
						}
						x = measure.x();
						if let Some(Key{fifths,..}) = key {
							let mut key = |fifths:i8, symbol| {
								let mut sign = |steps: &mut dyn Iterator<Item=&Step>| {
									for step in steps {
										for (index, Staff{clef, ..}) in staves.iter().enumerate() {
											measure.pitch(x, StaffRef{index, staff: &Staff{clef: *clef, octave: 0}}, &Pitch::new(clef.as_ref().unwrap(), step), symbol);
										}
									}
								};
								let steps = {use Step::*; [B,E,A,D,G,C,F].iter()};
								if fifths>0 { sign(&mut steps.rev().take(fifths as usize)) } else { sign(&mut steps.take((-fifths) as usize)) }
							};
							//if fifths == 0 { key(system.fifths, accidental::natural) } else
							key(*fifths, if *fifths<0 { accidental::flat } else { accidental::sharp });
						}
						x = measure.x();
						if let Some(Time{beats, beat_type}) = time {
							fn time_signature_digit(digit: char) -> char { use std::convert::TryInto; u32::try_into(u32::from(time_signature)+digit.to_digit(10).unwrap()).unwrap() }
							let texts : [String; 2] = framework::array::Iterator::collect(
								[beats, beat_type].iter().map(|number| number.to_string().chars().map(time_signature_digit).collect::<String>())
							);
							let texts : [Text; 2] = framework::array::Iterator::collect(
								texts.as_ref().iter().map(|text| Text::new(&font, text, &*default_style))
							);
							let width = texts.iter().map(|text| text.size().x).max().unwrap();
							use framework::array::IntoIterator;
							for (text, step) in texts.iter().zip([6,2].into_iter()) {
								let x = x + ((width-text.size().x)/2) as i32;
								for text::Layout{x: dx, glyph: (_, id), ..} in text.font.glyphs(text.text.char_indices()).layout() {
									for index in 0..staves.len() {
										measure.push(x + dx, index, step, id);
									}
								}
							}
						}
						x = measure.x();
					},
					_ => {},
				}
			}
			x = measure.x();
			if system.x + x > size.x as i32 {
				system.x = 0;
				system.y += 2*sheet.staff_distance as i32;
				fill.extend(sheet.raster(staves.iter()).map(|mut x| { x.translate(xy{x:0, y: system.y}); x }));
			}
			/*trait Apply : Iterator { fn apply<F:Fn(&mut Self::Item), A: Iterator<Item=Self::Item>>(self, f: F) -> A; }
			impl<I:Iterator> Apply for I { fn apply<F:Fn(&mut Self::Item), A: Iterator<Item=Self::Item>>(self, f: F) -> A { self.map(|x| { f(x); x }) } }
			glyph.extend(measure.glyph.iter().apply(|mut x| x.translate(system)));*/
			fill.extend(measure.fill.into_iter().map(|mut x| { x.translate(system); x }));
			glyph.extend(measure.glyph.into_iter().map(|mut x| { x.translate(system); x }));
			if system.x > 0 { fill.push(Rect::vertical(system.x, sheet.engraving_defaults.thin_barline_thickness, system.y+sheet.y(staves.len()-1, 8), system.y+sheet.y(0, 0))); }
			system.x += x;
		}
	}
	Graphic{scale, fill, font: &font, glyph}
}

#[throws] fn main() {
	framework::rstack_self()?; framework::sigint_trace();
	//window::run(&mut GraphicView::new(|size| layout(&xml::from_document(&xml::parse(&std::fs::read("../test.xml")?)?)?, size.x)))?
	window::run(&mut graphic::Widget(|size| Ok(layout(&xml::from_document(&xml::parse(&std::fs::read("../test.xml")?)?)?, size))))?
}
