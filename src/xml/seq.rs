use {serde::de::{self, Visitor}, super::{Error, ElementDeserializer}, core::throws};

pub(super) struct EmptySeqDeserializer;

impl<'de> de::Deserializer<'de> for EmptySeqDeserializer {
	type Error = Error;
	#[throws] fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> V::Value {
		//println!("empty seq [seq]");
		visitor.visit_seq(::serde::de::value::SeqDeserializer::<_,Error>::new(std::iter::empty::<EmptySeqDeserializer>()))?
	}

	#[throws] fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> V::Value {
		panic!("empty seq any {}'", &visitor as &dyn de::Expected);
	}
	serde::forward_to_deserialize_any!{
		char bytes byte_buf str string identifier bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 option unit map unit_struct newtype_struct tuple tuple_struct struct enum ignored_any}
}

pub(super) struct SeqDeserializer<'t, 'de>{pub node: std::cell::RefMut<'t, &'t mut ElementDeserializer<'de>>, pub tag: &'t str}

impl<'t, 'de> de::Deserializer<'de> for SeqDeserializer<'t, 'de> {
	type Error = Error;
	#[throws] fn deserialize_seq<V: Visitor<'de>>(mut self, visitor: V) -> V::Value {
		visitor.visit_seq(::serde::de::value::SeqDeserializer::new( std::iter::from_fn(||
			loop {
				//println!("seq [seq] {:?} {}", self.node, self.tag);
				if let Some(child) = self.node.children.peek() {
					if child.is_element() {
						if child.tag_name().name() == self.tag {
							//println!("item [seq]");
							break Some(ElementDeserializer::new(self.node.children.next().unwrap())) // Leave content context
						} else {
							//println!("{:?} does not match {}", child, self.tag);
							break None;
						}
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
