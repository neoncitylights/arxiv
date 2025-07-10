#![cfg_attr(docsrs, feature(doc_cfg))]

use crate::as_unique_ident;
use crate::{ArticleIdError, ArticleIdV2Result, ArticleVersion};
use std::fmt::{Display, Formatter, Result as FmtResult};

use super::{ArticleId, ArticleIdParser};

/// A unique identifier for articles published on arXiv.org
///
/// See also: [Official arXiv.org documentation][arxiv-docs]
///
/// # Examples
/// ```
/// use arxiv::ArticleId;
///
/// let id = ArticleId::try_from("arXiv:2001.00001");
/// assert!(id.is_ok());
/// ```
///
/// [arxiv-docs]: https://info.arxiv.org/help/arxiv_identifier.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArticleIdV2<'a> {
	year: i16,
	month: i8,
	number: &'a str,
	version: ArticleVersion,
}

impl<'a> ArticleIdV2<'a> {
	pub const MIN_YEAR: i16 = 2007i16;
	pub const MAX_YEAR: i16 = 2099i16;
	pub const MIN_NUM_DIGITS: usize = 4usize;
	pub const MAX_NUM_DIGITS: usize = 5usize;

	/// This allows manually creating an [`ArticleId`] from the given components without any
	/// validation. Only do this if you have already verified that the components are valid.
	///
	/// # Safety
	///  - The year is between the inclusive range of [2007, 2009].
	///  - The month is between the inclusive range of [1, 12].
	///  - The unique number string only contains 4 to 5 ASCII digits.
	///
	/// # Examples
	/// ```
	/// use arxiv::{ArticleId, ArticleVersion};
	///
	/// let id = ArticleId::new(2011, 1, "00001", ArticleVersion::Num(1));
	/// ```
	#[inline]
	pub const fn new(year: i16, month: i8, number: &'a str, version: ArticleVersion) -> Self {
		Self {
			year,
			month,
			number,
			version,
		}
	}

	/// This allows manually creating an [`ArticleId`] from the given components without any version
	/// (assuming it is the latest version). Only do this if you have already verified that the
	/// components are valid:
	///
	///  - The year is between the inclusive range of [2007, 2009].
	///  - The month is between the inclusive range of [1, 12].
	///  - The unique number string only contains 4 to 5 ASCII digits.
	///
	/// # Examples
	/// ```
	/// use arxiv::{ArticleId, ArticleVersion};
	///
	/// let id = ArticleId::new_latest(2011, 1, "00001");
	/// ```
	#[inline]
	pub const fn new_latest(year: i16, month: i8, id: &'a str) -> Self {
		Self::new(year, month, id, ArticleVersion::Latest)
	}

	/// This allows manually creating an [`ArticleId`] from the given components with a specific version.
	/// Only do this if you have already verified that the components are valid:
	///
	///  - The year is between the inclusive range of [2007, 2009].
	///  - The month is between the inclusive range of [1, 12].
	///  - The unique number string only contains 4 to 5 ASCII digits.
	///
	/// # Examples
	/// ```
	/// use arxiv::{ArticleId, ArticleVersion};
	///
	/// let id = ArticleId::new_versioned(2011, 1, "00001", 1);
	/// assert_eq!(id.version(), ArticleVersion::Num(1));
	/// ```
	pub const fn new_versioned(year: i16, month: i8, id: &'a str, version: u8) -> Self {
		Self::new(year, month, id, ArticleVersion::Num(version))
	}

	/// This allows manually creating an [`ArticleId`] from the given components with a version, and
	/// will also validate each component for correctness. If any component is invalid, it will return
	/// an [`ArticleIdError`].
	///
	/// # Examples
	/// ```
	/// use arxiv::{ArticleId, ArticleVersion};
	///
	/// let id = ArticleId::try_new(2011, 1, "00001", ArticleVersion::Num(1));
	/// assert!(id.is_ok());
	/// ```
	pub fn try_new(
		year: i16,
		month: i8,
		number: &'a str,
		version: ArticleVersion,
	) -> ArticleIdV2Result<'a> {
		use crate::ArticleIdErrorKind::*;

		if !(Self::MIN_YEAR..=Self::MAX_YEAR).contains(&year) {
			return Err(ArticleIdError(InvalidYear));
		}

		if !(ArticleId::MIN_MONTH..=ArticleId::MAX_MONTH).contains(&month) {
			return Err(ArticleIdError(InvalidMonth));
		}

		let length_check = (Self::MIN_NUM_DIGITS..=Self::MAX_NUM_DIGITS).contains(&number.len());
		let digit_check = number.chars().all(|c| c.is_ascii_digit());
		if !length_check || !digit_check {
			return Err(ArticleIdError(InvalidId));
		}

		Ok(Self::new(year, month, number, version))
	}

	/// This allows manually creating an [`ArticleId`] from the given components without a version
	/// (assuming it is the latest version), and will also validate each component for correctness.
	/// If any component is invalid, it will return an [`ArticleIdError`].
	///
	/// # Examples
	/// ```
	/// use arxiv::ArticleId;
	///
	/// let id = ArticleId::try_latest(2011, 1, "00001");
	/// assert!(id.is_ok());
	/// ```
	#[inline]
	pub fn try_latest(year: i16, month: i8, number: &'a str) -> ArticleIdV2Result<'a> {
		Self::try_new(year, month, number, ArticleVersion::Latest)
	}

	/// Whether or not the identifier refers to the most recent version of the arXiv article
	#[inline]
	pub const fn is_latest(&self) -> bool {
		matches!(self.version, ArticleVersion::Latest)
	}

	/// The year the arXiv publication was published in
	///
	/// # Examples
	/// ```
	/// use arxiv::ArticleId;
	///
	/// let id = ArticleId::try_from("arXiv:2304.11188v1").unwrap();
	/// assert_eq!(id.year(), 2023);
	/// ```
	#[must_use]
	#[inline]
	pub const fn year(&self) -> i16 {
		self.year
	}

	/// The month the arXiv publication was published in
	///
	/// # Examples
	/// ```
	/// use arxiv::ArticleId;
	///
	/// let id = ArticleId::try_from("arXiv:2304.11188v1").unwrap();
	/// assert_eq!(id.month(), 04);
	/// ```
	#[must_use]
	#[inline]
	pub const fn month(&self) -> i8 {
		self.month
	}

	/// The uniquely assigned identifier of the arXiv publication
	///
	/// # Examples
	/// ```
	/// use arxiv::ArticleId;
	///
	/// let id = ArticleId::try_from("arXiv:2304.11188v1").unwrap();
	/// assert_eq!(id.number(), "11188");
	/// ```
	#[must_use]
	#[inline]
	pub const fn number(&self) -> &'a str {
		self.number
	}

	/// The latest version of the arXiv publication, if any.
	///
	/// # Examples
	/// ```
	/// use arxiv::{ArticleId, ArticleVersion};
	///
	/// let id = ArticleId::try_from("arXiv:2304.11188v1").unwrap();
	/// assert_eq!(id.version(), ArticleVersion::Num(1));
	/// ```
	#[must_use]
	#[inline]
	pub const fn version(&self) -> ArticleVersion {
		self.version
	}

	/// Sets the version of the arXiv article.
	///
	/// # Examples
	/// ```
	/// use arxiv::{ArticleId, ArticleVersion};
	///
	/// let mut id = ArticleId::try_from("arXiv:2001.00001").unwrap();
	/// assert_eq!(id.version(), ArticleVersion::Latest);
	///
	/// id.set_version(1);
	/// assert_eq!(id.version(), ArticleVersion::Num(1));
	/// ```
	#[inline]
	pub fn set_version(&mut self, version: u8) {
		self.version = ArticleVersion::Num(version)
	}

	/// Sets the version of the arXiv article to the latest version.
	///
	/// # Examples
	/// ```
	/// use arxiv::{ArticleId, ArticleVersion};
	///
	/// let mut id = ArticleId::try_from("arXiv:2001.00001v1").unwrap();
	/// assert_eq!(id.version(), ArticleVersion::Num(1));
	///
	/// id.set_latest();
	/// assert_eq!(id.version(), ArticleVersion::Latest);
	/// ```
	#[inline]
	pub fn set_latest(&mut self) {
		self.version = ArticleVersion::Latest;
	}

	/// Display the id as a unique identifier (after the arXiv literal)
	///
	/// ```
	/// use arxiv::{ArticleId};
	///
	/// let id = ArticleId::new_versioned(2020, 10, "14462", 2);
	/// assert_eq!(id.as_unique_ident(), "2010.14462");
	/// ```
	pub fn as_unique_ident(&self) -> String {
		as_unique_ident(self.year, self.month, self.number, true)
	}

	/// Converts the article identifier to a URL where the abstract page is.
	///
	/// ```
	/// use arxiv::ArticleId;
	/// use url::Url;
	///
	/// let id = ArticleId::new_versioned(2020, 10, "14462", 2);
	/// let url = Url::from(id);
	/// assert_eq!(url.to_string(), "https://arxiv.org/abs/2010.14462v2");
	/// ```
	#[cfg(feature = "url")]
	#[cfg_attr(docsrs, doc(cfg(feature = "url")))]
	pub fn as_url(&self) -> url::Url {
		url::Url::from(*self)
	}
}

impl Display for ArticleIdV2<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "arXiv:{}{}", self.as_unique_ident(), self.version)
	}
}

impl<'a> TryFrom<&'a str> for ArticleIdV2<'a> {
	type Error = ArticleIdError;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		let parser = ArticleIdParser::new_v2();
		parser.parse(value).map(|id| id.as_v2())
	}
}

#[cfg(feature = "url")]
#[cfg_attr(docsrs, doc(cfg(feature = "url")))]
impl<'a> From<ArticleIdV2<'a>> for url::Url {
	fn from(id: ArticleIdV2<'a>) -> Self {
		let f = &format!("https://arxiv.org/abs/{}{}", id.as_unique_ident(), id.version);
		Self::parse(f).unwrap()
	}
}

#[cfg(test)]
mod test_display {
	use crate::ArticleIdV2;

	#[test]
	fn with_version() {
		let id = ArticleIdV2::new_versioned(2007, 1, "0001", 1);
		assert_eq!(id.to_string(), "arXiv:0701.0001v1");
	}

	#[test]
	fn without_version() {
		let id = ArticleIdV2::new_latest(2007, 1, "0001");
		assert_eq!(id.to_string(), "arXiv:0701.0001");
	}
}

#[cfg(test)]
mod tests_parse_ok {
	use crate::{ArticleIdV2, ArticleVersion};

	#[test]
	fn from_readme() {
		let id = ArticleIdV2::try_from("arXiv:0706.0001v1");
		assert_eq!(id, Ok(ArticleIdV2::new_versioned(2007, 6, "0001", 1)));
	}

	#[test]
	fn without_version() {
		let id = ArticleIdV2::try_from("arXiv:1501.00001");
		assert_eq!(id, Ok(ArticleIdV2::new_latest(2015, 1, "00001")));
	}

	#[test]
	fn with_version() {
		let id = ArticleIdV2::try_from("arXiv:9912.12345v2");
		assert_eq!(id, Ok(ArticleIdV2::new(2099, 12, "12345", ArticleVersion::Num(2))))
	}

	#[test]
	fn with_number_4_digits() {
		let id1 = ArticleIdV2::new_latest(2014, 1, "7878");
		assert_eq!(id1.to_string(), String::from("arXiv:1401.7878"));

		let id2 = ArticleIdV2::new_latest(2014, 12, "7878");
		assert_eq!(id2.to_string(), String::from("arXiv:1412.7878"));
	}

	#[test]
	fn with_number_5_digits() {
		let id1 = ArticleIdV2::new_latest(2014, 1, "00008");
		assert_eq!(id1.to_string(), String::from("arXiv:1401.00008"));

		let id2 = ArticleIdV2::new_latest(2014, 12, "00008");
		assert_eq!(id2.to_string(), String::from("arXiv:1412.00008"));
	}
}

#[cfg(test)]
mod tests_parse_err {
	use crate::{ArticleIdError, ArticleIdErrorKind::*, ArticleIdV2};

	#[test]
	fn empty_string() {
		let id = ArticleIdV2::try_from("");
		assert_eq!(id, Err(ArticleIdError(ExpectedPrefix)));
	}

	#[test]
	fn no_number() {
		let id = ArticleIdV2::try_from("arXiv:1501");
		assert_eq!(id, Err(ArticleIdError(ExpectedNumberV2)));
	}

	#[test]
	fn invalid_year() {
		let maybe_id = ArticleIdV2::try_latest(2006, 1, "00001");
		assert_eq!(maybe_id, Err(ArticleIdError(InvalidYear)));
	}

	#[test]
	fn invalid_month() {
		let maybe_id = ArticleIdV2::try_latest(2007, i8::MAX, "00001");
		assert_eq!(maybe_id, Err(ArticleIdError(InvalidMonth)));
	}

	#[test]
	fn invalid_id() {
		let maybe_id = ArticleIdV2::try_latest(2007, 11, "");

		assert_eq!(maybe_id, Err(ArticleIdError(InvalidId)));
	}
}

#[cfg(test)]
#[cfg(feature = "url")]
mod tests_url {
	use crate::{ArticleIdV2, ArticleVersion};
	use url::Url;

	#[test]
	fn url_from_id() {
		let id = ArticleIdV2::try_new(2007, 1, "00001", ArticleVersion::Latest).unwrap();
		let url = Url::from(id);

		assert_eq!(url.scheme(), "https");
		assert_eq!(url.domain(), Some("arxiv.org"));
		assert_eq!(url.path(), "/abs/0701.00001");
		assert_eq!(url.to_string(), "https://arxiv.org/abs/0701.00001");
	}
}
