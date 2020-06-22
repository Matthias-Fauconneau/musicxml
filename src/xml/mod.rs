pub trait OptionExt<T> { fn try_map<E, U, F:FnOnce(T)->Result<U, E>>(self, f: F) -> Result<Option<U>, E>; }
impl<T> OptionExt<T> for Option<T> {
	fn try_map<E, U, F:FnOnce(T) ->Result<U, E>>(self, f: F) -> Result<Option<U>, E> { self.map(f).transpose() }
}

#[macro_use] mod serde;

#[derive(Debug)] pub struct Error(anyhow::Error);
impl std::fmt::Display for Error { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.0.fmt(f) } }
impl std::error::Error for Error {}
impl ::serde::de::Error for Error { fn custom<T: std::fmt::Display>(msg: T) -> Self { Error(anyhow::Error::msg(msg.to_string())) } }
impl From<de::value::Error> for Error { fn from(t: de::value::Error) -> Self { ::serde::de::Error::custom(t) } }
impl From<anyhow::Error> for Error { fn from(t: anyhow::Error) -> Self { Error(t) } }
impl From<std::num::ParseIntError> for Error { fn from(t: std::num::ParseIntError) -> Self { ::serde::de::Error::custom(t) } }
impl From<std::num::ParseFloatError> for Error { fn from(t: std::num::ParseFloatError) -> Self { ::serde::de::Error::custom(t) } }
macro_rules! bail { ($($arg:tt)*) => { throw!(<Error as ::serde::de::Error>::custom(format!($($arg)*))) } }
macro_rules! ensure { ($cond:expr, $($arg:tt)*) => { if !$cond { bail!($($arg)*) } } }

mod content; use content::ContentDeserializer;

use {fehler::*, ::serde::de::{self, Visitor, Deserializer, IntoDeserializer}};

///

struct AttributeDeserializer<'de>(&'de str);
impl<'de> Deserializer<'de> for AttributeDeserializer<'de> {
	type Error = Error;
	#[throws] fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_u32::<Self::Error>(self.0.parse()?)? }
	#[throws] fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_f32::<Self::Error>(self.0.parse()?)? }
	#[throws] fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_str::<Self::Error>(self.0)? }
	::serde::forward_to_deserialize_any!{
		char bytes byte_buf str string identifier bool
		u8 u16 u64 u128 i8 i16 i32 i64 i128 f64
		option unit unit_struct newtype_struct tuple tuple_struct struct enum seq map ignored_any
	}
}

pub struct NodeDeserializer<'de> {
	name: &'de str,
	attributes: std::iter::Peekable<std::slice::Iter<'de, roxmltree::Attribute<'de>>>,
	children: std::iter::Peekable<roxmltree::Children<'de, 'de>>,
}

impl std::fmt::Debug for NodeDeserializer<'_> {
    #[throws(std::fmt::Error)] fn fmt(&self, f: &mut std::fmt::Formatter) {
        use itertools::Itertools;
        write!(f, "{} [{:?}] {{{:?}}}", self.name, self.attributes.clone().format(" "), self.children.clone().format(" "))?
    }
}

impl<'de> NodeDeserializer<'de> {
    fn new(node: roxmltree::Node<'de, 'de>) -> Self { Self{name: node.tag_name().name(), attributes: node.attributes().iter().peekable(), children: node.children().peekable()} }
}

enum Value<'de> { Attribute(AttributeDeserializer<'de>), Node(NodeDeserializer<'de>), Content(ContentDeserializer<'de>) }
delegatable_trait!{Value}
IntoDeserializer_for_Deserializer!{AttributeDeserializer}
IntoDeserializer_for_Deserializer!{NodeDeserializer}
IntoDeserializer_for_Deserializer!{ContentDeserializer}
IntoDeserializer_for_Deserializer!{Value}

impl<'de> Deserializer<'de> for NodeDeserializer<'de> {
	type Error = Error;
	#[throws] fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> V::Value { println!("{}", &visitor as &dyn de::Expected); self.deserialize_map(visitor)? }

	#[throws] fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> V::Value {
		println!("seq");
		visitor.visit_seq(::serde::de::value::SeqDeserializer::new(self.children.filter(|child| child.is_element()).map(|child| {
			println!("item");
			/*Item*/ContentDeserializer(NodeDeserializer::new(child)) // Item flatten => tag enum
		})))?
	}

	#[throws] fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> V::Value { self.deserialize_struct("", &[], visitor)? }

	#[throws] fn deserialize_struct<V: Visitor<'de>>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> V::Value {
		println!("struct '{}' {:?}", name, fields);
		assert_eq!(self.name, name, "expected struct '{}', got '{}'", name, self.name);
		visitor.visit_map(::serde::de::value::MapDeserializer::new( std::iter::from_fn({let mut node = Some(self); move || {
			if let Some(Some(a)) = node.as_mut().map(|x| x.attributes.next().map(|a| {
				println!("attribute {}", a.name());
				(a.name(), Value::Attribute(AttributeDeserializer(a.value())))
			} ) ) { Some(a) }
			else {
				loop {
					if let Some(Some(child)) = node.as_mut().map(|n| n.children.peek()) {
						let name = child.tag_name().name();
						if fields.contains(&name) {
							println!("field {}", name);
							break Some((name, Value::Node(NodeDeserializer::new(node.as_mut().unwrap().children.next().unwrap()))));
						} else if fields.contains(&"$content") && child.is_element() {
							println!("no field '{}' in {:?}, deserializing to $content", name, fields);
							break Some(("$content", Value::Content(ContentDeserializer(node.take().unwrap())))); // Content flatten => tag enum
						} else { node.as_mut().unwrap().children.next(); }
					} else { break None; }
				}
			}
		}})))?
	}

	#[throws] fn deserialize_enum<V: Visitor<'de>>(mut self, name: &'static str, variants: &'static [&'static str], visitor: V) -> V::Value {
		println!("enum '{}' {:?} {:?}", name, variants, self.name);
		if name ==  self.name {
			let text = self.children.next().ok_or_else(|| anyhow::Error::msg("Expected variant"))?;
			assert!(text.is_text() && self.children.next().is_none() && self.attributes.next().is_none(), "{:?}", self);
			visitor.visit_enum(<&str as IntoDeserializer<Error>>::into_deserializer(text.text().unwrap()))?
		} else {
			visitor.visit_enum(::serde::de::value::MapAccessDeserializer::new(::serde::de::value::MapDeserializer::new(std::iter::once((self.name, self)))))?
		}
    }

    #[throws] fn deserialize_option<V:Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_some(self)? }
    #[throws] fn deserialize_unit<V:Visitor<'de>>(mut self, visitor: V) -> V::Value {
		assert!(self.attributes./*is_empty*/next().is_none() && self.children.next().is_none());
		visitor.visit_unit::<Self::Error>()?
	}

	::serde::forward_to_deserialize_any!{
		char bytes byte_buf str string identifier bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 unit_struct newtype_struct tuple tuple_struct ignored_any
	}
}

#[throws(anyhow::Error)] pub fn from_node<'input: 'de, 't: 'de, 'de, T: ::serde::Deserialize<'de>>(node: roxmltree::Node<'t, 'input>) -> T {
	T::deserialize(NodeDeserializer::new(node))?
}
#[throws(anyhow::Error)] pub fn from_document<'input: 'de, 'de, T: ::serde::Deserialize<'de>>(document: &'de roxmltree::Document<'input>) -> T {
	from_node(document.root())?
}
#[throws(anyhow::Error)] pub fn parse(bytes: &[u8]) -> roxmltree::Document { roxmltree::Document::parse(std::str::from_utf8(bytes)?)? }
