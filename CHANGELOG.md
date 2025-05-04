# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.6](https://github.com/cloudray-io/cloudray-agent/compare/v0.0.5...v0.0.6) - 2025-05-04

### Fixed

- Don't let `runlog_run` task block the `report` task. ([#17](https://github.com/cloudray-io/cloudray-agent/pull/17))
- Support compilation on musl target. ([#24](https://github.com/cloudray-io/cloudray-agent/pull/24))

### Other

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
