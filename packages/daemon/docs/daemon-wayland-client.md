# План реализации Waio Daemon

> **Цель**: Создать production-ready скелет демона на Rust с правильной архитектурой, который отрисовывает Ауры через **Slint → Wayland SHM → Экран**.

---

## Оглавление

1. [Контекст проекта](#контекст-проекта)
2. [Архитектурные принципы](#архитектурные-принципы)
3. [Технический стек](#технический-стек)
4. [Структура проекта](#структура-проекта)
5. [План реализации](#план-реализации)
6. [Детальная реализация](#детальная-реализация)
7. [Чеклист готовности](#чеклист-готовности)

---

## Контекст проекта

### Видение

Waio — это **«WeakAuras для Linux»**:

1. **Daemon** — Wayland client (layer-shell) + **Slint interpreter** + **Lua API**
2. **Client** — Tauri + React приложение для управления Аурами

### Ключевая особенность

**Ауры — динамический UI на Slint + Lua**, компилируемый **во время выполнения**:
- Копируешь строку с кодом Ауры
- Вставляешь в приложение
- Аура отображается **без перекомпиляции демона**

### Архитектурный принцип

**Прямой путь: Slint → Wayland SHM → Экран**

```
┌─────────────┐     ┌──────────────┐     ┌────────────┐
│   Slint     │ ──► │  Wayland SHM │ ──► │   Экран    │
│  Renderer   │     │   Buffer     │     │            │
└─────────────┘     └──────────────┘     └────────────┘
```

Никаких GLFW, Qt или промежуточных бэкендов.

---

## Архитектурные принципы

Следуем [Конституции чистой архитектуры](./rust-const.md):

```
┌─────────────────────────────────────────────┐
│              Controller                     │
└─────────────┬───────────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────────┐
│              UseCase                        │
└─────────┬───────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────┐
│              Entity                         │
└─────────────────────────────────────────────┘

Infrastructure (Slint, Lua, Wayland) реализует traits из UseCase
```

### Ключевые правила

1. **Инверсия зависимостей** — внутренние слои НЕ зависят от внешних
2. **Traits в UseCase** — Infrastructure реализует traits, объявленные в UseCase
3. **Без глобальных переменных** — Dependency Injection через конструкторы
4. **Async safety** — tokio для всех I/O операций
5. **Error handling** — thiserror для доменных ошибок

---

## Технический стек

```toml
# Slint - динамический UI
slint = "1.5"
slint-interpreter = { version = "1.5", features = ["async", "display-diagnostics"] }

# Lua - скриптовый язык для Аур
mlua = { version = "0.9", features = ["lua54", "send"] }

# Wayland через smithay-client-toolkit
smithay-client-toolkit = "0.18"
wayland-protocols-wlr = "0.2"
wayland-protocols = { version = "0.31", features = ["client"] }

# Event loop интеграция
calloop = "0.12"
calloop-wayland-source = "0.2"

# Async runtime
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Utilities
uuid = { version = "1.0", features = ["v4"] }
dirs = "5.0"
spin_on = "0.1"
chrono = "0.4"
tempfile = "3.0"
```

---

## Структура проекта

```
waio/
├── Cargo.toml                    # Workspace definition
├── .gitignore
├── README.md
├── LICENSE
│
├── packages/
│   ├── shared/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── entity/
│   │       │   ├── mod.rs
│   │       │   ├── aura.rs
│   │       │   └── error.rs
│   │       └── protocol/
│   │           └── messages.rs
│   │
│   └── daemon/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── app.rs
│           ├── usecase/
│           │   ├── mod.rs
│           │   ├── aura.rs
│           │   └── render.rs
│           ├── infrastructure/
│           │   ├── mod.rs
│           │   ├── wayland/
│           │   │   ├── mod.rs
│           │   │   ├── connection.rs
│           │   │   └── surface.rs
│           │   ├── slint/
│           │   │   ├── mod.rs
│           │   │   ├── interpreter.rs
│           │   │   └── platform.rs
│           │   ├── lua/
│           │   │   ├── mod.rs
│           │   │   ├── time.rs
│           │   │   ├── timer.rs
│           │   │   └── slint_bridge.rs
│           │   └── repository/
│           │       └── file_aura.rs
│           ├── controller/
│           │   ├── mod.rs
│           │   └── cli.rs
│           └── error.rs
│
└── examples/
    └── clock-bar/
        ├── clock-bar.slint
        ├── clock-bar.lua
        └── aura.json
```

---

## План реализации

### Этап 0: Подготовка (1 день)
- [ ] Инициализация Cargo workspace
- [ ] Настройка .gitignore, README.md

### Этап 1: Entity слой (1 день)
- [ ] packages/shared: Aura entity с Slint code
- [ ] Domain errors

### Этап 1.5: Примеры Аур (30 минут) ⭐ НОВОЕ
- [ ] Создать папку `examples/clock-bar/`
- [ ] Добавить `clock-bar.slint` (UI разметка)
- [ ] Добавить `clock-bar.lua` (логика обновления)
- [ ] Добавить `aura.json` (метаданные)
- [ ] Обновить `DEFAULT_BAR_SLINT` с примером из файла

### Этап 2: UseCase слой (1 день)
- [ ] AuraUseCase + AuraRepository trait
- [ ] RenderUseCase + Renderer trait

### Этап 3: Infrastructure слой (4-5 дней)

#### 3.1. Repository (0.5 дня)
- [ ] FileAuraRepository

#### 3.2. Wayland (1.5 дня) ⚠️ КРИТИЧНО
- [ ] smithay-client-toolkit подключение
- [ ] layer-shell инициализация
- [ ] **surface.rs: Wayland SHM буфер с NamedTempFile**
- [ ] **Добавить sync_to_file() в ShmBuffer**
- [ ] **Вызвать sync_to_file() перед commit()**
- [ ] **Добавить публичное поле `conn: Connection`**

#### 3.3. Lua API (1.5 дня)
- [ ] **time.rs: time.now(), time.format(), time.unix()**
- [ ] **timer.rs: timer.interval() с callback.clone()**
- [ ] **slint_bridge.rs: slint.set_property()**

#### 3.4. Slint Platform (2 дня) ⚠️ КРИТИЧНО
- [ ] **platform.rs: render_by_line с копированием в Wayland буфер**
- [ ] **interpreter.rs: Slint рендеринг + commit**

### Этап 3.5: Event Loop Integration (1 день)
- [ ] calloop + tokio интеграция
- [ ] Обработка Wayland событий

### Этап 4: Сборка и запуск (1 день) ⚠️ КРИТИЧНО
- [ ] app.rs: Dependency Injection + Event Loop
- [ ] main.rs: Точка входа
- [ ] **Добавить Lua код для тестовой Ауры (timer.interval)**
- [ ] Тестирование на Weston

---

## Детальная реализация

### Этап 0: Инициализация workspace

**Файл**: `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = ["packages/shared", "packages/daemon"]

[workspace.dependencies]
slint = "1.5"
slint-interpreter = { version = "1.5", features = ["async", "display-diagnostics"] }
mlua = { version = "0.9", features = ["lua54", "send"] }
smithay-client-toolkit = "0.18"
wayland-protocols-wlr = "0.2"
wayland-protocols = { version = "0.31", features = ["client"] }
calloop = "0.12"
calloop-wayland-source = "0.2"
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
uuid = { version = "1.0", features = ["v4"] }
dirs = "5.0"
spin_on = "0.1"
chrono = "0.4"
tempfile = "3.0"
```

---

### Этап 1: Entity слой

**Файл**: `packages/shared/src/entity/aura.rs`

```rust
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuraError {
    #[error("Invalid aura syntax: {0}")]
    InvalidSyntax(String),
    #[error("Aura not found: {0}")]
    NotFound(String),
    #[error("Compilation failed: {0}")]
    CompilationFailed(String),
    #[error("Lua error: {0}")]
    LuaError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuraType { Bar, Widget, Overlay }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aura {
    pub id: String,
    pub name: String,
    pub aura_type: AuraType,
    pub slint_code: String,
    pub lua_code: Option<String>,
    pub config: AuraConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraConfig {
    pub anchor: LayerAnchor,
    pub size: Size,
    pub exclusive_zone: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size { pub width: u32, pub height: u32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayerAnchor { Top, Bottom, Left, Right }

impl Aura {
    pub fn new(id: String, name: String, aura_type: AuraType, slint_code: String) -> Self {
        Self { id, name, aura_type, slint_code, lua_code: None, config: AuraConfig::default() }
    }

    pub fn validate(&self) -> Result<(), AuraError> {
        if self.name.is_empty() || self.slint_code.is_empty() {
            return Err(AuraError::InvalidSyntax("Name and Slint code required".into()));
        }
        Ok(())
    }
}

impl Default for Aura {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Untitled".into(),
            aura_type: AuraType::Bar,
            slint_code: DEFAULT_BAR_SLINT.to_string(),
            lua_code: None,
            config: AuraConfig::default(),
        }
    }
}

impl Default for AuraConfig {
    fn default() -> Self {
        Self {
            anchor: LayerAnchor::Top,
            size: Size { width: 1920, height: 30 },
            exclusive_zone: 30,
        }
    }
}

const DEFAULT_BAR_SLINT: &str = r#"
    export component AuraBar inherits Window {
        width: 1920px;
        height: 40px;
        background: #2d2d2d;
        in property <string> time_text: "";
        in property <string> date_text: "";
        Text {
            text: parent.time_text;
            color: #ffffff;
            font-size: 18px;
            font-weight: bold;
            x: 10px;
            y: (parent.height - self.height) / 2;
        }
        Text {
            text: parent.date_text;
            color: #aaaaaa;
            font-size: 14px;
            x: 150px;
            y: (parent.height - self.height) / 2;
        }
    }
"#;
```

**Файл**: `packages/shared/src/lib.rs`

```rust
pub mod entity;
pub use entity::*;
```

**Файл**: `packages/shared/Cargo.toml`

```toml
[package]
name = "waio-shared"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { workspace = true }
thiserror = { workspace = true }
uuid = { workspace = true }
```

---

### Этап 1.5: Примеры Аур

**Файл**: `examples/clock-bar/clock-bar.slint`

```slint
// examples/clock-bar/clock-bar.slint
export component AuraBar inherits Window {
    width: 1920px;
    height: 40px;
    background: #2d2d2d;
    
    in property <string> time_text: "";
    in property <string> date_text: "";
    
    // Часы
    Text {
        text: parent.time_text;
        color: #ffffff;
        font-size: 18px;
        font-weight: bold;
        x: 10px;
        y: (parent.height - self.height) / 2;
    }
    
    // Дата
    Text {
        text: parent.date_text;
        color: #aaaaaa;
        font-size: 14px;
        x: 150px;
        y: (parent.height - self.height) / 2;
    }
}
```

**Файл**: `examples/clock-bar/clock-bar.lua`

```lua
-- examples/clock-bar/clock-bar.lua

-- Обновляем время каждую секунду
waio.timer.interval(1000, function()
    local time = waio.time.now()
    local date = waio.time.format("%Y-%m-%d")
    
    slint.set_property("time_text", time.str)
    slint.set_property("date_text", date)
end)

print("Clock bar aura loaded!")
```

**Файл**: `examples/clock-bar/aura.json`

```json
{
    "id": "clock-bar-example",
    "name": "Clock Bar",
    "aura_type": "Bar",
    "slint_code": "export component AuraBar inherits Window { width: 1920px; height: 40px; background: #2d2d2d; in property <string> time_text: \"\"; in property <string> date_text: \"\"; Text { text: parent.time_text; color: #ffffff; font-size: 18px; x: 10px; y: (parent.height - self.height) / 2; } Text { text: parent.date_text; color: #aaaaaa; font-size: 14px; x: 150px; y: (parent.height - self.height) / 2; } }",
    "lua_code": "waio.timer.interval(1000, function() local time = waio.time.now() local date = waio.time.format('%Y-%m-%d') slint.set_property('time_text', time.str) slint.set_property('date_text', date) end)",
    "config": {
        "anchor": "Top",
        "size": {
            "width": 1920,
            "height": 40
        },
        "exclusive_zone": 40
    }
}
```

---

### Этап 2: UseCase слой

**Файл**: `packages/daemon/src/usecase/aura.rs`

```rust
use waio_shared::entity::{Aura, AuraError};

#[async_trait::async_trait]
pub trait AuraRepository: Send + Sync {
    async fn get(&self, id: &str) -> Result<Aura, AuraError>;
    async fn save(&self, aura: &Aura) -> Result<(), AuraError>;
    async fn delete(&self, id: &str) -> Result<(), AuraError>;
    async fn list(&self) -> Result<Vec<Aura>, AuraError>;
}

pub struct AuraUseCase<R: AuraRepository> { repo: R }

impl<R: AuraRepository> AuraUseCase<R> {
    pub fn new(repo: R) -> Self { Self { repo } }

    pub async fn create_aura(&self, name: String, slint_code: String, aura_type: waio_shared::entity::AuraType) -> Result<Aura, AuraError> {
        let id = uuid::Uuid::new_v4().to_string();
        let aura = Aura::new(id, name, aura_type, slint_code);
        aura.validate()?;
        self.repo.save(&aura).await?;
        Ok(aura)
    }

    pub async fn get_aura(&self, id: &str) -> Result<Aura, AuraError> { self.repo.get(id).await }
    pub async fn list_auras(&self) -> Result<Vec<Aura>, AuraError> { self.repo.list().await }
    pub async fn delete_aura(&self, id: &str) -> Result<(), AuraError> { self.repo.delete(id).await }
}
```

**Файл**: `packages/daemon/src/usecase/render.rs`

```rust
use waio_shared::entity::Aura;

#[async_trait::async_trait]
pub trait Renderer: Send + Sync {
    async fn init(&self) -> Result<(), RenderError>;
    async fn render_aura(&self, aura: &Aura) -> Result<(), RenderError>;
    async fn remove_aura(&self, aura_id: &str) -> Result<(), RenderError>;
    async fn update_aura(&self, aura_id: &str, slint_code: &str) -> Result<(), RenderError>;
    async fn shutdown(&self) -> Result<(), RenderError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Wayland error: {0}")] WaylandError(String),
    #[error("Slint compilation failed: {0}")] SlintCompilationFailed(String),
    #[error("Lua error: {0}")] LuaError(String),
    #[error("Rendering failed: {0}")] RenderFailed(String),
    #[error("Aura not found: {0}")] AuraNotFound(String),
}

pub struct RenderUseCase<R: Renderer> { renderer: R }

impl<R: Renderer> RenderUseCase<R> {
    pub fn new(renderer: R) -> Self { Self { renderer } }
    pub async fn init(&self) -> Result<(), RenderError> { self.renderer.init().await }
    pub async fn render_aura(&self, aura: &Aura) -> Result<(), RenderError> { self.renderer.render_aura(aura).await }
    pub async fn shutdown(&self) -> Result<(), RenderError> { self.renderer.shutdown().await }
}
```

**Файл**: `packages/daemon/src/usecase/mod.rs`

```rust
pub mod aura;
pub mod render;
pub use aura::*;
pub use render::*;
```

---

### Этап 3: Infrastructure слой

#### 3.1. Repository

**Файл**: `packages/daemon/src/infrastructure/repository/file_aura.rs`

```rust
use crate::usecase::aura::AuraRepository;
use waio_shared::entity::{Aura, AuraError};
use std::path::PathBuf;
use tokio::fs;

pub struct FileAuraRepository { base_path: PathBuf }

impl FileAuraRepository {
    pub fn new(base_path: PathBuf) -> Self { Self { base_path } }
    fn aura_path(&self, id: &str) -> PathBuf { self.base_path.join(format!("{}.json", id)) }
}

#[async_trait::async_trait]
impl AuraRepository for FileAuraRepository {
    async fn get(&self, id: &str) -> Result<Aura, AuraError> {
        let content = fs::read_to_string(self.aura_path(id)).await
            .map_err(|e| AuraError::NotFound(e.to_string()))?;
        serde_json::from_str(&content).map_err(|e| AuraError::InvalidSyntax(e.to_string()))
    }

    async fn save(&self, aura: &Aura) -> Result<(), AuraError> {
        let content = serde_json::to_string(aura).map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
        fs::write(self.aura_path(&aura.id), content).await
            .map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), AuraError> {
        fs::remove_file(self.aura_path(id)).await
            .map_err(|e| AuraError::NotFound(e.to_string()))?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Aura>, AuraError> {
        let mut auras = Vec::new();
        let mut entries = fs::read_dir(&self.base_path).await
            .map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
        while let Ok(Some(entry)) = entries.next_entry().await {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&entry.path()).await {
                    if let Ok(aura) = serde_json::from_str::<Aura>(&content) {
                        auras.push(aura);
                    }
                }
            }
        }
        Ok(auras)
    }
}
```

#### 3.2. Wayland Surface ⚠️ КРИТИЧНО

**Файл**: `packages/daemon/src/infrastructure/wayland/connection.rs`

```rust
use smithay_client_toolkit::{compositor::CompositorState, output::OutputState, reexports::client::globals::registry_queue_init, shell::layer::LayerShell};
use tracing::info;
use anyhow::Result;

pub struct WaylandConnection {
    pub compositor: CompositorState,
    pub outputs: OutputState,
    pub layer_shell: LayerShell,
    pub event_queue: wayland_client::EventQueue<()>,
    pub conn: wayland_client::Connection, // ⭐ ПУБЛИЧНОЕ ПОЛЕ
}

impl WaylandConnection {
    pub fn connect() -> Result<Self> {
        info!("Connecting to Wayland...");
        let conn = wayland_client::Connection::connect_to_env()?;
        let (globals, event_queue) = registry_queue_init(&conn)?;
        let qh = event_queue.handle();
        let compositor = CompositorState::bind(&globals, &qh)?;
        let outputs = OutputState::new(&globals, &qh);
        let layer_shell = LayerShell::bind(&globals, &qh)?;
        info!("Connected to Wayland, layer-shell: {}", layer_shell.is_supported());
        Ok(Self { conn, compositor, outputs, layer_shell, event_queue })
    }
}
```

**Файл**: `packages/daemon/src/infrastructure/wayland/surface.rs`

```rust
use smithay_client_toolkit::compositor::Surface;
use smithay_client_toolkit::shell::layer::{LayerSurface, Layer};
use wayland_client::protocol::wl_shm;
use crate::infrastructure::wayland::WaylandConnection;
use waio_shared::entity::{AuraConfig, LayerAnchor};
use anyhow::Result;
use std::io::Write;

pub struct AuraSurface {
    pub surface: Surface,
    pub layer_surface: LayerSurface,
    pub buffer: ShmBuffer,
}

impl AuraSurface {
    pub fn new(conn: &WaylandConnection, config: &AuraConfig) -> Result<Self> {
        let surface = conn.compositor.create_surface();
        let layer_surface = conn.layer_shell.create_layer_surface(&surface, None, Layer::Top, "waio-aura".into());
        
        let anchor = match config.anchor {
            LayerAnchor::Top => smithay_client_toolkit::shell::layer::Anchor::TOP | smithay_client_toolkit::shell::layer::Anchor::LEFT | smithay_client_toolkit::shell::layer::Anchor::RIGHT,
            LayerAnchor::Bottom => smithay_client_toolkit::shell::layer::Anchor::BOTTOM | smithay_client_toolkit::shell::layer::Anchor::LEFT | smithay_client_toolkit::shell::layer::Anchor::RIGHT,
            LayerAnchor::Left => smithay_client_toolkit::shell::layer::Anchor::TOP | smithay_client_toolkit::shell::layer::Anchor::BOTTOM | smithay_client_toolkit::shell::layer::Anchor::LEFT,
            LayerAnchor::Right => smithay_client_toolkit::shell::layer::Anchor::TOP | smithay_client_toolkit::shell::layer::Anchor::BOTTOM | smithay_client_toolkit::shell::layer::Anchor::RIGHT,
        };
        layer_surface.set_anchor(anchor);
        layer_surface.set_size(config.size.width, config.size.height);
        layer_surface.set_exclusive_zone(config.exclusive_zone);
        
        let buffer = ShmBuffer::new(&conn.compositor, config.size.width, config.size.height)?;
        Ok(Self { surface, layer_surface, buffer })
    }
    
    pub fn commit(&self) {
        // ⭐ КРИТИЧНО: Синхронизация Vec -> файл перед отправкой
        let _ = self.buffer.sync_to_file();
        
        self.surface.attach(Some(&self.buffer.buffer), 0, 0);
        self.surface.damage_buffer(0, 0, self.buffer.width as i32, self.buffer.height as i32);
        self.surface.commit();
    }
    
    pub fn get_buffer_data(&mut self) -> &mut [u8] {
        &mut self.buffer.data
    }
}

pub struct ShmBuffer {
    pub buffer: wayland_client::protocol::wl_buffer::WlBuffer,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    _file: tempfile::NamedTempFile,
}

impl ShmBuffer {
    pub fn new(compositor: &smithay_client_toolkit::compositor::CompositorState, width: u32, height: u32) -> Result<Self> {
        let file = tempfile::NamedTempFile::new()?;
        let size = (width * height * 4) as usize;
        file.as_file().set_len(size as u64)?;
        let shm = compositor.wl_shm();
        let pool = shm.create_pool(file.as_file().as_raw_fd(), size as i32);
        let buffer = pool.create_buffer(0, width as i32, height as i32, (width * 4) as i32, wl_shm::Format::Argb8888);
        Ok(Self { buffer, width, height, data: vec![0u8; size], _file: file })
    }
    
    /// ⭐ КРИТИЧНО: Синхронизирует данные из Vec в файл
    pub fn sync_to_file(&self) -> std::io::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open(format!("/proc/self/fd/{}", self._file.as_file().as_raw_fd()))?;
        file.write_all(&self.data)?;
        file.sync_all()
    }
}
```

**Файл**: `packages/daemon/src/infrastructure/wayland/mod.rs`

```rust
pub mod connection;
pub mod surface;
pub use connection::*;
pub use surface::*;
```

#### 3.3. Lua API

**Файл**: `packages/daemon/src/infrastructure/lua/time.rs`

```rust
use mlua::{Lua, Result, Table};
use chrono::Local;

pub fn create_module(lua: &Lua) -> Result<Table> {
    let m = lua.create_table()?;
    m.set("now", lua.create_function(|lua, ()| {
        let now = Local::now();
        let r = lua.create_table()?;
        r.set("year", now.year())?; r.set("month", now.month())?; r.set("day", now.day())?;
        r.set("hour", now.hour())?; r.set("min", now.minute())?; r.set("sec", now.second())?;
        r.set("str", now.format("%H:%M:%S").to_string())?;
        Ok(r)
    })?)?;
    m.set("format", lua.create_function(|_, fmt: String| Ok(Local::now().format(&fmt).to_string()))?)?;
    m.set("unix", lua.create_function(|_, ()| Ok(Local::now().timestamp()))?)?;
    Ok(m)
}
```

**Файл**: `packages/daemon/src/infrastructure/lua/timer.rs`

```rust
use mlua::{Lua, Result, Table, Function};
use std::time::Duration;

pub fn create_module(lua: &Lua) -> Result<Table> {
    let m = lua.create_table()?;
    m.set("interval", lua.create_function(|_, (ms, cb): (u32, Function)| {
        let cb_clone = cb.clone(); // ⭐ Function можно клонировать
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(ms as u64));
            loop {
                interval.tick().await;
                let _ = cb_clone.call::<_, ()>();
            }
        });
        Ok(())
    })?)?;
    m.set("timeout", lua.create_function(|_, (ms, cb): (u32, Function)| {
        let cb_clone = cb.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(ms as u64)).await;
            let _ = cb_clone.call::<_, ()>();
        });
        Ok(())
    })?)?;
    Ok(m)
}
```

**Файл**: `packages/daemon/src/infrastructure/lua/slint_bridge.rs`

```rust
use mlua::{Lua, Result, UserData};
use slint_interpreter::{ComponentInstance, Value, SharedString};

pub struct SlintBridge { instance: ComponentInstance }

impl SlintBridge {
    pub fn new(instance: ComponentInstance) -> Self { Self { instance } }
}

impl UserData for SlintBridge {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("set_property", |_, this, (name, value): (String, String)| {
            this.instance.set_property(&name, Value::from(SharedString::from(value)))
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
        })?;
        methods.add_method("get_property", |_, this, name: String| {
            this.instance.get_property(&name)
                .map(|v| match v { Value::String(s) => Ok(s.to_string()), Value::Number(n) => Ok(n.to_string()), _ => Err(mlua::Error::RuntimeError("Unsupported type".into())) })
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?
        })?;
    }
}

pub fn register(lua: &Lua, instance: ComponentInstance) -> Result<()> {
    lua.globals().set("slint", lua.create_userdata(SlintBridge::new(instance))?)?;
    Ok(())
}
```

**Файл**: `packages/daemon/src/infrastructure/lua/mod.rs`

```rust
pub mod time;
pub mod timer;
pub mod slint_bridge;

use mlua::{Lua, Result};

pub fn register_all(lua: &Lua) -> Result<()> {
    let waio = lua.create_table()?;
    waio.set("time", time::create_module(lua)?)?;
    waio.set("timer", timer::create_module(lua)?)?;
    lua.globals().set("waio", waio)?;
    Ok(())
}
```

#### 3.4. Slint Platform

**Файл**: `packages/daemon/src/infrastructure/slint/platform.rs`

```rust
use slint::platform::SoftwareRenderer;
use slint::{PhysicalSize, Rgb8Pixel};

/// Рендерит Slint компонент в Wayland SHM буфер
pub fn render_frame(
    renderer: &SoftwareRenderer,
    window: &slint::Window,
    surface_buffer: &mut [u8],
    width: u32,
    height: u32,
) {
    renderer.render(window.clone(), PhysicalSize::new(width, height));
    
    let mut line_y = 0;
    renderer.render_by_line(|scanline: &mut [Rgb8Pixel]| {
        let offset = (line_y * width * 4) as usize;
        for (i, pixel) in scanline.iter().enumerate() {
            let buf_offset = offset + (i * 4);
            surface_buffer[buf_offset] = pixel.r;
            surface_buffer[buf_offset + 1] = pixel.g;
            surface_buffer[buf_offset + 2] = pixel.b;
            surface_buffer[buf_offset + 3] = pixel.a;
        }
        line_y += 1;
    });
}
```

**Файл**: `packages/daemon/src/infrastructure/slint/interpreter.rs`

```rust
use crate::usecase::render::{RenderError, Renderer};
use crate::infrastructure::wayland::{WaylandConnection, AuraSurface};
use crate::infrastructure::lua;
use waio_shared::entity::Aura;
use slint_interpreter::{Compiler, ComponentInstance};
use slint::platform::SoftwareRenderer;
use tracing::{info, error};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct AuraInstance {
    instance: ComponentInstance,
    surface: AuraSurface,
    renderer: SoftwareRenderer,
}

pub struct SlintRenderer {
    connection: Arc<WaylandConnection>,
    auras: Arc<Mutex<HashMap<String, AuraInstance>>>,
    lua_state: Arc<Mutex<mlua::Lua>>,
}

impl SlintRenderer {
    pub fn new(connection: Arc<WaylandConnection>) -> Self {
        let lua = mlua::Lua::new();
        let _ = lua::register_all(&lua);
        Self { connection, auras: Arc::new(Mutex::new(HashMap::new())), lua_state: Arc::new(Mutex::new(lua)) }
    }

    fn compile(&self, code: &str) -> Result<slint_interpreter::ComponentDefinition, RenderError> {
        let compiler = Compiler::default();
        let result = spin_on::spin_on(compiler.build_from_source(code.into(), Default::default()));
        let diagnostics: Vec<_> = result.diagnostics().collect();
        if !diagnostics.is_empty() {
            return Err(RenderError::SlintCompilationFailed(diagnostics.iter().map(|d| d.to_string()).collect::<Vec<_>>().join("; ")));
        }
        result.component("AuraBar").or_else(|| result.components().next())
            .ok_or_else(|| RenderError::SlintCompilationFailed("No component found".into()))
    }
}

#[async_trait::async_trait]
impl Renderer for SlintRenderer {
    async fn init(&self) -> Result<(), RenderError> {
        info!("SlintRenderer initialized");
        Ok(())
    }

    async fn render_aura(&self, aura: &Aura) -> Result<(), RenderError> {
        info!("Rendering aura: {}", aura.name);
        
        let definition = self.compile(&aura.slint_code)?;
        let instance = definition.create().map_err(|e| RenderError::RenderFailed(e.to_string()))?;
        let mut surface = AuraSurface::new(&self.connection, &aura.config)
            .map_err(|e| RenderError::WaylandError(e.to_string()))?;
        
        let lua = self.lua_state.lock().map_err(|e| RenderError::LuaError(e.to_string()))?;
        let _ = lua::slint_bridge::register(&lua, instance.clone());
        
        if let Some(ref code) = aura.lua_code {
            let _ = lua.load(code).exec();
        }
        
        let renderer = SoftwareRenderer::default();
        renderer.render(instance.window().clone(), slint::PhysicalSize::new(aura.config.size.width, aura.config.size.height));
        
        // Копируем пиксели из Slint в Wayland буфер
        let buffer_data = surface.get_buffer_data();
        let mut line_y = 0;
        renderer.render_by_line(|scanline: &mut [slint::Rgb8Pixel]| {
            let offset = (line_y * aura.config.size.width * 4) as usize;
            for (i, pixel) in scanline.iter().enumerate() {
                let buf_offset = offset + (i * 4);
                buffer_data[buf_offset] = pixel.r;
                buffer_data[buf_offset + 1] = pixel.g;
                buffer_data[buf_offset + 2] = pixel.b;
                buffer_data[buf_offset + 3] = pixel.a;
            }
            line_y += 1;
        });
        
        surface.commit();
        
        let mut auras = self.auras.lock().map_err(|e| RenderError::RenderFailed(e.to_string()))?;
        auras.insert(aura.id.clone(), AuraInstance { instance, surface, renderer });
        
        info!("Aura '{}' rendered", aura.name);
        Ok(())
    }

    async fn remove_aura(&self, aura_id: &str) -> Result<(), RenderError> {
        let mut auras = self.auras.lock().map_err(|e| RenderError::RenderFailed(e.to_string()))?;
        if auras.remove(aura_id).is_some() { Ok(()) } else { Err(RenderError::AuraNotFound(aura_id.to_string())) }
    }

    async fn update_aura(&self, aura_id: &str, slint_code: &str) -> Result<(), RenderError> {
        self.remove_aura(aura_id).await?;
        let aura = Aura { id: aura_id.to_string(), name: "Updated".into(), aura_type: waio_shared::entity::AuraType::Bar, slint_code: slint_code.to_string(), lua_code: None, config: Default::default() };
        self.render_aura(&aura).await
    }

    async fn shutdown(&self) -> Result<(), RenderError> {
        let mut auras = self.auras.lock().map_err(|e| RenderError::RenderFailed(e.to_string()))?;
        auras.clear();
        Ok(())
    }
}
```

**Файл**: `packages/daemon/src/infrastructure/slint/mod.rs`

```rust
pub mod interpreter;
pub mod platform;
pub use interpreter::*;
```

**Файл**: `packages/daemon/src/infrastructure/mod.rs`

```rust
pub mod repository;
pub mod wayland;
pub mod slint;
pub mod lua;
```

---

### Этап 4: Event Loop + app.rs ⚠️ КРИТИЧНО

**Файл**: `packages/daemon/src/app.rs`

```rust
use crate::usecase::aura::AuraUseCase;
use crate::usecase::render::RenderUseCase;
use crate::infrastructure::repository::FileAuraRepository;
use crate::infrastructure::wayland::WaylandConnection;
use crate::infrastructure::slint::SlintRenderer;
use crate::controller::cli::CliController;
use anyhow::Result;
use std::sync::Arc;
use tokio::signal;
use tracing::{info, error};
use tracing_subscriber::{fmt, EnvFilter};
use calloop::{EventLoop, LoopHandle};
use calloop_wayland_source::WaylandSource;

pub struct AppState {
    pub aura_use_case: AuraUseCase<FileAuraRepository>,
    pub render_use_case: RenderUseCase<SlintRenderer>,
    pub cli_controller: CliController,
}

impl AppState {
    pub fn new() -> Result<Self> {
        init_logging();
        if std::env::var("WAYLAND_DISPLAY").is_err() {
            anyhow::bail!("WAYLAND_DISPLAY not set. Try: WAYLAND_DISPLAY=wayland-0 cargo run -p waio-daemon");
        }
        let data_dir = get_data_dir();
        let repo = FileAuraRepository::new(data_dir);
        let connection = Arc::new(WaylandConnection::connect()?);
        let renderer = SlintRenderer::new(connection.clone());
        let aura_use_case = AuraUseCase::new(repo);
        let render_use_case = RenderUseCase::new(renderer);
        let cli_controller = CliController::new(aura_use_case.clone());
        Ok(Self { aura_use_case, render_use_case, cli_controller })
    }
}

pub async fn run() -> Result<()> {
    info!("Starting Waio Daemon...");
    
    let mut event_loop = EventLoop::try_new()?;
    let handle = event_loop.handle();
    let state = AppState::new()?;
    
    // ⭐ Теперь conn - публичное поле
    let conn = state.render_use_case.renderer.connection.clone();
    WaylandSource::new(conn.conn.clone(), conn.event_queue).insert_into_handle(handle)?;
    
    std::thread::spawn(move || {
        let _ = event_loop.run(None, &mut (), |_| {});
        Ok::<_, anyhow::Error>(())
    });
    
    state.render_use_case.init().await?;
    
    // ⭐ КРИТИЧНО: Создаём Ауру с Lua кодом для обновления времени
    let lua_code = r#"
        waio.timer.interval(1000, function()
            local time = waio.time.now()
            local date = waio.time.format("%Y-%m-%d")
            slint.set_property("time_text", time.str)
            slint.set_property("date_text", date)
        end)
        print("Clock aura started!")
    "#;
    
    let mut test_aura = state.aura_use_case
        .create_aura("test-bar".into(), waio_shared::entity::Aura::default().slint_code, waio_shared::entity::AuraType::Bar)
        .await?;
    
    // ⭐ Добавляем Lua код и сохраняем
    test_aura.lua_code = Some(lua_code.to_string());
    state.aura_use_case.repo.save(&test_aura).await?;
    
    info!("Created test aura: {}", test_aura.id);
    
    // ⭐ Рендерим с Lua кодом
    state.render_use_case.render_aura(&test_aura).await?;
    
    info!("Waio Daemon running. Press Ctrl+C to stop.");
    
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel(1);
    tokio::select! {
        _ = signal::ctrl_c() => { info!("Shutting down..."); let _ = shutdown_tx.send(()); }
        _ = shutdown_rx.recv() => {}
    }
    
    state.render_use_case.shutdown().await?;
    info!("Waio Daemon stopped");
    Ok(())
}

fn init_logging() {
    let _ = EnvFilter::try_from_default_env()
        .map(|f| fmt().with_env_filter(f).init())
        .or_else(|_| Ok(fmt().with_env_filter(EnvFilter::new("info,waio_daemon=debug")).init()));
}

fn get_data_dir() -> std::path::PathBuf {
    std::env::var("XDG_DATA_HOME").ok().map(std::path::PathBuf::from)
        .unwrap_or_else(|| dirs::data_local_dir().unwrap_or_else(|| std::path::PathBuf::from("~/.local/share")).join("waio"))
}
```

**Файл**: `packages/daemon/src/main.rs`

```rust
fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(async {
        if let Err(e) = app::run().await {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    });
}

mod app;
mod usecase;
mod infrastructure;
mod controller;
mod error;
```

**Файл**: `packages/daemon/src/controller/cli.rs`

```rust
use crate::usecase::aura::AuraUseCase;
use crate::infrastructure::repository::FileAuraRepository;
use anyhow::Result;

pub struct CliController { aura_use_case: AuraUseCase<FileAuraRepository> }

impl CliController {
    pub fn new(uc: AuraUseCase<FileAuraRepository>) -> Self { Self { aura_use_case: uc } }
    pub async fn create_bar(&self, name: String, slint_code: String) -> Result<String, String> {
        let aura = self.aura_use_case.create_aura(name, slint_code, waio_shared::entity::AuraType::Bar).await
            .map_err(|e| e.to_string())?;
        Ok(format!("Created bar: {}", aura.id))
    }
}
```

**Файл**: `packages/daemon/src/controller/mod.rs`

```rust
pub mod cli;
pub use cli::*;
```

**Файл**: `packages/daemon/Cargo.toml`

```toml
[package]
name = "waio-daemon"
version = "0.1.0"
edition = "2021"

[dependencies]
waio-shared = { path = "../shared" }
slint = { workspace = true }
slint-interpreter = { workspace = true }
mlua = { workspace = true }
chrono = { workspace = true }
smithay-client-toolkit = { workspace = true }
wayland-protocols-wlr = { workspace = true }
wayland-protocols = { workspace = true }
calloop = { workspace = true }
calloop-wayland-source = { workspace = true }
tokio = { workspace = true }
async-trait = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
anyhow = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
uuid = { workspace = true }
dirs = { workspace = true }
spin_on = { workspace = true }
tempfile = { workspace = true }
```

---

## Чеклист готовности

### Архитектура

- [ ] Cargo workspace настроен
- [ ] packages/shared: Entity слой
- [ ] packages/daemon: UseCase + Infrastructure + Controller
- [ ] Traits в UseCase, реализации в Infrastructure
- [ ] Dependency Injection через конструкторы
- [ ] Нет глобальных переменных

### Функциональность ⚠️ КРИТИЧНО

- [ ] Wayland подключение через smithay-client-toolkit
- [ ] layer-shell поверхность
- [ ] **SHM буфер с NamedTempFile**
- [ ] **sync_to_file() вызывается перед commit()**
- [ ] **conn: Connection - публичное поле**
- [ ] Slint компиляция из строки
- [ ] **render_by_line → копирование в Wayland буфер**
- [ ] Lua API: time.now(), time.format()
- [ ] Lua API: timer.interval() с callback.clone()
- [ ] Lua ↔ Slint bridge
- [ ] calloop + tokio интеграция
- [ ] **Тестовая Аура с Lua кодом (timer.interval)**
- [ ] Graceful shutdown

### Код-стайл

- [ ] Нет `unwrap()` в продакшен коде (кроме main.rs)
- [ ] thiserror для доменных ошибок
- [ ] tracing для логирования
- [ ] Async функции используют tokio

---

## Запуск и тестирование

```bash
# 1. Проверь WAYLAND_DISPLAY
echo $WAYLAND_DISPLAY

# 2. Проверь layer-shell
wayland-info | grep layer-shell

# 3. Запусти в Weston (безопасно)
weston &
WAYLAND_DISPLAY=wayland-1 cargo run -p waio-daemon
```

### Ожидаемые логи

```
INFO Connecting to Wayland...
INFO Connected to Wayland, layer-shell: true
INFO Starting Waio Daemon...
INFO SlintRenderer initialized
INFO Created test aura: <uuid>
INFO Rendering aura: test-bar
INFO Aura 'test-bar' rendered
Clock aura started!
INFO Waio Daemon running. Press Ctrl+C to stop.
```

### Пример Lua скрипта

```lua
waio.timer.interval(1000, function()
    local time = waio.time.now()
    local date = waio.time.format("%Y-%m-%d")
    slint.set_property("time_text", time.str)
    slint.set_property("date_text", date)
end)
print("Clock aura started!")
```

---

## Риски и митигация

| Риск | Вероятность | Влияние | Митигация |
| :--- | :--- | :--- | :--- |
| **sync_to_file не вызывается** | 🟡 Средняя | Критическое | **Вызов в commit()** ✅ |
| **conn приватное** | 🟡 Средняя | Критическое | **Публичное поле** ✅ |
| **Lua код не добавлен** | 🟡 Средняя | Высокое | **Добавить в app.rs** ✅ |
| Slint render_by_line | 🟡 Средняя | Критическое | Копировать построчно в SHM |
| calloop + tokio | 🟡 Средняя | Высокое | calloop в отдельном потоке |
| Разные композиторы | 🟡 Средняя | Среднее | Тестировать на Weston |

---

## Ресурсы

- [Slint SoftwareRenderer](https://docs.slint.dev/latest/docs/rust/slint/platform/struct.SoftwareRenderer.html)
- [Slint Interpreter](https://docs.slint.dev/latest/docs/rust/slint_interpreter/)
- [mlua::Function](https://docs.rs/mlua/latest/mlua/struct.Function.html)
- [smithay-client-toolkit](https://docs.rs/smithay-client-toolkit/latest/)
- [calloop-wayland-source](https://docs.rs/calloop-wayland-source/latest/)
- [Wayland Book](https://wayland-book.com/)

---

## Changelog v4.1

### Исправленные критические баги

| Баг | Было | Стало |
|-----|------|-------|
| **SHM синхронизация** | Нет sync_to_file | **Вызов в commit()** ✅ |
| **conn приватное** | `conn` приватное | **`pub conn`** ✅ |
| **Тестовая Аура** | `lua_code: None` | **С Lua кодом** ✅ |
| **Примеры Аур** | Нет файлов | **examples/clock-bar/** ✅ |

### 7 дней до MVP

| День | Задача | Результат |
|------|--------|-----------|
| 1 | Этап 0-2 | Workspace, Entity, UseCase ✅ |
| 2 | Этап 3.1-3.2 | Wayland + SHM + **sync_to_file** ✅ |
| 3 | Этап 3.4 | **Полоска с часами на экране** 🎉 |
| 4 | Этап 3.3 | Lua time + timer API ✅ |
| 5 | Этап 3.3 | Slint bridge + часы тикают 🎉 |
| 6 | Этап 3.5 | Event loop + configure ✅ |
| 7 | Примеры + тесты | **MVP готов** 🚀 |

---

*Версия: 4.1*
*Дата: 2026-04-01*

---
