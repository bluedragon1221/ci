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
Usage: ci-term [OPTIONS]

Options:
  -i <INCLUDE>                     Name of library to include. Pass multiple times for multiple libraries
  -m, --parser-mode <PARSER_MODE>  Treat line as an infix {} or as parens () [default: normal] [possible values: normal, virtual-infix, virtual-paren]
      --no-math                    Disable built-in math functions. eg. add, sub, inc, dec, etc
  -h, --help                       Print help
  -V, --version                    Print version
```

For the full ci-lisp experience, try this command
```
cargo run -- -i ../lib/prelude.ci
```

