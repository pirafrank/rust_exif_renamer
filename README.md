# Rust EXIF Renamer

[![GitHub Release](https://img.shields.io/github/v/release/pirafrank/rust_exif_renamer)](https://github.com/pirafrank/rust_exif_renamer/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/exif_renamer)](https://crates.io/crates/exif_renamer)
[![Crates.io MSRV](https://img.shields.io/crates/msrv/exif_renamer)](https://github.com/pirafrank/rust_exif_renamer/blob/main/Cargo.toml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

[![CI-CD](https://github.com/pirafrank/rust_exif_renamer/actions/workflows/ci.yml/badge.svg)](https://github.com/pirafrank/rust_exif_renamer/actions/workflows/ci.yml)
[![Release](https://github.com/pirafrank/rust_exif_renamer/actions/workflows/release.yml/badge.svg)](https://github.com/pirafrank/rust_exif_renamer/actions/workflows/release.yml)

A simple cross-platform photo renamer and data editor. It renames JPEG files based on the `DateTimeOriginal` in their EXIF metadata, and viceversa.

It processes in parallel all files in the given folder. If anything fails, it prints an error message and continues with the next file.

It defaults into using the `YYYYMMDD_hh24mmss` pattern. Use the `--pattern` option to change it (but use 24h format).

Written in Rust.

## Install

### Binary

Download the binary from [latest release](https://github.com/pirafrank/rust_exif_renamer/releases/latest) and move to `PATH`.

### cargo

```sh
cargo install exif_renamer
```

### binstall

If you have [binstall](https://github.com/cargo-bins/cargo-binstall), you can get the binary and skip compilation.

```sh
cargo binstall exif_renamer
```

### From source

Build and install to `$HOME/.cargo/bin` compiling from source:

```sh
cargo install --locked --git https://github.com/pirafrank/rust_exif_renamer --tag VERSION
```

Note: Replace `VERSION` with the desired version to install. Not specifying a tag will install from `main` branch. `main` branch should be stable, but it's unreleased software and may contain bugs or breaking changes. It should considered beta quality software.

## Update

Download the new binary version and overwrite old one.

If you have installed via `cargo`, then re-run the `cargo install` command specifying the new tag.

## Usage

```txt
Usage: exif_renamer.exe [OPTIONS] --path <PATH> <COMMAND>

Arguments:
  <COMMAND>
          Command to execute

          Possible values:
          - exif-to-filename: Rename files based on EXIF data
          - filename-to-exif: Update EXIF data based on filename

Options:
  -p, --path <PATH>
          Directory containing the images to process

  -P, --pattern <PATTERN>
          Date format pattern to parse from or set to filename

          [default: %Y%m%d_%H%M%S]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Examples

Use EXIF data to rename all JPEG files in some folder:

```sh
exif_renamer exif-to-filename --path somefolder
```

Use filename pattern to set DateTimeOriginal in EXIF data for all JPEG files in some folder:

```sh
exif_renamer filename-to-exif --path somefolder
```

## To Do

0.1.0

- [x] Rename files based on EXIF data

0.2.x

- [x] Cross-platform support
- [x] Reverse mode: set DateTimeOriginal based on filename
- [x] Add `--pattern` option
- [x] Improve help and version options
- [ ] Add colored output
- [x] CI pipeline
- [x] Release binaries via pipeline
- [ ] Unit tests

0.3.x

- [ ] TUI/GUI

## Guarantee

This plugin is provided as is, without any guarantee. Use at your own risk.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
