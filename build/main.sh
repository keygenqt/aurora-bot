#!/bin/bash

################
## Build project
################

read -rp "What kind of package build? (deb/tar)? " choice
case "$choice" in
  deb ) echo "Let's start assembling...";;
  tar ) echo "Let's start assembling...";;
  * ) echo "Required [deb/tar]"; exit;;
esac

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

# Params
NAME='aurora-bot'
VERSION=$(grep -m 1 version ./Cargo.toml | xargs | sed 's/version = //g')
BUILD='target/release/aurora-bot'

# Build
source build/cargo.sh
source build/upx.sh $BUILD

# Build deb
if [ "$choice" = "deb" ]; then
  source build/deb.sh "$NAME" "$VERSION" "$BUILD"
fi

# Build tar
if [ "$choice" = "tar" ]; then
  source build/tar.sh "$NAME" "$VERSION" "$BUILD"
fi

