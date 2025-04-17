use std::fmt::{Display, Formatter, Result as FmtResult};

/// The version of an article as declared in an arXiv identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArticleVersion {
	#[default]
	Latest,
	Num(u8),
}

impl From<u8> for ArticleVersion {
	fn from(val: u8) -> Self {
		Self::Num(val)
	}
}

impl Display for ArticleVersion {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Latest => f.write_str(""),
			Self::Num(v) => write!(f, "v{}", v),
		}
	}
}

/// Parses a string in the format of "number{vV}",
/// where:
/// - `number` is a unique integer up 4 to 5 digits
/// - `{vV}` (optional): a `v` literal followed by 1 or more digits
pub(crate) fn parse_numbervv(s: &str) -> Option<(&str, ArticleVersion)> {
	if s.len() < 4 {
		return None;
	}

	let first4 = &s[..4];
	let are_digits = first4.chars().all(|c| c.is_ascii_digit());
	if !are_digits {
		return None;
	}

	let mut peek = s[4..].chars().peekable();
	let number = match peek.next_if(|c| c.is_ascii_digit()) {
		Some(_) => &s[..5],
		None => &s[..4],
	};

	let mut version = ArticleVersion::Latest;
	if s.len() > number.len() {
		let after_number = &mut s[number.len()..].chars().peekable();
		if after_number.next_if(|c| *c == 'v').is_some() {
			let consume = after_number
				.take_while(|c| c.is_ascii_digit())
				.collect::<String>();
			let version_u8 = consume.parse::<u8>().ok()?;
			version = ArticleVersion::Num(version_u8);
		}
	}

	Some((number, version))
}

#[cfg(test)]
mod test_parse_numbervv {
	use crate::{parse_numbervv, ArticleVersion};

	#[test]
	fn is_fine() {
		let parsed = parse_numbervv("0001v1").unwrap();
		assert_eq!(parsed.0, "0001");
		assert_eq!(parsed.1, ArticleVersion::Num(1));
	}
}
