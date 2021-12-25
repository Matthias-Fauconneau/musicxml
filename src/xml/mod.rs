pub trait VecExt {
	type Item;
	fn take_first<P:Fn(&Self::Item)->bool>(&mut self, predicate: P) -> Option<Self::Item>;
}
impl<T> VecExt for Vec<T> {
	type Item = T;
	fn take_first<P:Fn(&Self::Item)->bool>(&mut self, predicate: P) -> Option<Self::Item> {
		Some(self.remove(self.iter().position(predicate)?))
	}
}

#[macro_use] mod serde;

#[derive(Debug,derive_more::Display,derive_more::From)] struct Error(String);
impl Error { pub fn msg(msg: impl std::fmt::Debug+std::fmt::Display+'static+Send+Sync) -> Error { Error(msg.to_string()) } }
impl std::error::Error for Error {}
impl ::serde::de::Error for Error { fn custom<T: std::fmt::Display>(msg: T) -> Self { Error(msg.to_string()) } }
impl From<de::value::Error> for Error { fn from(t: de::value::Error) -> Self { Error(t.to_string()) } }
impl From<std::num::ParseIntError> for Error { fn from(t: std::num::ParseIntError) -> Self { Error(t.to_string()) } }
impl From<std::num::ParseFloatError> for Error { fn from(t: std::num::ParseFloatError) -> Self { Error(t.to_string()) } }
impl From<std::str::ParseBoolError> for Error { fn from(t: std::str::ParseBoolError) -> Self { Error(t.to_string()) } }

use {fehler::throws, ::serde::de::{self, Visitor, Deserializer}};

struct DefaultDeserializer;

impl<'de> de::Deserializer<'de> for DefaultDeserializer {
	type Error = Error;
	#[throws] fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> V::Value {
		visitor.visit_map(::serde::de::value::MapDeserializer::<_,Error>::new(std::iter::empty::<(&str,DefaultDeserializer)>()))?
	}
	#[throws] fn deserialize_struct<V: Visitor<'de>>(self, _name: &'static str, fields: &'static [&'static str], visitor: V) -> V::Value {
		visitor.visit_map(::serde::de::value::MapDeserializer::new(fields.iter().map(|&field| (field, DefaultDeserializer))))?
	}
	#[throws] fn deserialize_option<V:Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_none::<Error>()? }
	::serde::forward_to_deserialize_any!{
		char bytes byte_buf str string identifier bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 unit seq map unit_struct newtype_struct tuple tuple_struct enum ignored_any}
}

///

#[throws] fn from_yes_no(s: &str) -> bool { match s { "yes" => true, "no" => false, _ => panic!("provided string was not `yes` or `no`") } }

struct TextDeserializer<'de>(&'de str);
impl<'de> Deserializer<'de> for TextDeserializer<'de> {
	type Error = Error;
	#[throws] fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_str::<Error>(self.0)? }
	#[throws] fn deserialize_option<V:Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_some(self)? }
	#[throws] fn deserialize_str<V:Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_str::<Error>(self.0)? }
	#[throws] fn deserialize_string<V:Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_string::<Error>(self.0.to_owned())? }
	#[throws] fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_u8::<Error>(self.0.parse()?)? }
	#[throws] fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_u16::<Error>(self.0.parse()?)? }
	#[throws] fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_u32::<Error>(self.0.parse()?)? }
	#[throws] fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_i8::<Error>(self.0.parse()?)? }
	#[throws] fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_i16::<Error>(self.0.parse()?)? }
	#[throws] fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_i32::<Error>(self.0.parse()?)? }
	#[throws] fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_f32::<Error>(self.0.parse()?)? }
	#[throws] fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_bool::<Error>(from_yes_no(self.0)?)? }
	#[throws] fn deserialize_enum<V: Visitor<'de>>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> V::Value {
		visitor.visit_enum(<&str as ::serde::de::IntoDeserializer<Error>>::into_deserializer(self.0))?
	}

	::serde::forward_to_deserialize_any!{
		char bytes byte_buf identifier u64 u128 i64 i128 f64 unit unit_struct newtype_struct tuple tuple_struct struct seq map ignored_any}
}

#[derive(Clone)] struct ElementDeserializer<'de> {
	name: &'de str,
	attributes: &'de [roxmltree::Attribute<'de>], //std::iter::Peekable<std::slice::Iter<'de, roxmltree::Attribute<'de>>>,
	children: std::iter::Peekable<roxmltree::Children<'de, 'de>>,
}

mod seq; use seq::{EmptySeqDeserializer, SeqDeserializer};
mod content; use content::ContentDeserializer;

impl std::fmt::Debug for ElementDeserializer<'_> {
    #[throws(std::fmt::Error)] fn fmt(&self, f: &mut std::fmt::Formatter) {
        use itertools::Itertools;
        write!(f, "{} [{:?}] {{{:?}}}", self.name, self.attributes/*.clone().format(" ")*/, self.children.clone().format(" "))?
    }
}

enum Value<'t, 'de> {
	Text(TextDeserializer<'de>),
	Element(ElementDeserializer<'de>),
	Content(ContentDeserializer<'t, 'de>),
	Seq(SeqDeserializer<'t, 'de>),
	EmptySeq(EmptySeqDeserializer),
	Default(DefaultDeserializer),
}
delegatable_trait!{Value}

impl<'de> ::serde::de::IntoDeserializer<'de, Error> for TextDeserializer<'de> { type Deserializer = Self; fn into_deserializer(self) -> Self::Deserializer { self } }
impl<'de> ::serde::de::IntoDeserializer<'de, Error> for ElementDeserializer<'de> { type Deserializer = Self; fn into_deserializer(self) -> Self::Deserializer { self } }
impl<'de> ::serde::de::IntoDeserializer<'de, Error> for &mut ElementDeserializer<'de> { type Deserializer = Self; fn into_deserializer(self) -> Self::Deserializer { self } }
impl<'t, 'de> ::serde::de::IntoDeserializer<'de, Error> for ContentDeserializer<'t, 'de> { type Deserializer = Self; fn into_deserializer(self) -> Self::Deserializer { self } }
impl<'t, 'de> ::serde::de::IntoDeserializer<'de, Error> for SeqDeserializer<'t, 'de> { type Deserializer = Self; fn into_deserializer(self) -> Self::Deserializer { self } }
impl<'de> ::serde::de::IntoDeserializer<'de, Error> for EmptySeqDeserializer { type Deserializer = Self; fn into_deserializer(self) -> Self::Deserializer { self } }
impl<'de> ::serde::de::IntoDeserializer<'de, Error> for DefaultDeserializer { type Deserializer = Self; fn into_deserializer(self) -> Self::Deserializer { self } }
impl<'t, 'de> ::serde::de::IntoDeserializer<'de, Error> for Value<'t, 'de> { type Deserializer = Self; fn into_deserializer(self) -> Self::Deserializer { self } }

impl<'de> ElementDeserializer<'de> {
    fn new(node: roxmltree::Node<'de, 'de>) -> Self {
		assert!(node.is_element() || node.is_root(), "{:?}", node);
		Self{name: node.tag_name().name(), attributes: node.attributes()/*.iter().peekable()*/, children: node.children().peekable()}
	}

	#[throws] fn deserialize_struct<V: Visitor<'de>>(&mut self, _name: &'static str, fields: &'static [&'static str], visitor: V) -> V::Value {
		let mut attributes = self.attributes.iter();
		let mut fields = fields.iter().map(|&field| (field, field.split_at(field.find(|c| "@$?*+{".contains(c)).unwrap_or(field.len())))).collect::<Vec<_>>();
		let cell = std::cell::RefCell::new(self);
		visitor.visit_map(::serde::de::value::MapDeserializer::new(std::iter::from_fn(|| {
			let mut node = cell.borrow_mut();
			while let Some(a) = attributes.next() {
				if let Some((field,_)) = fields.take_first(|(_,(name,_))| name == &a.name()) {
					return Some((field, Value::Text(TextDeserializer(a.value()))));
				}
				else if let Some(index) = fields.iter().position(|(field,(_,def))| field.is_empty() || def==&"?" ) {
					let (field,(_,_def)) = fields[index];
					fields.remove(index);
					return Some((field, Value::Content(ContentDeserializer(node)))); // Flatten
				}
			}
			while let Some(child) = node.children.peek() {
				let name = child.tag_name().name();
				if !name.is_empty() {
					if let Some((field,(tag,def))) = fields.take_first(|(_,(id,_))| id == &name) {
						if !def.is_empty() {
							return Some((field, Value::Seq(SeqDeserializer{node, tag}))); // External sequence
						} else {
							use roxmltree::NodeType::*; match child.node_type() {
								Text => return Some((field, Value::Text(TextDeserializer(node.children.next().unwrap().text().unwrap())))),
								Element => return Some((field, Value::Element(ElementDeserializer::new(node.children.next().unwrap())))),
								_ => todo!(),
							}
						}
					}
				}/*else*/ if child.is_element() /*&&*/{ if let Some(index) = fields.iter().position(|(_,(id,_))| id.is_empty() /*|| id.parse()==Ok(index)*/) {
					let (field,(_,_def)) = fields[index];
					fields.remove(index);
					return Some((field, Value::Content(ContentDeserializer(node)))); // External enum tag
				}} /*else*/ if child.is_text() { if let Some((field,_)) = fields.take_first(|(_,(_,def))| def==&"$") {
					return Some((field, Value::Text(TextDeserializer(node.children.next().unwrap().text().unwrap())))); // External enum tag
				}} /*else*/ {
					if child.is_comment() || (child.is_text() && child.text().unwrap().trim().is_empty()) {
						node.children.next();
					} else if let Some((field,_)) = fields.take_first(|(_,(_,def))| def==&"*" || def.starts_with("{0,")) {
						return Some((field, Value::EmptySeq(EmptySeqDeserializer)));
					} else if let Some((field,_)) = fields.take_first(|(_,(_,def))| def==&"?") {
						return Some((field, Value::Default(DefaultDeserializer)));
					} else {
						return None;
					}
				}
			}
			if let Some((field,_)) = fields.take_first(|(_,(_,def))| def==&"*" || def.starts_with("{0,")) {
				Some((field, Value::EmptySeq(EmptySeqDeserializer)))
			} else if let Some((field,_)) = fields.take_first(|(_,(_,def))| def==&"?") {
				Some((field, Value::Default(DefaultDeserializer)))
			} else {
				None
			}
		})))?
	}

	#[throws] fn simple_content(&mut self) -> &'de str {
		if let Some(text) = self.children.next() {
			assert!(text.is_text() && self.children.peek().is_none() && self.attributes.is_empty(), "Expected simple content got {self:?} {text:?}");
			text.text().unwrap()
		} else {
			"" // Empty content yields empty string
		}
    }
}

impl<'de> Deserializer<'de> for &mut ElementDeserializer<'de> {
	type Error = Error;
	#[throws] fn deserialize_unit<V:Visitor<'de>>(self, visitor: V) -> V::Value {
		assert!(self.attributes.is_empty() && self.children.next().is_none());
		visitor.visit_unit::<Error>()?
	}
	#[throws] fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> V::Value { TextDeserializer(self.simple_content()?).deserialize_str(visitor)? }
	#[throws] fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> V::Value { TextDeserializer(self.simple_content()?).deserialize_string(visitor)? }
	#[throws] fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> V::Value { TextDeserializer(self.simple_content()?).deserialize_u8(visitor)? }
	#[throws] fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> V::Value { TextDeserializer(self.simple_content()?).deserialize_u16(visitor)? }
	#[throws] fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> V::Value { TextDeserializer(self.simple_content()?).deserialize_u32(visitor)? }
	#[throws] fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> V::Value { TextDeserializer(self.simple_content()?).deserialize_i8(visitor)? }
	#[throws] fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> V::Value { TextDeserializer(self.simple_content()?).deserialize_i16(visitor)? }
	#[throws] fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> V::Value { TextDeserializer(self.simple_content()?).deserialize_i32(visitor)? }
	#[throws] fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> V::Value { TextDeserializer(self.simple_content()?).deserialize_f32(visitor)? }
	#[throws] fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> V::Value { TextDeserializer(self.simple_content()?).deserialize_bool(visitor)? }

	#[throws] fn deserialize_option<V:Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_some(self)? }

	#[throws] fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> V::Value {
		visitor.visit_seq(::serde::de::value::SeqDeserializer::new(self.children.by_ref().filter(|child| child.is_element()).map(|child| ElementDeserializer::new(child))))?
	}

	#[allow(unreachable_code)] #[throws] fn deserialize_map<V: Visitor<'de>>(self, _visitor: V) -> V::Value { unreachable!() }

	#[throws] fn deserialize_struct<V: Visitor<'de>>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> V::Value {
		let value = self.deserialize_struct(name, fields, visitor)?;
		use itertools::Itertools;
		while self.children.peek().filter(|child| child.is_text() && child.text().unwrap().trim().is_empty()).is_some() { self.children.next(); }
		assert!(self.children.peek().is_none(), "Remaining elements '{:?}' in {name}", self.children.clone().format(" "));
		value
	}

	#[allow(unreachable_code)] #[throws] fn deserialize_enum<V: Visitor<'de>>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> V::Value {
		if name ==  self.name {
			TextDeserializer(self.simple_content()?).deserialize_enum(name, variants, visitor)?
		} else {
			visitor.visit_enum(::serde::de::value::MapAccessDeserializer::new(::serde::de::value::MapDeserializer::new(std::iter::once((self.name, self)))))?
		}
    }

    #[throws] fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> V::Value { self.deserialize_map(visitor)? }
	::serde::forward_to_deserialize_any!{char bytes byte_buf identifier u64 u128 i64 i128 f64 unit_struct newtype_struct tuple tuple_struct ignored_any}
}

impl<'de> Deserializer<'de> for ElementDeserializer<'de> {
	type Error = Error;
	#[throws] fn deserialize_any<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_any(&mut self, visitor)? }
    #[throws] fn deserialize_unit<V:Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_unit(&mut self, visitor)? }
	#[throws] fn deserialize_str<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_str(&mut self, visitor)? }
	#[throws] fn deserialize_string<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_string(&mut self, visitor)? }
	#[throws] fn deserialize_u8<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_u8(&mut self, visitor)? }
	#[throws] fn deserialize_u16<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_u16(&mut self, visitor)? }
	#[throws] fn deserialize_u32<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_u32(&mut self, visitor)? }
	#[throws] fn deserialize_i8<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_i8(&mut self, visitor)? }
	#[throws] fn deserialize_i16<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_i16(&mut self, visitor)? }
	#[throws] fn deserialize_i32<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_i32(&mut self, visitor)? }
	#[throws] fn deserialize_f32<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_f32(&mut self, visitor)? }
	#[throws] fn deserialize_option<V:Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_option(&mut self, visitor)? }
	#[throws] fn deserialize_seq<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_seq(&mut self, visitor)? }
	#[throws] fn deserialize_map<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { <&mut Self as Deserializer>::deserialize_map(&mut self, visitor)? }
	#[throws] fn deserialize_struct<V: Visitor<'de>>(mut self, name: &'static str, fields: &'static [&'static str], visitor: V) -> V::Value {
		<&mut Self as Deserializer>::deserialize_struct(&mut self, name, fields, visitor)?
	}
	#[throws] fn deserialize_enum<V: Visitor<'de>>(mut self, name: &'static str, variants: &'static [&'static str], visitor: V) -> V::Value {
		<&mut Self as Deserializer>::deserialize_enum(&mut self, name, variants, visitor)?
    }
	::serde::forward_to_deserialize_any!{char bytes byte_buf identifier bool u64 u128 i64 i128 f64 unit_struct newtype_struct tuple tuple_struct ignored_any}
}

#[throws(crate::Error)] pub fn from_node<'input: 'de, 't: 'de, 'de, T: ::serde::Deserialize<'de>>(node: roxmltree::Node<'t, 'input>) -> T {
	T::deserialize(ElementDeserializer::new(node))?
}
#[throws(crate::Error)] pub fn from_document<'input: 'de, 'de, T: ::serde::Deserialize<'de>>(document: &'de roxmltree::Document<'input>) -> T {
	from_node(document.root())?
}
#[throws(crate::Error)] pub fn parse(bytes: &[u8]) -> roxmltree::Document { roxmltree::Document::parse(std::str::from_utf8(bytes)?)? }
