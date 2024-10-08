# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2024-10-01

### 🚀 Features

- Initial commit
- Added anilist API support
- Added readable byte size

### 🐛 Bug Fixes

- Graphql query
- Removed leading backslash from anime name
- Added path filters for movies, music
- Missing title on request
   - Fixed issue where English title is missing on Anilist, will now fallback to romaji and then to native language

### 🚜 Refactor

- Changed responsibilities for torrent type parsing

### 📚 Documentation

- Updated readme, added logger file
- Updated documentation

### ⚙️ Miscellaneous Tasks

- Added github actions
   - Github actions will check for build errors, lints and formatting, and create and publish a release build.
- Updated dependencies

<!-- generated by git-cliff -->
