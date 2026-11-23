## 🇬🇧 EN

# Router Configuration

## Introduction

This document describes the router-side configuration of the YouTube Relay service.

The upstream HTTP proxy settings are stored in the OpenWrt UCI configuration system.

The LuCI interface uses the same configuration.

## 🇷🇺 RU

# Настройка роутера

## Введение

В этом документе описана настройка службы YouTube Relay непосредственно на роутере.

Параметры внешнего HTTP-прокси хранятся в системе конфигурации UCI, используемой в OpenWrt.

Веб-интерфейс LuCI работает с той же конфигурацией.

## UCI configuration

The configuration file is `/etc/config/youtube-relay`.

Example:

    config youtube-relay 'main'
        option enabled '1'
        option server 'proxy.example.com'
        option port '8080'
        option username 'demo_user'
        option password 'CHANGE_ME'

All values above are placeholders. Replace them with the actual upstream proxy settings before enabling the service.

The `enabled` option controls whether the relay process is started.

## Конфигурация UCI

Файл конфигурации находится по пути `/etc/config/youtube-relay`.

Пример:

    config youtube-relay 'main'
        option enabled '1'
        option server 'proxy.example.com'
        option port '8080'
        option username 'demo_user'
        option password 'CHANGE_ME'

Все приведённые значения являются условными. Перед включением службы замените их фактическими параметрами внешнего прокси.

Параметр `enabled` определяет, должен ли запускаться процесс релея.

## How the service uses the configuration

The init script `/etc/init.d/youtube-relay` reads the `youtube-relay` UCI configuration.

It performs the following operations:

1. Reads the enabled state and upstream proxy parameters.
2. Stops without starting the relay if the service is disabled.
3. Generates `/tmp/youtube-relay.json` with the HTTP proxy settings.
4. Starts `/usr/bin/youtube-relay` with the generated configuration.
5. Uses procd to monitor the relay process.

The generated JSON file contains proxy credentials in plaintext.

## Использование конфигурации службой

Скрипт запуска `/etc/init.d/youtube-relay` читает настройки UCI из конфигурации `youtube-relay`.

Он выполняет следующие действия:

1. Читает состояние включения службы и параметры внешнего прокси.
2. Не запускает релей, если служба отключена.
3. Создаёт файл `/tmp/youtube-relay.json` с настройками HTTP-прокси.
4. Запускает `/usr/bin/youtube-relay`, передавая ему созданную конфигурацию.
5. Использует procd для контроля процесса релея.

Создаваемый JSON-файл содержит учётные данные прокси в открытом виде.

## Applying configuration changes

After editing the UCI configuration, apply the changes with:

    uci commit youtube-relay
    /etc/init.d/youtube-relay restart

Check the service status:

    /etc/init.d/youtube-relay status

Inspect the service log:

    logread -e youtube-relay

## Применение изменений конфигурации

После изменения настроек UCI примените их командами:

    uci commit youtube-relay
    /etc/init.d/youtube-relay restart

Проверьте состояние службы:

    /etc/init.d/youtube-relay status

Просмотрите сообщения службы в системном журнале:

    logread -e youtube-relay
