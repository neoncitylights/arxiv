use crate::{ArticleId, ArticleIdError, CategoryId};
use jiff::civil::Date;
use jiff::fmt::strtime::format as jiff_format;
use jiff::fmt::strtime::parse as jiff_parse;
use jiff::Error as JiffError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Convenient type alias for a [`Result`] holding either a [`Stamp`] or [`StampError`]
pub type StampResult<'a> = Result<Stamp<'a>, StampError>;

/// An error that can occur when parsing and validating arXiv stamps
///
/// # Examples
/// ```
/// use arxiv::Stamp;
///
/// let stamp = Stamp::try_from("arXiv:2001.00001 [cs.LG] 1 Jan 2000");
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StampError {
	InvalidArxivId(ArticleIdError),
	InvalidDate,
	InvalidCategory,
	NotEnoughComponents,
}

impl Error for StampError {}

impl Display for StampError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::InvalidArxivId(e) => write!(f, "Invalid arXiv ID: {}", e),
			Self::InvalidDate => f.write_str("Invalid date"),
			Self::InvalidCategory => f.write_str("Invalid category"),
			Self::NotEnoughComponents => f.write_str("Not enough components"),
		}
	}
}

/// A stamp that is added onto the side of PDF version of arXiv articles
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stamp<'a> {
	pub id: ArticleId<'a>,
	pub category: CategoryId<'a>,
	pub submitted: Date,
}

impl<'a> Stamp<'a> {
	/// Manually create a new [`Stamp`] from the given components.
	///
	/// # Examples
	/// ```
	/// use arxiv::{Archive, ArticleId, CategoryId, Stamp};
	/// use jiff::civil::date;
	///
	/// let stamp = Stamp::new(
	///     ArticleId::try_latest(2011, 1, "00001").unwrap(),
	///     CategoryId::try_new(Archive::Cs, "LG").unwrap(),
	///     date(2011, 1, 1)
	/// );
	/// ```
	#[inline]
	pub const fn new(id: ArticleId<'a>, category: CategoryId<'a>, submitted: Date) -> Self {
		Self {
			id,
			category,
			submitted,
		}
	}
}

impl Display for Stamp<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{} [{}] {}",
			self.id,
			self.category,
			jiff_format("%-e %b %Y", self.submitted).map_err(|_| core::fmt::Error)?
		)
	}
}

impl<'a> TryFrom<&'a str> for Stamp<'a> {
	type Error = StampError;

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		use StampError::*;

		let wsp_indices: Vec<_> = s.match_indices(' ').collect();
		if wsp_indices.len() < 2 {
			return Err(NotEnoughComponents);
		}

		// parse an id
		let space1 = wsp_indices[0].0;
		let id = ArticleId::try_from(&s[0..space1]).map_err(InvalidArxivId)?;

		// parse a category
		let space2 = wsp_indices[1].0;
		let cat_str = &s[space1 + 1..space2];
		let category = CategoryId::parse_bracketed(cat_str).ok_or(InvalidCategory)?;

		// parse a date
		let date_str = &s[space2 + 1..];
		let date = parse_date(date_str).map_err(|_| InvalidDate)?;

		Ok(Self::new(id, category, date))
	}
}

/// Parses a date in the form of "1 Jan 2000", where:
///  - the day is a number without zero padding
///  - the month is the first three letters of the full month name
///  - the year is a 4-digit number
fn parse_date(date_str: &str) -> Result<Date, JiffError> {
	jiff_parse("%e %b %Y", date_str)?.to_date()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Archive;
	use jiff::civil::date;

	#[test]
	fn display_stamp() {
		let stamp = Stamp::new(
			ArticleId::try_from("arXiv:2011.00001").unwrap(),
			CategoryId::try_new(Archive::Cs, "LG").unwrap(),
			date(2011, 1, 1),
		);
		assert_eq!(stamp.to_string(), "arXiv:2011.00001 [cs.LG] 1 Jan 2011");
	}
}

#[cfg(test)]
mod tests_parse_ok {
	use super::*;
	use crate::Archive;
	use jiff::civil::date;

	#[test]
	fn parse_stamp() {
		let stamp = "arXiv:2001.00001 [cs.LG] 1 Jan 2000";
		let parsed = Stamp::try_from(stamp);
		assert_eq!(
			parsed,
			Ok(Stamp::new(
				ArticleId::try_from("arXiv:2001.00001").unwrap(),
				CategoryId::try_new(Archive::Cs, "LG").unwrap(),
				date(2000, 1, 1)
			))
		);
	}

	#[test]
	fn parse_stamp_readme() {
		let stamp = "arXiv:0706.0001v1 [q-bio.CB] 1 Jun 2007";
		let parsed = Stamp::try_from(stamp);

		assert_eq!(
			parsed,
			Ok(Stamp::new(
				ArticleId::try_from("arXiv:0706.0001v1").unwrap(),
				CategoryId::try_new(Archive::QBio, "CB").unwrap(),
				date(2007, 6, 1)
			))
		)
	}
}

#[cfg(test)]
mod tests_parse_err {
	use super::*;

	#[test]
	fn parse_stamp_empty() {
		let stamp = "";
		let parsed = Stamp::try_from(stamp);
		assert_eq!(parsed, Err(StampError::NotEnoughComponents));
	}

	#[test]
	fn parse_stamp_not_enough_components() {
		let stamp = "arXiv:2001.00001";
		let parsed = Stamp::try_from(stamp);
		assert_eq!(parsed, Err(StampError::NotEnoughComponents));
	}

	#[test]
	fn parse_stamp_invalid_category() {
		let stamp = "arXiv:2001.00001 [cs.LG 1 Jan 2000";
		let parsed = Stamp::try_from(stamp);
		assert_eq!(parsed, Err(StampError::InvalidCategory));
	}

	#[test]
	fn parse_stamp_invalid_date_day() {
		let stamp = "arXiv:2001.00001 [cs.LG] 32 Jan 2000";
		let parsed = Stamp::try_from(stamp);
		assert_eq!(parsed, Err(StampError::InvalidDate));
	}

	#[test]
	fn parse_stamp_invalid_date_month() {
		let stamp = "arXiv:2001.00001 [cs.LG] 1 Zan 2000";
		let parsed = Stamp::try_from(stamp);
		assert_eq!(parsed, Err(StampError::InvalidDate));
	}
}
