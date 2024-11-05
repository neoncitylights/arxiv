//! ```rust
//! use arxiv::*;
//!
//! // Parse an arXiv identifier
//! let id = ArxivId::try_from("arXiv:9912.12345v2").unwrap();
//! assert_eq!(id.month(), 12);
//! assert_eq!(id.year(), 2099);
//! assert_eq!(id.number, "12345");
//! assert_eq!(id.version, ArticleVersion::Num(2));
//!
//! // Parse an arXiv category
//! let category = ArxivCategoryId::try_from("astro-ph.HE").unwrap();
//! assert_eq!(category.group(), ArxivGroup::Physics);
//! assert_eq!(category.archive(), ArxivArchive::AstroPh);
//! assert_eq!(category.subject(), String::from("HE"));
//!
//! // // Parse an arXiv stamp
//! // let maybestamp = ArxivStamp::try_from("arXiv:0706.0001v1 [q-bio.CB] 1 Jun 2007").unwrap();
//! // assert_eq!(stamp.category(), Some(&ArxivCategoryId::try_new(ArxivArchive::QBio, "CB").unwrap()));
//! // assert_eq!(stamp.submitted().year(), 2007);
//! ```

mod category;
mod identifier;
mod stamp;
mod subject_tables;
pub use crate::category::*;
pub use crate::identifier::*;
pub use crate::stamp::*;

/// Represents the versioned grammar that defines an arXiv identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArxivIdScheme {
	/// Identifier scheme up to [March 2007][arxiv-march-2007]
	///
	/// [arxiv-march-2007]: https://info.arxiv.org/help/arxiv_identifier.html#identifiers-up-to-march-2007-9107-0703
	Old,

	/// Identifier scheme since [1 April 2007][arxiv-april-2007]
	///
	/// [arxiv-april-2007]: https://info.arxiv.org/help/arxiv_identifier.html#identifier-scheme-since-1-april-2007-0704-
	New,
}
