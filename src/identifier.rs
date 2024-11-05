use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Convenient type alias for a [`Result`] holding either an [`ArticleId`] or [`ArticleIdError`]
pub type ArticleIdResult<'a> = Result<ArticleId<'a>, ArticleIdError>;

/// An error that can occur when parsing and validating arXiv identifiers
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticleIdError {
	/// Expected the identifier to start with the string literal "arXiv"
	ExpectedBeginningLiteral,
	/// Expected to find a `numbervV` component
	ExpectedNumberVv,
	/// An invalid month outside of the inclusive [1, 12] interval
	InvalidMonth,
	/// An invalid year outside of the inclusive [2007, 2099] interval
	InvalidYear,
	/// An invalid identifier outside of the inclusive [1, 99999] interval
	InvalidId,
}

impl Error for ArticleIdError {}

impl Display for ArticleIdError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::ExpectedBeginningLiteral => {
				f.write_str("Expected the identifier to start with the literal \"arXiv\".")
			}
			Self::ExpectedNumberVv => {
				f.write_str("Expected the identifier to have a component of format .number{{vV}}.")
			}
			Self::InvalidMonth => f.write_str("A valid month must be between 1 and 12."),
			Self::InvalidYear => f.write_str("A valid year must be be between 2007 and 2099."),
			Self::InvalidId => f.write_str("A valid identifier must be between 1 and 99999"),
		}
	}
}

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArticleId<'a> {
	year: i16,
	month: i8,
	number: &'a str,
	version: ArticleVersion,
}

impl<'a> ArticleId<'a> {
	pub const MIN_YEAR: i16 = 2007i16;
	pub const MAX_YEAR: i16 = 2099i16;
	pub const MIN_NUM_DIGITS: usize = 4usize;
	pub const MAX_NUM_DIGITS: usize = 5usize;
	pub(crate) const MIN_MONTH: i8 = 1i8;
	pub(crate) const MAX_MONTH: i8 = 12i8;
	pub(crate) const TOKEN_COLON: char = ':';
	pub(crate) const TOKEN_DOT: char = '.';
	pub(crate) const TOKEN_VERSION: char = 'v';

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
	/// use arxiv::{ArticleId, ArticleIdError, ArticleVersion};
	///
	/// let id = ArticleId::try_new(2011, 1, "00001", ArticleVersion::Num(1));
	/// assert!(id.is_ok());
	/// ```
	pub fn try_new(
		year: i16,
		month: i8,
		number: &'a str,
		version: ArticleVersion,
	) -> ArticleIdResult {
		if !(Self::MIN_YEAR..=Self::MAX_YEAR).contains(&year) {
			return Err(ArticleIdError::InvalidYear);
		}

		if !(Self::MIN_MONTH..=Self::MAX_MONTH).contains(&month) {
			return Err(ArticleIdError::InvalidMonth);
		}

		let length_check = (Self::MIN_NUM_DIGITS..=Self::MAX_NUM_DIGITS).contains(&number.len());
		let digit_check = number.chars().all(|c| c.is_ascii_digit());
		if !length_check || !digit_check {
			return Err(ArticleIdError::InvalidId);
		}

		Ok(Self::new(year, month, number, version))
	}

	/// This allows manually creating an [`ArticleId`] from the given components without a version
	/// (assuming it is the latest version), and will also validate each component for correctness.
	/// If any component is invalid, it will return an [`ArticleIdError`].
	///
	/// # Examples
	/// ```
	/// use arxiv::{ArticleId, ArticleIdError};
	///
	/// let id = ArticleId::try_latest(2011, 1, "00001");
	/// assert!(id.is_ok());
	/// ```
	#[inline]
	pub fn try_latest(year: i16, month: i8, number: &'a str) -> ArticleIdResult {
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
	pub fn number(&self) -> &'a str {
		self.number
	}

	/// The latest version of the arXiv publication, if any.
	///
	/// # Examples
	/// ```
	/// use arxiv::{ArticleId, ArticleVersion};
	///
	///  let id = ArticleId::try_from("arXiv:2304.11188v1").unwrap();
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
		let mut year_str = self.year.to_string();
		let (_, half_year) = year_str.as_mut_str().split_at(2);

		if self.number.len() == 4usize {
			format!("{:02}{:02}.{:04}", half_year, self.month, self.number)
		} else {
			format!("{:02}{:02}.{:05}", half_year, self.month, self.number)
		}
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
	pub fn as_url(&self) -> url::Url {
		url::Url::from(*self)
	}
}

impl<'a> Display for ArticleId<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "arXiv:{}{}", self.as_unique_ident(), self.version)
	}
}

impl<'a> TryFrom<&'a str> for ArticleId<'a> {
	type Error = ArticleIdError;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		use ArticleIdError::*;

		// break down the arxiv string into its components
		let parts: Vec<&str> = value.split(ArticleId::TOKEN_COLON).collect();
		if parts.len() != 2 || parts[0] != "arXiv" {
			return Err(ExpectedBeginningLiteral);
		}

		let inner_parts: Vec<&str> = parts[1].split(ArticleId::TOKEN_DOT).collect();
		if inner_parts.len() != 2 {
			return Err(ExpectedNumberVv);
		}

		let date = inner_parts[0];
		let numbervv = inner_parts[1];

		// validate and compose the final Arxiv struct
		let year = date[0..2].parse::<i16>().map_err(|_| InvalidYear)?;
		let month = date[2..4].parse::<i8>().map_err(|_| InvalidMonth)?;
		let (number, version) = parse_numbervv(numbervv).ok_or(ExpectedNumberVv)?;
		ArticleId::try_new(year + 2000i16, month, number, version)
	}
}

#[cfg(feature = "url")]
impl<'a> From<ArticleId<'a>> for url::Url {
	fn from(id: ArticleId<'a>) -> url::Url {
		url::Url::parse(&format!("https://arxiv.org/abs/{}{}", id.as_unique_ident(), id.version,))
			.unwrap()
	}
}

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
		if after_number
			.next_if(|c| *c == ArticleId::TOKEN_VERSION)
			.is_some()
		{
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
mod test_display {
	use super::ArticleId;

	#[test]
	fn with_version() {
		let id = ArticleId::new_versioned(2007, 1, "0001", 1);
		assert_eq!(id.to_string(), "arXiv:0701.0001v1");
	}

	#[test]
	fn without_version() {
		let id = ArticleId::new_latest(2007, 1, "0001");
		assert_eq!(id.to_string(), "arXiv:0701.0001");
	}
}

#[cfg(test)]
mod test_parse_numbervv {
	use super::*;

	#[test]
	fn is_fine() {
		let parsed = parse_numbervv("0001v1").unwrap();
		assert_eq!(parsed.0, "0001");
		assert_eq!(parsed.1, ArticleVersion::Num(1));
	}
}

#[cfg(test)]
mod tests_parse_ok {
	use super::*;

	#[test]
	fn from_readme() {
		let id = ArticleId::try_from("arXiv:0706.0001v1").unwrap();
		assert_eq!(id.year, 2007);
		assert_eq!(id.month, 6);
		assert_eq!(id.number(), "0001");
		assert_eq!(id.version(), ArticleVersion::Num(1));
	}

	#[test]
	fn without_version() {
		let id = ArticleId::try_from("arXiv:1501.00001");
		assert_eq!(id, Ok(ArticleId::new_latest(2015, 1, "00001")));
	}

	#[test]
	fn with_version() {
		let id = ArticleId::try_from("arXiv:9912.12345v2");
		assert_eq!(id, Ok(ArticleId::new(2099, 12, "12345", ArticleVersion::Num(2))))
	}

	#[test]
	fn with_number_4_digits() {
		let id1 = ArticleId::new_latest(2014, 1, "7878");
		assert_eq!(id1.to_string(), String::from("arXiv:1401.7878"));

		let id2 = ArticleId::new_latest(2014, 12, "7878");
		assert_eq!(id2.to_string(), String::from("arXiv:1412.7878"));
	}

	#[test]
	fn with_number_5_digits() {
		let id1 = ArticleId::new_latest(2014, 1, "00008");
		assert_eq!(id1.to_string(), String::from("arXiv:1401.00008"));

		let id2 = ArticleId::new_latest(2014, 12, "00008");
		assert_eq!(id2.to_string(), String::from("arXiv:1412.00008"));
	}
}

#[cfg(test)]
mod tests_parse_err {
	use super::*;

	#[test]
	fn empty_string() {
		let id = ArticleId::try_from("");
		assert_eq!(id, Err(ArticleIdError::ExpectedBeginningLiteral));
	}

	#[test]
	fn no_numbervv() {
		let id = ArticleId::try_from("arXiv:1501");
		assert_eq!(id, Err(ArticleIdError::ExpectedNumberVv));
	}

	#[test]
	fn invalid_year() {
		let maybe_id = ArticleId::try_latest(2006, 1, "00001");
		assert_eq!(maybe_id, Err(ArticleIdError::InvalidYear));
	}

	#[test]
	fn invalid_month() {
		let maybe_id = ArticleId::try_latest(2007, i8::MAX, "00001");
		assert_eq!(maybe_id, Err(ArticleIdError::InvalidMonth));
	}

	#[test]
	fn invalid_id() {
		let maybe_id = ArticleId::try_latest(2007, 11, "");
		assert_eq!(maybe_id, Err(ArticleIdError::InvalidId));
	}
}

#[cfg(test)]
#[cfg(feature = "url")]
mod tests_url {
	use super::*;
	use url::Url;

	#[test]
	fn url_from_id() {
		let id = ArticleId::try_new(2007, 01, "00001", ArticleVersion::Latest).unwrap();
		let url = Url::from(id);
		assert_eq!(url.scheme(), "https");
		assert_eq!(url.domain(), Some("arxiv.org"));
		assert_eq!(url.path(), "/abs/0701.00001");
		assert_eq!(url.to_string(), "https://arxiv.org/abs/0701.00001");
	}
}
