#!/bin/bash

################
## Build project
################

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

# Params
NAME='aurora-bot'
VERSION=$(grep -m 1 version ./Cargo.toml | xargs | sed 's/version = //g')
BUILD='target/release/aurora-bot'

# Build
source build/cargo.sh
source build/upx.sh $BUILD

source build/deb.sh "$NAME" "$VERSION" "$BUILD"
source build/tar.sh "$NAME" "$VERSION" "$BUILD"
