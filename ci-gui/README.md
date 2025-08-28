# `ci-gui`
A minimal graphical interface for [`ci-lisp`]("../ci-lisp/README.md"), using `egui`.

https://github.com/user-attachments/assets/db341f60-7107-45ce-b5a7-9bfbda064342

## Getting Started
Make sure you have git, cargo, and a few runtime dependencies which are defined in [`flake.nix`]("../flake.nix").
```sh
git clone https://github.com/bluedragon1221/ci
cd ci/ci-gui
```

Now you can run it:
```
$ cargo run -- --help
Usage: ci-gui [OPTIONS]

Options:
  -i <INCLUDE>                     Name of library to include. Pass multiple times for multiple libraries
  -m, --parser-mode <PARSER_MODE>  Treat line as an infix {} or as parens () [default: normal] [possible values: normal, virtual-infix, virtual-paren]
      --no-math                    Disable built-in math functions. eg. add, sub, inc, dec, etc
  -h, --help                       Print help
  -V, --version                    Print version
```

For the full ci-gui experience, try this command:
```sh
cargo run -- -i ../lib/prelude.ci
```

## Overview
The interface of ci-gui is based around cells.
A cell has a text box where you can type your lisp code, and a space below to show its output after evaluation.
Pressing enter inside a cell will evaluate it, and create a new cell.

Keybindings:
- `Enter`: Evaluate the current cell, and create a new one if it makes sense to do so
- `Up`/`Down`: Jump between cells
- `Ctrl+d`: Delete the current cell
- `Ctrl+l`: Clear the current cell without deleting it
