use crate::{ArxivCategoryId, ArxivId, ArxivIdError};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
			Self::InvalidCategory => write!(f, "Invalid category"),
			Self::NotEnoughComponents => write!(f, "Not enough components"),
		}
	}
}

/// A stamp that is added onto the side of PDF version of arXiv articles
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArxivStamp<'a> {
	id: ArxivId<'a>,
	category: Option<ArxivCategoryId>,
	submitted: Date,
}

impl<'a> ArxivStamp<'a> {
	pub(crate) const TOKEN_SPACE: char = ' ';

	/// Manually create a new [`ArxivStamp`] from the given components.

	/// # Examples
	/// ```
	/// use arxiv::{ArxivArchive, ArxivCategoryId, ArxivId, ArxivStamp};
	/// use time::{Date, Month};
	///
	/// let stamp = ArxivStamp::new(
	///    ArxivId::try_latest(2011, 1, "00001").unwrap(),
	///    Some(ArxivCategoryId::try_new(ArxivArchive::Cs, "LG").unwrap()),
	///    Date::from_calendar_date(2011, Month::January, 1).unwrap()
	/// );
	/// ```
	#[inline]
	pub const fn new(id: ArxivId<'a>, category: Option<ArxivCategoryId>, submitted: Date) -> Self {
		Self {
			id,
			category,
			submitted,
		}
	}

	/// The unique arXiv identifier of the stamp
	#[must_use]
	#[inline]
	pub const fn id(&self) -> &ArxivId {
		&self.id
	}

	/// The category of the stamp
	#[must_use]
	#[inline]
	pub const fn category(&self) -> Option<&ArxivCategoryId> {
		match &self.category {
			Some(c) => Some(c),
			None => None,
		}
	}

	/// The submitted date of the given publication for the stamp
	#[must_use]
	#[inline]
	pub const fn submitted(&self) -> Date {
		self.submitted
	}
}

impl<'a> Display for ArxivStamp<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		// A stamp string is *at least* 25 characters long:
		// - 16: longest possible arXiv identifier
		// - 2: string length of a day
		// - 3: string length of an abbreviated month
		// - 4: string length of a 4-digit year
		let mut partial_stamp_str = String::with_capacity(16usize);
		partial_stamp_str.push_str(&self.id.to_string());
		if let Some(c) = &self.category {
			// This is the longest possible length of a category string,
			// such as "cond-mat.quant-gas"
			partial_stamp_str.reserve(18usize);
			partial_stamp_str.push_str(" [");
			partial_stamp_str.push_str(&c.to_string());
			partial_stamp_str.push(']');
		}

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
		let parts = s.splitn(2, ArxivStamp::TOKEN_SPACE).collect::<Vec<&str>>();

		if parts.len() == 1 {
			return Err(ArxivStampError::NotEnoughComponents);
		}

		println!("{:?}", parts[0]);
		let id = ArxivId::try_from(parts[0]).map_err(ArxivStampError::InvalidArxivId)?;

		// category is opitional, so we need to check if the second part is a category
		// and decide which index to use to parse each component
		let mut category: Option<ArxivCategoryId> = None;

		let part2_is_category = parts[1].starts_with('[');
		let date = if part2_is_category {
			let category_date = parts[1]
				.splitn(2, ArxivStamp::TOKEN_SPACE)
				.collect::<Vec<&str>>();

			let str_in_brackets =
				parse_brackets(category_date[0]).map_err(|_| ArxivStampError::InvalidCategory)?;
			let parsed_category = ArxivCategoryId::from_str(&str_in_brackets);
			if parsed_category.is_err() {
				return Err(ArxivStampError::InvalidCategory);
			}

			category = parsed_category.ok();
			parse_date(category_date[1])
		} else {
			parse_date(parts[1])
		}?;

		// if we got this far, we can safely unwrap the results
		Ok(Self::new(id, category, date))
	}
}

pub(super) fn parse_brackets(s: &str) -> Result<String, ()> {
	match brackets_match(s) {
		true => Ok(s[1..s.len() - 1].to_string()),
		false => Err(()),
	}
}

/// We only care *and* allow for straight brackets,
/// we don't care for parentheses or curly brackets
pub(super) fn brackets_match(s: &str) -> bool {
	s.starts_with('[') && s.ends_with(']')
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
fn parse_date(date_str: &str) -> Result<Date, ArxivStampError> {
	Date::parse(date_str, &format_description!("[day padding:none] [month repr:short] [year]"))
		.map_err(ArxivStampError::InvalidDate)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ArxivArchive;
	use time::error::ParseFromDescription;
	use time::Date;

	#[test]
	fn display_stamp() {
		let stamp = ArxivStamp::new(
			ArxivId::try_from("arXiv:2011.00001").unwrap(),
			Some(ArxivCategoryId::try_new(crate::ArxivArchive::Cs, "LG").unwrap()),
			Date::from_calendar_date(2011, Month::January, 1).unwrap(),
		);
		assert_eq!(stamp.to_string(), "arXiv:2011.00001 [cs.LG] 1 Jan 2011");
	}

	#[test]
	fn display_stamp_without_category() {
		let stamp = ArxivStamp::new(
			ArxivId::try_from("arXiv:2011.00001").unwrap(),
			None,
			Date::from_calendar_date(2011, Month::January, 1).unwrap(),
		);

		assert_eq!(stamp.to_string(), "arXiv:2011.00001 1 Jan 2011");
	}

	#[test]
	fn parse_stamp() {
		let stamp = "arXiv:2001.00001 [cs.LG] 1 Jan 2000";
		let parsed = ArxivStamp::try_from(stamp);
		assert_eq!(
			parsed,
			Ok(ArxivStamp::new(
				ArxivId::try_from("arXiv:2001.00001").unwrap(),
				Some(ArxivCategoryId::try_new(ArxivArchive::Cs, "LG").unwrap()),
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
				Some(ArxivCategoryId::try_new(ArxivArchive::QBio, "CB").unwrap()),
				Date::from_calendar_date(2007, Month::June, 1).unwrap(),
			))
		)
	}

	#[test]
	fn parse_stamp_without_category() {
		let stamp = "arXiv:2001.00001 1 Jan 2000";
		let parsed = ArxivStamp::try_from(stamp);
		assert_eq!(
			parsed,
			Ok(ArxivStamp::new(
				ArxivId::try_from("arXiv:2001.00001").unwrap(),
				None,
				Date::from_calendar_date(2000, Month::January, 1).unwrap(),
			))
		);
	}

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
		assert_eq!(parsed, Err(date));
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

	#[test]
	fn test_parse_brackets() {
		assert_eq!(Err(()), parse_brackets(""));
		assert_eq!(Ok(String::new()), parse_brackets("[]"));
		assert_eq!(Ok(String::from("test")), parse_brackets("[test]"));
	}

	#[test]
	fn test_brackets_match() {
		assert!(!brackets_match(""));
		assert!(brackets_match("[]"));
		assert!(!brackets_match("{}"));
		assert!(!brackets_match("()"));
	}
}
