#!/bin/bash

################
## Build project
################

read -p "What kind of package build? (deb/rpm/tar)? " choice
case "$choice" in
  deb ) echo "Let's start build deb...";;
  rpm ) echo "Let's start build rpm...";;
  tar ) echo "Let's start build tar...";;
  * ) echo "Required [deb/rpm/tar]"; exit;;
esac

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

# Params
NAME='aurora-bot'
VERSION=$(grep -m 1 version ./Cargo.toml | xargs | sed 's/version = //g')
BUILD='target/release/aurora-bot'

# Check first build clear PC
if [ -z "$(ldconfig -p | grep libavfilter 2>/dev/null)" ]; then
    # Dependency Ubuntu
    if [ "$choice" = "deb" ] || [ "$choice" = "tar" ]; then
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
    # Dependency ALT Linux
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
fi

# Rust
if [ -z "$(cargo --version 2>/dev/null)" ]; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source ~/.bashrc
fi
if [ -z "$(cargo --version 2>/dev/null)" ]; then
    echo 'После установки rust выполните `source ~/.bashrc` и перезапустите скрипт'
    exit;
fi

# Build
source build/cargo.sh
source build/upx.sh $BUILD
chmod +x $BUILD

# Build deb
if [ "$choice" = "deb" ]; then
    source build/deb.sh "$NAME" "$VERSION" "$BUILD"
fi

# Build rpm
if [ "$choice" = "rpm" ]; then
    source build/rpm.sh "$NAME" "$VERSION" "$BUILD"
fi

# Build tar
if [ "$choice" = "tar" ]; then
    source build/tar.sh "$NAME" "$VERSION" "$BUILD"
fi
