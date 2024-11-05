use crate::{ArxivCategoryId, ArxivId, ArxivIdError};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use time::error::Parse as TimeParseError;
use time::macros::format_description;
use time::{Date, Month};

/// Convenient type alias for a [`Result`] holding either an [`ArxivStamp`] or [`ArxivStampError`]
pub type ArxivStampResult<'a> = Result<ArxivStamp<'a>, ArxivStampError>;

/// An error that can occur when parsing and validating arXiv stamps
///
/// # Examples
/// ```
/// use arxiv::ArxivStamp;
///
/// let stamp = ArxivStamp::try_from("arXiv:2001.00001 [cs.LG] 1 Jan 2000");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArxivStampError {
	InvalidArxivId(ArxivIdError),
	InvalidDate(TimeParseError),
	InvalidCategory,
	NotEnoughComponents,
}

impl Error for ArxivStampError {}

impl Display for ArxivStampError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::InvalidArxivId(e) => write!(f, "Invalid arXiv ID: {}", e),
			Self::InvalidDate(e) => write!(f, "Invalid date: {}", e),
			Self::InvalidCategory => f.write_str("Invalid category"),
			Self::NotEnoughComponents => f.write_str("Not enough components"),
		}
	}
}

/// A stamp that is added onto the side of PDF version of arXiv articles
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArxivStamp<'a> {
	pub id: ArxivId<'a>,
	pub category: ArxivCategoryId<'a>,
	pub submitted: Date,
}

impl<'a> ArxivStamp<'a> {
	/// Manually create a new [`ArxivStamp`] from the given components.

	/// # Examples
	/// ```
	/// use arxiv::{ArxivArchive, ArxivCategoryId, ArxivId, ArxivStamp};
	/// use time::{Date, Month};
	///
	/// let stamp = ArxivStamp::new(
	///    ArxivId::try_latest(2011, 1, "00001").unwrap(),
	///    ArxivCategoryId::try_new(ArxivArchive::Cs, "LG").unwrap(),
	///    Date::from_calendar_date(2011, Month::January, 1).unwrap()
	/// );
	/// ```
	#[inline]
	pub const fn new(id: ArxivId<'a>, category: ArxivCategoryId<'a>, submitted: Date) -> Self {
		Self {
			id,
			category,
			submitted,
		}
	}
}

impl<'a> Display for ArxivStamp<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let id = &self.id.to_string();
		let cat = &self.category.to_string();
		// A stamp string is *at least* 25 characters long:
		// - 16: longest possible arXiv identifier
		// - 2: string length of a day
		// - 3: string length of an abbreviated month
		// - 4: string length of a 4-digit year
		let mut partial_stamp_str = String::with_capacity(9 + id.len() + cat.len());
		partial_stamp_str.push_str(id);
		partial_stamp_str.push_str(" [");
		partial_stamp_str.push_str(cat);
		partial_stamp_str.push(']');

		write!(
			f,
			"{} {} {} {}",
			partial_stamp_str,
			self.submitted.day(),
			month_as_abbr(self.submitted.month()),
			self.submitted.year()
		)
	}
}

impl<'a> TryFrom<&'a str> for ArxivStamp<'a> {
	type Error = ArxivStampError;

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		use ArxivStampError::*;

		let wsp_indices: Vec<_> = s.match_indices(' ').collect();
		if wsp_indices.len() < 2 {
			return Err(NotEnoughComponents);
		}

		// parse an id
		let space1 = wsp_indices[0].0;
		let id = ArxivId::try_from(&s[0..space1]).map_err(InvalidArxivId)?;

		// parse a category
		let space2 = wsp_indices[1].0;
		let cat_str = &s[space1 + 1..space2];
		let category = ArxivCategoryId::parse_bracketed(cat_str).ok_or(InvalidCategory)?;

		// parse a date
		let date_str = &s[space2 + 1..];
		let date = parse_date(date_str).map_err(ArxivStampError::InvalidDate)?;

		Ok(Self::new(id, category, date))
	}
}

const fn month_as_abbr<'a>(month: Month) -> &'a str {
	match month {
		Month::January => "Jan",
		Month::February => "Feb",
		Month::March => "Mar",
		Month::April => "Apr",
		Month::May => "May",
		Month::June => "June",
		Month::July => "July",
		Month::August => "Aug",
		Month::September => "Sept",
		Month::October => "Oct",
		Month::November => "Nov",
		Month::December => "Dec",
	}
}

/// Parses a date in the form of "1 Jan 2000", where:
///  - the day is a number without zero padding
///  - the month is the first three letters of the full month name
///  - the year is a 4-digit number
///
/// See also: [`time` documentation for format descriptions][time-format-desc]
///
/// [time-format-desc]: https://time-rs.github.io/book/api/format-description.html
fn parse_date(date_str: &str) -> Result<Date, time::error::Parse> {
	Date::parse(date_str, &format_description!("[day padding:none] [month repr:short] [year]"))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ArxivArchive;
	use time::Date;

	#[test]
	fn display_stamp() {
		let stamp = ArxivStamp::new(
			ArxivId::try_from("arXiv:2011.00001").unwrap(),
			ArxivCategoryId::try_new(ArxivArchive::Cs, "LG").unwrap(),
			Date::from_calendar_date(2011, Month::January, 1).unwrap(),
		);
		assert_eq!(stamp.to_string(), "arXiv:2011.00001 [cs.LG] 1 Jan 2011");
	}
}

#[cfg(test)]
mod tests_parse_ok {
	use super::*;
	use crate::ArxivArchive;
	use time::Date;

	#[test]
	fn parse_stamp() {
		let stamp = "arXiv:2001.00001 [cs.LG] 1 Jan 2000";
		let parsed = ArxivStamp::try_from(stamp);
		assert_eq!(
			parsed,
			Ok(ArxivStamp::new(
				ArxivId::try_from("arXiv:2001.00001").unwrap(),
				ArxivCategoryId::try_new(ArxivArchive::Cs, "LG").unwrap(),
				Date::from_calendar_date(2000, Month::January, 1).unwrap(),
			))
		);
	}

	#[test]
	fn parse_stamp_readme() {
		let stamp = "arXiv:0706.0001v1 [q-bio.CB] 1 Jun 2007";
		let parsed = ArxivStamp::try_from(stamp);

		assert_eq!(
			parsed,
			Ok(ArxivStamp::new(
				ArxivId::try_from("arXiv:0706.0001v1").unwrap(),
				ArxivCategoryId::try_new(ArxivArchive::QBio, "CB").unwrap(),
				Date::from_calendar_date(2007, Month::June, 1).unwrap(),
			))
		)
	}
}

#[cfg(test)]
mod tests_parse_err {
	use super::*;
	use time::error::ParseFromDescription;

	#[test]
	fn parse_stamp_empty() {
		let stamp = "";
		let parsed = ArxivStamp::try_from(stamp);
		assert_eq!(parsed, Err(ArxivStampError::NotEnoughComponents));
	}

	#[test]
	fn parse_stamp_not_enough_components() {
		let stamp = "arXiv:2001.00001";
		let parsed = ArxivStamp::try_from(stamp);
		assert_eq!(parsed, Err(ArxivStampError::NotEnoughComponents));
	}

	#[test]
	fn parse_stamp_invalid_category() {
		let stamp = "arXiv:2001.00001 [cs.LG 1 Jan 2000";
		let parsed = ArxivStamp::try_from(stamp);
		assert_eq!(parsed, Err(ArxivStampError::InvalidCategory));
	}

	#[test]
	fn parse_stamp_invalid_date_day() {
		let stamp = "arXiv:2001.00001 [cs.LG] 32 Jan 2000";
		let parsed = ArxivStamp::try_from(stamp);

		let date = parse_date("32 Jan 2000").unwrap_err();
		assert_eq!(parsed, Err(ArxivStampError::InvalidDate(date)));
	}

	#[test]
	fn parse_stamp_invalid_date_month() {
		let stamp = "arXiv:2001.00001 [cs.LG] 1 Zan 2000";
		let parsed = ArxivStamp::try_from(stamp);
		assert_eq!(parsed, Err(invalid_date_component("month")));
	}

	#[test]
	fn parse_stamp_invalid_date_year() {
		let stamp = "arXiv:2001.00001 [cs.LG] 1 Jan 200";
		let parsed = ArxivStamp::try_from(stamp);
		assert_eq!(parsed, Err(invalid_date_component("year")));
	}

	fn invalid_date_component(component: &'static str) -> ArxivStampError {
		ArxivStampError::InvalidDate(TimeParseError::ParseFromDescription(
			ParseFromDescription::InvalidComponent(component),
		))
	}
}
