# yug

`yug` (юг) — система сборки проектов на языках C и C++ под платформу AVR, основная задача 
которой в предоставлении удобной для пользователя организации компиляции кода и последующей
его загрузки на микроконтроллер.

Ключевой элемент дизайна yug'а состоит в том, что вся настройка тулчейна происходит посредством удобного конфиг файла в формате TOML.

## Зависимости
```bash
# Debian/Ubuntu
apt install gcc-avr avr-libc avrdude 
```

## Установка

### Из бинарных пакетов

Смотрите [страницу релизов](https://blejzer.ru/unqtd/yug/releases).

### Из исходников
Для сборка и установка из исходников используйте [`cargo`](https://rustup.rs/):
```bash
cargo install --git https://github.com/dx3mod/yug.git
```

## Документация

* [Быстрый старт](./handbook/basic-usage.md) - руководство по использованию утилиты.

Справка:
* [Структура проекта и ее настройка](./handbook/project-structure.md)
* [Подключение внешних библиотек](./handbook/include-libs.md)
