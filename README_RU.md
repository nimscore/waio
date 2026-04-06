<!-- LOGO -->
<h1>
<p align="center">
  <!-- <img src="https://github.com/waio/waio/assets/logo.png" alt="Лого" width="128"> -->
  <br>Waio
</h1>
  <p align="center">
    Нативные виджеты и панели для Wayland. Одна платформа. Без Electron.
    <br />
    Декларативный UI через Slint, скрипты на Lua, рендеринг через SHM.
    <br />
    <a href="#about">О проекте</a>
    ·
    <a href="#download">Загрузка</a>
    ·
    <a href="#documentation">Документация</a>
    ·
    <a href="#contributing">Участие</a>
    ·
    <a href="#roadmap">План развития</a>
    <br />
    <a href="README.md">🇬🇧 Read in English</a>
  </p>
</p>

<br>

> [!NOTE]
>
> Waio находится на стадии **beta** — основные функции стабильны, полный
> Lua API доступен. API и формат файлов могут меняться. Используйте на свой
> страх и риск.

## About

Waio — это платформа для создания нативных виджетов и панелей для
Wayland-рабочего стола без Electron, WebView или любых браузерных
технологий. Хотя существует множество способов вывести что-то на
рабочий стол — conky, waybar, polybar, eww — все они заставляют вас
выбирать между гибкостью, производительностью и простотой. Waio
предоставляет все три.

Один `.wa` файл описывает всё: декларативный UI на Slint, логику на
Lua 5.4, конфигурацию в YAML. Демон компилирует, рендерит и управляет
виджетами через wlr-layer-shell и SHM-буферы Wayland.

**`waio-daemon`** — фоновый процесс, который владеет Wayland-подключением,
компилирует Slint-компоненты на лету, исполняет Lua-скрипты в
изолированной среде с системой разрешений и rate limiting, и общается
через JSON-RPC по Unix-сокету.

**`waio-cli`** — интерфейс командной строки для загрузки, выгрузки,
обновления и просмотра виджетов.

**`waio-shared`** — общая Rust-библиотека с типами сущностей, парсером
`.wa` файлов и протоколом IPC. Используется и демоном, и CLI.

## Download

Соберите из исходного кода:

```bash
git clone https://github.com/nimscore/waio.git
cd waio
cargo build --release
```

Бинарные файлы: `target/release/waio-daemon`, `target/release/waio-cli`.

## Documentation

### Начало работы
- [Создание первой ауры](docs/auras/README.md) — пошаговое руководство
- [Формат файла ауры](docs/auras/file-format.md) — YAML + Slint + Layout + Lua
- [Slint UI](docs/auras/slint.md) — компоненты, свойства, лейауты
- [Справочник Lua API](docs/auras/lua-api.md) — все модули `waio.*`

### Архитектура
- [Чистая архитектура](docs/clean-architecture.md) — конституция проекта
- [QWEN.md](QWEN.md) — справочник разработчика (полные детали кодовой базы)

### Быстрый старт

```bash
# Запустите демон (в фоне)
WAYLAND_DISPLAY=wayland-1 target/release/waio-daemon &

# Загрузите виджет
target/release/waio-cli load examples/clock-bar/aura.wa
```

Виджет появится на экране. Отредактируйте `examples/clock-bar/aura.wa`
и используйте `waio-cli update` для перезагрузки.

## Lua API — краткий обзор

Waio предоставляет богатый Lua API для создания интерактивных виджетов:

| Модуль | Функции | Разрешение |
|--------|---------|------------|
| `waio.timer` | `interval()`, `timeout()`, `cancel()` | — |
| `waio.time` | `now()`, `format()`, `unix()` | — |
| `waio.sys` | `battery()`, `cpu_usage()`, `memory()`, `temperatures()`, `disks()`, `uptime()`, `network()` | — |
| `waio.fs` | `read_file()`, `list_dir()` | `fs_read` |
| `waio.http` | `get()`, `post()` | `http` |
| `waio.input` | `on_click()`, `on_scroll()`, `on_hover()`, `set_cursor()`, `get_layout()` | `input` |
| `waio.exec` | `exec()` | `exec` |
| `waio.audio` | `get_volume()`, `set_volume()`, `get_mute()`, `set_mute()`, `list_sinks()` | `system` |
| `waio.notify` | `notify()` | `system` |
| `waio.power` | `shutdown()`, `reboot()`, `suspend()`, `hibernate()`, `lock_screen()` | `system` |
| `waio.backlight` | `get()`, `get_max()`, `set()`, `change()` | `system` |
| `waio.bluetooth` | `scan()`, `connect()`, `disconnect()`, `enabled()`, `status()` | `system` |
| `waio.wifi` | `scan()`, `connect()`, `current()`, `disconnect()`, `enabled()` | `network` |
| `waio.clipboard` | `get()`, `set()` | — |

Все модули с разрешениями имеют встроенный rate limiting.

Полная документация: [Справочник Lua API](docs/auras/lua-api.md).

## Contributing and Developing

Если у вас есть идеи, баги и т.д. по Waio, или вы хотите внести вклад
через pull requests — откройте issue или отправьте PR. Тем, кто хочет
участвовать в разработке Waio, стоит начать со структуры проекта и
запуска тестов.

```bash
cargo test --workspace      # запустить тесты
cargo clippy --workspace    # линтинг
cargo fmt --all -- --check  # проверка форматирования
```

## Roadmap and Status

Waio находится на стадии beta. Высокоуровневый план развития проекта:

|  #  | Шаг                                                     | Статус |
| :-: | ------------------------------------------------------- | :----: |
|  1  | Core: Wayland lifecycle, per-widget rendering           |   ✅   |
|  2  | Lua-скриптинг с таймерами на базе calloop               |   ✅   |
|  3  | Поддержка нескольких мониторов, авто-определение output   |   ✅   |
|  4  | Безопасность: Lua sandbox, разрешения, rate limiting    |   ✅   |
|  5  | Персистентность: сохранение/восстановление после рестарта |   ✅   |
|  6  | Суб-компонентный рендеринг (многослойная композиция)     |   ✅   |
|  7  | Mouse/touch события, hit-testing                        |   ✅   |
|  8  | Системные модули: audio, exec, sysinfo, notify, power   |   ✅   |
|  9  | Десктопные модули: wifi, bluetooth, clipboard, backlight|   ✅   |
|  10 | Production: интеграционные тесты, обработка сигналов     |   ❌   |
|  11 | Desktop GUI (Tauri + React) для управления виджетами    |   ❌   |
|  12 | Экосистема: реестр пакетов, темы, галерея виджетов      |   ❌   |

Подробные детали каждого шага:

#### Core: Wayland Lifecycle, Per-Widget Rendering

Waio реализует протокол wlr-layer-shell для создания панелей и баров
на краях рабочего стола, и SHM (shared memory) буферы для рендеринга
пикселей без GPU-композитинга. Жизненный цикл поверхности следует
корректной последовательности wlr-layer-shell: создать поверхность →
commit (без буфера) → композитор шлёт configure → ack configure →
рендер первого кадра → цикл frame callback.

Каждый виджет получает собственную Wayland-поверхность и Slint
software renderer. Поддерживается многослойная отрисовка: демон
композирует несколько Slint под-окон в единый ARGB8888 буфер и
отправляет его на поверхность через `draw_precomposited()`.

#### Lua-скриптинг с таймерами на базе Calloop

Lua 5.4 скрипты исполняются в изолированной среде через mlua.
Опасные функции (`os.execute`, `io.popen`, `dofile`, `loadfile`,
`debug.*`) блокируются на этапе создания и на уровне глобалов.
Каждый виджет работает в ограниченном `_ENV`, который предоставляет
только языковые примитивы и явно зарегистрированные модули.

Таймеры реализованы через `calloop::channel`: потоки таймеров спят
и отправляют события `TimerFire` через канал, который главный event
loop принимает и диспетчеризует в Lua callback'и — всё в главном
потоке, безопасно. Без polling, без `try_recv()` на горячем пути.

#### Mouse/Touch события и Hit-Testing

Pointer-события (клик, скролл, hover) принимаются через Wayland
`wl_seat` → `wl_pointer` и диспетчеризуются в окна Slint через
`WindowEvent::PointerPressed/Moved/Released/Scrolled`. Многослойные
виджеты используют AABB hit-testing: верхний слой, содержащий курсор,
получает событие. Lua-скрипты регистрируют callback'и через
`waio.input.on_click(callback)`.

#### Поддержка нескольких мониторов, авто-определение output

Виджеты можно привязать к конкретному output (монитору) через поле
`output` в конфиге `.wa`. Если не указано — демон автоматически
определяет первый подключённый output. Трекинг output'ов корректно
обрабатывает события подключения/отключения.

#### Безопасность: Lua Sandbox, разрешения, Rate Limiting

Lua sandbox имеет два уровня защиты:

1. `StdLib::ALL_SAFE` — блокирует `debug` и C-модули при создании
2. `sanitize_globals()` — зануляет опасные функции (`os.execute`,
   `io.popen` и т.д.)
3. Ограниченный `_ENV` — среда для каждого виджета только с
   безопасными модулями

Дополнительные модули ограничены явными разрешениями с rate limiting:

| Модуль | Разрешение | Лимит |
|--------|-----------|-------|
| `waio.fs` | `fs_read` | 100/мин |
| `waio.http` | `http` | 60/мин |
| `waio.input` | `input` | — |
| `waio.exec` | `exec` | 10/мин |
| `waio.audio`, `.notify`, `.power`, `.backlight`, `.bluetooth` | `system` | 20/мин |
| `waio.wifi` | `network` | 30/мин |

Модуль `exec` также поддерживает белый список команд через
`exec_commands` в манифесте `.wa`.

#### Персистентность: сохранение/восстановление после рестарта

Виджеты сохраняются в `data_dir/{uuid}.json` при загрузке и
автоматически восстанавливаются при перезапуске демона. Файл
конфигурации в `~/.config/waio/config.yaml` контролирует путь к
data directory и используется демоном и CLI.

#### Суб-компонентный рендеринг (многослойная композиция)

Сложные виджеты можно собирать из нескольких Slint-компонентов,
каждый рендерится в свой буфер. Демон композирует эти слои в
финальный буфер: сначала фон, затем динамические слои сверху.
Статические слои (например, фон) рендерятся один раз, динамические
обновляются при изменении свойств. Такой подход устраняет артефакты
ghosting, которые возникают при partial rendering Slint через
`ReusedBuffer`.

#### Desktop GUI (Tauri + React)

Долгосрочная цель — десктопное приложение для визуального управления
виджетами: создание, редактирование, позиционирование, предпросмотр
и загрузка виджетов без командной строки. Построено на Tauri + React,
общение с демоном через существующий JSON-RPC IPC.

#### Экосистема: реестр пакетов, темы, галерея виджетов

Мечта — сообщество, где люди делятся виджетами, темами и
конфигурациями. Реестр пакетов позволит легко находить и
устанавливать виджеты от других пользователей.

## License

MIT
