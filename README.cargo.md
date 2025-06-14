# AuroraBot

The application that simplifies interaction with the [Aurora OS](https://auroraos.ru/) infrastructure for software development.

![preview](https://github.com/keygenqt/aurora-bot/blob/main/data/preview.png?raw=true)

### Features

- Interacting with devices.
- Interacting with emulators.
- Working with [Aurora SDK](https://developer.auroraos.ru/doc/sdk).
- Working with [Platform SDK](https://developer.auroraos.ru/doc/sdk/psdk).
- Working with [Flutter SDK](https://developer.auroraos.ru/doc/extended/flutter).
- Answers to questions.

### Install

[Download](https://github.com/keygenqt/aurora-bot/tree/main/build/systemd) services to `/etc/systemd/user`.

```shell
# Install dependency
sudo apt-get install \
    clang \
    libssl-dev \
    libdbus-1-dev \
    libavutil-dev \
    libavcodec-dev \
    libavformat-dev \
    libavfilter-dev \
    libavdevice-dev
# Install app
cargo install aurora-bot
# Enable services
systemctl --user enable aurora-bot.client.service
systemctl --user enable aurora-bot.dbus.service
# Start services
systemctl --user start aurora-bot.client.service
systemctl --user start aurora-bot.dbus.service
```

For use the client service, authorization in the application is required:<br/>
https://aurora-bot.keygenqt.com/book/telegram-bot/auth.html
