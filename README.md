# AuroraBot

> Subscribe and like ⭐

[![Releases DEB](https://img.shields.io/badge/dynamic/json?url=https://api.github.com/repos/keygenqt/aurora-bot/releases/latest&query=assets[0][download_count]&label=Releases%20DEB&color=BE8731&logo=github&prefix=0.0.6%20(&suffix=))](https://github.com/keygenqt/aurora-bot/releases)
[![Releases TAR](https://img.shields.io/badge/dynamic/json?url=https://api.github.com/repos/keygenqt/aurora-bot/releases/latest&query=assets[1][download_count]&label=Releases%20TAR&color=040404&logo=github&prefix=0.0.6%20(&suffix=))](https://github.com/keygenqt/aurora-bot/releases)

> The application is under development.

Fast, smart, lightweight, client of the Aurora Bot project.

![preview](data/preview.png)

### Features

- `cli` - Classic command line.
- `cmd` - Smart command line.
- `faq` - Answers on the Aurora OS ecosystem.
- `svc` - Services: dbus api, telegram client.

### Todo

- Target install package (psdk_target_package_install).
- Target remove package (psdk_target_package_uninstall).
- Sign package (psdk_package_sign).
- Add sudoers (psdk_sudoers_add).
- Remove sudoers (psdk_sudoers_remove).
- Format Qt/C++ (sdk_project_format).
- Format Dart/C++ (flutter_project_format).
- Get report plugins (flutter_project_report).
- Remove sdk (sdk_remove).
- Remove psdk (psdk_remove).
- Remove flutter (flutter_remove).
- Install sdk (sdk_install).
- Install psdk (psdk_install).
- Install flutter (flutter_install).

### Architecture

![preview](data/preview2.png)

### Install DEB

```shell
# Install
sudo dpkg -i aurora-bot_0.0.6-1_amd64.deb
# Enable services
systemctl --user enable aurora-bot.client.service
systemctl --user enable aurora-bot.dbus.service
```

### Install TAR

1. Download and unzip the archive
2. Copy `bin/*` to `/usr/local/bin`
3. Copy `systemd/*` services to `/etc/systemd/user`
4. Enable services
   - `systemctl --user enable aurora-bot.client.service`
   - `systemctl --user enable aurora-bot.client.service`

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
