# Changelog

## Unreleased
This version marks the first stable release of the library. The API has been significantly reworked since the initial release to be more idiomatic. The dependency for handling dates was switched from the `time` crate to the `jiff` crate, and the MSRV has been bumped to 1.70.0 to align with the `jiff` crate.

### Documentation
- Fixed a typo in the summary of the `ArticleIdError::InvalidId` variant

### Bug fixes
- Fix `ArticleId`'s implementation of `Display` to conditionally include the version number within the formatted string (depending on if it's the latest version or a specific version)
- Fix `ArticleId::try_new()` to check if the given number string is all ASCII digits
- Fix `ArticleId::try_new()` to return `ArticleIdError::InvalidYear`/`ArticleIdError::InvalidMonth` for invalid year/months instead of `ArticleIdError::Syntax` for either

### Features
- Introduce `ArticleVersion` enum
- Introduce `ArticleIdError::ExpectedBeginningLiteral`, `ArticleIdError::ExpectedNumberVv` variants
- implement `Copy` for: `ArticleId`, `CategoryId`, `Stamp`
- implement `TryFrom<&'a str>` for: `ArticleId`, `CategoryId`, `Stamp`
- `ArticleId`: make `number` and `version` fields public
- `ArticleId`: add `new()` and `new_latest()` methods (replacing `new_unchecked()` and `new_unchecked_latest()` respectively)
- `Stamp`: make all fields public

### Breaking changes
- MSRV: Bumps the minimum supported Rust version from 1.63.0 to 1.70.0, since jiff 0.1.14 requires 1.70.0
- Migrate date handling from the time crate to the jiff crate
- Rename `ArxivArchive` to `Archive`
- Rename `ArxivId` to `ArticleId`
- Rename `ArticleIdError` to `ArticleIdError`
- Rename `ArxivIdResult` to `ArticleIdResult`
- Rename `ArxivIdScheme` to `ArticleIdScheme`
- Rename `ArxivCategoryId` to `Category`
- Rename `ArxivGroup` to `Group`
- Rename `ArxivStamp` to `Stamp`
- Rename `ArxivStampError` to `StampError`
- Rename `ArxivStampResult` to `StampResult`
- `ArticleIdError`: now marked with `#[non_exhaustive]`
- `ArticleIdError`: removed `ArticleIdError::Syntax` variant
- `StampError`: now marked with `#[non_exhaustive]`
- `StampError`: the `InvalidDate` variant no longer contains any associated data
- Due to lifetimes, remove implementation of `FromStr` for: `ArticleId`, `CategoryId`, `Stamp`
- `ArticleId`: change `number` field type from `String` to `&'a str`
- `ArticleId`: change `year` field type from `u16` to `i16` to sync with jiff
- `ArticleId`: change `month` field type from `u8` to `i8` to sync with jiff
- `ArticleId`: change `MIN_YEAR`, `MAX_YEAR` constants from `u16` to `i16` to sync with jiff
- `ArticleId`: made `MIN_MONTH`, `MAX_MONTH` constants private
- `ArticleId`: change `version` field type from `Option<u8>` to `ArticleVersion`
- `ArticleId`: remove `new_unchecked()` and `new_unchecked_latest()`
- `Stamp`: change `category` field type from `Option<ArxivCategoryId>` to `CategoryId<'a>`
- `Stamp`: remove all getter methods (since fields are now public)
- `Stamp`: `submitted` field now uses `jiff::civil::Date` instead of `time::Date`
- `CategoryId`: change `subject` field type from `String` to `&'a str`

### Internal
- Cleaned up implementation of `Display` for `ArxivStamp`.

## 0.1.0 (2023-03-28)

- Initial release of library
