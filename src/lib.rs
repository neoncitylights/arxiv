#![deny(missing_copy_implementations, missing_debug_implementations)]

//! A Rust library for parsing `arXiv` categories, identifiers and references.
//!
//! ## Identifiers
//! ```rust
//! use arxiv::{ArticleId, ArticleVersion};
//!
//! let id = ArticleId::try_from("arXiv:9912.12345v2").unwrap();
//! assert_eq!(id.month(), 12);
//! assert_eq!(id.year(), 2099);
//! assert_eq!(id.number(), "12345");
//! assert_eq!(id.version(), ArticleVersion::Num(2));
//! ```
//!
//! ## Categories
//! ```rust
//! use arxiv::{Archive, CategoryId, Group};
//!
//! let category = CategoryId::try_from("astro-ph.HE").unwrap();
//! assert_eq!(category.group(), Group::Physics);
//! assert_eq!(category.archive(), Archive::AstroPh);
//! assert_eq!(category.subject(), "HE");
//! ```
//!
//! ## Stamps
//! ```rust
//! use arxiv::{Archive, CategoryId, Stamp};
//!
//! let stamp = Stamp::try_from("arXiv:0706.0001v1 [q-bio.CB] 1 Jun 2007").unwrap();
//! assert_eq!(stamp.category, CategoryId::try_new(Archive::QBio, "CB").unwrap());
//! assert_eq!(stamp.submitted.year(), 2007);
//! ```
//!
//! ## Feature flags
//! The crate has the following feature flags:
//!  - `url` (default): Enables converting types into URLs where possible,
//! such as converting an article identifier into a URL that leads to its abstract page.

mod category;
mod identifier;
mod stamp;
mod subject_tables;
pub use crate::category::*;
pub use crate::identifier::*;
pub use crate::stamp::*;

/// Represents the versioned grammar that defines an arXiv identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticleIdScheme {
	/// Identifier scheme up to [March 2007][arxiv-march-2007]
	///
	/// [arxiv-march-2007]: https://info.arxiv.org/help/arxiv_identifier.html#identifiers-up-to-march-2007-9107-0703
	Old,

	/// Identifier scheme since [1 April 2007][arxiv-april-2007]
	///
	/// [arxiv-april-2007]: https://info.arxiv.org/help/arxiv_identifier.html#identifier-scheme-since-1-april-2007-0704-
	New,
}
