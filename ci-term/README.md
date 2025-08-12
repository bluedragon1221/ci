# `ci-term`
A standard repl for interacting with ci-lisp.

## Getting Started
Make sure you have `git` and `cargo` installed, then clone the `ci` monorepo:
```
git clone https://github.com/bluedragon1221/ci
cd ci/ci-lisp
```

Now you can run it:
```
$ cargo run -- --help
Usage: ci-lisp [OPTIONS]

Options:
  -i <PRELOAD>      Name of library to preload
  -m                Treat every line as an infix {...}
      --math        Enable built-in math functions. eg. add, sub, inc, dec, etc
  -h, --help        Print help
  -V, --version     Print version
```

For the full ci-lisp experience, try this command
```
cargo run -- --math -i ../lib/ext_math.ci ../lib/ext_symbols.ci -m
```

