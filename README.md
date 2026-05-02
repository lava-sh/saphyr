# This crate contained the fork of saphyr-parser intended to work with serde-saphyr

`saphyr-parser-bw` (indernal crate under `parser` folder) was a short-lived fork of `saphyr-parser`. The name was
confusingly close to the upstream crate and was never intended to be a permanent package name.

The parser is actively developed and maintained under the new name [granit-parser](https://github.com/bourumir-wyngs/granit-parser) name

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

`granit-parser 0.0.1` is API compatible with the last release of saphyr-parser-bw, 0.0.612.

If you would encounter any issues, please report them on the [granit-parser issue tracker](https://github.com/bourumir-wyngs/granit-parser/issues). If they still apply to ```granit-parser``` as well, we will work on them.

As [saphyr](https://github.com/saphyr-rs/saphyr) is a monoprepo, the whole code of ```saphyr``` was forked. No development was ever done on parts outside the parser.