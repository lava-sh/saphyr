# saphyr-parser-bw

Deprecated: this crate has been renamed to
[`granit-parser`](https://crates.io/crates/granit-parser).

`saphyr-parser-bw` was a short-lived fork of `saphyr-parser`. The name was
confusingly close to the upstream crate and was never intended to be a permanent
package name.

Please migrate to:

```toml
[dependencies]
granit-parser = "X.Y.Z"
```

If you want to test compatibility without changing your code, you can temporarily keep the old crate name and redirect it in Cargo.toml:

```toml
[dependencies]
saphyr-parser-bw = { package = "granit-parser", version = "0.0.1" }
```

This is intended as a short-term migration aid.

`granit-parser 0.0.1` is API compatible with the last release of saphyr-parser-bw, 0.0.612
