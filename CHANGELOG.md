# Changelog

## Unreleased

### Documentation
- Fixed a typo in the summary of the `ArxivIdError::InvalidId` variant

### Bug fixes
- Fix `ArxivId`'s implementation of `Display` to conditionally include the version number within the formatted string (depending on if it's the latest version or a specific version)
- Fix `ArxivId::try_new()` to check if the given number string is all ASCII digits
- Fix `ArxivId::try_new()` to return `ArxivIdError::InvalidYear`/`ArxivIdError::InvalidMonth` for invalid year/months instead of `ArxivIdError::Syntax` for either

### Features
- Introduce `ArticleVersion` enum
- Introduce `ArxivIdError::ExpectedBeginningLiteral`, `ArxivIdError::ExpectedNumberVv` variants
- implement `Copy` for: `ArxivId`, `ArxivCategoryId`, `ArxivStamp`
- implement `TryFrom<&'a str>` for: `ArxivId`, `ArxivCategoryId`, `ArxivStamp`
- `ArxivId`: make `number` and `version` fields public
- `ArxivId`: add `new()` and `new_latest()` methods (replacing `new_unchecked()` and `new_unchecked_latest()` respectively)
- `ArxivStamp`: make all fields public

### Breaking changes
- MSRV: Bumps the minimum supported Rust version from 1.63.0 to 1.70.0, since jiff 0.1.14 requires 1.70.0
- Migrate date handling from the time crate to the jiff crate
- `ArxivIdError`: now marked with `#[non_exhaustive]`
- `ArxivIdError`: removed `ArxividError::Syntax` variant
- `ArxivStampError`: now marked with `#[non_exhaustive]`
- `ArxivStampError`: the `InvalidDate` variant no longer contains any associated data
- Due to lifetimes, remove implementation of `FromStr` for: `ArxivId`, `ArxivCategoryId`, `ArxivStamp`
- `ArxivId`: change `number` field type from `String` to `&'a str`
- `ArxivId`: change `year` field type from `u16` to `i16` to sync with jiff
- `ArxivId`: change `month` field type from `u8` to `i8` to sync with jiff
- `ArxivId`: change `MIN_YEAR`, `MAX_YEAR` constants from `u16` to `i16` to sync with jiff
- `ArxivId`: made `MIN_MONTH`, `MAX_MONTH` constants private
- `ArxivId`: change `version` field type from `Option<u8>` to `ArticleVersion`
- `ArxivId`: remove `new_unchecked()` and `new_unchecked_latest()`
- `ArxivStamp`: change `category` field type from `Option<ArxivCategoryId>` to `ArxivCategoryId<'a>`
- `ArxivStamp`: remove all getter methods (since fields are now public)
- `ArxivStamp`: `submitted` field now uses `jiff::civil::Date` instead of `time::Date`
- `ArxivCategoryId`: change `subject` field type from `String` to `&'a str`

### Internal
- Cleaned up implementation of `Display` for `ArxivStamp`.

## 0.1.0 (2023-03-28)

- Initial release of library
