macro_rules! dispatch {
	($self:ident, $method:ident($($arg:ident),*) ) => (
		match $self {
			Self::Text(x) => x.$method($($arg),*),
			Self::Element(x) => x.$method($($arg),*),
			Self::Content(x) => x.$method($($arg),*),
			Self::Seq(x) => x.$method($($arg),*),
			Self::EmptySeq(x) => x.$method($($arg),*),
		}
	)
}

macro_rules! delegate {
	{$( fn $method:ident<V:Visitor<'de>>(self, $($arg:ident: $param:ty),*) -> $R:ty; )*} => {$(
		fn $method<V:Visitor<'de>>(self, $($arg: $param),*) -> $R { dispatch!(self, $method($($arg),*)) }
	)*}
}

macro_rules! delegatable_trait {
	{ $t:ident } => {
		impl<'t, 'de> Deserializer<'de> for $t<'t, 'de> {
			type Error = Error;
			delegate! {
				fn deserialize_any<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_bool<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_i8<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_i16<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_i32<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_i64<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_u8<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_u16<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_u32<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_u64<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_f32<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_f64<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_char<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_str<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_string<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_bytes<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_byte_buf<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_option<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_unit<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_unit_struct<V:Visitor<'de>>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_newtype_struct<V:Visitor<'de>>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_seq<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_tuple<V:Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_tuple_struct<V:Visitor<'de>>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_map<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_struct<V:Visitor<'de>>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_enum<V:Visitor<'de>>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_identifier<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
				fn deserialize_ignored_any<V:Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error>;
			}
			fn is_human_readable(&self) -> bool { true /*dispatch!(self, is_human_readable())*/ }
		}
	}
}
