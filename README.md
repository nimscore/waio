# Waio

> **Виджеты и панели для Wayland-рабочего стола. Одна платформа. Без Electron.**

<p align="center">
  <img src="https://img.shields.io/badge/Status-Beta-yellow?style=flat-square" alt="Status">
  <img src="https://img.shields.io/badge/Wayland-✓-00B3E0?style=flat-square" alt="Wayland">
  <img src="https://img.shields.io/badge/Rust-2021-orange?style=flat-square&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Slint-1.15-2d7332?style=flat-square" alt="Slint">
  <img src="https://img.shields.io/badge/Lua-5.4-2C2D72?style=flat-square&logo=lua" alt="Lua">
</p>

---

## 🖼️ Что вы увидите

Панель на экране с часами и системным мониторингом. Прозрачный фон, плавные анимации, **0% CPU в простое**. Никаких Electron, никаких WebView — чистый Rust и Slint software rendering через Wayland SHM.

---

## ✨ Возможности

- 🎨 **Декларативный UI** — описывайте виджеты на `.wa` файлах (YAML + Slint + Lua)
- 🦀 **Родный Wayland** — wlr-layer-shell protocol, SHM буферы, корректный lifecycle
- ⚡ **Минимум ресурсов** — <1% CPU при обновлении раз в секунду
- 🌙 **Lua-скрипты** — таймеры, системные метрики, HTTP-запросы прямо в виджете
- 🔌 **Daemon + CLI** — демон работает в фоне, `waio-cli` управляет загрузкой/выгрузкой
- 🔒 **Система разрешений** — каждый виджет ограничен явными permission'ами

---

## 🚀 Быстрый старт

### Сборка

```bash
git clone https://github.com/waio/waio.git
cd waio
cargo build --release
```

### Запуск

```bash
# 1. Запустите демон (в фоне)
WAYLAND_DISPLAY=wayland-1 target/release/waio-daemon &

# 2. Загрузите виджет из примера
target/release/waio-cli load examples/clock-bar/aura.wa

# Готово — часы отображаются на экране
```

### Что происходит

```
waio-cli  ──JSON-RPC──▶  waio-daemon  ───wlr-layer-shell──▶  Comp
                                                              │
Slint render ◄── Lua timer ───┘                       SHM buffer
```

---

## 📖 Формат `.wa` файла

```yaml
---
meta:
  id: my-clock
  name: Clock Widget
  version: 1.0.0
  permissions: [timer]

config:
  anchor: top
  width: 1920
  height: 40
  exclusive_zone: 40

<slint>
export component AuraBar inherits Window {
    width: 1920px; height: 40px;
    background: #1a1a2e;
    in property <string> time_text: "00:00:00";
    Text {
        text: parent.time_text;
        color: #e0e0e0;
        font-size: 20px;
        x: 15px; y: 10px;
    }
}
</slint>

<lua>
waio.timer.interval(1000, function()
    local t = waio.time.now()
    slint.set_property("time_text", t.str)
end)
</lua>
```

Один файл — один виджет. Без скрытых зависимостей.

---

## 🏗️ Архитектура

| Компонент | Роль |
|-----------|------|
| **waio-daemon** | Wayland-клиент, Slint-рендерер, Lua-рантайм, IPC-сервер |
| **waio-cli** | CLI для загрузки/выгрузки/списка виджетов |
| **waio-shared** | Общие типы, парсер `.waio` файлов, JSON-RPC протокол |
| **examples/** | Готовые виджеты для быстрого старта |

### Технологии

- [**smithay-client-toolkit**](https://github.com/Smithay/client-toolkit) — Wayland клиент (wlr-layer-shell, SHM)
- [**Slint**](https://slint.dev/) — декларативный UI-фреймворк + software renderer
- [**mlua**](https://github.com/mlua-rs/mlua) — Lua 5.4 рантайм
- [**calloop**](https://github.com/Smithay/calloop) — event loop для Wayland + таймеров

---

## 🤝 Contributing

Проект в активной разработке. Содержит только идею и минимум функционала!

### Development

```bash
cargo check -p waio-daemon    # быстрая проверка
cargo run -p waio-daemon       # запуск демона
cargo run -p waio-cli -- list  # список загруженных виджетов
```

### Структура

```
packages/
├── daemon/     # waio-daemon (Wayland + Slint + Lua)
├── cli/        # waio-cli (CLI клиент)
└── shared/     # общие типы и протокол
examples/       # готовые виджеты
```

---

## 📋 Roadmap

| Этап | Статус | Что |
|------|--------|-----|
| **Phase 1: Core** | ✅ Готово | Wayland lifecycle, per-aura rendering, Lua timers |
| **Phase 2: QoL** | 🔜 В работе | Error handling, output tracking, multi-aura support |
| **Phase 3: Polish** | 📋 Planned | Damage tracking, calloop timers, profiling |
| **Phase 4: Ecosystem** | 💭 Ideas | Package registry, waio:// URLs, theme support |

---

> «Сделай свой рабочий стол своим. Без компромиссов.»
