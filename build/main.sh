#!/bin/bash

################
## Build project
################

read -p "What kind of package build? (deb/rpm)? " choice
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

# Dependency
if [ "$choice" = "deb" ]; then
    sudo apt-get update
    sudo apt-get install \
    curl \
    clang \
    libssl-dev \
    libdbus-1-dev \
    libavutil-dev \
    libavcodec-dev \
    libavformat-dev \
    libavfilter-dev \
    libavdevice-dev
fi

if [ "$choice" = "rpm" ]; then
    sudo apt-get update
    sudo apt-get install \
    rpm-build \
    gcc-c++ \
    clang-devel \
    libssl-devel \
    libdbus-devel \
    libavutil-devel \
    libavcodec-devel \
    libavformat-devel \
    libavfilter-devel \
    libavdevice-devel \
    libswscale-devel \
    libswresample-devel
fi

if [ -z "$(cargo --version 2>/dev/null)" ]; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    # @todo not work on Ubuntu
    source ~/.bashrc
fi

if [ -z "$(cargo --version 2>/dev/null)" ]; then
    echo 'После установки rust выполните `source ~/.bashrc` и перезапустите скрипт'
    exit;
fi

# Build
source build/cargo.sh
source build/upx.sh $BUILD
sudo chmod +x $BUILD

# Build tar
source build/tar.sh "$NAME" "$VERSION" "$BUILD"

# Build deb
if [ "$choice" = "deb" ]; then
    source build/deb.sh "$NAME" "$VERSION" "$BUILD"
fi

# Build deb
if [ "$choice" = "rpm" ]; then
    source build/rpm.sh "$NAME" "$VERSION" "$BUILD"
fi
