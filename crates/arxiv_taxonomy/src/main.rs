#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod generate;
mod http;
mod types;

use crate::http::find_axonomy_types;
use crate::types::Category;
use generate::{new_enum, pretty_print};
use quote::{format_ident, quote};
use std::collections::HashMap;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
	let taxonomy = find_axonomy_types().await?;
	println!("{taxonomy:?}");

	for (group, categories) in taxonomy.iter() {
		let group_enum_ident = format!("Group{group}");
		let variants = categories
			.iter()
			.map(|c| c.id_as_enum_variant())
			.collect::<Vec<String>>();
		let group_enum = new_enum(group_enum_ident, variants);

		println!("{}", pretty_print(&group_enum));
	}

	Ok(())
}
