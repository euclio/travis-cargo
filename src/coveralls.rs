use docopt::Docopt;

#[cfg_attr(rustfmt, rustfmt_skip)]
const USAGE: &'static str = "
Usage:
    travis-cargo coveralls [options] -- [ARGS...]

Record coverage of `cargo test` and upload to coveralls.io with kcov, this
runs all binaries that `cargo test` runs but not doc tests. Merged kcov
results can be accessed in `target/kcov`.

positional arguments:
  ARGS        arguments to pass to `cargo test`

optional arguments:
  -h, --help  show this help message and exit
  --no-sudo   don't use `sudo` to install kcov's deps. Requires that libcurl4
              -openssl-dev, libelf-dev and libdw-dev are installed (e.g. via
              `addons: apt: packages:`)
  --verify    pass `--verify` to kcov, to avoid some crashes. See
              <https://github.com/huonw/travis-cargo/issues/12>. This requires
              installing the `binutils-dev` package.
";
