# Util

Команда для выполнения пользовательского sh-кода.

Синтаксис: 
```bash
yug util <NAME>
```

Пример:
```toml
# yug.toml

[utils]
size = "avr-size _build/firmware.hex"
```
```bash
$ yug util size
```
