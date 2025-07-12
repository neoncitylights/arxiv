use crate::Archive;

/// A type of classification for arXiv publications
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[cfg(test)]
mod tests {
	use crate::{Archive, Group};

	#[test]
	fn group_from_archive() {
		let cat_id = Group::from(Archive::AstroPh);
		assert_eq!(cat_id, Group::Physics);
	}
}
