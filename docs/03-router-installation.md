## 🇬🇧 EN

# Router Installation

## Introduction

This document describes how to install the compiled `youtube-relay` executable on a GL.iNet GL-BE6500 router.

The router must run compatible OpenWrt-based firmware and use the AArch64 architecture.

The executable must be built for `aarch64-unknown-linux-musl`.

## 🇷🇺 RU

# Установка на роутер

## Введение

В этом документе описана установка собранного исполняемого файла `youtube-relay` на роутер GL.iNet GL-BE6500.

На роутере должна быть установлена совместимая прошивка на базе OpenWrt с архитектурой AArch64.

Исполняемый файл должен быть собран для целевой платформы `aarch64-unknown-linux-musl`.

## Copying the executable

After a successful cross-compilation, the executable is located at:

    target/aarch64-unknown-linux-musl/release/youtube-relay

Copy the file to the router using SCP:

    scp target/aarch64-unknown-linux-musl/release/youtube-relay root@ROUTER_IP:/tmp/youtube-relay

Replace `ROUTER_IP` with the router's actual IP address.

## Installing the executable

Connect to the router over SSH and copy the executable to `/usr/bin`:

    cp /tmp/youtube-relay /usr/bin/youtube-relay

Set executable permissions:

    chmod 755 /usr/bin/youtube-relay

## Копирование исполняемого файла

После успешной кросс-компиляции исполняемый файл находится по пути:

    target/aarch64-unknown-linux-musl/release/youtube-relay

Скопируйте его на роутер с помощью SCP:

    scp target/aarch64-unknown-linux-musl/release/youtube-relay root@ROUTER_IP:/tmp/youtube-relay

Замените `ROUTER_IP` фактическим IP-адресом роутера.

## Установка исполняемого файла

Подключитесь к роутеру по SSH и скопируйте файл в каталог `/usr/bin`:

    cp /tmp/youtube-relay /usr/bin/youtube-relay

Установите права на выполнение:

    chmod 755 /usr/bin/youtube-relay

## Verifying the installation

Run the following commands on the router:

    file /usr/bin/youtube-relay
    ls -lh /usr/bin/youtube-relay

The executable should be an AArch64 ELF binary with execute permissions.

The tested router used a statically linked executable.

## Проверка установки

Выполните на роутере следующие команды:

    file /usr/bin/youtube-relay
    ls -lh /usr/bin/youtube-relay

Исполняемый файл должен иметь формат ELF для архитектуры AArch64 и права на выполнение.

На проверенном роутере использовался статически скомпонованный исполняемый файл.
