# Basalt

LavaMoat analyzer, linter and bundler.

To get started [install Rust][rust-install] then add the `nightly` toolchain:

```
rustup toolchain install nightly
```

Now you can compile and test using `cargo`.

## Test

To run the tests:

```
cargo +nightly test -- --nocapture
```

## List

To print the module graph for a file:

```
cargo +nightly run -- ls tests/fixtures/basic-tree/main.js
```

## Static Module Record

To print the static module record meta data for a file:

```
cargo +nightly run -- meta tests/fixtures/static-module-record/main.js
```

To print the static module record functor for a file:

```
cargo +nightly run -- transform tests/fixtures/static-module-record/main.js
```

### Compartment Mapper

To test the static module record transform in the context of the [compartment-mapper][] create a release build and copy `target/release/basalt` into `PATH`.

Then copy [parse-archive-mjs.js](/parse-archive-mjs.js) to overwrite [parse-archive-mjs.js](https://github.com/endojs/endo/blob/master/packages/compartment-mapper/src/parse-archive-mjs.js) and run `yarn test` in the [compartment-mapper][] directory.

## API Documentation

```
cargo +nightly doc --open --no-deps
```

[rust-install]: https://www.rust-lang.org/tools/install

## Release Build

```
cargo +nightly build --release
```

Then copy the executable into `PATH`, for example:

```
cp -r target/release/basalt ~/bin
```

[compartment-mapper]: https://github.com/endojs/endo/tree/master/packages/compartment-mapper
