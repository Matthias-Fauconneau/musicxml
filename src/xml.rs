pub trait OptionExt<T> { fn try_map<E, U, F:FnOnce(T)->Result<U, E>>(self, f: F) -> Result<Option<U>, E>; }
impl<T> OptionExt<T> for Option<T> {
	fn try_map<E, U, F:FnOnce(T) ->Result<U, E>>(self, f: F) -> Result<Option<U>, E> { self.map(f).transpose() }
}

mod error {
#[derive(Debug)] pub struct Error(anyhow::Error);
impl std::fmt::Display for Error { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.0.fmt(f) } }
impl std::error::Error for Error {}
impl serde::de::Error for Error { fn custom<T: std::fmt::Display>(msg: T) -> Self { Error(anyhow::Error::msg(msg.to_string())) } }
impl From<anyhow::Error> for Error { fn from(t: anyhow::Error) -> Self { Error(t) } }
impl From<std::num::ParseIntError> for Error { fn from(t: std::num::ParseIntError) -> Self { serde::de::Error::custom(t) } }
impl From<std::num::ParseFloatError> for Error { fn from(t: std::num::ParseFloatError) -> Self { serde::de::Error::custom(t) } }
}

/// ~serde/quick-xml with roxmltree

//#[derive(Clone)]
//pub struct Deserializer<'t, 'de, I: Iterator<Item: 'de>> (&'t mut std::iter::Peekable<I>);
pub struct Deserializer<'t, 'de, I: Iterator<Item=roxmltree::Node<'de, 'de>>> (&'t mut std::iter::Peekable<I>);

/*impl std::fmt::Debug for Deserializer<'_> {
	#[throws(std::fmt::Error)] fn fmt(&self, f: &mut std::fmt::Formatter) {
		use itertools::Itertools;
		write!(f, "{{{:?}}}", self.0.clone().format(" "))?
	}
}*/

/*impl<'de> Deserializer<'de> {
	fn new(node: roxmltree::Node<'de, 'de>) -> Self { Self(node.children().peekable()) }
}*/

use {fehler::*, serde::de::{self, Visitor, Deserializer as deserialize, DeserializeSeed, IntoDeserializer, Error as invalid_type}};

impl<'t, 'de, I: Iterator<Item=roxmltree::Node<'de, 'de>>> de::Deserializer<'de> for Deserializer<'t, 'de, I> {
	type Error = error::Error;

	#[throws(Self::Error)] fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> V::Value {
		println!("any expects {}", &visitor as &dyn de::Expected);
		self.deserialize_struct("", &[], visitor)?
	}

	serde::forward_to_deserialize_any!{
		char bytes byte_buf str string identifier bool
		u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64
		option unit unit_struct newtype_struct tuple tuple_struct map ignored_any
	}

	#[throws(Self::Error)] fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> V::Value {
		struct Seq<'t, 'de: 't, I: Iterator<Item=roxmltree::Node<'de, 'de>>>{name: Option<&'t str>, iter: &'t mut std::iter::Peekable<I>}
		impl<'t, 'de, I: Iterator<Item=roxmltree::Node<'de, 'de>>> de::SeqAccess<'de> for Seq<'t, 'de, I> {
			type Error = error::Error;
			#[throws(Self::Error)] fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Option<T::Value> {
				println!("element '{:?}'", self.iter.peek());
				if let Some(child) = self.iter.peek() { if self.name.map_or(true, |name| child.tag_name().name() == name) {
					//Some(seed.deserialize(Deserializer(&mut self.iter.next().unwrap().children().peekable()))?)
					Some(seed.deserialize(Deserializer(&mut self.iter))?) // deserialize_struct enter nodes, seq is external
				} else { None } } else { None }
			}
		}
		println!("seq '{:?}'", self.0.peek().map(|c| c.tag_name().name()));
		visitor.visit_seq(Seq{name: self.0.peek().map(|c| c.tag_name().name()), iter: self.0})?
	}

	#[throws(Self::Error)]
	fn deserialize_struct<V: Visitor<'de>>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> V::Value {//Result<V::Value,Self::Error> {
		struct Struct<'de, C: Iterator> {
			fields: &'static [&'static str],
			attributes: std::slice::Iter<'de, roxmltree::Attribute<'de>>,
			children: std::iter::Peekable<C>,
			next_value: Option<&'de str>,
		}
		//use itertools::Itertools;
		// std::iter::Filter<roxmltree::Children<'de, 'de>, for<'r0,'r1,'r2> fn(&'r0 roxmltree::Node<'r1,'r2>)->bool>
		impl<'de, C: Iterator<Item=roxmltree::Node<'de, 'de>>> de::MapAccess<'de> for Struct<'de, C> {
			type Error = error::Error;

			//#[throws(Self::Error)]
			fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> //Option<K::Value> {
			Result<Option<K::Value>,Self::Error> {
								 self.attributes.next().map(|a| (a.name(), Some(a.value())) )
				.or_else(|| self.children .peek().map(|c| (c.tag_name().name(), None) ))
				.try_map(|(name,value)| {
					println!("key '{}'", name);
					assert!(!name.is_empty());
					self.next_value = value;
					seed.deserialize(if self.fields.contains(&name) { name } else { "$content" }.into_deserializer())
				})//?
			}
			//#[throws(Self::Error)]
			fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> //V::Value {
			Result<V::Value, Self::Error> {
				//println!("value from '{:?}'", self.next_value.as_ref().map(|v| v as &dyn std::fmt::Debug).unwrap_or(&self.children.clone().format(" ")));
				if let Some(value) = self.next_value {
					//seed.deserialize(<&str as IntoDeserializer<Self::Error>>::into_deserializer(value))?
					struct Deserializer<'de>(&'de str);
					impl<'de> de::Deserializer<'de> for Deserializer<'de> {
						type Error = error::Error;
						#[throws(Self::Error)] fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_u32::<Self::Error>(self.0.parse()?)? }
						#[throws(Self::Error)] fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_f32::<Self::Error>(self.0.parse()?)? }
						#[throws(Self::Error)] fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> V::Value { visitor.visit_str::<Self::Error>(self.0)? }
						serde::forward_to_deserialize_any!{
							char bytes byte_buf str string identifier bool
							u8 u16 u64 u128 i8 i16 i32 i64 i128 f64
							option unit unit_struct newtype_struct tuple tuple_struct struct enum seq map ignored_any
						}
					}
					seed.deserialize(Deserializer(value))
				} else {
					println!("value peek '{:?}'", self.children.peek());
					seed.deserialize(Deserializer(&mut self.children))
				}
			}
		}

		println!("struct '{}' {:?}", name, fields);
		//println!("in {:?}", self.0.clone().format(" "));
		//let context = self.clone(); // Clone iterators before consumption to be reported on missing tag
		//use anyhow::Context;
		//assert!(!name.is_empty());
		let child = self.0 .by_ref().filter(|e| e.is_element()) .filter(|e| name.is_empty() || e.tag_name().name() == name) .next()
			.ok_or_else(|| Self::Error::invalid_type(de::Unexpected::Other("End"), &visitor))?; //.with_context(|| format!("({:?})", context))?;
		println!("from {:?}", &child);
		//macro_rules! trace { {$e:expr} => { let v = $e; eprintln!("{}", line!()); v } } // breaks fehler
		//trace!{ visitor.visit_map(Deserializer::new(child))? }
		visitor.visit_map(Struct{
			fields,
			attributes: child.attributes().iter(),
			children: child.children().filter(roxmltree::Node::is_element).peekable(),
			next_value: None,
		})?
	}

	#[throws(Self::Error)] fn deserialize_enum<V: Visitor<'de>>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> V::Value {
		struct Enum<'t, 'de, I: Iterator<Item=roxmltree::Node<'de, 'de>>>(Deserializer<'t, 'de, I>);
		impl<'t, 'de, I: Iterator<Item=roxmltree::Node<'de, 'de>>> de::EnumAccess<'de> for Enum<'t, 'de, I> {
			type Error = error::Error;
			type Variant = Self;
			//#[allow(unreachable_code)]
			#[throws(Self::Error)] fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> (V::Value, Self::Variant) {
				//use itertools::Itertools; panic!("{:?}", (self.0).0.format(" "))
				let tag = (self.0).0.next().ok_or_else(|| Self::Error::invalid_type(de::Unexpected::Other("End"), &"tag"))?.tag_name().name();
				//(seed.deserialize(tag.into_deserializer())?, self)
				(seed.deserialize::<de::value::StrDeserializer<Self::Error>>(de::IntoDeserializer::into_deserializer(tag))?, self)
			}
		}
		impl<'t, 'de, I: Iterator<Item=roxmltree::Node<'de, 'de>>> de::VariantAccess<'de> for Enum<'t, 'de, I> {
			type Error = error::Error;
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

#[throws(anyhow::Error)] pub fn from_node<'input: 'de, 't: 'de, 'de, T: serde::Deserialize<'de>>(node: roxmltree::Node<'t, 'input>) -> T {
	T::deserialize(Deserializer(&mut node.children().peekable()))?
}
#[throws(anyhow::Error)] pub fn from_document<'input: 'de, 'de, T: serde::Deserialize<'de>>(document: &'de roxmltree::Document<'input>) -> T {
	from_node(document.root())?
}
#[throws(anyhow::Error)] pub fn parse(bytes: &[u8]) -> roxmltree::Document { roxmltree::Document::parse(std::str::from_utf8(bytes)?)? }
