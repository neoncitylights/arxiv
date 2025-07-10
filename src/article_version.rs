use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// The version of an article as declared in an arXiv identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
pub enum ArticleVersion {
	#[default]
	Latest,
	Num(u8),
}

impl ArticleVersion {
	pub const fn is_latest(&self) -> bool {
		matches!(self, Self::Latest)
	}

	pub const fn is_version(&self, v: u8) -> bool {
		matches!(self, Self::Num(version) if *version == v)
	}
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
			Self::Num(v) => write!(f, "v{v}"),
		}
	}
}

impl FromStr for ArticleVersion {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.is_empty() {
			return Ok(Self::Latest);
		}
		if !s.starts_with('v') {
			return Err(());
		}

		let version = s[1..].parse::<u8>().map_err(|_| ())?;
		Ok(Self::Num(version))
	}
}

#[cfg(test)]
mod tests {
	use crate::ArticleVersion;
	use std::str::FromStr;

	#[test]
	fn parse_latest() {
		assert_eq!(ArticleVersion::from_str(""), Ok(ArticleVersion::Latest));
	}

	#[test]
	fn parse_version_ok() {
		assert_eq!(ArticleVersion::from_str("v1"), Ok(ArticleVersion::Num(1)));
	}

	#[test]
	fn parse_version_err() {
		assert_eq!(ArticleVersion::from_str("v"), Err(()));
		assert_eq!(ArticleVersion::from_str("vfoo"), Err(()));
	}
}
