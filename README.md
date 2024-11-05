# arXiv

[![License][license-badge]][license-url]
[![CI][ci-badge]][ci-url]
[![Nightly Docs][docs-nightly-badge]][docs-nightly-url]
[![Docs.rs][docs-badge]][docs-url]

[license-badge]: https://img.shields.io/badge/License-MIT%20%26%20Apache%202.0-blue?style=flat-square
[license-url]: #license
[ci-badge]: https://img.shields.io/github/actions/workflow/status/neoncitylights/arxiv/.github/workflows/main.yml?style=flat-square
[ci-url]: https://github.com/neoncitylights/arxiv/actions/workflows/main.yml
[docs-nightly-badge]: https://img.shields.io/github/deployments/neoncitylights/arxiv/github-pages?style=flat-square&label=nightly%20docs
[docs-nightly-url]: https://neoncitylights.github.io/arxiv/arxiv/
[docs-badge]: https://img.shields.io/docsrs/arxiv?style=flat-square&label=docs.rs
[docs-url]: https://docs.rs/arxiv

A Rust library for parsing `arXiv` categories, identifiers and references.

## Install

```shell
cargo add arxiv
```

## Usage

### Identifiers
```rust
use arxiv::{ArticleVersion, ArxivId};

let id = ArxivId::try_from("arXiv:9912.12345v2").unwrap();
assert_eq!(id.month(), 12);
assert_eq!(id.year(), 2099);
assert_eq!(id.number(), "12345");
assert_eq!(id.version(), ArticleVersion::Num(2));
```

### Categories
```rust
use arxiv::{ArxivArchive, ArxivCategoryId, ArxivGroup};

let category = ArxivCategoryId::try_from("astro-ph.HE").unwrap();
assert_eq!(category.group(), ArxivGroup::Physics);
assert_eq!(category.archive(), ArxivArchive::AstroPh);
assert_eq!(category.subject(), "HE");
```

### Stamps
```rust
use arxiv::{ArxivArchive, ArxivCategoryId, ArxivStamp};

let stamp = ArxivStamp::try_from("arXiv:0706.0001v1 [q-bio.CB] 1 Jun 2007").unwrap();
assert_eq!(stamp.category, ArxivCategoryId::try_new(ArxivArchive::QBio, "CB").unwrap());
assert_eq!(stamp.submitted.year(), 2007);
```

## License

Licensed under either of

* Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([`LICENSE-MIT`](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
