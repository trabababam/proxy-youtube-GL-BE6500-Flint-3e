## 🇬🇧 EN

# Security Considerations

## Introduction

The relay uses an upstream HTTP proxy and may require authentication credentials.

These credentials are stored in the router configuration and are used by the init script to generate the runtime configuration.

Protecting proxy credentials and router configuration files is important.

## 🇷🇺 RU

# Вопросы безопасности

## Введение

Релей использует внешний HTTP-прокси, для которого может требоваться аутентификация.

Учётные данные хранятся в конфигурации роутера и используются скриптом запуска для создания рабочей конфигурации.

Защита учётных данных прокси и файлов конфигурации роутера имеет важное значение.

## Credential storage

The upstream proxy username and password are stored in `/etc/config/youtube-relay`.

The init script generates `/tmp/youtube-relay.json` from these settings.

The generated JSON file contains the credentials in plaintext. The password field being masked in LuCI does not encrypt the stored value.

The file permissions should be reviewed and restricted so that unauthorized local users cannot read the credentials.

## Хранение учётных данных

Логин и пароль внешнего прокси хранятся в `/etc/config/youtube-relay`.

Скрипт запуска создаёт файл `/tmp/youtube-relay.json` на основе этих настроек.

Создаваемый JSON-файл содержит учётные данные в открытом виде. Скрытие пароля в интерфейсе LuCI не означает, что сохранённое значение зашифровано.

Необходимо проверить и ограничить права доступа к этому файлу, чтобы посторонние локальные пользователи не могли прочитать учётные данные.

## Safe project publication

Before publishing the project, make sure that the repository does not contain:

- Real proxy IP addresses or hostnames.
- Proxy usernames or passwords.
- Router-specific configuration containing private information.
- Runtime JSON files containing credentials.
- Build artifacts or local toolchain files with machine-specific paths.

Use placeholder values in documentation and example configuration files.

The `target/` directory contains build artifacts and should not be included in the source repository.

## Безопасная публикация проекта

Перед публикацией проекта убедитесь, что в репозитории отсутствуют:

- Реальные IP-адреса и доменные имена прокси.
- Логины и пароли прокси.
- Конфигурации конкретного роутера, содержащие личные данные.
- Рабочие JSON-файлы с учётными данными.
- Сборочные артефакты и локальные файлы кросс-компилятора с путями, привязанными к конкретному компьютеру.

В документации и примерах конфигурации используйте условные значения.

Каталог `target/` содержит сборочные артефакты и не должен включаться в исходный репозиторий.

## Traffic and transport security

The relay establishes HTTP CONNECT tunnels through the configured upstream proxy.

It does not terminate or decrypt the destination TLS connection. HTTPS traffic remains protected by the TLS connection between the client and the destination server.

The upstream proxy connection itself uses HTTP proxy protocol. Do not assume that the connection between the router and the proxy is encrypted.

Use a trusted upstream proxy and consider the confidentiality of credentials and traffic metadata when selecting a proxy provider.

## Безопасность передачи трафика

Релей создаёт туннели HTTP CONNECT через настроенный внешний прокси.

Он не завершает и не расшифровывает TLS-соединение с конечным сервером. HTTPS-трафик остаётся защищён TLS-соединением между клиентом и сервером назначения.

Для подключения к внешнему прокси используется протокол HTTP. Не следует считать, что соединение между роутером и прокси зашифровано.

Используйте надёжного поставщика прокси и учитывайте конфиденциальность учётных данных и метаданных трафика при выборе сервиса.
