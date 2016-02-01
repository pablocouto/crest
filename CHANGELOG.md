# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [0.3.1] - 2016-02-01
### Changed
- Refactor to facilitate overriding request methods

## [0.3.0] - 2016-01-31
### Added
- Support for deserialization from JSON responses
- Support for setting the parameters of a request
- `prelude` module
### Changed
- `Result` is passed up at `Request` creation
- Improved documentation
- `path` and `params` arguments accept more types
- `Endpoint::post` method does not require a body anymore
### Fixed
- Added missing `Debug` impls

## [0.2.0] - 2016-01-25
### Added
- Traits to specify functionality according to request type
### Changed
- Requests are represented by specific types instead of a common one
### Fixed
- Make `error` module public

## [0.1.1] - 2016-01-24
### Added
- Documentation for `Endpoint` and `Request`, and a simple usage example
- This change log

## [0.1.0] - 2016-01-23
### Added
- Initial code
- Licensing terms
