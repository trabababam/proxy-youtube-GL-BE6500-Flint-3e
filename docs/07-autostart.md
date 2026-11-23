## 🇬🇧 EN

# Service Management and Autostart

## Introduction

The Rust relay is managed by an OpenWrt init script at `/etc/init.d/youtube-relay`.

The script uses procd to start and monitor the relay process.

## 🇷🇺 RU

# Управление службой и автозапуск

## Введение

Rust-релей управляется скриптом запуска OpenWrt, расположенным по пути `/etc/init.d/youtube-relay`.

Скрипт использует procd для запуска и контроля процесса релея.

## procd process management

The init script enables procd integration with `USE_PROCD=1`.

When the service is enabled and valid proxy settings are available, the script starts:

    /usr/bin/youtube-relay /tmp/youtube-relay.json 0.0.0.0:12348

The `procd_set_param respawn` setting instructs procd to restart the relay process if it exits unexpectedly.

The service uses the startup priority `START=99`.

## Управление процессом через procd

Скрипт запуска включает интеграцию с procd с помощью параметра `USE_PROCD=1`.

Если служба включена и параметры прокси заданы, запускается команда:

    /usr/bin/youtube-relay /tmp/youtube-relay.json 0.0.0.0:12348

Параметр `procd_set_param respawn` указывает procd перезапускать процесс релея при его неожиданном завершении.

Для службы установлен приоритет запуска `START=99`.

## Service control commands

Run these commands on the router.

Enable automatic startup:

    /etc/init.d/youtube-relay enable

Disable automatic startup:

    /etc/init.d/youtube-relay disable

Start the service:

    /etc/init.d/youtube-relay start

Stop the service:

    /etc/init.d/youtube-relay stop

Restart the service:

    /etc/init.d/youtube-relay restart

Check the service status:

    /etc/init.d/youtube-relay status

## Команды управления службой

Выполняйте следующие команды на роутере.

Включить автозапуск:

    /etc/init.d/youtube-relay enable

Отключить автозапуск:

    /etc/init.d/youtube-relay disable

Запустить службу:

    /etc/init.d/youtube-relay start

Остановить службу:

    /etc/init.d/youtube-relay stop

Перезапустить службу:

    /etc/init.d/youtube-relay restart

Проверить состояние службы:

    /etc/init.d/youtube-relay status

## Configuration reload trigger

The init script defines a procd reload trigger for the `youtube-relay` UCI configuration.

When the configuration changes, the reload handler stops and starts the service again.

The `reload_service()` function performs:

    stop
    start

This ensures that the relay process receives the updated proxy settings.

## Автоматическое применение изменений конфигурации

В скрипте запуска определён триггер procd для отслеживания изменений конфигурации UCI `youtube-relay`.

При изменении конфигурации обработчик перезагрузки останавливает службу и запускает её снова.

Функция `reload_service()` выполняет команды:

    stop
    start

Это позволяет процессу релея получить обновлённые параметры прокси.
