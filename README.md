```
$ git clone https://github.com/illicitonion/cargo-fix-2018-repro.git
$ cd cargo-fix-2018-repro
$ rustc +beta --version
rustc 1.31.0-beta.17 (1a4f1f398 2018-11-25)
$ cargo +beta --version
cargo 1.31.0-beta (339d9f9c8 2018-11-16)
$ cargo +beta build --manifest-path=dependee/Cargo.toml
   Compiling dependency v0.0.1 (/home/dwh/src/github.com/illictonion/cargo-2018-multiple-defines/dependency)
   Compiling dependee v0.0.1 (/home/dwh/src/github.com/illictonion/cargo-2018-multiple-defines/dependee)
error[E0259]: the name `ignore` is defined multiple times
  |
  = note: `ignore` must be defined only once in the type namespace of this module

error: aborting due to previous error

For more information about this error, try `rustc --explain E0259`.
error: Could not compile `dependee`.

To learn more, run the command again with --verbose.
```
