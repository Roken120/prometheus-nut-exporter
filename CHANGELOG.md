# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added UPS description to `nut_ups_info`.
- Added lots of more metrics.

### Changed

- Replace Docker image tags `stable` and `bleeding` with `latest`, `X`, `X.Y` and `X.Y.Z` (parts of the semantic version).
- Renamed a few voltage-related metrics (slightly breaking for existing setups, no easy way to add legacy aliases).

### Deprecated

### Removed

### Fixed

- Added Tini as container entrypoint to handler signals properly (i.e. not stall when exiting).
- Fixed parsing error when multiple UPSes exist.

### Security

## [1.0.1] - 2020-06-29

### Added

- Added metadata metrics `nut_info` for the NUT server and `nut_exporter_info` for the exporter.

### Changed

- Improved error messages sent to client.

### Fixed

- Fixed malformed labels for `nut_ups_info`.

## [1.0.0] - 2020-06-18

Initial release.
