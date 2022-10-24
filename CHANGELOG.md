This project uses a **major**.**minor**.**micro** versioning scheme, where:

* Bumping the major version resets the minor version to zero.
* Bumping the minor version resets the micro version to zero.
* The major version is bumped if the output format or behaviour of the `omst` or `omst-be` binaries
  changes, or there are breaking changes to the `omst` crate as defined by Rust RFC 1122.
* The minor version is bumped on minor changes to the `omst` crate, as defined by Rust RFC 1122.
* The micro version is bumped in all other cases.

# v2.0.0

* [added] `omst-be` binary which fully prints errors
* [changed] `omst` function now returns a `Result`

# v1.0.2

* [changed] updated `atoi` dependency

# v1.0.1

* [fixed] no longer requires `#[feature(inherent_ascii_escape)]` (feature was stabilised)

# v1.0.0

This is the first release.
