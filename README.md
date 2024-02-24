# Rust EXIF Renamer

A dummy file renamer renaming JPEG files based on `DateTimeOriginal` in their EXIF metadata. Written in Rust.

## How it works

It expects a single argument, the directory containing the JPEG files to rename.

Then it looks for the `DateTimeOriginal` field in the EXIF data, which typically stores the date and time the photo was taken.

If it finds that field, it parses the date and time of it, and renames the file accordingly using the `YYYYMMDD_hh24mmss` pattern.

If anything fails, it prints an error message and continues with the next file.

## Build

```sh
cargo build --release
```

## Install

```sh
cargo install --path .
```

## Usage

```sh
exif-renamer somefolder
```
