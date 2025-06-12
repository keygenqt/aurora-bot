#!/bin/bash

####################
## Build tar archive
####################

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

NAME="$1"
VERSION="$2"
BUILD="$3"

TAR_FOLDER="${NAME}_${VERSION}-1_tar"

# Create folders
mkdir -p "$TAR_FOLDER"
mkdir -p "$TAR_FOLDER"/bin
mkdir -p "$TAR_FOLDER"/systemd

# Systemd
cp build/systemd/aurora-bot.client.service "$TAR_FOLDER"/systemd
cp build/systemd/aurora-bot.dbus.service "$TAR_FOLDER"/systemd

# Bin
chmod +x "$BUILD"
cp "$BUILD" "$TAR_FOLDER"/bin

# Create control
tee -a "$TAR_FOLDER"/Install-Linux-tar.txt > /dev/null <<EOT
Aurora Bot

ИНСТРУКЦИЯ ПО УСТАНОВКЕ
=======================

1. Скопируйте bin/* в директорию /usr/bin

2. Скопируйте systemd/* сервисы в директорию /etc/systemd/user

3. Активируйте сервисы
  - systemctl --user enable aurora-bot.client.service
  - systemctl --user start aurora-bot.client.service
  - systemctl --user enable aurora-bot.dbus.service
  - systemctl --user start aurora-bot.dbus.service

4. Установите зависимости:
  - libavutil58
  - libavcodec60
  - libavformat60
  - libavfilter9
  - libavdevice60

5. Готово!

EOT

tar cfJ "$TAR_FOLDER".tar.xz "$TAR_FOLDER"

sleep 1s
rm -rf "$PWD/$TAR_FOLDER"
