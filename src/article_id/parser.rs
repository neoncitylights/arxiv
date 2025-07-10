use crate::CategoryId;
use crate::{ArticleId, ArticleIdV1, ArticleIdV2};
use crate::{ArticleIdError, ArticleIdScheme, ArticleVersion};
use std::str::FromStr;

// utility to parse components of something with a str,
// which also contains a &'a str of everything after the given
// component to keep parsing
pub struct ParseOk<'a, T> {
	value: T,
	rest_str: &'a str,
}

impl<'a, T> ParseOk<'a, T> {
	#[inline]
	pub const fn new(value: T, rest_str: &'a str) -> Self {
		Self { value, rest_str }
	}
}

/// A parser that can parse an article ID
/// even if the identifier's scheme is unknown
pub struct ArticleIdParser {
	scheme: Option<ArticleIdScheme>,
}

impl ArticleIdParser {
	const TOKEN_COLON: char = ':';

	#[inline]
	pub const fn new(scheme: Option<ArticleIdScheme>) -> Self {
		Self { scheme }
	}

	#[inline]
	pub const fn new_v1() -> Self {
		Self::new(Some(ArticleIdScheme::V1))
	}

	#[inline]
	pub const fn new_v2() -> Self {
		Self::new(Some(ArticleIdScheme::V2))
	}

	pub fn parse<'a>(&self, id: &'a str) -> Result<ArticleId<'a>, ArticleIdError> {
		use crate::ArticleIdErrorKind::*;

		// split identifier with prefix and content after prefix
		let (prefix, after_prefix) = id
			.split_once(Self::TOKEN_COLON)
			.ok_or(ArticleIdError(ExpectedPrefix))?;

		if prefix != "arXiv" {
			return Err(ArticleIdError(ExpectedPrefix));
		}

		let mut peek = after_prefix.chars().peekable();

		// maybe validate category, depending on scheme
		let is_likely_category = peek.next_if(|c| c.is_ascii_alphabetic()).is_some();
		let (category, rest_str) = match self.scheme {
			None | Some(ArticleIdScheme::V1) => {
				if !is_likely_category {
					eprintln!("ExpectedCategory: Failed first");
					return Err(ArticleIdError(ExpectedCategory));
				}
				// Find the position of the '/' delimiter
				let slash_idx = after_prefix
					.find('/')
					.ok_or(ArticleIdError(ExpectedSlash))?;
				let category = Some(
					CategoryId::try_from(&after_prefix[..slash_idx])
						.map_err(|_| ArticleIdError(ExpectedCategory))?,
				);
				let numbervv_str = &after_prefix[slash_idx + 1..];
				(category, numbervv_str)
			}

			Some(ArticleIdScheme::V2) => {
				if is_likely_category {
					eprintln!("ExpectedCategory: Failed second");
					return Err(ArticleIdError(ExpectedCategory));
				}
				(None, after_prefix)
			}
		};

		// the category is the very first part in the identifier that
		// can disambiguate which scheme that we should be looking for
		let scheme = match category.is_some() {
			true => ArticleIdScheme::V1,
			false => ArticleIdScheme::V2,
		};

		// get year and month
		let year2 = rest_str[..2].parse::<i16>().map_err(|_| InvalidYear)?;
		let year4 = Self::full_year(scheme, year2);
		let month = rest_str[2..4].parse::<i8>().map_err(|_| InvalidMonth)?;

		// parse number
		let after_date_str = &rest_str[4..];
		let number = Self::parse_number(scheme, after_date_str)?;

		// parse version (if there is one)
		println!("{}", number.rest_str);
		let version = ArticleVersion::from_str(number.rest_str).map_err(|_| InvalidVersion)?;

		match scheme {
			ArticleIdScheme::V1 => {
				let id =
					ArticleIdV1::try_new(category.unwrap(), year4, month, number.value, version);
				id.map(ArticleId::V1)
			}
			ArticleIdScheme::V2 => {
				let id = ArticleIdV2::try_new(year4, month, number.value, version);
				id.map(ArticleId::V2)
			}
		}
	}

	fn full_year(scheme: ArticleIdScheme, year: i16) -> i16 {
		match scheme {
			ArticleIdScheme::V1 => match year {
				91..=99 => 1900 + year,
				_ => 2000 + year,
			},
			ArticleIdScheme::V2 => 2000 + year,
		}
	}

	fn parse_number<'a>(
		scheme: ArticleIdScheme,
		s: &'a str,
	) -> Result<ParseOk<'a, &'a str>, ArticleIdError> {
		use crate::ArticleIdErrorKind::*;
		match scheme {
			ArticleIdScheme::V1 => {
				let id = &s[..3];
				if !id.chars().all(|c| c.is_ascii_digit()) {
					eprintln!("ExpectedNumberV1: failed second, {}", id);
					return Err(ArticleIdError(ExpectedNumberV1));
				}

				Ok(ParseOk::new(&s[..3], &s[3..]))
			}
			ArticleIdScheme::V2 => {
				if &s[0..1] != "." {
					return Err(ArticleIdError(ExpectedDot));
				}

				let first4 = &s[1..5];
				if !first4.chars().all(|c| c.is_ascii_digit()) {
					eprintln!("ExpectedNumberV2: failed second, content was {}", first4);
					return Err(ArticleIdError(ExpectedNumberV2));
				}

				let mut peek = s[5..].chars().peekable();
				match peek.next_if(|c| c.is_ascii_digit()) {
					Some(_) => Ok(ParseOk::new(&s[1..6], &s[6..])),
					None => Ok(ParseOk::new(&s[1..5], &s[5..])),
				}
			}
		}
	}
}
