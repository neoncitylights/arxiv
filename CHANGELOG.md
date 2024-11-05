# Changelog

## Unreleased

### Fixed
- Fix `ArxivId::try_new()` to check if the given number string is all ASCII digits
- Fix `ArxivId::try_new()` to return `ArxivIdError::InvalidYear`/`ArxivIdError::InvalidMonth` for invalid year/months instead of `ArxivIdError::Syntax` for either

### Added
- Introduce `ArticleVersion` enum
- Introduce `ArxivIdError::ExpectedBeginningLiteral`, `ArxivIdError::ExpectedNumberVv` variants
- implement `Copy` for: `ArxivId`, `ArxivCategoryId`, `ArxivStamp`
- implement `TryFrom<&'a str>` for: `ArxivId`, `ArxivCategoryId`, `ArxivStamp`
- `ArxivId`: make `number` and `version` fields public
- `ArxivId`: add `new()` and `new_latest()` methods (replacing `new_unchecked()` and `new_unchecked_latest()` respectively)
- `ArxivStamp`: make all fields public

### Removed

### Breaking changes
- `ArxivIdError`: now marked with `#[non_exhaustive]`
- `ArxivIdError`: removed `ArxividError::Syntax` variant
- Due to lifetimes, remove implementation of `FromStr` for: `ArxivId`, `ArxivCategoryId`, `ArxivStamp`
- `ArxivId`: change `number` field type from `String` to `&'a str`
- `ArxivId`: change `version` field type from `Option<u8>` to `ArticleVersion`
- `ArxivId`: remove `new_unchecked()` and `new_unchecked_latest()`
- `ArxivStamp`: change `category` field type from `Option<ArxivCategoryId>` to `ArxivCategoryId<'a>`
- `ArxivStamp`: remove all getter methods (since fields are now public)
- `ArxivCategoryId`: change `subject` field type from `String` to `&'a str`

## 0.1.0 (2023-03-28)

- Initial release of library
