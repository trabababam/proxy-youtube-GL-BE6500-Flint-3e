## 🇬🇧 EN

# Project Overview

## Introduction

YouTube Relay is a transparent TCP relay for the GL.iNet GL-BE6500 router.

It redirects selected traffic from devices on the local network through an external HTTP CONNECT proxy.

The solution combines several components:

- A Rust application that handles TCP connections and establishes HTTP CONNECT tunnels.
- dnsmasq, which resolves domain names and populates nftables address sets.
- nftables, which redirects selected traffic to the local relay.
- procd, which manages the relay process and starts it automatically.
- LuCI, which provides a web interface for configuring the upstream proxy.

## 🇷🇺 RU

# Обзор проекта

## Введение

YouTube Relay — это прозрачный TCP-релей для роутера GL.iNet GL-BE6500.

Он перенаправляет выбранный трафик устройств локальной сети через внешний HTTP CONNECT-прокси.

Решение объединяет несколько компонентов:

- Приложение на Rust, которое обрабатывает TCP-соединения и устанавливает HTTP CONNECT-туннели.
- dnsmasq, который разрешает доменные имена и заполняет наборы адресов nftables.
- nftables, который перенаправляет выбранный трафик на локальный релей.
- procd, который управляет процессом релея и обеспечивает его автоматический запуск.
- LuCI, который предоставляет веб-интерфейс для настройки внешнего прокси.

## Architecture

The relay is part of a traffic-processing chain:

1. dnsmasq resolves configured domain names.
2. Resolved IPv4 addresses are added to the `youtube_proxy4` nftables set.
3. nftables redirects matching TCP connections to the relay listening on port `12348`.
4. The Rust application identifies the original destination address.
5. For HTTPS, it inspects the TLS ClientHello and attempts to extract the SNI hostname.
6. The relay connects to the upstream HTTP proxy and sends a CONNECT request.
7. After a successful connection, TCP data is forwarded in both directions.

## Component responsibilities

- **Rust relay:** handles incoming TCP connections, proxy authentication, CONNECT requests and bidirectional data forwarding.
- **dnsmasq:** resolves domain names and populates dynamic IPv4 address sets.
- **nftables:** selects and redirects matching traffic and blocks UDP port `443` for selected destinations.
- **procd:** starts the relay, monitors the process and restarts it after unexpected termination.
- **LuCI:** provides the router administration interface for upstream proxy settings.

## Архитектура

Релей является частью общей цепочки обработки сетевого трафика:

1. dnsmasq разрешает настроенные доменные имена.
2. Полученные IPv4-адреса добавляются в набор nftables `youtube_proxy4`.
3. nftables перенаправляет подходящие TCP-соединения на порт `12348`, который прослушивает релей.
4. Приложение на Rust определяет исходный адрес назначения.
5. Для HTTPS оно анализирует TLS ClientHello и пытается извлечь имя хоста из SNI.
6. Релей подключается к внешнему HTTP-прокси и отправляет запрос CONNECT.
7. После успешного подключения TCP-данные передаются в обоих направлениях.

## Назначение компонентов

- **Rust-релей:** обрабатывает входящие TCP-соединения, аутентификацию на прокси, запросы CONNECT и двустороннюю передачу данных.
- **dnsmasq:** разрешает доменные имена и заполняет динамические наборы IPv4-адресов.
- **nftables:** выбирает и перенаправляет подходящий трафик, а также блокирует UDP-порт `443` для выбранных адресов.
- **procd:** запускает релей, контролирует его процесс и перезапускает его после неожиданного завершения.
- **LuCI:** предоставляет интерфейс администрирования роутера для настройки внешнего прокси.

## Traffic processing

### HTTPS connections

For TCP connections to port `443`, the relay reads the initial TLS ClientHello data and attempts to extract the SNI hostname.

If a hostname is available, the relay uses it as the destination in the HTTP CONNECT request. If SNI cannot be extracted, the original destination IP address and port are used.

The relay does not terminate the TLS connection. Encryption remains between the original client and the destination server.

### Other TCP connections

For connections that do not use port `443`, the relay uses the original destination IP address and port obtained through `SO_ORIGINAL_DST`.

The relay then establishes a TCP connection to the configured HTTP proxy and requests a tunnel to the destination.

### Data forwarding

After the proxy accepts the CONNECT request, the relay forwards data between the client and the proxy in both directions.

The relay does not interpret or modify the application data inside the established tunnel.

## Обработка трафика

### HTTPS-соединения

Для TCP-соединений с портом `443` релей считывает начальные данные TLS ClientHello и пытается извлечь имя хоста из SNI.

Если имя хоста удалось получить, релей использует его в качестве адреса назначения в запросе HTTP CONNECT. Если извлечь SNI не удалось, используются исходный IP-адрес и порт назначения.

Релей не завершает TLS-соединение. Шифрование сохраняется между исходным клиентом и сервером назначения.

### Другие TCP-соединения

Для соединений, использующих порт, отличный от `443`, релей применяет исходный IP-адрес и порт назначения, полученные через `SO_ORIGINAL_DST`.

Затем релей устанавливает TCP-соединение с настроенным HTTP-прокси и запрашивает туннель до адреса назначения.

### Передача данных

После того как прокси принимает запрос CONNECT, релей передаёт данные между клиентом и прокси в обоих направлениях.

Релей не анализирует и не изменяет данные приложений внутри установленного туннеля.
