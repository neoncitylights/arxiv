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
	println!("{:?}", taxonomy);

	// let taxonomy: HashMap<String, Vec<Category>> = HashMap::from([(
	// 	"CompSci".to_owned(),
	// 	vec![
	// 		Category::new_str("csg", "Graphics"),
	// 		Category::new_str("csv", "Vision"),
	// 	],
	// )]);

	for (group, categories) in taxonomy.iter() {
		let group_enum_ident = format!("Group{}", group);
		let variants = categories
			.iter()
			.map(|c| c.id_as_enum_variant())
			.collect::<Vec<String>>();
		let group_enum = new_enum(group_enum_ident, variants);

		println!("{}", pretty_print(&group_enum));
	}

	// let my_enum = new_enum("test".to_owned(), vec!["Blue".to_owned(), "Red".to_owned()]);
	// println!("{}", pretty_print(&my_enum));
	// let v = vec!["Blue", "Red", "Green"];
	// let v = v.into_iter().map(|s| format_ident!("{}", s));

	// let my_enum = new_enum(
	// 	String::from("Color"),
	// 	vec![
	// 		String::from("Blue"),
	// 		String::from("Red"),
	// 		String::from("Green"),
	// 	],
	// );
	// println!("{}", pretty_print(&my_enum));
	Ok(())
}
