#!/bin/bash

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

BUILD="$1"

### Optimized build
# https://github.com/johnthagen/min-sized-rust#strip-symbols-from-binary

if [ ! -d "upx" ]; then
    curl -L "https://github.com/upx/upx/releases/download/v4.2.4/upx-4.2.4-amd64_linux.tar.xz" -o upx.tar.xz
    tar xf upx.tar.xz
    rm upx.tar.xz
    # shellcheck disable=SC2010
    mv "$(ls | grep "upx-*")" upx
fi

chmod +x ./upx/upx
./upx/upx --best --lzma "$BUILD"
