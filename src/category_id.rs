#![cfg_attr(docsrs, feature(doc_cfg))]

use crate::{Archive, Group};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// [`Result`] type alias holding either a [`CategoryId`] or [`CategoryIdError`]
pub type CategoryIdResult<'a> = Result<CategoryId<'a>, CategoryIdError<'a>>;

/// An error that can occur when parsing and validating arXiv category identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CategoryIdError<'a> {
	ExpectedSubject,
	ExpectedNoSubject,
	InvalidArchive(&'a str),
	InvalidArchiveSubject(Archive, &'a str),
}

impl Error for CategoryIdError<'_> {}

impl Display for CategoryIdError<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::ExpectedSubject => f.write_str("Expected a subject identifier, but found none"),
			Self::ExpectedNoSubject => f.write_str("Expected no subject identifier, but found one"),
			Self::InvalidArchive(s) => write!(f, "Invalid arXiv archive identifier: {s}"),
			Self::InvalidArchiveSubject(archive, subject_str) => write!(
				f,
				"The arXiv subject \"{archive}\" does not fall under the archive \"{subject_str}\""
			),
		}
	}
}

/// An identifier for arXiv categories,
/// which are composed of an archive and optionally a subject
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CategoryId<'a> {
	group: Group,
	archive: Archive,
	subject: Option<&'a str>,
}

impl<'a> CategoryId<'a> {
	pub(crate) const TOKEN_DELIM: char = '.';

	pub(super) const fn new(group: Group, archive: Archive, subject: Option<&'a str>) -> Self {
		Self {
			group,
			archive,
			subject,
		}
	}

	/// Checks if the string is a valid group identifier,
	/// based on the archive and category.
	///
	/// Valid archive identifiers are listed under the
	/// official website's page for [category taxonomy][arxiv-cat].
	///
	/// [arxiv-cat]: <https://arxiv.org/category_taxonomy>
	pub fn try_new(archive: Archive, subject: &'a str) -> Result<Self, CategoryIdError<'a>> {
		let is_valid_subject = archive.is_valid_subject(subject);

		match (is_valid_subject, subject.is_empty()) {
			(true, false) => Ok(Self::new(Group::from(archive), archive, Some(subject))),
			(true, true) => Ok(Self::new(Group::from(archive), archive, None)),
			(false, true) => Err(CategoryIdError::ExpectedNoSubject),
			(false, false) => Err(CategoryIdError::ExpectedNoSubject),
		}
	}

	/// Parse a bracketed string like `[astro-ph.CE]`
	///
	/// # Examples
	/// ```
	/// use arxiv::{Archive, CategoryId, Group};
	///
	/// let category = CategoryId::try_from("astro-ph.EP").unwrap();
	/// assert_eq!(category.group(), Group::Physics);
	/// assert_eq!(category.archive(), Archive::AstroPh);
	/// assert_eq!(category.subject(), "EP");
	/// ```
	pub fn parse_bracketed(s: &'a str) -> Option<Self> {
		match s.starts_with('[') && s.ends_with(']') {
			true => Self::try_from(&s[1..s.len() - 1]).ok(),
			false => None,
		}
	}

	/// The group, which contains one or more archives
	#[must_use]
	#[inline]
	pub const fn group(&self) -> Group {
		self.group
	}

	/// The archive, representing a collection of publications
	/// that relate to each other by a specific field of study
	#[must_use]
	#[inline]
	pub const fn archive(&self) -> Archive {
		self.archive
	}

	/// The subject class of the arXiv category
	#[must_use]
	#[inline]
	pub fn subject(&self) -> Option<&'a str> {
		self.subject
	}
}

impl Display for CategoryId<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self.subject {
			Some(s) => write!(f, "{}.{}", self.archive, s),
			None => write!(f, "{}", self.archive),
		}
	}
}

impl<'a> TryFrom<&'a str> for CategoryId<'a> {
	type Error = CategoryIdError<'a>;

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		use CategoryIdError::*;

		let mut dot_index = None;
		for (i, c) in s.char_indices() {
			if c == Self::TOKEN_DELIM {
				dot_index = Some(i);
				break;
			}
		}

		let (archive_str, subject_str) = match dot_index {
			Some(i) => (&s[..i], Some(&s[i + 1..])),
			None => (s, None),
		};

		let archive = Archive::from_str(archive_str).map_err(|_| InvalidArchive(archive_str))?;
		match (archive.contains_subjects(), subject_str) {
			(true, Some(s)) => CategoryId::try_new(archive, s),
			(true, None) if archive.can_omit_subjects() => CategoryId::try_new(archive, ""),
			(true, None) => Err(ExpectedSubject),
			(false, None) => CategoryId::try_new(archive, ""),
			(false, Some(_)) => Err(ExpectedNoSubject),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{Archive, CategoryId, CategoryIdError, Group};
	use CategoryIdError::*;

	#[test]
	fn parse_ok() {
		let cat_id = CategoryId::try_from("cs.LG");
		assert_eq!(cat_id, Ok(CategoryId::new(Group::Cs, Archive::Cs, Some("LG"))));
	}

	// special case because cond_mat might not have a subject sometimes
	#[test]
	fn parse_cond_mat() {
		let cat_id = CategoryId::try_from("cond-mat");
		assert_eq!(cat_id, Ok(CategoryId::new(Group::Physics, Archive::CondMat, None)));
	}

	#[test]
	fn parse_err_expected_subject() {
		let cat_id = CategoryId::try_from("cs");
		assert_eq!(cat_id, Err(ExpectedSubject));
	}

	#[test]
	fn parse_err_expected_subject_empty() {
		let cat_id = CategoryId::try_from("cs.");
		assert_eq!(cat_id, Err(ExpectedSubject));
	}

	#[test]
	fn parse_err_invalid_archive() {
		let cat_id = CategoryId::try_from("ecot.LG");
		assert_eq!(cat_id, Err(InvalidArchive("ecot")));
	}

	#[test]
	fn parse_err_invalid_subject() {
		let cat_id = CategoryId::try_from("econ.foo");
		assert_eq!(cat_id, Err(InvalidArchiveSubject(Archive::Econ, "foo")));
	}

	#[test]
	fn display_category() {
		let cat_id = CategoryId::try_new(Archive::AstroPh, "HE").unwrap();
		assert_eq!(cat_id.to_string(), "astro-ph.HE");
	}
}
