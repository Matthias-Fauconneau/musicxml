//#![allow(unreachable_code)]
mod error {
#[derive(Debug)] pub struct Error(anyhow::Error);
impl std::fmt::Display for Error { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.0.fmt(f) } }
impl std::error::Error for Error {}
impl serde::de::Error for Error { fn custom<T: std::fmt::Display>(msg: T) -> Self { Error(anyhow::Error::msg(msg.to_string())) } }
}

/// ~serde/quick-xml with roxmltree

pub struct Deserializer<'de> {
	attributes: std::iter::Peekable<std::slice::Iter<'de, roxmltree::Attribute<'de>>>,
	children: std::iter::Peekable<roxmltree::Children<'de, 'de>>
}

impl<'de> Deserializer<'de> {
	fn new(node: roxmltree::Node<'de, 'de>) -> Self { Self{attributes: node.attributes().iter().peekable(), children: node.children().peekable()} }
}

use {fehler::*, serde::de::{self, Visitor, Deserializer as deserialize, DeserializeSeed, IntoDeserializer, Error as invalid_type}};

impl<'t, 'de> serde::Deserializer<'de> for &'t mut Deserializer<'de> {
	type Error = error::Error;

	fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value,Self::Error> {
		println!("{}", &visitor as &dyn de::Expected);
		self.deserialize_struct("", &[], visitor)
	}

	serde::forward_to_deserialize_any!{
		char bytes byte_buf str string identifier bool
		u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64
		option unit unit_struct newtype_struct tuple tuple_struct map ignored_any
	}

	#[throws(Self::Error)] fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> V::Value {
		struct Seq<'t, 'de: 't>{name: Option<&'t str>, iter: &'t mut std::iter::Peekable<roxmltree::Children<'de, 'de>>}
		impl<'t, 'de> de::SeqAccess<'de> for Seq<'t, 'de> {
			type Error = error::Error;
			#[throws(Self::Error)] fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Option<T::Value> {
				if let Some(child) = self.iter.peek() { if self.name.map_or(true, |name| child.tag_name().name() == name) {
					Some(seed.deserialize(&mut Deserializer::new(self.iter.next().unwrap()))?)
				} else { None } } else { None }
			}
		}
		visitor.visit_seq(Seq{name: self.children.peek().map(|c| c.tag_name().name()), iter: &mut self.children})?
	}

	#[throws(Self::Error)] fn deserialize_struct<V: Visitor<'de>>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> V::Value {
		println!("struct {} {:?}", _name, _fields);
		impl<'t, 'de> de::MapAccess<'de> for Deserializer<'de> {
			type Error = error::Error;

			#[throws(error::Error)] fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Option<K::Value> {
				if let Some(a) = self.attributes.peek() {
					Some(seed.deserialize(<&str as IntoDeserializer<Self::Error>>::into_deserializer(a.name()))?)
				} else
				if let Some(c) = self.children.peek() {
					Some(seed.deserialize(<&str as IntoDeserializer<Self::Error>>::into_deserializer(c.tag_name().name()))?)
				} else
				{ None }
			}
			#[throws(Self::Error)] fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> V::Value {
				if let Some(a) = self.attributes.next() {
					seed.deserialize(<&str as IntoDeserializer<Self::Error>>::into_deserializer(a.value()))?
				} else
				{ seed.deserialize(&mut *self)? }
				//{ throw!(Self::Error::custom("Missing value for key")) }
			}
		}
		let child = self.children.next().ok_or_else(|| Self::Error::invalid_type(de::Unexpected::Other("End"), &visitor))?;
		visitor.visit_map(Deserializer::new(child))?
    }

	#[throws(Self::Error)] fn deserialize_enum<V: Visitor<'de>>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> V::Value {
		struct Enum<'t, 'de>(&'t mut Deserializer<'de>);
		impl<'t, 'de> de::EnumAccess<'de> for Enum<'t, 'de> {
			type Error = error::Error;
			type Variant = Self;
			#[throws(Self::Error)] fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> (V::Value, Self::Variant) { (seed.deserialize(&mut *self.0)?, self) }
		}
		impl<'t, 'de> de::VariantAccess<'de> for Enum<'t, 'de> {
			type Error = error::Error;
			#[throws(Self::Error)] fn unit_variant(self) {}
			#[throws(Self::Error)] fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> T::Value { seed.deserialize(&mut *self.0)? }
			#[throws(Self::Error)] fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> V::Value { self.0.deserialize_seq(visitor)? }
			#[throws(Self::Error)] fn struct_variant<V: Visitor<'de>>(self, _fields: &'static [&'static str], visitor: V) -> V::Value { self.0.deserialize_map(visitor)? }
		}
		visitor.visit_enum(Enum(self))?
    }

    /*#[throws] fn deserialize_identifier<V:Visitor<'de>>(self, visitor: V) -> V::Value {
    	//visitor.visit_borrowed_str::<Error>(std::str::from_utf8(id).map_err(|e| Error(e.into()))?)?
	}
	#[throws] fn deserialize_str<V:Visitor<'de>>(self, visitor: V) -> V::Value { self.deserialize_identifier(visitor)? }
	#[throws] fn deserialize_string<V:Visitor<'de>>(self, visitor: V) -> V::Value {
    	//visitor.visit_string::<Error>(String::from_utf8(path.iter().map(|&b| if b==b'\\' { b'/' } else { b }).collect()).map_err(|e| Error(e.into()))?)?
	}
	#[throws] fn deserialize_u8<V:Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_u8()? }
	#[throws] fn deserialize_i8<V:Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_i8()? }
	#[throws] fn deserialize_f32<V:Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_f32()? }

	/*#[throws] fn deserialize_tuple_struct<V:Visitor<'de>>(self, _name: &'static str, len: usize, visitor: V) -> V::Value {
		struct Seq<'t, 'de: 't>{de: &'t mut Deserializer<'de>, remaining: usize};
		impl<'de, 't> de::SeqAccess<'de> for Seq<'t, 'de> {
			type Error = Error;
			#[throws] fn next_element_seed<T:de::DeserializeSeed<'de>>(&mut self, seed: T) -> Option<T::Value> {
				if self.remaining>0 { self.remaining -= 1; Some(seed.deserialize(&mut *self.de)?) } else { None }
			}
		}
		visitor.visit_seq(Seq{de: &mut *self, remaining: len})?
	}*/
	#[throws] fn deserialize_newtype_struct<V:Visitor<'de>>(self, _name: &'static str, visitor: V) -> V::Value { visitor.visit_newtype_struct(self)? }*/
}

#[throws(anyhow::Error)] pub fn from_node<'input: 'de, 't: 'de, 'de, T: serde::Deserialize<'de>>(node: roxmltree::Node<'t, 'input>) -> T {
	T::deserialize(&mut Deserializer::new(node))?
}
#[throws(anyhow::Error)] pub fn from_document<'input: 'de, 'de, T: serde::Deserialize<'de>>(document: &'de roxmltree::Document<'input>) -> T {
	from_node(document.root())?
}
#[throws(anyhow::Error)] pub fn parse(bytes: &[u8]) -> roxmltree::Document { roxmltree::Document::parse(std::str::from_utf8(bytes)?)? }
