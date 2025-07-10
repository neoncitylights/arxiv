mod parser;
mod v1;
mod v2;
pub(crate) use parser::*;
pub use v1::*;
pub use v2::*;

use crate::CategoryId;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Represents the versioned grammar that defines an arXiv identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticleIdScheme {
	/// Identifier scheme up to [March 2007][arxiv-march-2007]
	///
	/// [arxiv-march-2007]: https://info.arxiv.org/help/arxiv_identifier.html#identifiers-up-to-march-2007-9107-0703
	V1,

	/// Identifier scheme since [April 1st, 2007][arxiv-april-2007]
	///
	/// [arxiv-april-2007]: https://info.arxiv.org/help/arxiv_identifier.html#identifier-scheme-since-1-april-2007-0704-
	V2,
}

/// Represents an ArXiv article identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArticleId<'a> {
	/// an ArXiv article identifier that follows the v1 scheme (up to March 2007)
	V1(ArticleIdV1<'a>),
	/// an ArXiv article identifier that follows the v2 scheme (since April 1st, 2007)
	V2(ArticleIdV2<'a>),
}

impl<'a> ArticleId<'a> {
	pub(crate) const MIN_MONTH: i8 = 1i8;
	pub(crate) const MAX_MONTH: i8 = 12i8;

	pub const fn category(&self) -> Option<CategoryId<'a>> {
		match self {
			Self::V1(v1) => Some(v1.category()),
			_ => None,
		}
	}

	pub const fn as_v1(self) -> ArticleIdV1<'a> {
		match self {
			Self::V1(id) => id,
			_ => panic!(),
		}
	}

	pub const fn as_v2(self) -> ArticleIdV2<'a> {
		match self {
			Self::V2(id) => id,
			_ => panic!(),
		}
	}

	#[inline]
	pub const fn year(&self) -> i16 {
		match self {
			Self::V1(v1) => v1.year(),
			Self::V2(v2) => v2.year(),
		}
	}

	#[inline]
	pub const fn month(&self) -> i8 {
		match self {
			Self::V1(v1) => v1.month(),
			Self::V2(v2) => v2.month(),
		}
	}

	#[inline]
	pub const fn number(&self) -> &'a str {
		match self {
			Self::V1(v1) => v1.number(),
			Self::V2(v2) => v2.number(),
		}
	}

	pub fn as_unique_ident(&self) -> String {
		match self {
			Self::V1(v1) => v1.as_unique_ident(),
			Self::V2(v2) => v2.as_unique_ident(),
		}
	}
}

impl<'a> TryFrom<&'a str> for ArticleId<'a> {
	type Error = ArticleIdError;
	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		// scheme is ambiguous, let the parser figure it out
		let parser = ArticleIdParser::new(None);
		parser.parse(value)
	}
}

/// Convenient type alias for a [`Result`] holding either an [`ArticleIdV1`] or [`ArticleIdError`]
pub type ArticleIdV1Result<'a> = Result<ArticleIdV1<'a>, ArticleIdError>;

/// Convenient type alias for a [`Result`] holding either an [`ArticleIdV2`] or [`ArticleIdError`]
pub type ArticleIdV2Result<'a> = Result<ArticleIdV2<'a>, ArticleIdError>;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArticleIdErrorKind {
	/// Expected the identifier to start with the string literal "arXiv"
	ExpectedPrefix,
	/// Expected a category identifier (an archive, a dot, then subject); for "version 1" identifiers
	ExpectedCategory,
	/// Expected a slash to delimit the identifier (for "version 1" identifiers)
	ExpectedSlash,
	/// Expected a dot to delimit the date and identifier (for "version 2" identifiers)
	ExpectedDot,
	/// Expected to find a number that is 3 digits long
	ExpectedNumberV1,
	// Expected to find a number that is 4-5 digits long
	ExpectedNumberV2,
	/// An invalid version
	InvalidVersion,
	/// An invalid month outside of the inclusive [1, 12] interval
	InvalidMonth,
	/// An invalid year outside of the inclusive [2007, 2099] interval
	InvalidYear,
	/// An invalid identifier outside of the inclusive [1, 99999] interval
	InvalidId,
}

/// An error that can occur when parsing and validating arXiv identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArticleIdError(pub(crate) ArticleIdErrorKind);

impl From<ArticleIdErrorKind> for ArticleIdError {
	fn from(kind: ArticleIdErrorKind) -> Self {
		Self(kind)
	}
}

impl Error for ArticleIdError {}

impl Display for ArticleIdError {
	#[rustfmt::skip]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		use ArticleIdErrorKind::*;
		match self.0 {
			ExpectedPrefix => f.write_str("Expected the identifier to start with \"arXiv\"."),
			ExpectedCategory => f.write_str("Expected a category identifier in the version 1 article identifier"),
			ExpectedSlash => f.write_str("Expected a slash delimiter in the version 1 article identifier"),
			ExpectedDot => f.write_str("Expected a dot delimiter in the version 2 article identifier"),
			ExpectedNumberV1 => f.write_str("Expected a 3-digit identifier (for a version 1 article identifier)"),
			ExpectedNumberV2 => f.write_str("Expected a 4 to 5 digit identifier (for a version 2 article identifier)"),
			InvalidMonth => f.write_str("A valid month must be between 1 and 12."),
			InvalidYear => f.write_str("A valid year must be be between 2007 and 2099."),
			InvalidId => f.write_str("A valid identifier must be between 1 and 99999"),
			InvalidVersion => f.write_str("Found a version, but the version was not of the format \"v{number}\"."),
		}
	}
}

pub(crate) fn as_unique_ident(year: i16, month: i8, number: &str, has_period: bool) -> String {
	let mut year_str = year.to_string();
	let (_, half_year) = year_str.as_mut_str().split_at(2);

	let maybe_period = if has_period { "." } else { "" };
	match number.len() == 4usize {
		true => format!("{half_year:02}{month:02}{maybe_period}{number:04}"),
		false => format!("{half_year:02}{month:02}{maybe_period}{number:05}"),
	}
}
