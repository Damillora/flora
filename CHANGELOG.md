# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Set environment variables in a seed (seed toml files only for now)

### Changed
- Make desktop entry category a configurable option
- Show app category when listing apps 
- Make default proton runtime config option not mandatory

## [0.2.0] - 2025-09-27

### Added
- Ability to generate menus for individual app entries
- Ability to use local `umu-run`
- Ability to use custom runtime folder from Flatpak Steam

### Fixed
- Errors while generating menus if required directories does not exist by creating the required directories first

### Changed
- BREAKING CHANGE: Move in `generate-menu` to an `app` subcommand, since it made more sense there
- BREAKING CHANGE: Shorten app-related flags in `flora` CLI
  - `default-application-name` -> `app-name`
  - `application-name` -> `app_name`
  - `default-application-location` -> `app-location`
  - `application-location` -> `app-location`
- Clean up error handling
- Add custom launcher commands for a `seed`

## [0.1.0] - 2025-09-10

### Added
- Manage Wine and Proton setups, configured in a `seed`. Each `seed` can have separate prefixes and runtimes.
- Define application entries to be launched from a `seed`.
- Generate application entries from Start Menu shortcuts.
- Automatically generate application menus for easy access to Windows applications
- Use custom Wine and Proton runtimes for a `seed`.
- Transparent configuration, everything is laid out in `toml` files

[unreleased]: https://github.com/Damillora/flora/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Damillora/flora/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Damillora/flora/releases/tag/v0.1.0
