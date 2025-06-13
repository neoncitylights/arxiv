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
	InvalidArchive(&'a str),
	InvalidArchiveSubject(Archive, &'a str),
}

impl Error for CategoryIdError<'_> {}

impl Display for CategoryIdError<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::ExpectedSubject => f.write_str("Expected to find a subject identifier"),
			Self::InvalidArchive(s) => write!(f, "Invalid arXiv archive identifier: {s}"),
			Self::InvalidArchiveSubject(archive, subject_str) => write!(
				f,
				"The arXiv subject \"{archive}\" does not fall under the archive \"{subject_str}\""
			),
		}
	}
}

/// An identifier for arXiv categories, which are composed of an archive and category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CategoryId<'a> {
	group: Group,
	archive: Archive,
	subject: &'a str,
}

impl<'a> CategoryId<'a> {
	pub(crate) const TOKEN_DELIM: char = '.';
	pub(crate) const COMPSCI_TABLE: &'static [&'static str] = &[
		"AI", "AR", "CC", "CE", "CG", "CL", "CR", "CV", "CY", "DB", "DC", "DL", "DM", "DS", "ET",
		"FL", "GL", "GR", "GT", "HC", "IR", "IT", "LG", "LO", "MA", "MM", "MS", "NA", "NI", "OH",
		"OS", "PF", "PL", "RO", "SC", "SD", "SE", "SI", "SY",
	];

	pub(crate) const MATH_TABLE: &'static [&'static str] = &[
		"AC", "AG", "AP", "AT", "CA", "CO", "CT", "CV", "DG", "DS", "FA", "GM", "GN", "GR", "GT",
		"HO", "IT", "KT", "LO", "MG", "MP", "NA", "NT", "OA", "OC", "PR", "QA", "RA", "RT", "SG",
		"SP", "ST",
	];

	pub(crate) const PHYSICS_TABLE: &'static [&'static str] = &[
		"acc-ph", "ao-ph", "app-ph", "atm-clus", "atom-ph", "bio-ph", "chem-ph", "class-ph",
		"comp-ph", "data-an", "ed-pn", "flu-dyn", "gen-ph", "geo-ph", "hist-ph", "ins-det",
		"med-ph", "optics", "plasm-ph", "pop-ph", "soc-ph", "space-ph",
	];

	pub(super) const fn new(group: Group, archive: Archive, subject: &'a str) -> Self {
		Self {
			group,
			archive,
			subject,
		}
	}

	/// Checks if the string is a valid group identifier, based on the archive and category.
	///
	/// Valid archive identifiers are listed under the official website's page for [category taxonomy][arxiv-cat].
	///
	/// [arxiv-cat]: <https://arxiv.org/category_taxonomy>
	pub fn try_new(archive: Archive, subject: &'a str) -> Option<Self> {
		let is_valid = match archive {
			Archive::AstroPh => matches!(subject, "CO" | "EP" | "GA" | "HE" | "IM" | "SR"),
			Archive::CondMat => matches!(subject, |"dis-nn"| "mes-hall"
				| "mtrl-sci" | "other"
				| "quant-gas"
				| "soft" | "stat-mech"
				| "str-el" | "supr-con"),
			Archive::Cs => Self::COMPSCI_TABLE.binary_search(&subject).is_ok(),
			Archive::Econ => matches!(subject, "EM" | "GN" | "TH"),
			Archive::Eess => matches!(subject, "AS" | "IV" | "SP" | "SY"),
			Archive::GrQc => subject.is_empty(),
			Archive::HepEx => subject.is_empty(),
			Archive::HepLat => subject.is_empty(),
			Archive::HepPh => subject.is_empty(),
			Archive::HepTh => subject.is_empty(),
			Archive::MathPh => subject.is_empty(),
			Archive::Math => Self::MATH_TABLE.binary_search(&subject).is_ok(),
			Archive::Nlin => matches!(subject, "AO" | "CD" | "CG" | "PS" | "SI"),
			Archive::NuclEx => subject.is_empty(),
			Archive::NuclTh => subject.is_empty(),
			Archive::Physics => Self::PHYSICS_TABLE.binary_search(&subject).is_ok(),
			Archive::QBio => matches!(
				subject,
				"BM" | "CB" | "GN" | "MN" | "NC" | "OT" | "PE" | "QM" | "SC" | "TO"
			),
			Archive::QFin => {
				matches!(subject, "CP" | "EC" | "GN" | "MF" | "PM" | "PR" | "RM" | "ST" | "SR")
			}
			Archive::QuantPh => subject.is_empty(),
			Archive::Stat => matches!(subject, "AP" | "CO" | "ME" | "ML" | "OT" | "TH"),
		};

		is_valid.then(|| Self::new(Group::from(archive), archive, subject))
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
	pub fn subject(&self) -> &'a str {
		self.subject
	}
}

impl Display for CategoryId<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}.{}", self.archive, self.subject)
	}
}

impl<'a> TryFrom<&'a str> for CategoryId<'a> {
	type Error = CategoryIdError<'a>;
	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		use CategoryIdError::*;

		let parts: Vec<&str> = s.split(Self::TOKEN_DELIM).collect();
		if parts.len() != 2 {
			return Err(ExpectedSubject);
		}

		let (archive_str, subject) = (parts[0], parts[1]);
		if subject.is_empty() {
			return Err(ExpectedSubject);
		}

		let archive = Archive::from_str(archive_str).map_err(|_| InvalidArchive(archive_str))?;
		Self::try_new(archive, subject).ok_or(InvalidArchiveSubject(archive, subject))
	}
}

#[cfg(test)]
mod tests {
	use crate::{Archive, CategoryId, CategoryIdError, Group};
	use CategoryIdError::*;

	#[test]
	fn parse_ok() {
		let cat_id = CategoryId::try_from("cs.LG");
		assert_eq!(cat_id, Ok(CategoryId::new(Group::Cs, Archive::Cs, "LG")));
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
