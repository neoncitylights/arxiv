#![cfg_attr(docsrs, feature(doc_cfg))]

use crate::as_unique_ident;
use crate::CategoryId;
use crate::{ArticleId, ArticleIdError, ArticleIdV1Result, ArticleVersion};
use std::fmt::{Display, Formatter, Result as FmtResult};

use super::ArticleIdParser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArticleIdV1<'a> {
	category: CategoryId<'a>,
	year: i16,
	month: i8,
	number: &'a str,
	version: ArticleVersion,
}

impl<'a> ArticleIdV1<'a> {
	pub const MAX_YEAR: i16 = 2007i16;
	pub const MIN_YEAR: i16 = 1991i16;
	pub(crate) const DIGITS: usize = 3;

	pub const fn new(
		category: CategoryId<'a>,
		year: i16,
		month: i8,
		number: &'a str,
		version: ArticleVersion,
	) -> Self {
		Self {
			category,
			year,
			month,
			number,
			version,
		}
	}

	pub fn try_new(
		category: CategoryId<'a>,
		year: i16,
		month: i8,
		number: &'a str,
		version: ArticleVersion,
	) -> ArticleIdV1Result<'a> {
		use crate::ArticleIdErrorKind::*;

		if !(Self::MIN_YEAR..=Self::MAX_YEAR).contains(&year) {
			return Err(ArticleIdError(InvalidYear));
		}

		if !(ArticleId::MIN_MONTH..=ArticleId::MAX_MONTH).contains(&month) {
			return Err(ArticleIdError(InvalidMonth));
		}

		let length_check = number.len() == Self::DIGITS;
		let digit_check = number.chars().all(|c| c.is_ascii_digit());
		if !length_check || !digit_check {
			return Err(ArticleIdError(ExpectedNumberV1));
		}

		Ok(Self::new(category, year, month, number, version))
	}

	#[must_use]
	#[inline]
	pub const fn category(&self) -> CategoryId<'a> {
		self.category
	}

	#[must_use]
	#[inline]
	pub const fn year(&self) -> i16 {
		self.year
	}

	#[must_use]
	#[inline]
	pub const fn month(&self) -> i8 {
		self.month
	}

	#[must_use]
	#[inline]
	pub const fn number(&self) -> &'a str {
		self.number
	}

	pub fn as_unique_ident(&self) -> String {
		as_unique_ident(self.year, self.month, self.number, false)
	}
}

impl<'a> Display for ArticleIdV1<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let cat = self.category();
		write!(f, "{}{}/{}", cat.archive(), cat.subject().unwrap_or(""), self.as_unique_ident())
	}
}

impl<'a> TryFrom<&'a str> for ArticleIdV1<'a> {
	type Error = ArticleIdError;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		let parser = ArticleIdParser::new_v1();
		parser.parse(value).map(|id| id.as_v1())
	}
}

#[cfg(test)]
mod parse_ok {
	use crate::{Archive, ArticleVersion, CategoryId, Group};

	use super::ArticleIdV1;

	#[test]
	fn from_gh_issue() {
		let s = "arXiv:cond-mat/0001448v1";
		assert_eq!(
			ArticleIdV1::try_from(s),
			Ok(ArticleIdV1 {
				category: CategoryId::new(Group::Physics, Archive::CondMat, None),
				year: 2000,
				month: 1,
				number: "448",
				version: ArticleVersion::Num(1u8),
			})
		);
	}
}
