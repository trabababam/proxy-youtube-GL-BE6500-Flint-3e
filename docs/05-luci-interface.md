## 🇬🇧 EN

# LuCI Web Interface

## Introduction

The project includes a custom LuCI page for configuring the upstream HTTP proxy.

The interface uses the OpenWrt UCI configuration system and stores its settings in `youtube-relay`.

## 🇷🇺 RU

# Веб-интерфейс LuCI

## Введение

В проект входит пользовательская страница LuCI для настройки внешнего HTTP-прокси.

Интерфейс использует систему конфигурации UCI в OpenWrt и сохраняет настройки в конфигурации `youtube-relay`.

## Page location

The menu entry is available under:

    Services → YouTube Proxy

The internal LuCI route is:

    admin/services/youtube-relay

The page is provided by the view `youtube-relay`.

## Configuration fields

The page provides the following settings:

- **Enable relay:** enables or disables the service.
- **Proxy server:** IP address or hostname of the upstream HTTP proxy.
- **Proxy port:** TCP port of the upstream proxy.
- **Username:** optional proxy authentication username.
- **Password:** optional proxy authentication password. The input is hidden.

## Расположение страницы

Пункт меню находится в разделе:

    Службы → YouTube Proxy

Внутренний маршрут LuCI:

    admin/services/youtube-relay

Страница реализована представлением `youtube-relay`.

## Поля настройки

Страница предоставляет следующие параметры:

- **Включить релей:** включает или отключает службу.
- **Сервер прокси:** IP-адрес или доменное имя внешнего HTTP-прокси.
- **Порт прокси:** TCP-порт внешнего прокси.
- **Логин:** необязательное имя пользователя для аутентификации на прокси.
- **Пароль:** необязательный пароль для аутентификации. Вводимое значение скрыто.

## Saving settings

After changing the fields, use the Save & Apply button in LuCI.

OpenWrt saves the values to the `youtube-relay` UCI configuration and applies the changes.

The service init script is configured to reload the relay when the `youtube-relay` configuration changes.

## Security notes

The password field is visually masked in the web interface. This does not mean that the password is encrypted in the UCI configuration.

The init script also generates `/tmp/youtube-relay.json`, which contains the proxy credentials in plaintext.

Restrict access to the router and avoid sharing configuration files or diagnostic output containing credentials.

## Сохранение настроек

После изменения полей нажмите кнопку Save & Apply в интерфейсе LuCI.

OpenWrt сохраняет значения в конфигурации UCI `youtube-relay` и применяет изменения.

Скрипт службы настроен на перезапуск релея при изменении конфигурации `youtube-relay`.

## Замечания по безопасности

Поле пароля визуально скрывает вводимое значение. Это не означает, что пароль зашифрован в конфигурации UCI.

Скрипт запуска также создаёт файл `/tmp/youtube-relay.json`, содержащий учётные данные прокси в открытом виде.

Ограничьте доступ к роутеру и не распространяйте файлы конфигурации или диагностический вывод, содержащие учётные данные.
