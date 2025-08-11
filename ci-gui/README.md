# `ci-gui`
A minimal graphical interface for [`ci-lisp`]("../ci-lisp/README.md"), using `egui`.

![video demo](./demo.mp4)

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
  -i <PRELOAD>      Name of library to preload
  -m                Treat every line as an infix {...}
      --math        Enable built-in math functions. eg. add, sub, inc, dec, etc
  -h, --help        Print help
  -V, --version     Print version
```

For the full ci-gui experience, try this command:
```sh
cargo run -- --math -i ../lib/ext_math.ci -i ../lib/ext_symbols.ci -m
```

## Overview
The interface of ci-gui is based around cells.
A cell has a text box where you can type your lisp code, and a space below to show its output after evaluation.
Pressing enter inside a cell will evaluate it, and create a new cell.

Keybindings:
- `Enter`: Evaluate the current cell, and create a new one if it makes sense to do so
- `Tab`/`Shift+Tab`: Jump between cells
- `Ctrl+j`: Create a new cell without evaluating the current one
- `Ctrl+d`: Delete the current cell
- `Ctrl+l`: Clear the current cell without deleting it

