# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/cloudray-io/cloudray-agent/compare/v0.4.0...v0.4.1) - 2025-11-16

### Other

- Bump actions/checkout from 4 to 5 ([#52](https://github.com/cloudray-io/cloudray-agent/pull/52))
- Bump tokio-tungstenite from 0.27.0 to 0.28.0 ([#53](https://github.com/cloudray-io/cloudray-agent/pull/53))
- Bump amannn/action-semantic-pull-request from 5 to 6 ([#51](https://github.com/cloudray-io/cloudray-agent/pull/51))
- Bump tokio from 1.45.1 to 1.48.0 ([#54](https://github.com/cloudray-io/cloudray-agent/pull/54))

## [0.4.0](https://github.com/cloudray-io/cloudray-agent/compare/v0.3.0...v0.4.0) - 2025-07-03

### Added

- Decrease the report interval from 60 seconds to 5 seconds. ([#40](https://github.com/cloudray-io/cloudray-agent/pull/40))
- Add uninstall.sh script. ([#39](https://github.com/cloudray-io/cloudray-agent/pull/39))

### Fixed

- Let HTTP be used for origin host URL for testing. ([#37](https://github.com/cloudray-io/cloudray-agent/pull/37))

### Other

- Bump prost-build from 0.13.5 to 0.14.1 ([#44](https://github.com/cloudray-io/cloudray-agent/pull/44))
- Bump prost from 0.13.5 to 0.14.1 ([#46](https://github.com/cloudray-io/cloudray-agent/pull/46))
- Bump sysinfo from 0.34.2 to 0.35.2 ([#45](https://github.com/cloudray-io/cloudray-agent/pull/45))
- Bump tokio-tungstenite from 0.26.2 to 0.27.0 ([#43](https://github.com/cloudray-io/cloudray-agent/pull/43))
- Bump tokio from 1.44.2 to 1.45.1 ([#41](https://github.com/cloudray-io/cloudray-agent/pull/41))

## [0.3.0](https://github.com/cloudray-io/cloudray-agent/compare/v0.2.0...v0.3.0) - 2025-05-25

### Added

- [**breaking**] Introduce `install-service` subcommand. ([#34](https://github.com/cloudray-io/cloudray-agent/pull/34))

## [0.2.0](https://github.com/cloudray-io/cloudray-agent/compare/v0.1.0...v0.2.0) - 2025-05-18

### Other

- [**breaking**] Remove Windows builds. ([#35](https://github.com/cloudray-io/cloudray-agent/pull/35))
- Make install.sh work on Mac. ([#32](https://github.com/cloudray-io/cloudray-agent/pull/32))

## [0.1.0](https://github.com/cloudray-io/cloudray-agent/compare/v0.0.6...v0.1.0) - 2025-05-04

### Other

- Cross build ARM on ubuntu-22.04 ([#30](https://github.com/cloudray-io/cloudray-agent/pull/30))

## [0.0.6](https://github.com/cloudray-io/cloudray-agent/compare/v0.0.5...v0.0.6) - 2025-05-04

### Fixed

- Don't let `runlog_run` task block the `report` task. ([#17](https://github.com/cloudray-io/cloudray-agent/pull/17))
- Support compilation on musl target. ([#24](https://github.com/cloudray-io/cloudray-agent/pull/24))

### Other

- Fix OS for ARM tests. ([#28](https://github.com/cloudray-io/cloudray-agent/pull/28))
- Create an installation script. ([#27](https://github.com/cloudray-io/cloudray-agent/pull/27))
- Build on all supported platforms. ([#26](https://github.com/cloudray-io/cloudray-agent/pull/26))

## [0.0.5](https://github.com/cloudray-io/cloudray-agent/compare/v0.0.4...v0.0.5) - 2025-04-23

### Added

- Improve the build name. ([#22](https://github.com/cloudray-io/cloudray-agent/pull/22))

## [0.0.4](https://github.com/cloudray-io/cloudray-agent/compare/v0.0.3...v0.0.4) - 2025-04-21

### Added

- Release binaries for Mac and Windows. ([#20](https://github.com/cloudray-io/cloudray-agent/pull/20))

## [0.0.3](https://github.com/cloudray-io/cloudray-agent/compare/v0.0.2...v0.0.3) - 2025-04-21

### Fixed

- merge building binaries workflow with release-plz. ([#18](https://github.com/cloudray-io/cloudray-agent/pull/18))

## [0.0.2](https://github.com/cloudray-io/cloudray-agent/compare/v0.0.1...v0.0.2) - 2025-04-12

### Added

- Collect CPU and memory metrics. ([#16](https://github.com/cloudray-io/cloudray-agent/pull/16))

### Other

- Releasing this version to test GitHub Actions workflows for releases.

## [0.0.1](https://github.com/cloudray-io/cloudray-agent/releases/tag/v0.0.1) - 2025-04-06

### Other

- Setup build system. ([#7](https://github.com/cloudray-io/cloudray-agent/pull/7))
- Make an example function to communicate with Rails' Action Cable. ([#5](https://github.com/cloudray-io/cloudray-agent/pull/5))
- Create rust.yml
- Created project with: cargo new cloudray-agent --bin --vcs git
