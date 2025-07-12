use crate::Category;
use futures::future::join_all;
use regex::Regex;
use std::collections::HashMap;
use thirtyfour::components::{Component, ElementResolver};
use thirtyfour::prelude::*;

pub type TaxonomyMap = HashMap<String, Vec<Category>>;

pub async fn find_axonomy_types() -> WebDriverResult<TaxonomyMap> {
	// setup I/O arguments
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		eprintln!("Usage: {} <port>", args[0]);
		std::process::exit(1);
	}
	let port: u16 = args[1]
		.parse()
		.expect("Port must be a number between [0, 65535]");
	println!("Running on port: {}", port);

	// start session
	let driver = start_webdriver(port).await?;
	let categories = iter_taxonomy_list(&driver).await?;

	// terminate session
	driver.quit().await?;
	Ok(categories)
}

pub async fn start_webdriver(port: u16) -> WebDriverResult<WebDriver> {
	let caps = DesiredCapabilities::chrome();
	let server = format!("http://localhost:{}", port);
	let driver = WebDriver::new(server, caps).await?;
	driver.goto("https://arxiv.org/category_taxonomy").await?;

	Ok(driver)
}

pub async fn iter_taxonomy_list(driver: &WebDriver) -> WebDriverResult<TaxonomyMap> {
	// elements
	let taxonomy_list = driver.find(By::Css("#category_taxonomy_list")).await?;
	let children = taxonomy_list.find_all(By::XPath("./*")).await?;

	// key is group name, item is categories
	let mut taxonomy_map: HashMap<String, Vec<Category>> = HashMap::new();
	let mut current_group = String::new();

	// walk through
	for (_, child) in children.iter().enumerate() {
		let tag = child.tag_name().await?;

		// check heading
		if tag == "h2" {
			let text = child.text().await?;
			current_group = String::from(&text);
			taxonomy_map.insert(text, vec![]);
		}

		// check group
		if let Some(class) = child.class_name().await?
			&& class == String::from("accordion-body")
		{
			// - Click to make content visible, otherwise content
			//   inside of accordion can't be detected by webdriver
			// - We don't click the accordion body element itself
			//   because the event listener is attached to the HTML heading
			let heading = child.find(By::XPath("preceding-sibling::*[1]")).await?;
			heading.click().await?;

			let mut categories = iter_arxiv_group(&child).await?;
			taxonomy_map
				.entry(current_group.clone())
				.or_default()
				.append(&mut categories);
		}
	}

	println!("{:?}", taxonomy_map);
	Ok(taxonomy_map)
}

async fn iter_arxiv_group(element: &WebElement) -> WebDriverResult<Vec<Category>> {
	let categories = element.find_all(By::Css(".columns > .column h4")).await?;
	let categories = categories
		.iter()
		.map(|c| c.clone())
		.map(async |c| CategoryComponent::from(c).as_category().await.unwrap());
	let categories = join_all(categories).await;

	Ok(categories)
}

/// Category headings (which can either be <h3> or <h4>)
/// are represented like the below:
///
/// ```html
/// <h4>cs.GR <span>(Graphics)</span></h4>
/// ```
#[derive(Debug, Clone, Component)]
pub struct CategoryComponent {
	/// an HTML heading
	base: WebElement,
	#[by(css = "span")]
	_full_name: ElementResolver<WebElement>,
}

impl CategoryComponent {
	async fn as_category(self) -> WebDriverResult<Category> {
		let text = self.base.text().await?;
		println!("{}", text);

		let mut id = String::new();
		let mut name = String::new();

		let regex = Regex::new(r"(.+) \((.+)\)").unwrap();
		for (_, [id_cap, name_cap]) in regex.captures_iter(&text).map(|c| c.extract()) {
			id = id_cap.to_owned();
			name = name_cap.to_owned();
		}

		Ok(Category::new(id, name))
	}
}
