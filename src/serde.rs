use crate::ArticleId;
use serde::de::{self, DeserializeSeed, Deserializer, Visitor};
use std::fmt::Formatter;
use std::fmt::Result as FormatResult;
use std::marker::PhantomData;

impl<'de: 'a, 'a> DeserializeSeed<'de> for ArticleId<'a> {
	type Value = ArticleId<'a>;

	fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_str(ArticleIdVisitor(PhantomData))
	}
}

struct ArticleIdVisitor<'a>(PhantomData<&'a ()>);

impl<'de: 'a, 'a> Visitor<'de> for ArticleIdVisitor<'a> {
	type Value = ArticleId<'a>;

	fn expecting(&self, formatter: &mut Formatter) -> FormatResult {
		formatter.write_str("a string to parse into ArticleId")
	}

	fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		ArticleId::try_from(v).map_err(|e| {
			E::custom(format!("An error occurred while parsing an ArXiv article identifier: {}", e))
		})
	}
}
