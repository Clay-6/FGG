# FGG

A CLI application to access [Infil](https://twitter.com/Infilament)'s [Fighting Game Glossary](https://glossary.infil.net/)

## Usage

Run the command

```shell
fgg [term]
```

where `[term]` is the term you want defined

## Installation

### Manually

Download the [latest release] from GitHub & place the executable anywhere on your PATH.

### Through Cargo

Run the command

```shell
cargo install fgg
```

## Building from source

Install the [latest stable Rust version](https://www.rust-lang.org/tools/install),
then `git clone` the repository & run `cargo build` for a debug build, or `cargo build --release` for a
release build.
