# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]
### Changed
#### Mat2 optimizations
* `Mat2` now uses a `Vec4` internally which gives it some performance
   improvements when SSE2 is available.

## [0.6.0] - 2019-06-13
### Changed
#### Switched from row vectors to column vectors
* Vectors are now on the right of multiplications with matrices and quaternions.


[Keep a Changelog]: https://keepachangelog.com/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
