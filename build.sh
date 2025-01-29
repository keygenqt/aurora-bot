#!/bin/bash

### Optimized build
# https://github.com/johnthagen/min-sized-rust#strip-symbols-from-binary

if [ ! -d "upx" ]; then
    curl -L "https://github.com/upx/upx/releases/download/v4.2.4/upx-4.2.4-amd64_linux.tar.xz" -o upx.tar.xz
    tar xf upx.tar.xz
    rm upx.tar.xz
    name=$(ls | grep upx-*)
    mv $name upx
fi

RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build --release

./upx/upx --best --lzma ./target/release/aurora-bot
