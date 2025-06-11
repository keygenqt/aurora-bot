# AuroraBot

> Subscribe and like ⭐

Fast, smart, lightweight, client of the Aurora Bot project.

> The application is under development.

![preview](data/preview.png)

### Features

- `cli` - Classic command line.
- `cmd` - Smart command line.
- `faq` - Answers on the Aurora OS ecosystem.
- `svc` - Services: dbus api, telegram client.

[More...](https://aurora-bot.keygenqt.com/book/aurora-bot/about.html)

### Architecture

![preview](data/architecture.png)

### Install DEB

```shell
# Install
sudo dpkg -i aurora-bot_0.1.2-1_amd64-noble.deb
sudo apt-get install -f
# Enable services
systemctl --user enable aurora-bot.client.service
systemctl --user enable aurora-bot.dbus.service
# Start services
systemctl --user start aurora-bot.client.service
systemctl --user start aurora-bot.dbus.service
```

> For use the client service, authorization in the application is required:<br/>
> https://aurora-bot.keygenqt.com/book/telegram-bot/auth.html

### Install TAR

1. Download and unzip the archive
2. Copy `bin/*` to `/usr/bin`
3. Copy `systemd/*` services to `/etc/systemd/user`
4. Enable services
   - `systemctl --user enable aurora-bot.client.service`
   - `systemctl --user enable aurora-bot.dbus.service`
5. Install dependency
  - libavutil58
  - libavcodec60
  - libavformat60
  - libavfilter9
  - libavdevice60

# Build

Building a DEB, RPM package and Tar archive for self-installation.

```shell
git clone https://github.com/keygenqt/aurora-bot.git
cd aurora-bot
chmod +x ./build/*
./build/main.sh
```

### License

```
Copyright 2025 Vitaliy Zarubin

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
