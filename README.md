# blog-project

## Описание

Набор из 4-х крейтов:
1. **blog-server**. Крейт-приложение - веб-сервер, который
предоставляет функционал ведения блога:

    - Регистрация
    - Вход
    - Создание поста
    - Обновление поста
    - Удаление поста
    - Получение существующего поста
    - Получение списка постов

Сервер слушает интерфейсы:

    - http
    - gRPC

2. **blog-client**. Крейт-библиотека, предоставляет интерфейс для взаимодействия с blog-server:

    - **grpc_client**. Модуль для отправки запросов по gRPC
    - **http_client**. Модуль для отправки запросов по http

3. **blog-cli**. Крейт-приложение, использующее библиотеку blog-client для взаимодействия с веб-сервером
через командную строку. Позволяет быстро проверить веб-сервр.

4. **blog-wasm**. Клиентское браузерное приложение для ведения блога. Взаимодействует с сервером по протоколу http.

## Сборка сервера (Ubuntu)

Установите утилиту curl:
```
sudo apt install curl
```
Установите компилятор rust и пакетный менеджер cargo:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Пропишите переменную окружения:
```
. "$HOME/.cargo/env"
```

Проверьте корректность установки:
```
rustc --version
cargo --version
```

Скачайте проект с репозитория:
```
git clone https://github.com/SergeyPalam/blog-project.git
```

Для сборки проекта необходимо установить набор инструментов для компиляции и сборки программы:
```
sudo apt install build-essential
```

Установите компилятор protocol buffers:
```
sudo apt install protobuf-compiler
```

войдите в директорию blog-server и выполните:
```
cargo build --release
```

В директории target будет сформировано приложениe blog-server

## Запуск сервера (Ubuntu)

Установите СУБД Postgres:
```
sudo apt install postgresql postgresql-contrib
```

Переключитесь на пользователя postgres и запустите утилиту командной строки psql:
```
sudo -u postgres psql
```

Создайте БД:
```
CREATE DATABASE blog;
```

Создайте роль-владельца БД и установите пароль:
```
CREATE USER bloguser WITH PASSWORD '1234';
GRANT ALL PRIVILEGES ON DATABASE blog TO bloguser;
\c blog;
GRANT USAGE, CREATE ON SCHEMA public TO bloguser;
```

Выйдите из psql:
```
exit
```

Задайте переменные окружения, подставляя актуальные значения:
```
export DB_NAME=blog
export DB_USER=bloguser
export DB_HOST=localhost
export DB_PORT=5432
export DB_PASS=1234
export DB_MAX_CONN=20
export DB_MIN_CONN=5
export LOG_LEVEL=info
export JWT_SECRET=some_secret
```

Запустите сервер:
```
./blog-server
```

## Сборка клиента-утилиты командной строки (Ubuntu)

Перейдите в директорию blog-cli и выполните:

```
cargo build --release
```

## Работа с утилитой blog-cli

Основные команды:

- register [--grpc] --username name --email mail --pass 1234. Регистрация нового пользователя
- login [--grpc] --username name --pass 1234. Вход зарегистрированного пользователя
- create [--grpc] --title title --content content. Создание нового поста (Требует входа)
- update [--grpc] --id id [--title new_title] [--content new_content]. Обновление поста (Требует входа)
- delete [--grpc] --id id. Удаление поста (Требует входа)
- get [--grpc] --id. Получить пост по id поста
- list [--grpc] --offset offset --limit limit. Получить список постов с пагинацией

Утилита может работать как с протоколом http, так и с gRPC (доп. флаг --grpc).

Если утилита запускается на удаленном хосте по отношению к серверу, то нужно установить переменные
окружения:

```
export HTTP_SERVER_ADDR=addr:port
export GRPC_SERVER_ADDR=addr:port
```
Если утилита работает на том же хосте, что и сервер, то переменные устанавливать не нужно.

Запустите сервер.
Запустите утилиту с нужной командой.


## Сборка приложения blog-wasm (Ubuntu)

Установите компонент компилятора для сборки под WASM:
```
rustup target add wasm32-unknown-unknown
```

Установите крейт wasm-pack:
```
cargo install wasm-pack
```

Перейдите в директорию blog-wasm и выполните:
```
wasm-pack build --target web
```

## Запуск UI клиента blog-wasm в браузере

UI-клиент работает по протоколу http.

В директории blow-wasm запустите локальный сервер:
```
python -m http.server
```

Откройте в браузере вкладку и введите:
```
http://localhost:8000/
```

Перед вами появится форма регистрации и входа.
