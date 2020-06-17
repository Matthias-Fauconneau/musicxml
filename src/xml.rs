#![allow(unreachable_code)]
pub trait OptionExt<T> { fn try_map<E, U, F:FnOnce(T)->Result<U, E>>(self, f: F) -> Result<Option<U>, E>; }
impl<T> OptionExt<T> for Option<T> {
	fn try_map<E, U, F:FnOnce(T) ->Result<U, E>>(self, f: F) -> Result<Option<U>, E> { self.map(f).transpose() }
}

mod error {
#[derive(Debug)] pub struct Error(anyhow::Error);
impl std::fmt::Display for Error { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.0.fmt(f) } }
impl std::error::Error for Error {}
impl serde::de::Error for Error { fn custom<T: std::fmt::Display>(msg: T) -> Self { Error(anyhow::Error::msg(msg.to_string())) } }
}

/// ~serde/quick-xml with roxmltree

pub struct Deserializer<'t, 'input> {
	node: roxmltree::Node<'t, 'input>,
}

use {fehler::*, serde::de::{self, Error, Visitor, Deserializer as deserialize, DeserializeSeed, IntoDeserializer}};

impl<'de, 't> serde::Deserializer<'de> for /*&'t mut*/ Deserializer<'t, 'de> {
	type Error = error::Error;

	fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value,Self::Error> {
		throw!(Self::Error::invalid_type(de::Unexpected::Other("unimplemented"), &visitor))
	}
	serde::forward_to_deserialize_any!{
		char bytes byte_buf str string identifier bool
		u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64
		option unit unit_struct newtype_struct tuple tuple_struct map ignored_any
	}

	#[throws(Self::Error)] fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> V::Value {
		struct Seq<'t, 'de: 't>(/*&'t mut*/ Deserializer<'t, 'de>);
		impl<'de, 't> de::SeqAccess<'de> for Seq<'t, 'de> {
			type Error = error::Error;
			#[throws(Self::Error)] fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, _seed: T) -> Option<T::Value> {
				panic!("next_element_seed") //seed.deserialize(&mut *self.0)?
			}
		}
		visitor.visit_seq(Seq(/*&mut **/self))?
	}

	#[throws(Self::Error)]
	fn deserialize_struct<V: Visitor<'de>>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> V::Value { //Result<V::Value,Error> {
		enum Value<'t, 'input: 't> { Attribute(&'t str), Descendant(roxmltree::Node<'t, 'input>) }
		/*impl<'de, 't> std::ops::Deref for Value<'t, 'de> {
			type Target = dyn IntoDeserializer<'de, error::Error, Deserializer = &'t mut self::Deserializer<'t, 'de, error::Error>>;
			fn deref(&self) -> &Self::Target {
				match self {
					Attribute(str) => str,
					Descendant(node) => node
				}
			}
		}*/
		/*impl<'de, 't> IntoDeserializer<'de, error::Error> for Value<'t, 'de> {
			type Deserializer = /*&'t mut*/ self::Deserializer<'t, 'de, error::Error>;
			fn into_deserializer(self) -> Self::Deserializer { match self {
				Attribute(str) => str.into_deserializer(),
				Descendant(node) => node.into_deserializer()
			} }
		}*/
		struct MapAccess<'t, 'input, I> {
			iter: I,
			next_value: Option<Value<'t, 'input>>,
		}
		impl<'de, 't, I:Iterator<Item=(&'t str, Value<'t, 'de>)>> de::MapAccess<'de> for MapAccess<'t, 'de, I> {
			type Error = error::Error;

			#[throws(error::Error)] fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Option<K::Value> {
				self.iter.next().try_map(/*#[throws]*/|(name, value)| {
					self.next_value = Some(value);
					Ok(seed.deserialize(<&str as IntoDeserializer<Self::Error>>::into_deserializer(name))?)
				})?
			}
			#[throws(error::Error)] fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> V::Value {
				//seed.deserialize(<Value as IntoDeserializer<Self::Error>>::into_deserializer(self.next_value.unwrap()))?
				match self.next_value.take().unwrap() {
					Attribute(str) => seed.deserialize(<&str as IntoDeserializer<Self::Error>>::into_deserializer(str)),
					Descendant(node) => seed.deserialize(Deserializer{node}),
				}?
			}
		}
		use Value::*;
		visitor.visit_map(MapAccess{
			iter: 				self.node.attributes().iter().map(|attribute| (attribute.name(), Attribute(attribute.value())))
					.chain( self.node.descendants().map(|descendant| (descendant.tag_name().name(), Descendant(descendant))) ),
			next_value: None,
		})?
    }

	#[throws(Self::Error)] fn deserialize_enum<V: Visitor<'de>>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> V::Value {
		struct Enum<'t, 'de: 't>(Deserializer<'t, 'de>);
		impl<'de, 't> de::EnumAccess<'de> for Enum<'t, 'de> {
			type Error = error::Error;
			type Variant = Self;
			#[throws(Self::Error)] fn variant_seed<V: DeserializeSeed<'de>>(self, _seed: V) -> (V::Value, Self::Variant) {
				unimplemented!() //(seed.deserialize(&mut *self.0)?, self)
			}
		}
		impl<'de, 'a> de::VariantAccess<'de> for Enum<'a, 'de> {
			type Error = error::Error;
			//#[allow(unreachable_code)]
			#[throws(Self::Error)] fn unit_variant(self) {}
			#[throws(Self::Error)] fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> T::Value { seed.deserialize(self.0)? }
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

#[throws(anyhow::Error)] pub fn from_bytes<'t, T: serde::Deserialize<'t>>(bytes: &'t [u8]) -> T {
	T::deserialize(Deserializer{node: roxmltree::Document::parse(std::str::from_utf8(&bytes)?)?.root_element()})?
}
