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

# ExternLibs

_Пока что сценарий сборки для зависимостей не реализован!_

На данный момент доступен лишь очень примитивный способ подключить внешнею библиотеку. Все third-party элементы должны лежать в директории `vendor`, в ней же находятся поддиректории с названием библиотек, внутри которых должны располагаться заголовочные файлы и объектные файлы. Пример:
```
vendor
└── uart
    ├── uart.h
    └── uart.o
```

Либо же объектные файлы могут быть указаны в поле `[externlibs]`. Пример:
```toml
# yug.toml

[externlibs]
foobar = { objs = ["foobar/fb.o"] }
```

# ProjectStruct

Структура по умолчанию:
```toml
# yug.toml

[structure]
sources = 'src'
builds = '_build'
includes = 'include'
```

# CompilerOptions 

Пример:
```toml
# yug.toml

[compiler]
args = ['-std=c++11']
```
