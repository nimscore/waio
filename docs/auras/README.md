# Ауры (Auras) — Руководство по созданию виджетов

> Аура — это виджет в терминологии Waio. Каждый виджет называется «aura» и
> описывается в одном `.wa` файле.

## Что такое аура

Аура — это самодостаточный виджет для Wayland-рабочего стола. Один `.wa` файл
содержит всё необходимое:

- **YAML** — метаданные и конфигурация (позиция, размер, монитор, разрешения)
- **Slint** — декларативное описание UI-компонентов
- **Layout** — позиционирование слоёв в финальном буфере
- **Lua** — логика виджета (таймеры, системные данные, сетевые запросы)

Демон `waio-daemon` компилирует Slint, исполняет Lua в песочнице и рендерит
виджет через Wayland SHM.

## Структура `.wa` файла

```
<yaml>
meta:
  id: ваш-uuid
  name: Название
  version: 1.0.0
  author: Ваше имя
  description: Описание виджета
  permissions: [timer, system_time, fs_read]
config:
  anchor: top
  width: 1920
  height: 40
  exclusive_zone: 40
  output: "eDP-1"       # опционально
</yaml>

<slint>
export component Background inherits Window { ... }
export component TimeLayer inherits Window { in property <string> time_text; ... }
</slint>

<layout>
Background: { x: 0, y: 0, w: 1920, h: 40, dynamic: false }
TimeLayer:  { x: 10, y: 10, w: 100, h: 24 }
</layout>

<lua>
waio.timer.interval(1000, function()
    slint.set_property("TimeLayer.time_text", waio.time.now().str)
end)
</lua>
```

## Как это работает

### 1. Загрузка

```bash
waio-cli load my-widget.wa
```

CLI читает файл, парсит YAML, отправляет JSON-RPC запрос демону. Демон:

1. Парсит `.wa` файл через `AuraFile::from_content()`
2. Компилирует Slint код через `slint-interpreter`
3. Создаёт Wayland surface через `wlr-layer-shell`
4. Запускает Lua скрипт в песочнице
5. Сохраняет ауру для восстановления после рестарта

### 2. Рендеринг

Каждый слой из `<layout>` становится отдельным `MinimalSoftwareWindow` в Slint.
Демон композирует все слои в один ARGB8888 буфер и отправляет его на Wayland
поверхность. Статические слои (`dynamic: false`) рендерятся один раз,
динамические — при изменении свойств.

### 3. Обновление свойств

Lua вызывает `slint.set_property("LayerName.prop", value)` → значение попадает
в командную очередь → главный цикл применяет свойство к Slint компоненту →
слой помечается как dirty → перерисовывается на следующем кадре.

## Жизненный цикл ауры

```
waio-cli load → parse → compile → create surface → run Lua → render
                      ↓
              waio-cli update → recompile → re-run Lua → re-render
                      ↓
              waio-cli unload → close surface → cleanup
```

## Разрешения (Permissions)

Каждый модуль Lua API требует определённого разрешения:

| Разрешение | Модули | Описание |
|-----------|--------|----------|
| *(нет)* | `waio.timer`, `waio.time`, `waio.sys`, `waio.clipboard` | Безопасные модули |
| `fs_read` | `waio.fs` | Чтение файлов с защитой от path traversal |
| `http` | `waio.http` | HTTP GET/POST с валидацией URL |
| `input` | `waio.input` | Mouse/keyboard events, cursor control |
| `exec` | `waio.exec` | Выполнение команд (с whitelist) |
| `system` | `waio.audio`, `.notify`, `.power`, `.backlight`, `.bluetooth` | Системные операции |
| `network` | `waio.wifi` | Управление WiFi |

Указываются в YAML блоке:
```yaml
meta:
  permissions: [timer, fs_read, http, input]
```

## Rate Limiting

Все модули с разрешениями имеют встроенные лимиты вызовов:

| Модуль | Лимит |
|--------|-------|
| `waio.fs` | 100/мин |
| `waio.http` | 60/мин |
| `waio.exec` | 10/мин |
| `waio.notify` | 30/мин |
| `system` модули | 20/мин |
| `waio.wifi` | 30/мин |

При превышении лимита вызов вернёт ошибку.

## Безопасность

Lua-скрипты запускаются в песочнице:

1. **StdLib::ALL_SAFE** — `debug` и C-модули заблокированы
2. **sanitize_globals()** — занулены `os.execute`, `io.popen`, `dofile`, `loadfile`
3. **Restricted `_ENV`** — каждый виджет видит только языковые примитивы и зарегистрированные `waio.*` модули

Модуль `exec` дополнительно проверяет whitelist команд из `exec_commands`.

## Следующие шаги

- [Формат файла ауры](file-format.md) — подробное описание каждого блока
- [Slint UI](slint.md) — создание интерфейса виджета
- [Lua API](lua-api.md) — справочник всех функций
