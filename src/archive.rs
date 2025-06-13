use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// A collection of publications that relate under the same field of study
///
/// Valid archive identifiers are listed under the official website's page for [category taxonomy][arxiv-cat].
///
/// [arxiv-cat]: <https://arxiv.org/category_taxonomy>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
	/// Checks if the archive contains any nested subjects.
	///
	/// ```
	/// use arxiv::Archive;
	///
	/// assert!(Archive::GrQc.contains_subjects());
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
	#[cfg_attr(docsrs, doc(cfg(feature = "url")))]
	pub fn as_url(&self) -> url::Url {
		url::Url::from(*self)
	}
}

impl Display for Archive {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::AstroPh => f.write_str("astro-ph"),
			Self::CondMat => f.write_str("cond-mat"),
			Self::Cs => f.write_str("cs"),
			Self::Econ => f.write_str("econ"),
			Self::Eess => f.write_str("eess"),
			Self::GrQc => f.write_str("gr-qc"),
			Self::HepEx => f.write_str("hep-ex"),
			Self::HepLat => f.write_str("hep-lat"),
			Self::HepPh => f.write_str("hep-ph"),
			Self::HepTh => f.write_str("hep-th"),
			Self::MathPh => f.write_str("math-ph"),
			Self::Math => f.write_str("math"),
			Self::Nlin => f.write_str("nlin"),
			Self::NuclEx => f.write_str("nucl-ex"),
			Self::NuclTh => f.write_str("nucl-th"),
			Self::Physics => f.write_str("physics"),
			Self::QBio => f.write_str("q-bio"),
			Self::QFin => f.write_str("q-fin"),
			Self::QuantPh => f.write_str("quant-ph"),
			Self::Stat => f.write_str("stat"),
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
