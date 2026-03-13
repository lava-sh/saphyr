# This crate contains the fork of saphyr-parser intended to work with serde-saphyr
The crate is otherwise identical to the upstream crate [saphyr](https://crates.io/crates/saphyr) if you want
to use it directly, please refer to the documentation there. 

### Changes made
* Added explicit handling for JSON-style Unicode surrogate pairs in quoted scalar escape sequences.
* `\uXXXX` escapes that encode a high surrogate are now required to be followed immediately by a valid low surrogate escape, and both escapes are combined into the corresponding Unicode scalar value.
* Unpaired high surrogates, unpaired low surrogates, and reversed surrogate pairs are now rejected during scanning instead of being treated as generic invalid Unicode escape codes.

### Other links
* [yaml-test-suite](https://github.com/yaml/yaml-test-suite)
* [YAML 1.2 specification](https://yaml.org/spec/1.2.2/)
