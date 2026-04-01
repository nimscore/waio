# План реализации Waio Daemon

> **Цель**: Создать production-ready скелет демона на Rust с правильной архитектурой, который отрисовывает серую полоску вверху экрана (аналог waybar/polybar).

---

## Оглавление

1. [Контекст проекта](#контекст-проекта)
2. [Архитектурные принципы](#архитектурные-принципы)
3. [Технический стек](#технический-стек)
4. [Структура проекта](#структура-проекта)
5. [План реализации по этапам](#план-реализации-по-этапам)
6. [Чеклист готовности](#чеклист-готовности)

---

## Контекст проекта

### Видение

Waio — это **универсальная платформа виджетов** для Wayland-композиторов. Проект состоит из двух частей:

1. **Daemon** — демон на Rust, который работает как Wayland compositor (или layer-shell клиент) и отрисовывает UI-элементы («Ауры»)
2. **Client** — Tauri + React приложение для управления Аурами

### Текущая задача

Создать **минимальный скелет Daemon** на Rust, который:
- Следует чистой архитектуре (Entity → UseCase → Infrastructure → Controller)
- Использует Cargo workspace
- Отрисовывает простую серую полоску вверху экрана
- Готов к расширению функционалом

### Стратегическое решение

**Не писать полноценный Wayland compositor с нуля.** Это путь в 2-3 года разработки.

**Выбранный подход**: Использовать **wlr-layer-shell** протокол для создания панелей поверх существующего композитора (Hyprland, Sway, KWin). Это позволит:
- Работать на любых Wayland композиторах
- Сфокусироваться на бизнес-логике, а не на рендеринге окон
- Быстро получить MVP

---

## Архитектурные принципы

Следуем [Конституции чистой архитектуры](./rust-const.md):

```
┌─────────────────────────────────────────────┐
│              Controller                     │
│         (IPC Handler / CLI)                 │
└─────────────┬───────────────────────────────┘
              │
              ▼
┌─────────────────────────────────────────────┐
│              UseCase                        │
│         (Бизнес-логика)                     │
└─────────┬───────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────┐
│              Entity                         │
│         (Модели данных)                     │
└─────────────────────────────────────────────┘

Infrastructure (Wayland, Rendering) реализует traits из UseCase
```

### Ключевые правила

1. **Инверсия зависимостей** — внутренние слои НЕ зависят от внешних
2. **Traits в UseCase** — Infrastructure реализует traits, объявленные в UseCase
3. **Без глобальных переменных** — Dependency Injection через конструкторы
4. **Async safety** — tokio для всех I/O операций
5. **Error handling** — thiserror для доменных ошибок

---

## Технический стек

### Основные зависимости

```toml
# Wayland / Rendering
wayland-client = "0.31"          # Wayland client protocol
wayland-protocols = "0.32"       # Wayland protocol extensions
wayland-protocols-wlr = "0.3"    # wlr-layer-shell для панелей

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
bitflags = "2.2"
uuid = { version = "1.0", features = ["v4"] }
dirs = "5.0"
```

### Почему не Slint?

Для **первой версии** используем **рендеринг через Wayland протоколы напрямую**, потому что:
- Slint требует дополнительной настройки для Wayland
- Нужно понять базовый протокол перед добавлением абстракций
- Минимальная полоска не требует сложного UI

**В будущем**: Slint будет использоваться для сложных Аур (виджеты, анимации).

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
│   ├── shared/                   # 🟢 Общая библиотека (Entity)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── entity/
│   │       │   ├── mod.rs
│   │       │   ├── aura.rs       # Aura entity
│   │       │   └── error.rs      # Domain errors
│   │
│   └── daemon/                   # 🔴 Демон
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs           # ТОЛЬКО запуск
│           ├── app.rs            # Сборка зависимостей
│           ├── usecase/
│           │   ├── mod.rs
│           │   ├── aura.rs       # AuraUseCase + traits
│           │   └── render.rs     # RenderUseCase + traits
│           ├── infrastructure/
│           │   ├── mod.rs
│           │   ├── wayland/
│           │   │   ├── mod.rs
│           │   │   ├── connection.rs
│           │   │   └── layer_shell.rs
│           │   ├── renderer/
│           │   │   ├── mod.rs
│           │   │   └── simple_bar.rs
│           │   └── repository/
│           │       └── file_aura.rs
│           ├── controller/
│           │   ├── mod.rs
│           │   └── cli.rs
│           └── error.rs
│
└── scripts/
    └── test-deploy.sh
```

---

## План реализации по этапам

### Этап 0: Подготовка (1 день)
- [ ] Инициализация Cargo workspace
- [ ] Настройка .gitignore, README.md

### Этап 1: Скелет архитектуры (2 дня)
- [ ] packages/shared: Entity слой
- [ ] packages/daemon: UseCase слой (без реализации)
- [ ] packages/daemon: Infrastructure traits

### Этап 2: Wayland интеграция (3-4 дня)
- [ ] Подключение к Wayland сокету
- [ ] wlr-layer-shell инициализация
- [ ] Создание layer-shell поверхности

### Этап 3: Рендеринг полоски (2-3 дня)
- [ ] Базовый рендеринг через shm (shared memory)
- [ ] Отрисовка серой полоски вверху экрана
- [ ] Обработка событий Wayland

### Этап 4: Сборка и запуск (1 день)
- [ ] app.rs: Dependency Injection
- [ ] main.rs: Точка входа
- [ ] Graceful shutdown
- [ ] Тестирование на Hyprland/Sway

---

## Детальная реализация

### Этап 0: Инициализация workspace

**Файл**: `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = [
    "packages/shared",
    "packages/daemon",
]

[workspace.dependencies]
# Wayland
wayland-client = "0.31"
wayland-protocols = "0.32"
wayland-protocols-wlr = "0.3"

# Async
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
bitflags = "2.2"
uuid = { version = "1.0", features = ["v4"] }
dirs = "5.0"
```

---

### Этап 1: Entity слой (packages/shared)

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
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuraType {
    Bar,
    Widget,
    Overlay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aura {
    pub id: String,
    pub name: String,
    pub aura_type: AuraType,
    pub config: AuraConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraConfig {
    pub position: Position,
    pub size: Size,
    pub style: Style,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub background_color: String,
    pub border_radius: u32,
}

impl Aura {
    pub fn new(id: String, name: String, aura_type: AuraType) -> Self {
        Self {
            id,
            name,
            aura_type,
            config: AuraConfig::default(),
        }
    }

    pub fn validate(&self) -> Result<(), AuraError> {
        if self.name.is_empty() {
            return Err(AuraError::InvalidSyntax("Name is required".into()));
        }
        Ok(())
    }
}

impl Default for AuraConfig {
    fn default() -> Self {
        Self {
            position: Position { x: 0, y: 0 },
            size: Size {
                width: 1920,
                height: 30,
            },
            style: Style {
                background_color: "#808080".into(),
                border_radius: 0,
            },
        }
    }
}
```

**Файл**: `packages/shared/src/entity/mod.rs`

```rust
mod aura;
pub use aura::*;
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

pub struct AuraUseCase<R: AuraRepository> {
    repo: R,
}

impl<R: AuraRepository> AuraUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_aura(
        &self,
        name: String,
        aura_type: waio_shared::entity::AuraType,
    ) -> Result<Aura, AuraError> {
        let id = uuid::Uuid::new_v4().to_string();
        let mut aura = Aura::new(id, name, aura_type);
        aura.validate()?;
        self.repo.save(&aura).await?;
        Ok(aura)
    }

    pub async fn get_aura(&self, id: &str) -> Result<Aura, AuraError> {
        self.repo.get(id).await
    }

    pub async fn list_auras(&self) -> Result<Vec<Aura>, AuraError> {
        self.repo.list().await
    }

    pub async fn delete_aura(&self, id: &str) -> Result<(), AuraError> {
        self.repo.delete(id).await
    }
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
    async fn shutdown(&self) -> Result<(), RenderError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Wayland connection failed: {0}")]
    WaylandError(String),
    #[error("Rendering failed: {0}")]
    RenderFailed(String),
}

pub struct RenderUseCase<R: Renderer> {
    renderer: R,
}

impl<R: Renderer> RenderUseCase<R> {
    pub fn new(renderer: R) -> Self {
        Self { renderer }
    }

    pub async fn init(&self) -> Result<(), RenderError> {
        self.renderer.init().await
    }

    pub async fn render_aura(&self, aura: &Aura) -> Result<(), RenderError> {
        self.renderer.render_aura(aura).await
    }

    pub async fn shutdown(&self) -> Result<(), RenderError> {
        self.renderer.shutdown().await
    }
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

**Файл**: `packages/daemon/src/infrastructure/repository/file_aura.rs`

```rust
use crate::usecase::aura::AuraRepository;
use waio_shared::entity::{Aura, AuraError};
use std::path::PathBuf;
use tokio::fs;

pub struct FileAuraRepository {
    base_path: PathBuf,
}

impl FileAuraRepository {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    fn aura_path(&self, id: &str) -> PathBuf {
        self.base_path.join(format!("{}.json", id))
    }
}

#[async_trait::async_trait]
impl AuraRepository for FileAuraRepository {
    async fn get(&self, id: &str) -> Result<Aura, AuraError> {
        let path = self.aura_path(id);
        let content = fs::read_to_string(path)
            .await
            .map_err(|e| AuraError::NotFound(e.to_string()))?;
        serde_json::from_str(&content)
            .map_err(|e| AuraError::InvalidSyntax(e.to_string()))
    }

    async fn save(&self, aura: &Aura) -> Result<(), AuraError> {
        let path = self.aura_path(&aura.id);
        let content = serde_json::to_string(aura)
            .map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
        fs::write(path, content)
            .await
            .map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), AuraError> {
        let path = self.aura_path(id);
        fs::remove_file(path)
            .await
            .map_err(|e| AuraError::NotFound(e.to_string()))?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Aura>, AuraError> {
        let mut auras = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.base_path)
            .await
            .map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path).await {
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

**Файл**: `packages/daemon/src/infrastructure/wayland/connection.rs`

```rust
use wayland_client::Connection;
use tracing::info;
use anyhow::Result;

pub struct WaylandConnection {
    conn: Connection,
}

impl WaylandConnection {
    pub fn connect() -> Result<Self> {
        info!("Connecting to Wayland display...");
        let conn = Connection::connect_to_env()
            .map_err(|e| anyhow::anyhow!("Failed to connect to Wayland: {}", e))?;
        info!("Connected to Wayland display");
        Self { conn }
    }

    pub fn display(&self) -> wayland_client::protocol::wl_display::WlDisplay {
        self.conn.display()
    }
}
```

**Файл**: `packages/daemon/src/infrastructure/wayland/layer_shell.rs`

```rust
use wayland_client::{
    protocol::{wl_compositor, wl_output, wl_registry, wl_shm},
    Connection, Dispatch, EventQueue,
};
use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_shell_v1;
use tracing::info;

pub struct LayerShellState {
    pub compositor: Option<wl_compositor::WlCompositor>,
    pub shm: Option<wl_shm::WlShm>,
    pub layer_shell: Option<zwlr_layer_shell_v1::ZwlrLayerShellV1>,
    pub outputs: Vec<wl_output::WlOutput>,
}

impl LayerShellState {
    pub fn new() -> Self {
        Self {
            compositor: None,
            shm: None,
            layer_shell: None,
            outputs: Vec::new(),
        }
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for LayerShellState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        _: &mut EventQueue<LayerShellState>,
    ) {
        match event {
            wl_registry::Event::Global {
                name, interface, version, ..
            } => {
                info!("Global: {} (v{})", interface, version);
                match interface.as_str() {
                    "wl_compositor" => {
                        state.compositor = Some(registry.bind(name, 4, &()));
                        info!("Bound wl_compositor");
                    }
                    "wl_shm" => {
                        state.shm = Some(registry.bind(name, 1, &()));
                        info!("Bound wl_shm");
                    }
                    "zwlr_layer_shell_v1" => {
                        state.layer_shell = Some(registry.bind(name, 1, &()));
                        info!("Bound zwlr_layer_shell_v1");
                    }
                    "wl_output" => {
                        state.outputs.push(registry.bind(name, 4, &()));
                        info!("Bound wl_output");
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

// Пустые реализации для остальных типов
macro_rules! impl_dispatch_empty {
    ($($t:ty),*) => {
        $(
            impl Dispatch<$t, ()> for LayerShellState {
                fn event(
                    _: &mut Self,
                    _: &$t,
                    _: <$t as wayland_client::Proxy>::Event,
                    _: &(),
                    _: &Connection,
                    _: &mut EventQueue<LayerShellState>,
                ) {}
            }
        )*
    };
}

impl_dispatch_empty!(
    wl_compositor::WlCompositor,
    wl_shm::WlShm,
    zwlr_layer_shell_v1::ZwlrLayerShellV1,
    wl_output::WlOutput
);
```

**Файл**: `packages/daemon/src/infrastructure/wayland/mod.rs`

```rust
pub mod connection;
pub mod layer_shell;
pub use connection::*;
pub use layer_shell::*;
```

---

**Файл**: `packages/daemon/src/infrastructure/renderer/simple_bar.rs`

```rust
use crate::usecase::render::{RenderError, Renderer};
use crate::infrastructure::wayland::WaylandConnection;
use waio_shared::entity::Aura;
use tracing::info;
use std::sync::Arc;

pub struct SimpleBarRenderer {
    connection: Arc<WaylandConnection>,
}

impl SimpleBarRenderer {
    pub fn new(connection: Arc<WaylandConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl Renderer for SimpleBarRenderer {
    async fn init(&self) -> Result<(), RenderError> {
        info!("Initializing SimpleBarRenderer...");
        info!("SimpleBarRenderer initialized");
        Ok(())
    }

    async fn render_aura(&self, aura: &Aura) -> Result<(), RenderError> {
        info!("Rendering aura: {}", aura.name);
        info!("Aura config: {:?}", aura.config);
        // TODO: Реализовать полноценный рендеринг
        Ok(())
    }

    async fn remove_aura(&self, aura_id: &str) -> Result<(), RenderError> {
        info!("Removing aura: {}", aura_id);
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), RenderError> {
        info!("Shutting down SimpleBarRenderer...");
        Ok(())
    }
}
```

**Файл**: `packages/daemon/src/infrastructure/mod.rs`

```rust
pub mod repository;
pub mod wayland;
pub mod renderer;
```

---

### Этап 4: Controller и app.rs

**Файл**: `packages/daemon/src/controller/cli.rs`

```rust
use crate::usecase::aura::AuraUseCase;
use crate::infrastructure::repository::FileAuraRepository;
use anyhow::Result;

pub struct CliController {
    aura_use_case: AuraUseCase<FileAuraRepository>,
}

impl CliController {
    pub fn new(uc: AuraUseCase<FileAuraRepository>) -> Self {
        Self { aura_use_case: uc }
    }

    pub async fn create_bar(&self, name: String) -> Result<String, String> {
        let aura = self
            .aura_use_case
            .create_aura(name, waio_shared::entity::AuraType::Bar)
            .await
            .map_err(|e| e.to_string())?;
        Ok(format!("Created bar with ID: {}", aura.id))
    }
}
```

**Файл**: `packages/daemon/src/controller/mod.rs`

```rust
pub mod cli;
pub use cli::*;
```

---

**Файл**: `packages/daemon/src/app.rs`

```rust
use crate::usecase::aura::AuraUseCase;
use crate::usecase::render::RenderUseCase;
use crate::infrastructure::repository::FileAuraRepository;
use crate::infrastructure::wayland::WaylandConnection;
use crate::infrastructure::renderer::SimpleBarRenderer;
use crate::controller::cli::CliController;

use anyhow::Result;
use std::sync::Arc;
use tokio::signal;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

pub struct AppState {
    pub aura_use_case: AuraUseCase<FileAuraRepository>,
    pub render_use_case: RenderUseCase<SimpleBarRenderer>,
    pub cli_controller: CliController,
}

impl AppState {
    pub fn new() -> Result<Self> {
        init_logging();
        
        let data_dir = get_data_dir();
        let repo = FileAuraRepository::new(data_dir);
        
        let connection = Arc::new(WaylandConnection::connect()?);
        let renderer = SimpleBarRenderer::new(connection);
        
        let aura_use_case = AuraUseCase::new(repo);
        let render_use_case = RenderUseCase::new(renderer);
        let cli_controller = CliController::new(aura_use_case.clone());

        Ok(Self {
            aura_use_case,
            render_use_case,
            cli_controller,
        })
    }
}

pub async fn run() -> Result<()> {
    info!("Starting Waio Daemon...");
    
    let state = AppState::new()?;
    state.render_use_case.init().await?;

    let test_aura = state
        .aura_use_case
        .create_aura("test-bar".into(), waio_shared::entity::AuraType::Bar)
        .await?;

    info!("Created test aura: {}", test_aura.id);
    state.render_use_case.render_aura(&test_aura).await?;

    info!("Waio Daemon is running. Press Ctrl+C to stop.");

    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel(1);

    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Starting shutdown...");
            let _ = shutdown_tx.send(());
        }
        _ = shutdown_rx.recv() => {}
    }

    state.render_use_case.shutdown().await?;
    info!("Waio Daemon stopped");
    Ok(())
}

fn init_logging() {
    if let Ok(env_filter) = EnvFilter::try_from_default_env() {
        fmt().with_env_filter(env_filter).init();
    } else {
        fmt().init();
    }
}

fn get_data_dir() -> std::path::PathBuf {
    std::env::var("XDG_DATA_HOME")
        .ok()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            dirs::data_local_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("~/.local/share"))
                .join("waio")
        })
}
```

---

**Файл**: `packages/daemon/src/main.rs`

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
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

---

**Файл**: `packages/daemon/Cargo.toml`

```toml
[package]
name = "waio-daemon"
version = "0.1.0"
edition = "2021"

[dependencies]
waio-shared = { path = "../shared" }

# Wayland
wayland-client = { workspace = true }
wayland-protocols = { workspace = true }
wayland-protocols-wlr = { workspace = true }

# Async
tokio = { workspace = true }
async-trait = { workspace = true }

# Serialization
serde = { workspace = true }
serde_json = { workspace = true }

# Error handling
thiserror = { workspace = true }
anyhow = { workspace = true }

# Logging
tracing = { workspace = true }
tracing-subscriber = { workspace = true }

# Utilities
bitflags = { workspace = true }
uuid = { workspace = true }
dirs = { workspace = true }
```

---

## Чеклист готовности

### Архитектура

- [ ] Cargo workspace настроен
- [ ] packages/shared содержит Entity слой
- [ ] packages/daemon содержит UseCase, Infrastructure, Controller
- [ ] Traits объявлены в UseCase, реализации в Infrastructure
- [ ] Dependency Injection через конструкторы
- [ ] Нет глобальных переменных

### Функциональность

- [ ] Подключение к Wayland
- [ ] wlr-layer-shell инициализация
- [ ] Создание layer-shell поверхности
- [ ] Отрисовка серой полоски
- [ ] Graceful shutdown

### Код-стайл

- [ ] Нет `unwrap()` в продакшен коде
- [ ] thiserror для доменных ошибок
- [ ] tracing для логирования
- [ ] Async функции используют tokio
- [ ] Тесты для UseCase и Entity

---

## Следующие шаги после MVP

1. **Полноценный рендеринг** — реализовать shm buffer и отрисовку
2. **События Wayland** — обработка resize, output events
3. **IPC для клиента** — добавить Tauri IPC handler
4. **Конфигурация** — загрузка из файла
5. **Сложные Ауры** — виджеты, текст, иконки

---

## Ресурсы

- [Smithay Getting Started](https://github.com/Smithay/smithay/blob/master/GETTING_STARTED.md)
- [smallvil example](https://github.com/Smithay/smithay/tree/master/smallvil)
- [wayland-rs documentation](https://smithay.github.io/wayland-rs/)
- [Wayland Book](https://wayland-book.com/)
- [wlr-layer-shell protocol](https://gitlab.freedesktop.org/wlroots/wlr-protocols/-/blob/master/protocol/wlr-layer-shell-unstable-v1.xml)

---

*Версия: 1.0*
*Дата: 2026-04-01*
