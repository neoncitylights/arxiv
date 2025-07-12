use heck::ToUpperCamelCase;

#[derive(Debug)]
pub struct Category {
	pub id: String,
	pub name: String,
}

impl Category {
	pub const fn new(id: String, name: String) -> Self {
		Self { id, name }
	}

	pub fn new_str(id: &str, name: &str) -> Self {
		Self::new(id.to_owned(), name.to_owned())
	}

	pub fn id_as_enum_variant(&self) -> String {
		self.id.to_upper_camel_case().to_owned()
	}
}
