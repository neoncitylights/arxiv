use crate::subject_tables::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// An identifier for arXiv categories, which are composed of an archive and category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CategoryId<'a> {
	group: Group,
	archive: Archive,
	subject: &'a str,
}

impl<'a> CategoryId<'a> {
	pub(crate) const TOKEN_DELIM: char = '.';

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
			Archive::Cs => COMPSCI_TABLE.binary_search(&subject).is_ok(),
			Archive::Econ => matches!(subject, "EM" | "GN" | "TH"),
			Archive::Eess => matches!(subject, "AS" | "IV" | "SP" | "SY"),
			Archive::GrQc => subject.is_empty(),
			Archive::HepEx => subject.is_empty(),
			Archive::HepLat => subject.is_empty(),
			Archive::HepPh => subject.is_empty(),
			Archive::HepTh => subject.is_empty(),
			Archive::MathPh => subject.is_empty(),
			Archive::Math => MATH_TABLE.binary_search(&subject).is_ok(),
			Archive::Nlin => matches!(subject, "AO" | "CD" | "CG" | "PS" | "SI"),
			Archive::NuclEx => subject.is_empty(),
			Archive::NuclTh => subject.is_empty(),
			Archive::Physics => PHYSICS_TABLE.binary_search(&subject).is_ok(),
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

		match is_valid {
			true => Some(Self::new(Group::from(archive), archive, subject)),
			false => None,
		}
	}

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

impl<'a> Display for CategoryId<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}.{}", self.archive, self.subject)
	}
}

impl<'a> TryFrom<&'a str> for CategoryId<'a> {
	type Error = ();
	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		let parts: Vec<&str> = s.split(Self::TOKEN_DELIM).collect();
		if parts.len() != 2 {
			return Err(());
		}

		let archive = Archive::from_str(parts[0])?;
		let subject = parts[1];

		Self::try_new(archive, subject).ok_or(())
	}
}

/// A type of classification for arXiv publications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
	/// Computer Science
	Cs,
	/// Economics
	Econ,
	/// Electrical Engineering and Systems Science
	Eess,
	/// Mathematics
	Math,
	/// Physics
	Physics,
	/// Quantitative Biology
	QBio,
	/// Quantitative Finance
	QFin,
	/// Statistics
	Stat,
}

impl From<Archive> for Group {
	fn from(archive: Archive) -> Self {
		match archive {
			Archive::Cs => Self::Cs,
			Archive::Econ => Self::Econ,
			Archive::Eess => Self::Eess,
			Archive::Math => Self::Math,
			Archive::AstroPh
			| Archive::CondMat
			| Archive::GrQc
			| Archive::HepEx
			| Archive::HepLat
			| Archive::HepPh
			| Archive::HepTh
			| Archive::MathPh
			| Archive::Nlin
			| Archive::NuclEx
			| Archive::NuclTh
			| Archive::Physics
			| Archive::QuantPh => Self::Physics,
			Archive::QBio => Self::QBio,
			Archive::QFin => Self::QFin,
			Archive::Stat => Self::Stat,
		}
	}
}

/// A collection of publications that relate under the same field of study
///
/// Valid archive identifiers are listed under the official website's page for [category taxonomy][arxiv-cat].
///
/// [arxiv-cat]: <https://arxiv.org/category_taxonomy>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Archive {
	/// Astro physics
	AstroPh,
	/// Condensed matter
	CondMat,
	/// Computer science
	Cs,
	/// Economics
	Econ,
	/// Electrical Engineering and Systems Science
	Eess,
	/// General Relativity and Quantum Cosmology
	GrQc,
	/// High energy physics - Experiment
	HepEx,
	/// High energy physics - Lattice
	HepLat,
	/// High energy physics - Phenomenology
	HepPh,
	/// High energy physics - Theory
	HepTh,
	/// Mathematical Physics
	MathPh,
	/// Mathematics
	Math,
	/// Nonlinear Sciences
	Nlin,
	/// Nuclear Experiment
	NuclEx,
	/// Nuclear Theory
	NuclTh,
	/// Physics
	Physics,
	/// Quantitative Biology
	QBio,
	/// Quantitative Finance
	QFin,
	/// Quantum Physics
	QuantPh,
	/// Statistics
	Stat,
}

impl Archive {
	/// Checks if the archive contains any nested subjects
	///
	/// ```
	/// use arxiv::Archive;
	///
	/// assert!(Archive::GrQc.contains_subjects());
	/// assert!(Archive::HepEx.contains_subjects());
	/// assert!(Archive::HepLat.contains_subjects());
	/// assert!(Archive::HepPh.contains_subjects());
	/// assert!(Archive::HepTh.contains_subjects());
	/// assert!(Archive::MathPh.contains_subjects());
	/// assert!(Archive::NuclEx.contains_subjects());
	/// assert!(Archive::NuclTh.contains_subjects());
	/// assert!(Archive::QuantPh.contains_subjects());
	/// ```
	pub const fn contains_subjects(&self) -> bool {
		matches!(
			self,
			Self::GrQc
				| Self::HepEx
				| Self::HepLat
				| Self::HepPh
				| Self::HepTh
				| Self::MathPh
				| Self::NuclEx
				| Self::NuclTh
				| Self::QuantPh
		)
	}

	/// Converts the article identifier to a URL where the abstract page is.
	///
	/// ```
	/// use arxiv::Archive;
	/// use url::Url;
	///
	/// let id = Archive::AstroPh;
	/// let url = Url::from(id);
	/// assert_eq!(url.to_string(), "https://arxiv.org/archive/astro-ph");
	/// ```
	#[cfg(feature = "url")]
	pub fn as_url(&self) -> url::Url {
		url::Url::from(*self)
	}
}

impl Display for Archive {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Archive::AstroPh => f.write_str("astro-ph"),
			Archive::CondMat => f.write_str("cond-mat"),
			Archive::Cs => f.write_str("cs"),
			Archive::Econ => f.write_str("econ"),
			Archive::Eess => f.write_str("eess"),
			Archive::GrQc => f.write_str("gr-qc"),
			Archive::HepEx => f.write_str("hep-ex"),
			Archive::HepLat => f.write_str("hep-lat"),
			Archive::HepPh => f.write_str("hep-ph"),
			Archive::HepTh => f.write_str("hep-th"),
			Archive::MathPh => f.write_str("math-ph"),
			Archive::Math => f.write_str("math"),
			Archive::Nlin => f.write_str("nlin"),
			Archive::NuclEx => f.write_str("nucl-ex"),
			Archive::NuclTh => f.write_str("nucl-th"),
			Archive::Physics => f.write_str("physics"),
			Archive::QBio => f.write_str("q-bio"),
			Archive::QFin => f.write_str("q-fin"),
			Archive::QuantPh => f.write_str("quant-ph"),
			Archive::Stat => f.write_str("stat"),
		}
	}
}

impl FromStr for Archive {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"astro-ph" => Ok(Self::AstroPh),
			"cond-mat" => Ok(Self::CondMat),
			"cs" => Ok(Self::Cs),
			"econ" => Ok(Self::Econ),
			"eess" => Ok(Self::Eess),
			"gr-qc" => Ok(Self::GrQc),
			"hep-ex" => Ok(Self::HepEx),
			"hep-lat" => Ok(Self::HepLat),
			"hep-ph" => Ok(Self::HepPh),
			"hep-th" => Ok(Self::HepTh),
			"math-ph" => Ok(Self::MathPh),
			"math" => Ok(Self::Math),
			"nlin" => Ok(Self::Nlin),
			"nucl-ex" => Ok(Self::NuclEx),
			"nucl-th" => Ok(Self::NuclTh),
			"physics" => Ok(Self::Physics),
			"q-bio" => Ok(Self::QBio),
			"q-fin" => Ok(Self::QFin),
			"quant-ph" => Ok(Self::QuantPh),
			"stat" => Ok(Self::Stat),
			_ => Err(()),
		}
	}
}

#[cfg(feature = "url")]
impl From<Archive> for url::Url {
	fn from(archive: Archive) -> url::Url {
		url::Url::parse(&format!("https://arxiv.org/archive/{}", archive)).unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_category_id() {
		let cat_id = CategoryId::try_from("cs.LG");
		assert_eq!(cat_id, Ok(CategoryId::new(Group::Cs, Archive::Cs, "LG")));
	}

	#[test]
	fn display_category() {
		let cat_id = CategoryId::try_new(Archive::AstroPh, "HE");
		assert_eq!(cat_id.unwrap().to_string(), "astro-ph.HE");
	}

	#[test]
	fn group_from_archive() {
		let cat_id = Group::from(Archive::AstroPh);
		assert_eq!(cat_id, Group::Physics);
	}

	#[test]
	fn parse_archive() {
		let archive = Archive::from_str("astro-ph");
		assert_eq!(archive, Ok(Archive::AstroPh));
	}
}

#[cfg(test)]
#[cfg(feature = "url")]
mod tests_url_archive {
	use super::*;
	use url::Url;

	#[test]
	fn url_from_id() {
		let id = Archive::AstroPh;
		let url = Url::from(id);
		assert_eq!(url.scheme(), "https");
		assert_eq!(url.domain(), Some("arxiv.org"));
		assert_eq!(url.path(), "/archive/astro-ph");
		assert_eq!(url.to_string(), "https://arxiv.org/archive/astro-ph");
	}
}
