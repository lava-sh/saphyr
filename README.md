[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![CI](https://github.com/bourumir-wyngs/saphyr/actions/workflows/ci.yml/badge.svg)](https://github.com/bourumir-wyngs/saphyr/actions/workflows/ci.yml)
[![Miri](https://github.com/bourumir-wyngs/saphyr/actions/workflows/miri.yml/badge.svg)](https://github.com/bourumir-wyngs/saphyr/actions/workflows/miri.yml)
[![Dependency Vulnerabilities](https://img.shields.io/endpoint?url=https%3A%2F%2Fapi-hooks.soos.io%2Fapi%2Fshieldsio-badges%3FbadgeType%3DDependencyVulnerabilities%26pid%3Dlw1ak1eza%26)](https://app.soos.io)

# This crate contains the fork of saphyr-parser intended to work with serde-saphyr
The crate is otherwise identical to the upstream crate [saphyr](https://crates.io/crates/saphyr). If you want
to use it directly, please refer to the documentation there. 

The main branch in this project is dev/saphyr-parser, not main. Main is kept untouched to make easier
to pull changes from and raise patches against an original saphyr project.

The parser crate contains many changes. Please refer to [parser/README.md](parser/README.md) for details.

### Other links
* [yaml-test-suite](https://github.com/yaml/yaml-test-suite)
* [YAML 1.2 specification](https://yaml.org/spec/1.2.2/)
