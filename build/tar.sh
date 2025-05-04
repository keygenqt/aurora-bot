#!/bin/bash

####################
## Build tar package
####################

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

NAME="$1"
VERSION="$2"
BUILD="$3"

DEB_FOLDER="${NAME}_${VERSION}-1_tar"

# Create folders
mkdir -p "$DEB_FOLDER"
mkdir -p "$DEB_FOLDER"/bin
mkdir -p "$DEB_FOLDER"/systemd

# Systemd
cp build/systemd/aurora-bot.client.service "$DEB_FOLDER"/systemd
cp build/systemd/aurora-bot.dbus.service "$DEB_FOLDER"/systemd

# Bin
chmod +x "$BUILD"
cp "$BUILD" "$DEB_FOLDER"/bin

# Create control
tee -a "$DEB_FOLDER"/Install-Linux-tar.txt > /dev/null <<EOT
Aurora Bot

ИНСТРУКЦИЯ ПО УСТАНОВКЕ
=======================

1. Скопируйте bin/* в директорию /usr/local/bin

2. Скопируйте systemd/* сервисы в директорию /etc/systemd/user

3. Активируйте сервисы
  - systemctl --user enable aurora-bot.client.service
  - systemctl --user enable aurora-bot.client.service

4. Готово!

EOT

tar cfJ "$DEB_FOLDER".tar.xz "$DEB_FOLDER"

rm -rf "$DEB_FOLDER"
