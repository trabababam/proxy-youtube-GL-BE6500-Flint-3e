## 🇬🇧 EN

# Troubleshooting

## Introduction

This document describes basic diagnostic procedures for common YouTube Relay problems.

Run router commands through an SSH session with root privileges.

## 🇷🇺 RU

# Устранение неполадок

## Введение

В этом документе описаны основные способы диагностики распространённых проблем YouTube Relay.

Команды для роутера выполняйте через SSH-сеанс с правами root.

## Relay process is not running

Check the service status:

    /etc/init.d/youtube-relay status

Inspect recent log messages:

    logread -e youtube-relay

Check that the executable exists and has execute permissions:

    ls -lh /usr/bin/youtube-relay
    file /usr/bin/youtube-relay

If the service does not start, check the UCI configuration and verify that the upstream proxy server and port are configured.

## Процесс релея не запущен

Проверьте состояние службы:

    /etc/init.d/youtube-relay status

Просмотрите последние сообщения журнала:

    logread -e youtube-relay

Убедитесь, что исполняемый файл существует и имеет права на выполнение:

    ls -lh /usr/bin/youtube-relay
    file /usr/bin/youtube-relay

Если служба не запускается, проверьте конфигурацию UCI и убедитесь, что сервер и порт внешнего прокси указаны правильно.

## Upstream proxy connection problems

If the relay process is running but connections fail, check the upstream proxy settings.

Inspect the UCI configuration without displaying the password:

    uci show youtube-relay | sed -E "s/(password=).*/\\1'REDACTED'/"

Verify the following:

- The proxy hostname or IP address is correct.
- The TCP port is correct.
- The username and password are valid, if authentication is required.
- The router can reach the upstream proxy.

Remember that `/tmp/youtube-relay.json` contains credentials in plaintext. Do not share its contents.

## Проблемы с подключением к внешнему прокси

Если процесс релея запущен, но соединения не устанавливаются, проверьте параметры внешнего прокси.

Просмотрите конфигурацию UCI, не отображая пароль:

    uci show youtube-relay | sed -E "s/(password=).*/\\1'REDACTED'/"

Проверьте следующее:

- Правильно ли указано доменное имя или IP-адрес прокси.
- Правильно ли указан TCP-порт.
- Верны ли логин и пароль, если требуется аутентификация.
- Может ли роутер подключиться к внешнему прокси.

Помните, что `/tmp/youtube-relay.json` содержит учётные данные в открытом виде. Не распространяйте его содержимое.

## DNS and firewall problems

If selected traffic does not reach the relay, check the DNS configuration:

    ls -lh /tmp/dnsmasq.d/youtube-proxy.conf
    logread -e dnsmasq

Check whether the dynamic IPv4 set exists:

    nft list set inet fw4 youtube_proxy4

Inspect the active redirect and QUIC-blocking rules:

    nft list chain inet fw4 youtube_relay_dstnat
    nft list chain inet fw4 youtube_relay_mangle

Check the Telegram redirect chain if needed:

    nft list chain inet fw4 telegram_relay_dstnat

Make sure the firewall rules are loaded and that the destination addresses are present in the expected set.

## Проблемы с DNS и межсетевым экраном

Если выбранный трафик не попадает в релей, проверьте DNS-конфигурацию:

    ls -lh /tmp/dnsmasq.d/youtube-proxy.conf
    logread -e dnsmasq

Проверьте наличие динамического набора IPv4:

    nft list set inet fw4 youtube_proxy4

Просмотрите активные правила перенаправления и блокировки QUIC:

    nft list chain inet fw4 youtube_relay_dstnat
    nft list chain inet fw4 youtube_relay_mangle

При необходимости проверьте цепочку перенаправления Telegram:

    nft list chain inet fw4 telegram_relay_dstnat

Убедитесь, что правила межсетевого экрана загружены, а адреса назначения присутствуют в соответствующем наборе.

## HTTPS, QUIC and Telegram limitations

The relay handles TCP connections. It is not a general-purpose VPN and does not proxy arbitrary UDP traffic.

UDP port 443 is blocked for destinations in the `youtube_proxy4` set to prevent QUIC from bypassing the TCP redirect.

For HTTPS connections, the relay extracts the SNI hostname from the TLS ClientHello when available. If SNI cannot be extracted, it uses the original destination IP address.

The relay does not decrypt TLS traffic. SNI masking is not active in the current connection-processing path.

Telegram IP ranges are redirected by firewall rules, but this does not guarantee compatibility with native Telegram MTProto connections.

## Ограничения HTTPS, QUIC и Telegram

Релей обрабатывает TCP-соединения. Это не универсальный VPN и не прокси для произвольного UDP-трафика.

UDP-порт 443 блокируется для адресов из набора `youtube_proxy4`, чтобы QUIC не обходил перенаправление TCP-трафика.

Для HTTPS-соединений релей извлекает имя SNI из TLS ClientHello, если оно доступно. Если извлечь SNI не удаётся, используется исходный IP-адрес назначения.

Релей не расшифровывает TLS-трафик. Маскировка SNI в текущем пути обработки соединений не активна.

IP-диапазоны Telegram перенаправляются правилами межсетевого экрана, но это не гарантирует совместимость с нативными соединениями Telegram по протоколу MTProto.
