use {fehler::*, serde::de::{self, Visitor}};
use super::{Error, NodeDeserializer};

pub(super) struct SeqDeserializer<'t, 'de>{pub node: std::cell::RefMut<'t, &'t mut NodeDeserializer<'de>>, pub tag: &'t str}

impl<'t, 'de> de::Deserializer<'de> for SeqDeserializer<'t, 'de> {
	type Error = Error;
	#[throws] fn deserialize_seq<V: Visitor<'de>>(mut self, visitor: V) -> V::Value {
		println!("seq [seq]");
		visitor.visit_seq(::serde::de::value::SeqDeserializer::new( std::iter::from_fn(||
			loop {
				if let Some(child) = self.node.children.peek() {
					if child.is_element() {
						if child.tag_name().name() == self.tag {
							//self.node.children.by_ref().filter(|child| child.tag_name().name() == tag).map(|child| {
							println!("item [seq]");
							break Some(NodeDeserializer::new(self.node.children.next().unwrap())) // Leave content context
						} else { break None; }
					} else {
						assert!(child.is_text() && child.text().unwrap().trim().is_empty(), "Ignored {:?}", child); // Helps complete format
						self.node.children.next();
					}
				} else { break None; }
			}
		)))?
	}
	#[throws] fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> V::Value {
		panic!("seq any {}'", &visitor as &dyn de::Expected);
	}
	serde::forward_to_deserialize_any!{
		char bytes byte_buf str string identifier bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 option unit map unit_struct newtype_struct tuple tuple_struct struct enum ignored_any}
}
