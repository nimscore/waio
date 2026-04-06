# Lua API — Полный справочник

Все модули `waio.*` доступны в Lua-скриптах аур. Модули без разрешений
доступны всегда, остальные требуют соответствующих `permissions` в YAML.

---

## waio.timer

**Разрешение:** не требуется

Таймеры на базе calloop channel. Callback'и исполняются в главном потоке.

### `waio.timer.interval(ms, callback)`

Периодический таймер. Вызывает `callback` каждые `ms` миллисекунд.

```lua
local id = waio.timer.interval(1000, function()
    print("Called every second")
end)
```

**Параметры:**
- `ms` (number) — интервал в миллисекундах
- `callback` (function) — функция без аргументов

**Возвращает:** `number` — ID таймера (для отмены)

### `waio.timer.timeout(ms, callback)`

Одноразовый таймер. Вызывает `callback` один раз через `ms` миллисекунд.

```lua
waio.timer.timeout(5000, function()
    print("Called once after 5 seconds")
end)
```

### `waio.timer.cancel(id)`

Отменяет таймер.

```lua
local id = waio.timer.interval(1000, function() ... end)
waio.timer.cancel(id)
```

**Параметры:**
- `id` (number) — ID таймера

**Возвращает:** `boolean` — `true` если таймер найден и отменён

---

## waio.time

**Разрешение:** не требуется

Системное время через `chrono`.

### `waio.time.now()`

Возвращает таблицу с текущим временем.

```lua
local t = waio.time.now()
-- t = {
--     year = 2026, month = 4, day = 6,
--     hour = 15, min = 30, sec = 45,
--     str = "15:30:45"
-- }
```

**Возвращает:** `{year, month, day, hour, min, sec, str}`

### `waio.time.format(fmt)`

Форматирует текущее время по шаблону (strftime-формат).

```lua
local date = waio.time.format("%Y-%m-%d")   -- "2026-04-06"
local time = waio.time.format("%H:%M:%S")   -- "15:30:45"
```

### `waio.time.unix()`

Unix timestamp (секунды с 1970-01-01).

```lua
local ts = waio.time.unix()  -- 1744000000
```

---

## waio.sys

**Разрешение:** не требуется

Системный мониторинг через `sysinfo` + sysfs fallback.

### `waio.sys.battery()`

```lua
local batt = waio.sys.battery()
-- {
--     level = 85,         -- процент заряда
--     charging = false,   -- заряжается ли
--     time_remaining = 3600  -- секунд до разряда (или nil)
-- }
```

### `waio.sys.cpu_usage()`

```lua
local cpu = waio.sys.cpu_usage()
-- {
--     global = 45.2,              -- общий % использования
--     cores = {12.5, 67.3, ...}   -- % по ядрам
-- }
```

### `waio.sys.memory()`

```lua
local mem = waio.sys.memory()
-- {
--     used = 8589934592,    -- использовано (байт)
--     total = 17179869184,  -- всего (байт)
--     percent = 50.0        -- процент использования
-- }
```

### `waio.sys.temperatures()`

```lua
local temps = waio.sys.temperatures()
-- [
--     {name = "Package id 0", temp = 65.0, max = 100.0},
--     {name = "thermal_zone0", temp = 55.0, max = 105.0}
-- ]
```

### `waio.sys.disks()`

```lua
local disks = waio.sys.disks()
-- [
--     {mount = "/", used = 50000000000, total = 100000000000, filesystem = "ext4"},
--     {mount = "/home", used = 200000000000, total = 500000000000, filesystem = "ext4"}
-- ]
```

### `waio.sys.uptime()`

```lua
local uptime = waio.sys.uptime()  -- секунд с загрузки
```

### `waio.sys.network()`

```lua
local net = waio.sys.network()
-- {
--     rx = 1234567890,  -- получено байт
--     tx = 987654321    -- отправлено байт
-- }
```

---

## waio.fs

**Разрешение:** `fs_read`

Чтение файлов с защитой от path traversal.

### `waio.fs.read_file(path)`

```lua
local content = waio.fs.read_file("/proc/stat")
```

**Параметры:**
- `path` (string) — путь к файлу

**Возвращает:** `string` или `nil` + ошибка

**Allowed paths:**
- Текущая директория
- `~/.config/waio/widgets/`
- `/proc`
- `/sys`

### `waio.fs.list_dir(path)`

```lua
local entries = waio.fs.list_dir("/sys/class/net")
-- {"eth0", "lo", "wlan0"}
```

**Возвращает:** `{string, ...}` — отсортированный список имён файлов

---

## waio.http

**Разрешение:** `http` | **Лимит:** 60/мин

HTTP-запросы через `ureq` (синхронные).

### `waio.http.get(url)`

```lua
local resp = waio.http.get("https://api.example.com/data")
-- resp = {status = 200, body = '{"key": "value"}'}
```

### `waio.http.post(url, body, content_type?)`

```lua
local resp = waio.http.post(
    "https://api.example.com/data",
    '{"name": "test"}',
    "application/json"
)
```

**Возвращает:** `{status: number, body: string}`

**Ограничения:**
- Только `http://` и `https://`
- Таймаут: 10 секунд
- Максимальный ответ: 10 МБ

---

## waio.input

**Разрешение:** `input`

Mouse/keyboard events и управление курсором.

### `waio.input.on_click(callback)`

```lua
waio.input.on_click(function(button, x, y, component_name)
    print("Clicked: button=" .. button .. " at " .. x .. "," .. y)
    print("Component: " .. component_name)
end)
```

**Callback получает:**
- `button` (number) — код кнопки (0x110 = левая, 0x111 = правая, 0x112 = средняя)
- `x` (number) — X-координата
- `y` (number) — Y-координата
- `component_name` (string) — имя Slint-компонента под курсором

### `waio.input.on_scroll(callback)`

```lua
waio.input.on_scroll(function(delta_x, delta_y, x, y)
    print("Scroll: " .. delta_y .. " at " .. x .. "," .. y)
end)
```

### `waio.input.on_hover(callback)`

```lua
waio.input.on_hover(function(x, y, entered)
    if entered then
        print("Mouse entered at " .. x .. "," .. y)
    else
        print("Mouse left")
    end
end)
```

**Callback получает:**
- `x`, `y` — координаты
- `entered` (boolean) — `true` при входе, `false` при выходе

### `waio.input.set_cursor(cursor_type)`

```lua
waio.input.set_cursor("pointer")   -- курсор-указатель
waio.input.set_cursor("text")      -- текстовый курсор
waio.input.set_cursor("grab")      -- рука (grab)
waio.input.set_cursor("crosshair") -- перекрестие
waio.input.set_cursor("wait")      -- ожидание
waio.input.set_cursor("default")   -- обычный
```

### `waio.input.get_layout()`

```lua
local layout = waio.input.get_layout()  -- "en", "ru", "us"
```

**Возвращает:** `string` — текущая раскладка клавиатуры

---

## waio.exec

**Разрешение:** `exec` | **Лимит:** 10/мин

Выполнение системных команд.

### `waio.exec(cmd, args?)`

```lua
-- Без аргументов
local result = waio.exec("kitty")

-- С аргументами
local result = waio.exec("kitty", {"-e", "htop"})
-- result = {
--     stdout = "...",
--     stderr = "",
--     status = 0
-- }
```

**Параметры:**
- `cmd` (string) — команда (должна быть в whitelist)
- `args` (table|string|nil) — аргументы

**Возвращает:** `{stdout, stderr, status}`

**Безопасность:**
- Whitelist команд в `exec_commands` YAML-блока
- Таймаут: 30 секунд
- Без stdin
- Автоматическое уничтожение процессов

---

## waio.audio

**Разрешение:** `system` | **Лимит:** 20/мин

Управление звуком через `pactl` (PulseAudio/PipeWire).

### `waio.audio.get_volume()`

```lua
local vol = waio.audio.get_volume()  -- 75.5 (проценты)
```

### `waio.audio.set_volume(percent)`

```lua
waio.audio.set_volume(50)  -- 50%
```

### `waio.audio.get_mute()`

```lua
local muted = waio.audio.get_mute()  -- true/false
```

### `waio.audio.set_mute(bool)`

```lua
waio.audio.set_mute(true)   -- замутить
waio.audio.set_mute(false)  -- размутить
```

### `waio.audio.list_sinks()`

```lua
local sinks = waio.audio.list_sinks()
-- [
--     {
--         name = "alsa_output.pci-0000_00_1f.3.analog-stereo",
--         description = "Built-in Audio Analog Stereo",
--         volume = 75,
--         muted = false,
--         is_default = true
--     }
-- ]
```

---

## waio.notify

**Разрешение:** `system` | **Лимит:** 30/мин

Десктопные уведомления через `notify-rust`.

### `waio.notify(options)`

```lua
waio.notify({
    title = "Warning",
    body = "Temperature above 80°C",
    icon = "dialog-warning",     -- опционально
    urgency = "critical",        -- "low", "normal", "critical"
    timeout = 10000              -- мс, по умолчанию 5000
})
```

**Параметры:**
- `title` (string) — заголовок
- `body` (string) — тело уведомления
- `icon` (string, опционально) — иконка (default: `"dialog-information"`)
- `urgency` (string, опционально) — `"low"`, `"normal"`, `"critical"`
- `timeout` (number, опционально) — мс (default: 5000)

---

## waio.power

**Разрешение:** `system` | **Лимит:** 20/мин

Управление питанием через `systemctl`.

### `waio.power.shutdown()`

```lua
waio.power.shutdown()  -- Выключение
```

### `waio.power.reboot()`

```lua
waio.power.reboot()  -- Перезагрузка
```

### `waio.power.suspend()`

```lua
waio.power.suspend()  -- Suspend to RAM
```

### `waio.power.hibernate()`

```lua
waio.power.hibernate()  -- Suspend to disk
```

### `waio.power.lock_screen()`

```lua
waio.power.lock_screen()  -- swaylock → loginctl lock-session
```

---

## waio.backlight

**Разрешение:** `system` | **Лимит:** 20/мин

Управление яркостью через sysfs.

### `waio.backlight.get()`

```lua
local brightness = waio.backlight.get()  -- 1500
```

### `waio.backlight.get_max()`

```lua
local max = waio.backlight.get_max()  -- 4096
```

### `waio.backlight.set(value)`

```lua
waio.backlight.set(2000)  -- установить яркость
```

### `waio.backlight.change(delta)`

```lua
waio.backlight.change(10)   -- +10%
waio.backlight.change(-10)  -- -10%
```

---

## waio.wifi

**Разрешение:** `network` | **Лимит:** 30/мин

Управление WiFi через `nmcli`.

### `waio.wifi.scan()`

```lua
local networks = waio.wifi.scan()
-- [
--     {ssid = "MyWiFi", strength = 85, security = "WPA2"},
--     {ssid = "NeighborWiFi", strength = 45, security = "WPA3"}
-- ]
```

### `waio.wifi.connect(ssid, options?)`

```lua
-- Открытая сеть
local result = waio.wifi.connect("OpenWiFi")

-- Защищённая сеть
local result = waio.wifi.connect("MyWiFi", {password = "secret123"})
-- result = {ok = true}  или  {ok = false, error = "..."}
```

### `waio.wifi.current()`

```lua
local ssid = waio.wifi.current()  -- "MyWiFi" или nil
```

### `waio.wifi.disconnect()`

```lua
waio.wifi.disconnect()
```

### `waio.wifi.enabled(bool)`

```lua
waio.wifi.enabled(true)   -- включить WiFi
waio.wifi.enabled(false)  -- выключить WiFi
```

---

## waio.bluetooth

**Разрешение:** `system` | **Лимит:** 20/мин

Управление Bluetooth через `bluetoothctl`.

### `waio.bluetooth.scan()`

```lua
local devices = waio.bluetooth.scan()
-- [
--     {
--         address = "AA:BB:CC:DD:EE:FF",
--         name = "My Headphones",
--         paired = true,
--         connected = false,
--         battery = 85
--     }
-- ]
```

### `waio.bluetooth.connect(address)`

```lua
local result = waio.bluetooth.connect("AA:BB:CC:DD:EE:FF")
-- {ok = true}  или  {ok = false, error = "..."}
```

### `waio.bluetooth.disconnect(address)`

```lua
waio.bluetooth.disconnect("AA:BB:CC:DD:EE:FF")
```

### `waio.bluetooth.enabled(bool)`

```lua
waio.bluetooth.enabled(true)   -- включить Bluetooth
waio.bluetooth.enabled(false)  -- выключить
```

### `waio.bluetooth.status()`

```lua
local status = waio.bluetooth.status()
-- {powered = true, discovering = false}
```

---

## waio.clipboard

**Разрешение:** не требуется

Буфер обмена через `wl-paste`/`wl-copy`.

### `waio.clipboard.get()`

```lua
local text = waio.clipboard.get()  -- "скопированный текст"
```

**Возвращает:** `string` или `nil` при ошибке

### `waio.clipboard.set(text)`

```lua
waio.clipboard.set("текст для копирования")
```

---

## slint

**Разрешение:** не требуется

Связь между Lua и Slint-компонентами.

### `slint.set_property(name, value)`

```lua
slint.set_property("TimeLayer.time_text", "15:30:45")
slint.set_property("CpuLayer.cpu_text", "cpu: 45%")
```

**Формат имени:**
- `"LayerName.property"` — для многослойных виджетов
- `"property"` — для однослойных

### `slint.get_property(name)`

```lua
local text = slint.get_property("TimeLayer.time_text")
```

**Возвращает:** `string` — текущее значение свойства

---

## Rate Limiting

Все модули с разрешениями имеют встроенные лимиты:

| Модуль | Лимит |
|--------|-------|
| `waio.fs` | 100/мин |
| `waio.http` | 60/мин |
| `waio.exec` | 10/мин |
| `waio.notify` | 30/мин |
| `waio.audio`, `.power`, `.backlight`, `.bluetooth` | 20/мин |
| `waio.wifi` | 30/мин |

При превышении лимита вызов вернёт ошибку:
```lua
local ok, err = pcall(function()
    waio.http.get("https://example.com")
end)
if not ok then
    print("Rate limited: " .. err)
end
```
