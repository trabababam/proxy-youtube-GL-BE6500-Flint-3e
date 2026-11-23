## 🇬🇧 EN

# YouTube Relay for GL.iNet GL-BE6500

Rust-based transparent TCP relay for routing selected network traffic through an external HTTP CONNECT proxy.

The project is designed for the GL.iNet GL-BE6500 router running OpenWrt-based firmware.

It combines a Rust relay, dnsmasq, nftables, procd and a custom LuCI configuration interface.

## Features

- Rust TCP relay with HTTP CONNECT proxy support.
- HTTP proxy authentication (Basic).
- Transparent destination detection using `SO_ORIGINAL_DST`.
- TLS ClientHello inspection and SNI-based destination selection.
- DNS-based IPv4 address collection through dnsmasq nftset.
- nftables-based redirection of selected LAN traffic.
- QUIC blocking for selected destinations to encourage TCP fallback.
- Automatic service restart through procd.
- LuCI web interface for configuring the upstream proxy.

## How it works

1. A LAN client requests the IP address of a domain.
2. dnsmasq resolves the domain and adds matching IPv4 addresses to the nftables set `youtube_proxy4`.
3. nftables redirects matching TCP connections from LAN clients to the local relay on port `12348`.
4. The Rust relay identifies the original destination using `SO_ORIGINAL_DST`.
5. For HTTPS connections, the relay attempts to extract the hostname from TLS SNI.
6. The relay establishes an HTTP CONNECT tunnel through the configured upstream proxy.
7. The relay forwards TCP data between the LAN client and the upstream proxy.

HTTPS traffic is forwarded through a tunnel. The relay does not decrypt TLS traffic and does not act as a man-in-the-middle proxy.

## Tested environment

| Component | Version or value |
|---|---|
| Router | GL.iNet GL-BE6500 |
| Firmware | GL.iNet 4.9.0 |
| OpenWrt base | 23.05-SNAPSHOT |
| CPU architecture | AArch64 |
| Rust edition | 2021 |
| Rust target | `aarch64-unknown-linux-musl` |
| Cross-compiler | Buildroot GCC 12.3.0 |
| Linux kernel | 5.4.213 |
| C library | musl 1.2.4 |

Other router models and firmware versions have not been verified.

## Limitations

- The relay handles TCP traffic, not general-purpose UDP traffic.
- It is not a VPN.
- Native Telegram MTProto compatibility is not guaranteed.
- The current SNI masking function is not called by the connection-processing path.
- DNS and nftables rules in the current configuration operate with IPv4 addresses.
- QUIC traffic is not proxied. UDP port 443 is blocked for selected destinations to encourage TCP fallback.
- HTTPS traffic is tunneled without TLS decryption.

## Security

Never publish real proxy credentials or runtime configuration files containing passwords.

Use placeholder values in documentation and examples:

- Proxy host: `proxy.example.com`
- Proxy port: `8080`
- Username: `demo_user`
- Password: `CHANGE_ME`

The router service generates `/tmp/youtube-relay.json`, which contains upstream proxy credentials in plaintext. Access to this file must be restricted.

Do not commit real credentials, private keys, personal configuration backups or router-specific runtime data.

## Project structure

- `README.md` — main project description.
- `Cargo.toml` — Rust package configuration.
- `Cargo.lock` — locked dependency versions.
- `.cargo/config.toml` — cross-compilation configuration.
- `src/main.rs` — Rust relay source code.
- `youtube-proxy.conf` — dnsmasq configuration.
- `docs/` — detailed installation and recovery documentation.

The `target/` directory contains local build artifacts. It is not required for building the project and should not be uploaded to GitHub.

## Documentation

Detailed setup and recovery instructions:

- [Project overview](docs/01-overview.md)
- [Building the Rust relay](docs/02-build.md)
- [Router installation](docs/03-router-installation.md)
- [Router configuration](docs/04-router-configuration.md)
- [LuCI web interface](docs/05-luci-interface.md)
- [DNS and firewall](docs/06-firewall-and-dns.md)
- [Automatic startup](docs/07-autostart.md)
- [Security considerations](docs/08-security.md)
- [Troubleshooting](docs/09-troubleshooting.md)

## License

This project is licensed under the [GNU General Public License v3.0 only](https://www.gnu.org/licenses/gpl-3.0.html). See the [LICENSE](LICENSE) file.

---

## 🇷🇺 RU

# YouTube Relay для GL.iNet GL-BE6500

Прозрачный TCP-релей на Rust для перенаправления выбранного сетевого трафика через внешний HTTP CONNECT-прокси.

Проект предназначен для роутера GL.iNet GL-BE6500 с прошивкой на базе OpenWrt. Решение объединяет Rust-релей, dnsmasq, nftables, procd и пользовательский интерфейс настройки LuCI.

## Возможности

- TCP-релей на Rust с поддержкой HTTP CONNECT.
- Аутентификация на HTTP-прокси (Basic).
- Определение исходного адреса назначения через `SO_ORIGINAL_DST`.
- Анализ TLS ClientHello и выбор адреса назначения по SNI.
- Сбор IPv4-адресов доменов через dnsmasq nftset.
- Перенаправление выбранного трафика локальной сети средствами nftables.
- Блокировка QUIC для выбранных адресов, чтобы стимулировать переход на TCP.
- Автоматический перезапуск службы через procd.
- Веб-интерфейс LuCI для настройки внешнего прокси.

## Принцип работы

1. Устройство локальной сети запрашивает IP-адрес домена.
2. dnsmasq разрешает домен и добавляет соответствующие IPv4-адреса в набор nftables `youtube_proxy4`.
3. nftables перенаправляет подходящие TCP-соединения из локальной сети на локальный порт релея `12348`.
4. Rust-релей определяет исходный адрес назначения через `SO_ORIGINAL_DST`.
5. Для HTTPS-соединений релей пытается извлечь имя хоста из TLS SNI.
6. Релей устанавливает HTTP CONNECT-туннель через настроенный внешний прокси.
7. Релей передаёт TCP-данные между клиентом локальной сети и внешним прокси.

HTTPS-трафик передаётся через туннель. Релей не расшифровывает TLS и не выполняет функции посредника для перехвата HTTPS.

## Проверенная среда

| Компонент | Версия или значение |
|---|---|
| Роутер | GL.iNet GL-BE6500 |
| Прошивка | GL.iNet 4.9.0 |
| Базовая версия OpenWrt | 23.05-SNAPSHOT |
| Архитектура процессора | AArch64 |
| Версия Rust Edition | 2021 |
| Целевая платформа Rust | `aarch64-unknown-linux-musl` |
| Кросс-компилятор | Buildroot GCC 12.3.0 |
| Ядро Linux | 5.4.213 |
| Библиотека C | musl 1.2.4 |

Другие модели роутеров и версии прошивок пока не проверялись.

## Ограничения

- Релей обрабатывает TCP-трафик, но не является универсальным прокси для UDP.
- Это не VPN.
- Совместимость с нативным протоколом Telegram MTProto не гарантируется.
- Текущая функция маскировки SNI не вызывается из основного пути обработки соединений.
- Используемые DNS- и nftables-правила работают с IPv4-адресами.
- QUIC не проксируется. UDP-порт 443 блокируется для выбранных адресов, чтобы стимулировать переход на TCP.
- HTTPS-трафик передаётся туннелем без расшифровки TLS.

## Безопасность

Никогда не публикуйте реальные учётные данные прокси и файлы рабочей конфигурации, содержащие пароли.

В документации и примерах используются только условные значения:

- Адрес прокси: `proxy.example.com`
- Порт прокси: `8080`
- Логин: `demo_user`
- Пароль: `CHANGE_ME`

Служба на роутере создаёт файл `/tmp/youtube-relay.json`, содержащий учётные данные внешнего прокси в открытом виде. Доступ к этому файлу необходимо ограничить.

Не добавляйте в репозиторий реальные пароли, закрытые ключи, личные резервные копии конфигураций и другие данные конкретного роутера.

## Структура проекта

- `README.md` — основное описание проекта.
- `Cargo.toml` — конфигурация пакета Rust.
- `Cargo.lock` — зафиксированные версии зависимостей.
- `.cargo/config.toml` — настройки кросс-компиляции.
- `src/main.rs` — исходный код Rust-релея.
- `youtube-proxy.conf` — конфигурация dnsmasq.
- `docs/` — подробная документация по установке и восстановлению.

Каталог `target/` содержит локальные результаты сборки. Он не требуется для сборки проекта и не должен загружаться на GitHub.

## Документация

Подробные инструкции по настройке и восстановлению:

- [Обзор проекта](docs/01-overview.md)
- [Сборка Rust-релея](docs/02-build.md)
- [Установка на роутер](docs/03-router-installation.md)
- [Настройка роутера](docs/04-router-configuration.md)
- [Веб-интерфейс LuCI](docs/05-luci-interface.md)
- [DNS и межсетевой экран](docs/06-firewall-and-dns.md)
- [Автоматический запуск](docs/07-autostart.md)
- [Вопросы безопасности](docs/08-security.md)
- [Диагностика неисправностей](docs/09-troubleshooting.md)

## Лицензия

Проект распространяется исключительно на условиях [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.html). Полный текст лицензии находится в файле [LICENSE](LICENSE).
