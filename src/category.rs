use crate::subject_tables::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// An identifier for arXiv categories, which are composed of an archive and category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArxivCategoryId<'a> {
	group: ArxivGroup,
	archive: ArxivArchive,
	subject: &'a str,
}

impl<'a> ArxivCategoryId<'a> {
	pub(crate) const TOKEN_DELIM: char = '.';

	pub(super) const fn new(group: ArxivGroup, archive: ArxivArchive, subject: &'a str) -> Self {
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
	pub fn try_new(archive: ArxivArchive, subject: &'a str) -> Option<Self> {
		let is_valid = match archive {
			ArxivArchive::AstroPh => matches!(subject, "CO" | "EP" | "GA" | "HE" | "IM" | "SR"),
			ArxivArchive::CondMat => matches!(subject, |"dis-nn"| "mes-hall"
				| "mtrl-sci" | "other"
				| "quant-gas"
				| "soft" | "stat-mech"
				| "str-el" | "supr-con"),
			ArxivArchive::Cs => COMPSCI_TABLE.binary_search(&subject).is_ok(),
			ArxivArchive::Econ => matches!(subject, "EM" | "GN" | "TH"),
			ArxivArchive::Eess => matches!(subject, "AS" | "IV" | "SP" | "SY"),
			ArxivArchive::GrQc => subject.is_empty(),
			ArxivArchive::HepEx => subject.is_empty(),
			ArxivArchive::HepLat => subject.is_empty(),
			ArxivArchive::HepPh => subject.is_empty(),
			ArxivArchive::HepTh => subject.is_empty(),
			ArxivArchive::MathPh => subject.is_empty(),
			ArxivArchive::Math => MATH_TABLE.binary_search(&subject).is_ok(),
			ArxivArchive::Nlin => matches!(subject, "AO" | "CD" | "CG" | "PS" | "SI"),
			ArxivArchive::NuclEx => subject.is_empty(),
			ArxivArchive::NuclTh => subject.is_empty(),
			ArxivArchive::Physics => PHYSICS_TABLE.binary_search(&subject).is_ok(),
			ArxivArchive::QBio => matches!(
				subject,
				"BM" | "CB" | "GN" | "MN" | "NC" | "OT" | "PE" | "QM" | "SC" | "TO"
			),
			ArxivArchive::QFin => {
				matches!(subject, "CP" | "EC" | "GN" | "MF" | "PM" | "PR" | "RM" | "ST" | "SR")
			}
			ArxivArchive::QuantPh => subject.is_empty(),
			ArxivArchive::Stat => matches!(subject, "AP" | "CO" | "ME" | "ML" | "OT" | "TH"),
		};

		match is_valid {
			true => Some(Self::new(ArxivGroup::from(archive), archive, subject)),
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
	pub const fn group(&self) -> ArxivGroup {
		self.group
	}

	/// The archive, representing a collection of publications
	/// that relate to each other by a specific field of study
	#[must_use]
	#[inline]
	pub const fn archive(&self) -> ArxivArchive {
		self.archive
	}

	/// The subject class of the arXiv category
	#[must_use]
	#[inline]
	pub fn subject(&self) -> &'a str {
		self.subject
	}
}

impl<'a> Display for ArxivCategoryId<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}.{}", self.archive, self.subject)
	}
}

impl<'a> TryFrom<&'a str> for ArxivCategoryId<'a> {
	type Error = ();
	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		let parts: Vec<&str> = s.split(Self::TOKEN_DELIM).collect();
		if parts.len() != 2 {
			return Err(());
		}

		let archive = ArxivArchive::from_str(parts[0])?;
		let subject = parts[1];

		Self::try_new(archive, subject).ok_or(())
	}
}

/// A type of classification for arXiv publications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArxivGroup {
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

impl From<ArxivArchive> for ArxivGroup {
	fn from(archive: ArxivArchive) -> Self {
		match archive {
			ArxivArchive::Cs => Self::Cs,
			ArxivArchive::Econ => Self::Econ,
			ArxivArchive::Eess => Self::Eess,
			ArxivArchive::Math => Self::Math,
			ArxivArchive::AstroPh
			| ArxivArchive::CondMat
			| ArxivArchive::GrQc
			| ArxivArchive::HepEx
			| ArxivArchive::HepLat
			| ArxivArchive::HepPh
			| ArxivArchive::HepTh
			| ArxivArchive::MathPh
			| ArxivArchive::Nlin
			| ArxivArchive::NuclEx
			| ArxivArchive::NuclTh
			| ArxivArchive::Physics
			| ArxivArchive::QuantPh => Self::Physics,
			ArxivArchive::QBio => Self::QBio,
			ArxivArchive::QFin => Self::QFin,
			ArxivArchive::Stat => Self::Stat,
		}
	}
}

/// A collection of publications that relate under the same field of study
///
/// Valid archive identifiers are listed under the official website's page for [category taxonomy][arxiv-cat].
///
/// [arxiv-cat]: <https://arxiv.org/category_taxonomy>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArxivArchive {
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

impl Display for ArxivArchive {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			ArxivArchive::AstroPh => f.write_str("astro-ph"),
			ArxivArchive::CondMat => f.write_str("cond-mat"),
			ArxivArchive::Cs => f.write_str("cs"),
			ArxivArchive::Econ => f.write_str("econ"),
			ArxivArchive::Eess => f.write_str("eess"),
			ArxivArchive::GrQc => f.write_str("gr-qc"),
			ArxivArchive::HepEx => f.write_str("hep-ex"),
			ArxivArchive::HepLat => f.write_str("hep-lat"),
			ArxivArchive::HepPh => f.write_str("hep-ph"),
			ArxivArchive::HepTh => f.write_str("hep-th"),
			ArxivArchive::MathPh => f.write_str("math-ph"),
			ArxivArchive::Math => f.write_str("math"),
			ArxivArchive::Nlin => f.write_str("nlin"),
			ArxivArchive::NuclEx => f.write_str("nucl-ex"),
			ArxivArchive::NuclTh => f.write_str("nucl-th"),
			ArxivArchive::Physics => f.write_str("physics"),
			ArxivArchive::QBio => f.write_str("q-bio"),
			ArxivArchive::QFin => f.write_str("q-fin"),
			ArxivArchive::QuantPh => f.write_str("quant-ph"),
			ArxivArchive::Stat => f.write_str("stat"),
		}
	}
}

impl FromStr for ArxivArchive {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_category_id() {
		let cat_id = ArxivCategoryId::try_from("cs.LG");
		assert_eq!(cat_id, Ok(ArxivCategoryId::new(ArxivGroup::Cs, ArxivArchive::Cs, "LG")));
	}

	#[test]
	fn display_category() {
		assert_eq!(
			ArxivCategoryId::try_new(ArxivArchive::AstroPh, "HE")
				.unwrap()
				.to_string(),
			"astro-ph.HE"
		);
	}

	#[test]
	fn group_from_archive() {
		assert_eq!(ArxivGroup::from(ArxivArchive::AstroPh), ArxivGroup::Physics);
	}

	#[test]
	fn parse_archive() {
		let archive = ArxivArchive::from_str("astro-ph");
		assert_eq!(archive, Ok(ArxivArchive::AstroPh));
	}
}
