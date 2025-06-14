# AuroraBot

Fast, smart, easy, fast way to interact with [Aurora OS](https://auroraos.ru/).

![preview](https://github.com/keygenqt/aurora-bot/blob/main/data/preview.png?raw=true)

### Features

- `cli` - Classic command line.
- `cmd` - Smart command line.
- `faq` - Answers on the Aurora OS ecosystem.
- `svc` - Services: dbus api, telegram client.

### Install

> [Download](https://github.com/keygenqt/aurora-bot/tree/main/build/systemd) services to `/etc/systemd/user`.

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

> For use the client service, authorization in the application is required:<br/>
> https://aurora-bot.keygenqt.com/book/telegram-bot/auth.html
