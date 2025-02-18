# Rust EXIF Renamer

A simple cross-platform photo renamer and data editor. It renames JPEG files based on the `DateTimeOriginal` in their EXIF metadata, and viceversa.

It processes in parallel all files in the given folder. If anything fails, it prints an error message and continues with the next file.

It defaults into using the `YYYYMMDD_hh24mmss` pattern. Use the `--pattern` option to change it (but use 24h format).

Written in Rust.

## Install

```sh
cargo install --git https://github.com/pirafrank/rust_exif_renamer
```

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
          Date format pattern (only used with exif-to-filename)

          [default: %Y%m%d_%H%M%S]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Examples

Use EXIF data to rename all JPEG files in some folder:

```sh
exif-renamer exif-to-filename --path somefolder
```

Use filename pattern to set DateTimeOriginal in EXIF data for all JPEG files in some folder:

```sh
exif-renamer filename-to-exif --path somefolder
```

## To Do

0.1.0

- [x] Rename files based on EXIF data

0.2.x

- [x] Reverse mode: set DateTimeOriginal based on filename
- [x] Release pipeline
- [ ] Add colored output
- [ ] Release binaries via pipeline
- [ ] Unit tests

0.3.x

- [ ] TUI/GUI

## Guarantee

This plugin is provided as is, without any guarantee. Use at your own risk.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
