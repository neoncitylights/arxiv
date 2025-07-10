#![cfg_attr(docsrs, feature(doc_cfg))]
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
//!    such as converting an article identifier into a URL that leads to its abstract page.

mod archive;
mod article_id;
mod article_version;
mod category_id;
mod group;
mod stamp;

pub use crate::archive::*;
pub use crate::article_id::*;
pub use crate::article_version::*;
pub use crate::category_id::*;
pub use crate::group::*;
pub use crate::stamp::*;
