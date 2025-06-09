#!/bin/bash

####################
## Build deb package
####################

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

NAME="$1"
VERSION="$2"
BUILD="$3"

DEB_FOLDER="${NAME}_${VERSION}-1_amd64"

# Create folders
mkdir -p "$DEB_FOLDER"
mkdir -p "$DEB_FOLDER"/DEBIAN
mkdir -p "$DEB_FOLDER"/usr/bin
mkdir -p "$DEB_FOLDER"/etc/systemd/user

# Systemd
cp build/systemd/aurora-bot.client.service "$DEB_FOLDER"/etc/systemd/user
cp build/systemd/aurora-bot.dbus.service "$DEB_FOLDER"/etc/systemd/user

# Bin
chmod +x "$BUILD"
cp "$BUILD" "$DEB_FOLDER"/usr/bin

# Create control
tee -a "$DEB_FOLDER"/DEBIAN/control > /dev/null <<EOT
Package: aurora-bot
Version: $VERSION
Architecture: amd64
Maintainer: Vitaliy Zarubin <keygenqt@gmail.com>
Description: An application that provides an easy start in the Aurora OS ecosystem.
EOT

if [[ $(lsb_release -a | grep Release) == *"24.04"* ]]; then
  echo 'Depends: libavutil58, libavcodec60, libavformat60, libavfilter9, libavdevice60' >> "$DEB_FOLDER/DEBIAN/control"
fi

if [[ $(lsb_release -a | grep Release) == *"22.04"* ]]; then
  echo 'Depends: libavutil56, libavcodec58, libavformat58, libavfilter7, libavdevice58' >> "$DEB_FOLDER/DEBIAN/control"
fi

dpkg-deb --build --root-owner-group "$DEB_FOLDER"

rm -rf "$DEB_FOLDER"

# DEB Install / Remove
# sudo dpkg -r aurora-bot
# sudo dpkg -i "$DEB_FOLDER".deb
# systemctl --user enable aurora-bot.client.service
# systemctl --user enable aurora-bot.dbus.service
# systemctl --user restart aurora-bot.client.service
# systemctl --user restart aurora-bot.dbus.service
