# Формат файла ауры (.wa)

Файл `.wa` — это текстовый файл, содержащий четыре XML-подобных блока:
`<yaml>`, `<slint>`, `<layout>`, `<lua>`.

## YAML блок

Содержит метаданные и конфигурацию виджета.

### Метаданные (`meta`)

```yaml
meta:
  id: 550e8400-e29b-41d4-a716-446655440000  # UUID виджета
  name: Clock Bar                             # Отображаемое имя
  version: 1.0.0                              # Версия
  author: Your Name                           # Автор
  description: Top panel with clock           # Описание
  permissions: [timer, system_time]           # Разрешения
```

**Поля:**

| Поле | Тип | Обязательно | Описание |
|------|-----|:-----------:|----------|
| `id` | UUID | Да | Уникальный идентификатор. Генерируется через `uuidgen` |
| `name` | string | Да | Человекочитаемое имя |
| `version` | string | Да | Семантическая версия |
| `author` | string | Да | Автор виджета |
| `description` | string | Да | Краткое описание |
| `permissions` | [string] | Нет | Список разрешений |
| `exec_commands` | [string] | Нет | Whitelist команд для `waio.exec` |

### Конфигурация (`config`)

```yaml
config:
  anchor: top            # Позиция на экране
  width: 1920            # Ширина в пикселях
  height: 40             # Высота в пикселях
  exclusive_zone: 40     # Резервируемое место (compozitor не перекрывает)
  output: "eDP-1"        # Имя монитора (опционально, auto-detect если нет)
  margin:                # Отступы (опционально)
    top: 0
    left: 0
    right: 0
    bottom: 0
```

**Якоря (`anchor`):**

| Значение | Описание |
|----------|----------|
| `top` | Верх экрана (панель) |
| `bottom` | Низ экрана |
| `left` | Левый край |
| `right` | Правый край |
| `top-left` | Левый верхний угол |
| `top-right` | Правый верхний угол |
| `bottom-left` | Левый нижний угол |
| `bottom-right` | Правый нижний угол |

## Slint блок

Декларативное описание UI-компонентов. Каждый слой — отдельный `export component`.

```slint
export component Background inherits Window {
    width: 1920px;
    height: 40px;
    Rectangle {
        width: parent.width;
        height: parent.height;
        background: #1a1a1a;
    }
}

export component TimeLayer inherits Window {
    width: 100px;
    height: 24px;
    background: transparent;
    in property <string> time_text: "00:00:00";
    Text {
        text: parent.time_text;
        color: #ffffff;
        font-size: 18px;
    }
}
```

### Правила Slint

1. **Каждый слой** — `export component ... inherits Window`
2. **Размеры** указываются в `px`: `width: 100px`
3. **Входные свойства** — `in property <тип> имя: значение;`
4. **Прозрачный фон** — `background: transparent;` (для не-fonовых слоёв)
5. **Компоненты** используют стандартные элементы Slint: `Rectangle`, `Text`, `Image`, `Path`, и т.д.

### Поддерживаемые типы свойств

| Slint тип | Lua значение |
|-----------|-------------|
| `<string>` | строка |
| `<int>` | целое число |
| `<float>` | дробное число |
| `<bool>` | true/false |
| `<length>` | размер в px |
| `<color>` | цвет (#RRGGBB) |
| `<[string]>` | массив строк (для ListView) |

## Layout блок

YAML-маппинг позиций слоёв. Каждый ключ — имя Slint-компонента.

```yaml
Background:   { x: 0,    y: 0,  w: 1920, h: 40, dynamic: false }
TimeLayer:    { x: 10,   y: 10, w: 100,  h: 24 }
DateLayer:    { x: 150,  y: 12, w: 100,  h: 20 }
```

### Поля слоя

| Поле | Тип | По умолчанию | Описание |
|------|-----|:-----------:|----------|
| `x` | int | 0 | X-позиция в буфере |
| `y` | int | 0 | Y-позиция в буфере |
| `w` | int | 1920 | Ширина слоя |
| `h` | int | 40 | Высота слоя |
| `dynamic` | bool | true | `false` = статический (рендерится один раз) |

### Порядок рендеринга

Слои рендерятся в порядке объявления. Первый слой — фон. Последующие —
поверх фона. Статические слои (`dynamic: false`) не перерисовываются при
изменении свойств, что экономит ресурсы.

## Lua блок

Скрипт Lua 5.4, исполняемый в песочнице.

```lua
-- Таймер обновляет время каждую секунду
waio.timer.interval(1000, function()
    local time = waio.time.now()
    slint.set_property("TimeLayer.time_text", time.str)
end)

-- Обновление даты раз в минуту
waio.timer.interval(60000, function()
    local date = waio.time.format("%Y-%m-%d")
    slint.set_property("DateLayer.date_text", date)
end)
```

### Доступные API

Смотрите полный справочник: [Lua API](lua-api.md)

Краткий список:
- `waio.timer.interval(ms, cb)`, `waio.timer.timeout(ms, cb)`, `waio.timer.cancel(id)`
- `waio.time.now()`, `waio.time.format(fmt)`, `waio.time.unix()`
- `waio.sys.battery()`, `waio.sys.cpu_usage()`, `waio.sys.memory()`, и др.
- `slint.set_property("Layer.prop", value)`, `slint.get_property("Layer.prop")`
- `waio.input.on_click(cb)`, `waio.input.set_cursor("pointer")`
- `waio.fs.read_file(path)`, `waio.http.get(url)`
- `waio.exec(cmd, args)`
- `waio.notify({title, body})`
- `waio.clipboard.get()`, `waio.clipboard.set(text)`

## Примеры

### Минимальный виджет (однослойный)

```
<yaml>
meta:
  id: a1b2c3d4-e5f6-7890-abcd-ef1234567890
  name: Simple Clock
  version: 1.0.0
  author: Me
  description: Simple clock
  permissions: [timer, system_time]
config:
  anchor: top
  width: 1920
  height: 30
  exclusive_zone: 30
</yaml>

<slint>
export component ClockBar inherits Window {
    width: 1920px;
    height: 30px;
    background: #2d2d2d;
    in property <string> time_text: "";
    Text {
        text: parent.time_text;
        color: #ffffff;
        font-size: 16px;
        x: 10px;
        y: (parent.height - self.height) / 2;
    }
}
</slint>

<lua>
waio.timer.interval(1000, function()
    slint.set_property("time_text", waio.time.now().str)
end)
</lua>
```

> Без `<layout>` блока виджет рендерится в однослойном режиме (single-component).

### Многослойный виджет с интерактивностью

```
<yaml>
meta:
  id: b2c3d4e5-f6a7-8901-bcde-f12345678901
  name: Interactive Bar
  version: 1.0.0
  author: Me
  description: Bar with clickable buttons
  permissions: [timer, system_time, input]
config:
  anchor: top
  width: 1920
  height: 40
  exclusive_zone: 40
</yaml>

<slint>
export component Background inherits Window {
    width: 1920px; height: 40px;
    Rectangle { width: parent.width; height: parent.height; background: #1a1a1a; }
}

export component ButtonLayer inherits Window {
    width: 100px; height: 30px; background: transparent;
    in property <string> btn_text: "Click me";
    in property <color> btn_color: #3b82f6;
    Rectangle {
        width: parent.width; height: parent.height;
        background: parent.btn_color;
        border-radius: 6px;
        Text {
            text: parent.btn_text;
            color: #ffffff;
            font-size: 14px;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}
</slint>

<layout>
Background:  { x: 0, y: 0, w: 1920, h: 40, dynamic: false }
ButtonLayer: { x: 10, y: 5, w: 100, h: 30 }
</layout>

<lua>
-- Клик по кнопке
waio.input.on_click(function(button, x, y, component)
    if component == "ButtonLayer" then
        slint.set_property("ButtonLayer.btn_text", "Clicked!")
        slint.set_property("ButtonLayer.btn_color", "#22c55e")
    end
end)

-- Hover-эффект
waio.input.on_hover(function(x, y, entered)
    if entered then
        slint.set_property("ButtonLayer.btn_color", "#60a5fa")
    else
        slint.set_property("ButtonLayer.btn_color", "#3b82f6")
    end
end)
</lua>
```

## Генерация UUID

```bash
uuidgen            # Linux/macOS
# или
python3 -c "import uuid; print(uuid.uuid4())"
```
