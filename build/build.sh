#!/bin/bash

################
## Build project
################

read -rp "What kind of package build? (deb/rpm)? " choice
case "$choice" in
  deb ) echo "Let's start assembling...";;
  rpm ) echo "Let's start assembling...";;
  * ) echo "Required [deb/rpm]"; exit;;
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

# Build rpm
if [ "$choice" = "rpm" ]; then
  source build/rpm.sh "$NAME" "$VERSION" "$BUILD"
fi

