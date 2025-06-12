#!/bin/bash

####################
## Build rpm package
####################

PACKAGE='com.keygenqt.aurora-bot'
ARCH="x86_64"
NAME="$1"
VERSION="$2"
BUILD="$PWD/$3"

SYSTEMD="$PWD/build/systemd/"

# Set root dir
cd "$(dirname "$(realpath "$0")")"/../ || exit

NAME_RPM="$NAME-$VERSION-1.$ARCH.rpm"
RPM_FOLDER=$PWD/rpmbuild

# Clear
rm -rf $RPM_FOLDER
mkdir -p $RPM_FOLDER

# Alt 10
if [[ $(lsb_release -a | grep Release) == *"10."* ]]; then
REQUIRES=$(cat <<EOF
Requires: libavutil56
Requires: libavcodec58
Requires: libavformat58
Requires: libavfilter7
Requires: libavdevice58
EOF
)
fi

# Alt 11
if [[ $(lsb_release -a | grep Release) == *"11."* ]]; then
REQUIRES=$(cat <<EOF
Requires: libavutil59
Requires: libavcodec61
Requires: libavformat61
Requires: libavfilter10
Requires: libavdevice61
EOF
)
fi

# Create control
tee -a $RPM_FOLDER/$NAME.spec > /dev/null <<EOT
%define fullname $PACKAGE
%define _build_id_links none
%set_verify_elf_method skip

Name: $NAME
Version: $VERSION
Release: 1
Summary: Simple, easy, fast way to interact with Aurora OS.
License: Apache-2.0
Group: Development/Tools
Url: https://aurora-bot.keygenqt.com/

$REQUIRES

%description
%summary.

%build
cp $BUILD $NAME
cp $SYSTEMD/* ./

%install
mkdir -p %{buildroot}/usr/bin
install -m 755 $NAME %{buildroot}/usr/bin/$NAME
mkdir -p %{buildroot}/etc/systemd/user
install -m 755 aurora-bot.dbus.service %{buildroot}/etc/systemd/user/aurora-bot.dbus.service
install -m 755 aurora-bot.client.service %{buildroot}/etc/systemd/user/aurora-bot.client.service

%post
chmod +x %{_bindir}/%{name}
ln -sf %{_bindir}/%{name} %{_bindir}/%{fullname}

%files
/usr/bin/$NAME
/etc/systemd/user/aurora-bot.dbus.service
/etc/systemd/user/aurora-bot.client.service
EOT

# Build
rpmbuild -bb $RPM_FOLDER/$NAME.spec || exit

# Move
cp ~/RPM/RPMS/$ARCH/*.rpm ./
rm -rf ~/rpmbuild
rm -rf $RPM_FOLDER
