## 🇬🇧 EN

# Building the Rust Relay

## Introduction

This document describes how to build the `youtube-relay` executable for the GL.iNet GL-BE6500 router.

The project uses Rust and the `aarch64-unknown-linux-musl` compilation target.

The resulting executable is a statically linked AArch64 Linux binary intended for the router.

## 🇷🇺 RU

# Сборка Rust-релея

## Введение

В этом документе описана сборка исполняемого файла `youtube-relay` для роутера GL.iNet GL-BE6500.

В проекте используется Rust и целевая платформа компиляции `aarch64-unknown-linux-musl`.

Результатом сборки является статически скомпонованный исполняемый файл для Linux AArch64, предназначенный для роутера.

## Build requirements

The build environment requires:

- Rust compiler (`rustc`).
- Cargo package manager.
- The Rust target `aarch64-unknown-linux-musl`.
- An AArch64 musl cross-compiler provided by Buildroot.
- The linker configuration in `.cargo/config.toml`.

The tested environment used Rust 1.100.0, Cargo 1.100.0 and Buildroot GCC 12.3.0.

## Cross-compilation configuration

The project uses `.cargo/config.toml` to select the AArch64 musl linker.

The configuration refers to the linker by its executable name: `aarch64-buildroot-linux-musl-gcc`.

Make sure that the directory containing this cross-compiler is included in `PATH`. The Rust toolchain binaries must also be available in `PATH`.

For example, adjust the following paths to match your installation:

    export PATH="/path/to/rust/bin:/path/to/buildroot/bin:$PATH"

No machine-specific absolute linker path is stored in the project configuration.

## Требования к среде сборки

Для сборки необходимы:

- Компилятор Rust (`rustc`).
- Менеджер пакетов Cargo.
- Целевая платформа Rust `aarch64-unknown-linux-musl`.
- Кросс-компилятор AArch64 musl из состава Buildroot.
- Настройка компоновщика в `.cargo/config.toml`.

В проверенной среде использовались Rust 1.100.0, Cargo 1.100.0 и Buildroot GCC 12.3.0.

## Настройка кросс-компиляции

В файле `.cargo/config.toml` задаётся компоновщик для AArch64 musl.

В конфигурации компоновщик указан по имени исполняемого файла: `aarch64-buildroot-linux-musl-gcc`.

Убедитесь, что каталог с этим кросс-компилятором добавлен в `PATH`. Исполняемые файлы Rust toolchain также должны быть доступны через `PATH`.

Например, измените пути в следующей команде в соответствии со своей установкой:

    export PATH="/path/to/rust/bin:/path/to/buildroot/bin:$PATH"

В конфигурации проекта больше не используется абсолютный путь, привязанный к конкретному компьютеру.

## Build command

Run the following command from the project root directory:

    cargo build --release --target aarch64-unknown-linux-musl

Cargo downloads the required dependencies and builds the release executable.

The resulting binary is located at:

    target/aarch64-unknown-linux-musl/release/youtube-relay

## Проверка результата сборки

После завершения сборки исполняемый файл должен находиться по пути:

    target/aarch64-unknown-linux-musl/release/youtube-relay

Проверить тип файла можно командой:

    file target/aarch64-unknown-linux-musl/release/youtube-relay

Ожидаемый результат — исполняемый файл ELF для архитектуры ARM AArch64 со статической компоновкой.

## Команда сборки

Выполните следующую команду из корневого каталога проекта:

    cargo build --release --target aarch64-unknown-linux-musl

Cargo загрузит необходимые зависимости и соберёт исполняемый файл в режиме release.

## Build warnings

The tested build completed successfully.

The compiler reported two warnings:

- The variable `начальные_данные` does not need to be mutable.
- The function `замаскировать_sni` is never used.

These warnings did not prevent the executable from being built.

The SNI masking function is not called by the current connection-processing path. Do not assume that SNI masking is active.

## Предупреждения компилятора

Проверенная сборка завершилась успешно.

Компилятор сообщил о двух предупреждениях:

- Переменная `начальные_данные` не требует объявления с `mut`.
- Функция `замаскировать_sni` нигде не используется.

Эти предупреждения не помешали созданию исполняемого файла.

Функция маскировки SNI не вызывается из текущего пути обработки соединений. Не следует считать, что маскировка SNI действительно работает.
