use {fehler::*, serde::de::{self, Visitor}};
use super::{Error, NodeDeserializer};

pub(super) struct ContentDeserializer<'t, 'de>(pub std::cell::RefMut<'t, &'t mut NodeDeserializer<'de>>);

impl<'t, 'de> de::Deserializer<'de> for ContentDeserializer<'t, 'de> {
	type Error = Error;
	#[throws] fn deserialize_str<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { visitor.visit_str::<Error>(self.0.text()?)? }
	#[throws] fn deserialize_string<V: Visitor<'de>>(mut self, visitor: V) -> V::Value { visitor.visit_string::<Error>(self.0.text()?.to_owned())? }

	#[throws] fn deserialize_seq<V: Visitor<'de>>(mut self, visitor: V) -> V::Value {
		println!("seq [content]");
		visitor.visit_seq(::serde::de::value::SeqDeserializer::new(self.0.children.by_ref().filter(roxmltree::Node::is_element).map(|e| {
			println!("item [content]");
			NodeDeserializer::new(e) // Leave content context
		})))?
	}

	#[throws] fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> V::Value {
		self.deserialize_struct("", &[], visitor)?
	}

	#[throws] fn deserialize_struct<V: Visitor<'de>>(mut self, name: &'static str, fields: &'static [&'static str], visitor: V) -> V::Value {
		println!("struct [content] '{}' {:?}", name, fields);
		self.0.deserialize_struct(name, fields, visitor)?
	}

	#[throws] fn deserialize_enum<V: Visitor<'de>>(mut self, name: &'static str, variants: &'static [&'static str], visitor: V) -> V::Value {
		let node = self.0.children.by_ref().filter(|child| child.is_element()).next().ok_or_else(|| anyhow::Error::msg("Expected variant"))?;
		let tag = node.tag_name().name();
		println!("enum [content {}] '{}' {:?} {:?}", self.0.name, name, tag, variants);
		ensure!(variants.contains(&tag), "enum [content] {}: no '{}' in {:?}", name, tag, variants);
		visitor.visit_enum(serde::de::value::MapAccessDeserializer::new(serde::de::value::MapDeserializer::new(std::iter::once((tag, NodeDeserializer::new(node))))))?
	}

	#[throws] fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> V::Value {
		println!("content 'any {}'", &visitor as &dyn de::Expected);
		self.deserialize_map(visitor)?
	}
	serde::forward_to_deserialize_any!{
		char bytes byte_buf identifier bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 option unit unit_struct newtype_struct tuple tuple_struct ignored_any}
}
