## 🇬🇧 EN

# Firewall and DNS Configuration

## Introduction

The relay uses dnsmasq and nftables to identify selected destinations and transparently redirect matching TCP connections to the local Rust relay.

DNS resolution populates a dynamic IPv4 address set. Firewall rules use this set to select traffic for redirection.

## 🇷🇺 RU

# Настройка DNS и межсетевого экрана

## Введение

Релей использует dnsmasq и nftables для определения выбранных сетевых ресурсов и прозрачного перенаправления соответствующих TCP-соединений на локальный Rust-релей.

При разрешении DNS-имён IP-адреса добавляются в динамический набор IPv4. Правила межсетевого экрана используют этот набор для выбора трафика, который необходимо перенаправить.

## DNS configuration

The file `/etc/dnsmasq.d/youtube-proxy.conf` contains domain rules using the `nftset` directive.

When a client resolves a matching domain, dnsmasq adds the resulting IPv4 address to the `youtube_proxy4` set in the `inet fw4` nftables table.

The domain list includes services such as YouTube, Instagram, Meta, OpenAI and Telegram.

The DNS configuration is copied to `/tmp/dnsmasq.d/youtube-proxy.conf` by the relay init script before dnsmasq is reloaded.

The `youtube_proxy4` set uses a one-hour timeout for dynamically added addresses.

## Конфигурация DNS

Файл `/etc/dnsmasq.d/youtube-proxy.conf` содержит правила для доменов с использованием директивы `nftset`.

Когда клиент разрешает доменное имя, соответствующее одному из правил, dnsmasq добавляет полученный IPv4-адрес в набор `youtube_proxy4` таблицы nftables `inet fw4`.

Список доменов включает сервисы YouTube, Instagram, Meta, OpenAI и Telegram.

Перед перезагрузкой dnsmasq скрипт запуска релея копирует DNS-конфигурацию в `/tmp/dnsmasq.d/youtube-proxy.conf`.

Для динамически добавляемых адресов в наборе `youtube_proxy4` установлен срок действия один час.

## nftables rules

The file `/etc/nftables.d/99-youtube-relay.nft` defines the firewall rules.

The `youtube_relay_dstnat` chain redirects TCP connections from LAN clients to addresses in `youtube_proxy4` when the destination port is 80 or 443.

Matching connections are redirected to the local relay port `12348`.

The `youtube_relay_mangle` chain drops UDP traffic to port 443 for the same destination set. This prevents QUIC connections from bypassing the TCP relay.

The rules apply to traffic entering through the `br-lan` interface.

## Правила nftables

Файл `/etc/nftables.d/99-youtube-relay.nft` содержит правила межсетевого экрана.

Цепочка `youtube_relay_dstnat` перенаправляет TCP-соединения клиентов локальной сети к адресам из набора `youtube_proxy4`, если порт назначения — 80 или 443.

Подходящие соединения перенаправляются на локальный порт релея `12348`.

Цепочка `youtube_relay_mangle` блокирует UDP-трафик на порт 443 для того же набора адресов. Это предотвращает обход TCP-релея посредством QUIC.

Правила применяются к трафику, поступающему через интерфейс `br-lan`.

## Telegram traffic rules

The nftables configuration also defines a `telegram_dc4` IPv4 address set containing Telegram data center network ranges.

The `telegram_relay_dstnat` chain redirects matching TCP connections on ports 80 and 443 to the local relay port `12348`.

This is a network-level redirection rule. It does not guarantee compatibility with Telegram's native MTProto protocol.

## Правила обработки трафика Telegram

Конфигурация nftables также определяет набор IPv4-адресов `telegram_dc4`, содержащий сетевые диапазоны дата-центров Telegram.

Цепочка `telegram_relay_dstnat` перенаправляет подходящие TCP-соединения на портах 80 и 443 на локальный порт релея `12348`.

Это правило перенаправления на сетевом уровне. Оно не гарантирует совместимость с нативным протоколом MTProto, используемым Telegram.

## Firewall integration and verification

OpenWrt firewall4 automatically includes files matching `/etc/nftables.d/*.nft`.

The relay rules are therefore loaded from `/etc/nftables.d/99-youtube-relay.nft` as part of the firewall ruleset.

Useful verification commands on the router:

    nft list set inet fw4 youtube_proxy4
    nft list chain inet fw4 youtube_relay_dstnat
    nft list chain inet fw4 youtube_relay_mangle
    nft list chain inet fw4 telegram_relay_dstnat

These commands display the dynamic destination set and the active relay rules.

## Интеграция с межсетевым экраном и проверка

В OpenWrt firewall4 автоматически подключает файлы, соответствующие шаблону `/etc/nftables.d/*.nft`.

Поэтому правила релея загружаются из `/etc/nftables.d/99-youtube-relay.nft` в составе общего набора правил межсетевого экрана.

Полезные команды проверки на роутере:

    nft list set inet fw4 youtube_proxy4
    nft list chain inet fw4 youtube_relay_dstnat
    nft list chain inet fw4 youtube_relay_mangle
    nft list chain inet fw4 telegram_relay_dstnat

Эти команды отображают динамический набор адресов назначения и активные правила релея.
