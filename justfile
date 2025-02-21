#!/usr/bin/env -S just

# Set default shell based on OS
set windows-powershell

# Task to uild the project
build:
    cargo build

# Task to build the project in release mode
release:
    cargo build --release

# Task to pack the project for GitHub release
pack:
    ./pack.sh

# Task to run tests
test:
    cargo test --verbose

help:
    cargo run -- --help

# Rename files based on EXIF data
exif:
    cargo run -- exif-to-filename --path test_assets/exif-to-name

# Update EXIF data based on filename
name:
    cargo run -- filename-to-exif --path test_assets/name-to-exif

# Rename files based on EXIF data
exifpattern:
    cargo run -- exif-to-filename --path test_assets/exif-to-name --pattern "%Y-%m-%d_%H-%M-%S"

# Update EXIF data based on filename
namepattern:
    cargo run -- filename-to-exif --path test_assets/name-to-exif --pattern "%Y-%m-%d_%H-%M-%S"
