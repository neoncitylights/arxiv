use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// A collection of publications that relate under the same field of study
///
/// Valid archive identifiers are listed under the official website's page for [category taxonomy][arxiv-cat].
///
/// [arxiv-cat]: <https://arxiv.org/category_taxonomy>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Archive {
	/// Astrophysics (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/astro-ph>
	AstroPh,

	/// Condensed Matter (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/cond-mat>
	CondMat,

	/// Computer Science (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/cs>
	Cs,

	/// Economics (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/econ>
	Econ,

	/// Electrical Engineering and Systems Science (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/eess>
	Eess,

	/// General Relativity and Quantum Cosmology (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/qr-qc>
	GrQc,

	/// High Energy Physics - Experiment (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/hep-ex>
	HepEx,

	/// High Energy Physics - Lattice (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/hep-lat>
	HepLat,

	/// High Energy Physics - Phenomenology (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/hep-ph>
	HepPh,

	/// High Energy Physics - Theory (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/hep-th>
	HepTh,

	/// Mathematical Physics (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/math-ph>
	MathPh,

	/// Mathematics (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/math>
	Math,

	/// Nonlinear Sciences (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/nlin>
	Nlin,

	/// Nuclear Experiment (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/nucl-ex>
	NuclEx,

	/// Nuclear Theory (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/nucl-th>
	NuclTh,

	/// Physics (link on [arXiv])
	///
	/// [arXiv]: < https://arxiv.org/archive/physics>
	Physics,

	/// Quantitative Biology (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/q-bio>
	QBio,

	/// Quantitative Finance (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/q-fin>
	QFin,

	/// Quantum Physics (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/quant-ph>
	QuantPh,

	/// Statistics (link on [arXiv])
	///
	/// [arXiv]: <https://arxiv.org/archive/stat>
	Stat,
}

impl Archive {
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

	pub fn is_valid_subject(&self, subject: &str) -> bool {
		match self {
			Self::AstroPh => matches!(subject, "CO" | "EP" | "GA" | "HE" | "IM" | "SR"),
			Self::CondMat => matches!(subject, |"dis-nn"| "mes-hall"
				| "mtrl-sci" | "other"
				| "quant-gas"
				| "soft" | "stat-mech"
				| "str-el" | "supr-con"),
			Self::Cs => Self::COMPSCI_TABLE.binary_search(&subject).is_ok(),
			Self::Econ => matches!(subject, "EM" | "GN" | "TH"),
			Self::Eess => matches!(subject, "AS" | "IV" | "SP" | "SY"),
			Self::GrQc => subject.is_empty(),
			Self::HepEx => subject.is_empty(),
			Self::HepLat => subject.is_empty(),
			Self::HepPh => subject.is_empty(),
			Self::HepTh => subject.is_empty(),
			Self::MathPh => subject.is_empty(),
			Self::Math => Self::MATH_TABLE.binary_search(&subject).is_ok(),
			Self::Nlin => matches!(subject, "AO" | "CD" | "CG" | "PS" | "SI"),
			Self::NuclEx => subject.is_empty(),
			Self::NuclTh => subject.is_empty(),
			Self::Physics => Self::PHYSICS_TABLE.binary_search(&subject).is_ok(),
			Self::QBio => matches!(
				subject,
				"BM" | "CB" | "GN" | "MN" | "NC" | "OT" | "PE" | "QM" | "SC" | "TO"
			),
			Self::QFin => {
				matches!(subject, "CP" | "EC" | "GN" | "MF" | "PM" | "PR" | "RM" | "ST" | "SR")
			}
			Self::QuantPh => subject.is_empty(),
			Self::Stat => matches!(subject, "AP" | "CO" | "ME" | "ML" | "OT" | "TH"),
		}
	}

	/// Checks if the archive will always have nested subjects.
	///
	/// ```
	/// use arxiv::Archive;
	///
	/// assert!(Self::GrQc.contains_subjects());
	/// ```
	#[rustfmt::skip]
	pub const fn contains_subjects(&self) -> bool {
		matches!( self,
				Self::Cs
				| Self::GrQc
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

	/// Checks if the archive can sometimes contain subjects, or have no subjects.
	pub const fn can_omit_subjects(&self) -> bool {
		matches!(self, Self::Physics | Self::CondMat)
	}

	/// Converts the article identifier to a URL where the abstract page is.
	///
	/// ```
	/// use arxiv::Archive;
	/// use url::Url;
	///
	/// let id = Self::AstroPh;
	/// let url = Url::from(id);
	/// assert_eq!(url.to_string(), "https://arxiv.org/archive/astro-ph");
	/// ```
	#[cfg(feature = "url")]
	#[cfg_attr(docsrs, doc(cfg(feature = "url")))]
	pub fn as_url(&self) -> url::Url {
		url::Url::from(*self)
	}
}

impl Display for Archive {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::AstroPh => "astro-ph",
			Self::CondMat => "cond-mat",
			Self::Cs => "cs",
			Self::Econ => "econ",
			Self::Eess => "eess",
			Self::GrQc => "gr-qc",
			Self::HepEx => "hep-ex",
			Self::HepLat => "hep-lat",
			Self::HepPh => "hep-ph",
			Self::HepTh => "hep-th",
			Self::MathPh => "math-ph",
			Self::Math => "math",
			Self::Nlin => "nlin",
			Self::NuclEx => "nucl-ex",
			Self::NuclTh => "nucl-th",
			Self::Physics => "physics",
			Self::QBio => "q-bio",
			Self::QFin => "q-fin",
			Self::QuantPh => "quant-ph",
			Self::Stat => "stat",
		})
	}
}

impl FromStr for Archive {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"astro-ph" => Self::AstroPh,
			"cond-mat" => Self::CondMat,
			"cs" => Self::Cs,
			"econ" => Self::Econ,
			"eess" => Self::Eess,
			"gr-qc" => Self::GrQc,
			"hep-ex" => Self::HepEx,
			"hep-lat" => Self::HepLat,
			"hep-ph" => Self::HepPh,
			"hep-th" => Self::HepTh,
			"math-ph" => Self::MathPh,
			"math" => Self::Math,
			"nlin" => Self::Nlin,
			"nucl-ex" => Self::NuclEx,
			"nucl-th" => Self::NuclTh,
			"physics" => Self::Physics,
			"q-bio" => Self::QBio,
			"q-fin" => Self::QFin,
			"quant-ph" => Self::QuantPh,
			"stat" => Self::Stat,
			_ => return Err(()),
		})
	}
}

#[cfg(feature = "url")]
#[cfg_attr(docsrs, doc(cfg(feature = "url")))]
impl From<Archive> for url::Url {
	fn from(archive: Archive) -> Self {
		Self::parse(&format!("https://arxiv.org/archive/{archive}")).unwrap()
	}
}

#[cfg(test)]
mod tests {
	use crate::Archive;
	use std::str::FromStr;

	#[test]
	fn test_contains_subject() {
		assert!(Archive::HepEx.contains_subjects());
		assert!(Archive::HepLat.contains_subjects());
		assert!(Archive::HepPh.contains_subjects());
		assert!(Archive::HepTh.contains_subjects());
		assert!(Archive::MathPh.contains_subjects());
		assert!(Archive::NuclEx.contains_subjects());
		assert!(Archive::NuclTh.contains_subjects());
		assert!(Archive::QuantPh.contains_subjects());
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
	use crate::Archive;
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
