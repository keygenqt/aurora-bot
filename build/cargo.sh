#!/bin/bash

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

### Optimized build
# https://github.com/johnthagen/min-sized-rust#strip-symbols-from-binary
RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build --release
