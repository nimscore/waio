# Slint UI — Создание интерфейса виджета

Slint — декларативный фреймворк для UI. В Waio он используется через
`slint-interpreter` — код компилируется на лету демоном, без отдельной
компиляции.

## Основы

### Компонент-слой

Каждый слой виджета — это `export component`, наследующий `Window`:

```slint
export component MyLayer inherits Window {
    width: 200px;
    height: 40px;
    background: transparent;

    in property <string> text_value: "Hello";

    Text {
        text: parent.text_value;
        color: #ffffff;
        font-size: 16px;
    }
}
```

### Размеры и единицы

| Единица | Описание | Пример |
|---------|----------|--------|
| `px` | Пиксели | `width: 100px` |
| `%` | Процент от родителя | `width: 50%` |
| `phx` | Физические пиксели (с учётом scale) | `width: 100phx` |

### Свойства

```slint
// Входное свойство (устанавливается из Lua)
in property <string> my_text: "default";

// Свойство с вычислением
property <int> doubled: root.my_number * 2;

// Цвет
in property <color> my_color: #3b82f6;

// Булево
in property <bool> is_active: false;

// Массив (для ListView)
in property <[{label: string, value: int}]> items: [];
```

### Типы данных Slint

| Тип | Описание | Lua значение |
|-----|----------|-------------|
| `string` | Строка | `"hello"` |
| `int` | Целое число | `42` |
| `float` | Дробное число | `3.14` |
| `bool` | Логическое | `true` / `false` |
| `color` | Цвет | `#ff0000`, `red`, `rgb(255, 0, 0)` |
| `length` | Размер | `10px` |
| `angle` | Угол | `45deg` |
| `duration` | Длительность | `500ms` |
| `image` | Изображение | `@image-url("path.png")` |
| `struct` | Структура | `{name: "test", value: 42}` |
| `array` | Массив | `[1, 2, 3]` |

## Элементы UI

### Rectangle

```slint
Rectangle {
    width: 100px;
    height: 50px;
    background: #3b82f6;
    border-radius: 8px;
    border-width: 1px;
    border-color: #1d4ed8;
}
```

### Text

```slint
Text {
    text: "Hello World";
    color: #ffffff;
    font-size: 16px;
    font-weight: 700;         // 400 = normal, 700 = bold
    font-family: "Monospace";
    horizontal-alignment: center;
    vertical-alignment: center;
    wrap: word-wrap;          // word-wrap, no-wrap, char-wrap
}
```

### Image

```slint
Image {
    source: @image-url("/path/to/icon.png");
    width: 24px;
    height: 24px;
    image-fit: contain;       // contain, cover, stretch, preserve-aspect-fit
}
```

### Path (рисование фигур)

```slint
Path {
    width: 100px;
    height: 100px;
    stroke: #ffffff;
    stroke-width: 2px;
    fill: #3b82f6;
    MoveTo { x: 50px; y: 0px; }
    LineTo { x: 100px; y: 100px; }
    LineTo { x: 0px; y: 100px; }
    Close { }
}
```

## Layouts

### Горизонтальный / Вертикальный

```slint
HorizontalBox {
    spacing: 10px;
    Rectangle { width: 50px; height: 50px; background: red; }
    Rectangle { width: 50px; height: 50px; background: blue; }
}

VerticalBox {
    spacing: 5px;
    Text { text: "Line 1"; }
    Text { text: "Line 2"; }
}
```

### Grid

```slint
GridLayout {
    spacing: 10px;
    Rectangle { width: 100px; height: 50px; background: red; }
    Rectangle { width: 100px; height: 50px; background: blue; }
}
```

### Абсолютное позиционирование

```slint
Rectangle {
    width: 100px; height: 100px; background: #333;
    Rectangle {
        x: 10px; y: 10px;
        width: 30px; height: 30px;
        background: #f00;
    }
}
```

## Условный рендеринг

```slint
// Показать/скрыть элемент
if (root.show_details) {
    Text {
        text: "Details here";
        color: #aaaaaa;
    }
}
```

## ListView

```slint
export component ListLayer inherits Window {
    width: 200px;
    height: 100px;
    background: transparent;

    in property <[{label: string, value: int}]> items: [];

    ListView {
        data: root.items;
        Rectangle {
            width: parent.width;
            height: 30px;
            background: #1a1a1a;
            Text {
                text: self.ListView.item.label;
                color: #ffffff;
                x: 10px;
            }
            Text {
                text: self.ListView.item.value + "%";
                color: #aaaaaa;
                x: 150px;
            }
        }
    }
}
```

Обновление из Lua:
```lua
local items = '[{"label":"CPU","value":45},{"label":"RAM","value":72}]'
slint.set_property("ListLayer.items", items)
```

## PopupWindow

Для меню, тултипов, попапов — объявляйте их заранее и управляйте видимостью:

```slint
export component MenuLayer inherits Window {
    width: 200px;
    height: 40px;
    background: transparent;

    in property <bool> menu_visible: false;

    // Основная кнопка
    Rectangle {
        width: parent.width; height: parent.height;
        background: #3b82f6;
        Text { text: "Menu"; color: white; }
    }

    // Popup — появляется при menu_visible = true
    PopupWindow {
        visible: parent.menu_visible;
        width: 200px;
        height: 150px;

        Rectangle {
            width: parent.width; height: parent.height;
            background: #1a1a1a;
            border-radius: 8px;
            // Содержимое меню
        }
    }
}
```

Управление из Lua:
```lua
slint.set_property("MenuLayer.menu_visible", "true")
```

## Анимации

```slint
Rectangle {
    width: 100px; height: 100px;
    background: root.is_hovered ? #60a5fa : #3b82f6;
    animate background { duration: 200ms; }
}
```

## Callbacks (обработка кликов в Slint)

```slint
export component ButtonLayer inherits Window {
    width: 100px; height: 30px; background: transparent;

    callback clicked;

    Rectangle {
        width: parent.width; height: parent.height;
        background: #3b82f6;
        border-radius: 6px;
        TouchArea {
            clicked => { root.clicked(); }
        }
    }
}
```

Однако для Waio **рекомендуемый подход** — обработка кликов через Lua:
```lua
waio.input.on_click(function(button, x, y, component)
    if component == "ButtonLayer" then
        -- обработать клик
    end
end)
```

## Цвета

### Именованные цвета

```
red, green, blue, yellow, orange, purple, pink,
white, black, gray, grey, transparent
```

### Hex-формат

```
#RRGGBB    — #ff0000 (красный)
#RGB       — #f00     (красный, сокращённо)
#RRGGBBAA  — #ff000080 (полупрозрачный красный)
```

### Функции

```slint
color: rgb(255, 0, 0);
color: rgba(255, 0, 0, 0.5);
color: hsl(0, 100%, 50%);
```

## Советы

### 1. Используйте `dynamic: false` для фона

Статические слои рендерятся один раз и не потребляют ресурсы при обновлениях:

```yaml
Background: { x: 0, y: 0, w: 1920, h: 40, dynamic: false }
```

### 2. Прозрачный фон для динамических слоёв

```slint
export component TimeLayer inherits Window {
    background: transparent;  // Прозрачный, не перекрывает фон
    // ...
}
```

### 3. Центрирование текста по вертикали

```slint
Text {
    y: (parent.height - self.height) / 2;
}
```

### 4. Монospace шрифт для системных виджетов

```slint
Text {
    font-family: "Monospace";
    font-size: 13px;
}
```

### 5. Избегайте сложных вычислений в Slint

Slint — для декларативного UI. Сложную логику (парсинг, вычисления)
делайте в Lua и передавайте готовые значения через свойства.

## Ограничения

- ❌ **Нельзя** создать новый компонент из Lua
- ❌ **Нельзя** добавить/удалить элемент из UI-дерева динамически
- ❌ **Нельзя** сгенерировать Slint-код на лету
- ✅ **Можно** менять свойства, модели ListView, видимость PopupWindow

## Примеры

Полный пример: `examples/clock-bar/aura.wa` в репозитории.
