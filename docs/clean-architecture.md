# Конституция чистой архитектуры для Waio на Rust

## Оглавление

1. [Фундаментальные принципы](#фундаментальные-принципы)
2. [Архитектурные слои](#архитектурные-слои)
3. [Структура проекта](#структура-проекта)
4. [Инверсия зависимостей](#инверсия-зависимостей)
5. [Dependency Injection](#dependency-injection)
6. [Правила организации кода](#правила-организации-кода)
7. [Работа с фреймворками](#работа-с-фреймворками)
8. [Тестирование](#тестирование)
9. [Обязательные требования](#обязательные-требования)

---

## Фундаментальные принципы

### Цели чистой архитектуры

Чистая архитектура решает следующие задачи:

- **Гибкость** - возможность менять UI, систему рендеринга или источники данных без переписывания ядра
- **Модульность** - минимальная связанность между демоном, клиентом и общими библиотеками
- **Тестируемость** - легкость написания unit-тестов без запуска Wayland-сессии
- **Стандартизация** - единообразие подходов в команде
- **Поддерживаемость** - долгосрочная поддержка кодовой базы

### Главный принцип

**Инверсия зависимостей** - внутренние слои НЕ зависят от внешних слоёв.

```
Внутренние слои (Entity, UseCase) НЕ зависят от внешних слоёв (Controller, Infrastructure)
```

### Rust-специфика

| Аспект | Реализация в Rust |
|--------|-------------------|
| Интерфейсы | `trait` |
| Ошибки | `Result<T, E>` |
| Память | Ownership/Borrowing |
| Зависимости | Cargo workspace |
| Асинхронность | Tokio/async-std |

---

## Архитектурные слои

### Схема архитектуры

```
┌─────────────────────────────────────────────┐
│              Controller                     │
│         (Tauri Frontend / IPC Handler)      │
└─────────────┬───────────────────────────────┘
              │ ↓ запрос
              │ ↑ ответ
              ▼
┌─────────────────────────────────────────────┐
│              UseCase                   ◄────┼──── Entity
│         (Бизнес-логика)                ─────┼────►
└─────────┬──────────┬──────────┬─────────────┘
     ↓ вызов   ↓ вызов  ↓ вызов
     ↑ данные  ↑ данные ↑ данные
          │          │          │
          ▼          ▼          ▼
    ┌─────────┐ ┌──────────┐ ┌────────────┐
    │  Slint  │ │  System  │ │   Wayland  │
    │   UI    │ │ Providers│ │   Client   │
    └─────────┘ └──────────┘ └────────────┘
       (Infrastructure - абстракции)
```

### 1. Entity (Сущности)

**Расположение**: `packages/shared/src/entity/`

**Назначение**:
- Содержат бизнес-логику приложения
- Полностью независимы от других слоёв
- Переносимы между демоном и клиентом

**Что находится**:
- Структуры данных (модели)
- Бизнес-правила
- Валидация бизнес-данных
- Ошибки домена

**Запрещено**:
- Импортировать пакеты из других слоёв
- Зависеть от фреймворков (кроме `serde` для сериализации)
- Содержать логику работы с Wayland, Slint, файлами

**Пример**:
```rust
// packages/shared/src/entity/aura.rs
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuraError {
    #[error("Invalid aura syntax: {0}")]
    InvalidSyntax(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aura {
    pub id: String,
    pub name: String,
    pub code: String,
    pub permissions: Vec<String>,
}

impl Aura {
    pub fn validate(&self) -> Result<(), AuraError> {
        if self.name.is_empty() {
            return Err(AuraError::InvalidSyntax("Name is required".into()));
        }
        if self.code.is_empty() {
            return Err(AuraError::InvalidSyntax("Code is required".into()));
        }
        Ok(())
    }
}
```

### 2. UseCase (Варианты использования)

**Расположение**: `packages/daemon/src/usecase/` или `packages/client/src/usecase/`

**Назначение**:
- Конкретные варианты использования бизнес-логики
- Оркестрация работы Entity
- Вызов абстракций Infrastructure через traits

**Что находится**:
- Структуры UseCase
- Методы, реализующие бизнес-сценарии
- Traits для работы с Infrastructure
- Конструкторы для DI

**Зависимости**:
- ✅ Зависит от Entity
- ❌ НЕ зависит от Controller
- ❌ НЕ зависит от конкретных реализаций Infrastructure
- ✅ Работает только через traits

**Правила**:
- UseCase должны быть короткими и простыми
- Сложная логика должна быть в Entity
- UseCase только создаёт объекты Entity и определяет порядок действий

**Пример**:
```rust
// packages/daemon/src/usecase/aura.rs
use crate::entity::{Aura, AuraError};

// Trait объявляется в UseCase (инверсия зависимостей!)
#[async_trait::async_trait]
pub trait AuraRepository: Send + Sync {
    async fn get(&self, id: &str) -> Result<Aura, AuraError>;
    async fn save(&self, aura: &Aura) -> Result<(), AuraError>;
    async fn delete(&self, id: &str) -> Result<(), AuraError>;
}

pub struct AuraUseCase<R: AuraRepository> {
    repo: R,
}

impl<R: AuraRepository> AuraUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn import_aura(&self, code: &str) -> Result<Aura, AuraError> {
        let aura = Aura::from_import_string(code)?;
        aura.validate()?;
        self.repo.save(&aura).await?;
        Ok(aura);
    }
}
```

### 3. Infrastructure (Инфраструктура)

**Расположение**: `packages/daemon/src/infrastructure/`

**Назначение**:
- Реализация traits, определённых в UseCase
- Работа с внешними системами (Wayland, Slint, ФС, сеть)

**Что находится**:
- Repository (работа с файлами/БД)
- WaylandClient (подключение к композитору)
- SlintRenderer (отрисовка UI)
- SystemProviders (CPU, RAM, сеть)

**Важно**:
- Traits объявляются в UseCase, реализации в Infrastructure
- Каждая реализация в своём подмодуле

**Пример**:
```rust
// packages/daemon/src/infrastructure/repository/file_aura.rs
use crate::usecase::aura::AuraRepository;
use crate::entity::{Aura, AuraError};
use std::path::PathBuf;
use tokio::fs;

pub struct FileAuraRepository {
    base_path: PathBuf,
}

impl FileAuraRepository {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
}

#[async_trait::async_trait]
impl AuraRepository for FileAuraRepository {
    async fn get(&self, id: &str) -> Result<Aura, AuraError> {
        let path = self.base_path.join(format!("{}.json", id));
        let content = fs::read_to_string(path)
            .await
            .map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
        serde_json::from_str(&content)
            .map_err(|e| AuraError::InvalidSyntax(e.to_string()))
    }

    async fn save(&self, aura: &Aura) -> Result<(), AuraError> {
        let path = self.base_path.join(format!("{}.json", aura.id));
        let content = serde_json::to_string(aura)
            .map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
        fs::write(path, content)
            .await
            .map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), AuraError> {
        let path = self.base_path.join(format!("{}.json", id));
        fs::remove_file(path)
            .await
            .map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
        Ok(())
    }
}
```

### 4. Controller (Контроллеры)

**Расположение**: `packages/daemon/src/controller/` и `packages/client/src-tauri/src/`

**Назначение**:
- Точки входа в приложение (IPC, HTTP, CLI)
- Получение событий извне
- Передача данных в UseCase
- Возврат результата

**Что находится**:
- IPC обработчики (Tauri ↔ Daemon)
- Wayland event handlers
- CLI команды

**Правила**:
- Controller НЕ содержит бизнес-логику
- Только сериализация/десериализация данных
- Вызов UseCase через traits
- Обработка ошибок и формирование ответа

**Пример**:
```rust
// packages/client/src-tauri/src/ipc/aura_handler.rs
use tauri::State;
use crate::usecase::aura::AuraUseCase;
use crate::infrastructure::repository::FileAuraRepository;

pub struct AuraController {
    use_case: AuraUseCase<FileAuraRepository>,
}

impl AuraController {
    pub fn new(uc: AuraUseCase<FileAuraRepository>) -> Self {
        Self { use_case: uc }
    }

    #[tauri::command]
    pub async fn import_aura(&self, code: String) -> Result<String, String> {
        let aura = self.use_case
            .import_aura(&code)
            .await
            .map_err(|e| e.to_string())?;
        Ok(aura.id)
    }
}
```

---

## Структура проекта

### Monorepo структура

```
waio/
├── Cargo.toml                    # Workspace definition
├── .gitignore
├── README.md
├── LICENSE
│
├── packages/
│   ├── shared/                   # 🟢 Общая библиотека
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── entity/
│   │       │   ├── mod.rs
│   │       │   ├── aura.rs
│   │       │   └── error.rs
│   │       └── protocol/         # IPC типы
│   │           ├── mod.rs
│   │           └── messages.rs
│   │
│   ├── daemon/                   # 🔴 Демон (Rust + Slint)
│   │   ├── Cargo.toml
│   │   ├── build.rs              # Slint компиляция
│   │   └── src/
│   │       ├── main.rs
│   │       ├── app.rs            # Сборка зависимостей
│   │       ├── usecase/
│   │       ├── infrastructure/
│   │       ├── controller/
│   │       └── aura/             # Slint interpreter
│   │
│   └── client/                   # 🔵 Клиент (Tauri + React)
│       ├── package.json
│       ├── Cargo.toml
│       └── src-tauri/
│           ├── main.rs
│           ├── ipc/
│           └── src/              # React код
│
├── website/                      # 🌐 Сайт-каталог
│   └── ...
│
└── scripts/                      # Утилиты сборки
```

### Главный Cargo.toml (Workspace)

```toml
[workspace]
resolver = "2"
members = [
    "packages/shared",
    "packages/daemon",
    "packages/client",
]

[workspace.dependencies]
# Общие зависимости для синхронизации версий
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
slint = "1.5"
mlua = { version = "0.9", features = ["lua54"] }
tauri = "1.5"
```

### Использование workspace зависимостей

```toml
# packages/daemon/Cargo.toml
[package]
name = "waio-daemon"
version = "0.1.0"

[dependencies]
waio-shared = { path = "../shared" }
serde = { workspace = true }
tokio = { workspace = true }
slint = { workspace = true }
```

---

## Инверсия зависимостей

### Правило зависимостей

```
Зависимости направлены ТОЛЬКО внутрь:
Controller → UseCase → Entity
Infrastructure → UseCase → Entity
```

### Rust-специфика: Traits вместо интерфейсов

```rust
// ✅ ПРАВИЛЬНО - Trait в UseCase
// packages/daemon/src/usecase/aura.rs
pub trait AuraRepository: Send + Sync {
    async fn get(&self, id: &str) -> Result<Aura, AuraError>;
}

pub struct AuraUseCase<R: AuraRepository> {
    repo: R,
}

// ✅ ПРАВИЛЬНО - Реализация в Infrastructure
// packages/daemon/src/infrastructure/repository/file_aura.rs
impl AuraRepository for FileAuraRepository {
    async fn get(&self, id: &str) -> Result<Aura, AuraError> {
        // ...
    }
}
```

### Поток управления vs Поток зависимостей

**Поток управления** (как идёт запрос):
```
Controller → UseCase → Infrastructure → UseCase → Controller
```

**Поток зависимостей** (кто от кого зависит):
```
Infrastructure ⇢ UseCase ⇠ Controller
        ↓
     Entity
```

---

## Dependency Injection

### Принципы DI в Rust

1. **Конструктор `new`** - единственный способ создания объектов
2. **Инъекция через traits** - передаём traits, а не конкретные типы
3. **Generics для статического DI** - компиляция проверяет совместимость
4. **Без глобальных переменных** - всё через `Arc<Mutex<>>` или передачу владения

### Пример полного DI

```rust
// 1. UseCase определяет что ему нужно (trait)
#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    async fn get(&self) -> Result<Aura, AuraError>;
}

pub struct UseCase<R: Repository> {
    repo: R,
}

impl<R: Repository> UseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn do_something(&self) -> Result<(), AuraError> {
        let aura = self.repo.get().await?;
        // ...
        Ok(())
    }
}
```

```rust
// 2. Controller определяет что ему нужно
pub struct Controller<R: Repository> {
    use_case: UseCase<R>,
}

impl<R: Repository> Controller<R> {
    pub fn new(uc: UseCase<R>) -> Self {
        Self { use_case: uc }
    }

    #[tauri::command]
    pub async fn handle(&self) -> Result<String, String> {
        self.use_case.do_something().await
            .map(|_| "OK".into())
            .map_err(|e| e.to_string())
    }
}
```

```rust
// 3. app.rs собирает всё вместе
// packages/daemon/src/app.rs
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Конфигурация
    let config = load_config()?;
    
    // 2. Инициализация Infrastructure
    let repo = FileAuraRepository::new(config.data_path);
    
    // 3. Создание UseCase
    let use_case = AuraUseCase::new(repo);
    
    // 4. Создание Controller
    let controller = AuraController::new(use_case);
    
    // 5. Запуск
    start_server(controller).await?;
    
    Ok(())
}
```

### Преимущества такого подхода

- ✅ UseCase можно передать **любую** реализацию репозитория (ФС, БД, mock)
- ✅ Код UseCase **не меняется** при смене реализации
- ✅ Легко писать тесты через mock-объекты
- ✅ Компилятор проверяет совместимость типов

### Mockery для тестов

```rust
// packages/daemon/src/usecase/aura_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::automock;

    #[automock]
    #[async_trait::async_trait]
    pub trait AuraRepository: Send + Sync {
        async fn get(&self, id: &str) -> Result<Aura, AuraError>;
        async fn save(&self, aura: &Aura) -> Result<(), AuraError>;
    }

    #[tokio::test]
    async fn test_import_aura_success() {
        let mut mock_repo = MockAuraRepository::new();
        mock_repo
            .expect_save()
            .times(1)
            .returning(|_| Ok(()));

        let use_case = AuraUseCase::new(mock_repo);
        let result = use_case.import_aura("valid_code").await;

        assert!(result.is_ok());
    }
}
```

**Рекомендация**: Использовать [`mockall`](https://crates.io/crates/mockall) для генерации моков.

---

## Правила организации кода

### 1. Никаких импортов между слоями

**Запрещено**:
```rust
// ❌ НЕПРАВИЛЬНО
// packages/shared/src/entity/aura.rs
use crate::usecase::aura::AuraRepository;  // ❌ Entity не может импортировать UseCase
use crate::infrastructure::wayland;        // ❌ Entity не может импортировать Infrastructure
```

**Правильно**:
```rust
// ✅ ПРАВИЛЬНО
// packages/shared/src/entity/aura.rs
use serde::{Serialize, Deserialize};  // ✅ Только внешние крейты
use thiserror::Error;

// ✅ ПРАВИЛЬНО
// packages/daemon/src/usecase/aura.rs
use waio_shared::entity::{Aura, AuraError};  // ✅ UseCase может импортировать ТОЛЬКО Entity
```

### 2. Traits там, где используются

**Правило**: Trait должен быть объявлен в том модуле, который его использует.

```rust
// ✅ ПРАВИЛЬНО
// packages/daemon/src/usecase/aura.rs
pub trait AuraRepository {
    async fn get(&self, id: &str) -> Result<Aura, AuraError>;
}
```

```rust
// ❌ НЕПРАВИЛЬНО - отдельный модуль traits
// packages/daemon/src/traits/repository.rs
pub trait AuraRepository { }  // ❌ Не создавать отдельный модуль
```

### 3. Ownership и Borrowing между слоями

**Правило**: Избегай клонирования больших структур. Используй ссылки и `Arc`.

```rust
// ✅ ПРАВИЛЬНО - передача по ссылке
pub async fn process_aura(&self, aura: &Aura) -> Result<(), AuraError> {
    // Читаем данные, не забираем владение
}

// ✅ ПРАВИЛЬНО - для shared state
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    repo: Arc<Mutex<FileAuraRepository>>,
}

// ❌ НЕПРАВИЛЬНО - лишнее клонирование
pub async fn process_aura(&self, aura: Aura) -> Result<(), AuraError> {
    // Забираем владение, вынуждая caller клонировать
}
```

### 4. Глобальные переменные запрещены

**Запрещено**:
```rust
// ❌ НЕПРАВИЛЬНО
static mut DB: Option<Database> = None;

fn init() {
    DB = Some(connect_db());
}
```

**Правильно**:
```rust
// ✅ ПРАВИЛЬНО
pub struct AppState {
    db: Arc<Database>,
    logger: Arc<Logger>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db: Arc::new(connect_db()),
            logger: Arc::new(new_logger()),
        }
    }
}

// Инъекция через конструктор
pub fn new_use_case(state: Arc<AppState>) -> UseCase {
    UseCase { state }
}
```

### 5. Graceful Shutdown

**Обязательно** реализовывать graceful shutdown в `app.rs`:

```rust
// packages/daemon/src/app.rs
use tokio::signal;
use tokio::sync::broadcast;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // ... инициализация
    
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
    
    // Запуск основных задач
    let daemon_handle = tokio::spawn(async move {
        // ...
    });
    
    // Ожидание сигнала
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Starting shutdown...");
            let _ = shutdown_tx.send(());
        }
        _ = &mut daemon_handle => {}
    }
    
    // Корректное завершение
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    
    Ok(())
}
```

---

## Работа с фреймворками

### Основное правило

**Фреймворки НЕ должны просачиваться в бизнес-логику (UseCase, Entity)**

### Допустимые места для фреймворков

1. **Controller** - полностью на фреймворке
   ```rust
   // ✅ Можно
   #[tauri::command]
   async fn handle(state: State<MyState>) { }
   ```

2. **Infrastructure** - частично на фреймворке
   ```rust
   // ✅ Можно
   pub struct SlintRenderer {
       handle: slint::ComponentHandle<MyComponent>,
   }
   ```

3. **app.rs** - инициализация фреймворков
   ```rust
   // ✅ Можно
   slint::ComponentHandle::new().unwrap();
   ```

### Запрещённые места для фреймворков

1. **UseCase** - только стандартная библиотека + shared
   ```rust
   // ❌ ЗАПРЕЩЕНО
   pub struct UseCase {
       slint_handle: slint::ComponentHandle,  // ❌ Нет фреймворков в UseCase
   }
   ```

2. **Entity** - исключение только `serde` теги
   ```rust
   // ✅ Допустимо (теги для сериализации)
   #[derive(Serialize, Deserialize)]
   pub struct Aura {
       pub id: String,
   }
   ```

### Slint-специфика

Slint имеет свою систему типов и макросов. Держи Slint-код изолированным:

```rust
// ✅ ПРАВИЛЬНО - Slint в отдельном модуле
// packages/daemon/src/aura/renderer.rs
slint::slint!{
    export component Widget {
        // ...
    }
}

pub struct AuraRenderer {
    // Slint типы только здесь
}
```

---

## Тестирование

### Принципы тестирования

1. **UseCase** - легко тестируется через моки traits
2. **Entity** - тестируется напрямую
3. **Controller** - тестируется через IPC-тесты (опционально)
4. **Infrastructure** - интеграционные тесты с реальными системами

### Пример теста UseCase

```rust
// packages/daemon/src/usecase/aura_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use waio_shared::entity::{Aura, AuraError};

    #[tokio::test]
    async fn test_import_aura_success() {
        let mut mock_repo = MockAuraRepository::new();
        let expected = Aura {
            id: "test".into(),
            name: "Test".into(),
            code: "test".into(),
            permissions: vec![],
        };
        
        mock_repo
            .expect_save()
            .withf(|a| a.name == "Test")
            .times(1)
            .returning(|_| Ok(()));

        let use_case = AuraUseCase::new(mock_repo);
        let result = use_case.import_aura("test_code").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_import_aura_validation_error() {
        let mock_repo = MockAuraRepository::new();
        let use_case = AuraUseCase::new(mock_repo);
        
        let result = use_case.import_aura("").await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AuraError::InvalidSyntax(_)));
    }
}
```

### Пример теста Entity

```rust
// packages/shared/src/entity/aura_test.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_validate_success() {
        let aura = Aura {
            id: "1".into(),
            name: "Test".into(),
            code: "export component {}".into(),
            permissions: vec![],
        };

        assert!(aura.validate().is_ok());
    }

    #[test]
    fn test_aura_validate_empty_name() {
        let aura = Aura {
            id: "1".into(),
            name: "".into(),
            code: "test".into(),
            permissions: vec![],
        };

        assert!(aura.validate().is_err());
    }
}
```

### Покрытие тестами

**Цель**: Минимум 80% покрытия UseCase и Entity

```bash
# Запуск тестов с покрытием
cargo test --workspace -- --test-threads=1

# Генерация отчёта (требует cargo-tarpaulin)
cargo tarpaulin --workspace --out Html
```

---

## Обязательные требования

### Правило 1: Направление зависимостей

```
✅ РАЗРЕШЕНО:
- UseCase → Entity (waio-shared)
- Controller → UseCase
- Infrastructure → UseCase (для реализации traits)

❌ ЗАПРЕЩЕНО:
- Entity → UseCase
- Entity → Infrastructure
- Entity → Controller
- UseCase → Controller
- UseCase → Infrastructure (только через traits!)
```

### Правило 2: Импорты пакетов

```rust
// ✅ РАЗРЕШЁННЫЕ ИМПОРТЫ

// Entity - НИЧЕГО не импортирует из internal (только внешние крейты)
// packages/shared/src/entity/aura.rs
use serde::{Serialize, Deserialize};  // ✅
use thiserror::Error;                  // ✅

// UseCase - импортирует только Entity
// packages/daemon/src/usecase/aura.rs
use waio_shared::entity::{Aura, AuraError};  // ✅

// Infrastructure - реализует traits из UseCase
// packages/daemon/src/infrastructure/repository/file_aura.rs
use crate::usecase::aura::AuraRepository;  // ✅ (для реализации trait)

// Controller - импортирует UseCase
// packages/client/src-tauri/src/ipc/aura_handler.rs
use crate::usecase::aura::AuraUseCase;  // ✅
```

### Правило 3: Никаких глобальных переменных

```rust
// ❌ ЗАПРЕЩЕНО
static mut CONFIG: Option<Config> = None;

// ✅ РАЗРЕШЕНО - через DI
pub struct AppState {
    config: Arc<Config>,
    logger: Arc<Logger>,
}

pub fn new_use_case(state: Arc<AppState>) -> UseCase {
    UseCase { state }
}
```

### Правило 4: Обязательные конструкторы

```rust
// ✅ ОБЯЗАТЕЛЬНО для каждой структуры
impl AuraUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

// ❌ ЗАПРЕЩЕНО создавать напрямую
let use_case = AuraUseCase { repo };  // ❌
```

### Правило 5: Async/Await консистентность

```rust
// ✅ ПРАВИЛЬНО - последовательный async
#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    async fn get(&self) -> Result<Aura, AuraError>;
}

// ✅ ПРАВИЛЬНО - tokio runtime
#[tokio::main]
async fn main() {
    // ...
}

// ❌ ЗАПРЕЩЕНО - blocking в async контексте
async fn process() {
    std::fs::read_to_string("file");  // ❌ Блокирует runtime
    // ✅ Правильно: tokio::fs::read_to_string("file").await;
}
```

### Правило 6: Структура main.rs и app.rs

```rust
// ✅ ОБЯЗАТЕЛЬНАЯ структура

// packages/daemon/src/main.rs - ТОЛЬКО запуск
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        if let Err(e) = app::run().await {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    });
}

// packages/daemon/src/app.rs - ВСЯ инициализация
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Config
    // 2. Logger
    // 3. Infrastructure
    // 4. UseCase
    // 5. Controller
    // 6. Graceful Shutdown
}
```

### Правило 7: Error Handling

```rust
// ✅ ПРАВИЛЬНО - thiserror для доменных ошибок
#[derive(Debug, Error)]
pub enum AuraError {
    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

// ✅ ПРАВИЛЬНО - Result с контекстом
pub async fn process(&self) -> Result<(), AuraError> {
    // ...
}

// ❌ ЗАПРЕЩЕНО - unwrap в продакшен коде
let value = some_result.unwrap();  // ❌

// ✅ ПРАВИЛЬНО - обработка ошибок
let value = some_result.map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
```

### Правило 8: Тестирование обязательно

```
✅ ОБЯЗАТЕЛЬНО тестировать:
- UseCase (через моки)
- Entity (бизнес-логика)

✅ ЖЕЛАТЕЛЬНО тестировать:
- Infrastructure (интеграционные тесты)

✅ ОПЦИОНАЛЬНО:
- Controller (e2e тесты)
```

### Правило 9: Именование

**Модули**:
- `entity` (не `models`, не `domain`)
- `usecase` (не `service`, не `business`)
- `infrastructure` (не `adapters`)
- `controller` (можно `transport`)

**Файлы**:
- Имя по доменной сущности: `aura.rs`, `widget.rs`
- Тесты: `aura_test.rs` или `mod tests { }` внутри файла

**Traits**:
- Имя по назначению: `Repository`, `Renderer`, `Provider`

### Правило 10: Slint Interpreter для динамических Аур

```rust
// ✅ ПРАВИЛЬНО - Slint interpreter для динамической загрузки
use slint_interpreter::{ComponentHandle, ComponentInstance};

pub struct AuraRenderer {
    instance: ComponentInstance,
}

impl AuraRenderer {
    pub fn from_source(code: &str) -> Result<Self, slint_interpreter::Diagnostic> {
        let instance = ComponentInstance::from_source(code, std::path::Path::new("virtual.slint"))?;
        Ok(Self { instance })
    }

    pub fn show(&self) {
        self.instance.show().unwrap();
    }
}

// ❌ ЗАПРЕЩЕНО - компиляция Slint в рантайме без interpreter
// slint::slint! { }  // Это компилируется в build.rs, не подходит для динамических Аур
```

---

## Контрольный чеклист

При проверке кода убедитесь:

- [ ] Нет импортов из внешних слоёв во внутренние
- [ ] Entity не импортирует UseCase, Controller, Infrastructure
- [ ] UseCase не импортирует Controller
- [ ] UseCase работает с Infrastructure только через traits
- [ ] Traits объявлены там, где используются
- [ ] Все структуры создаются через конструктор `new()`
- [ ] Нет глобальных переменных
- [ ] main.rs содержит только запуск
- [ ] app.rs содержит всю сборку зависимостей
- [ ] Реализован graceful shutdown
- [ ] Фреймворки изолированы в Controller и Infrastructure
- [ ] UseCase и Entity используют только stdlib + waio-shared
- [ ] UseCase простые и короткие (< 100 строк на метод)
- [ ] Бизнес-логика в Entity, а не в UseCase
- [ ] Написаны тесты для UseCase и Entity
- [ ] Покрытие тестами > 80%
- [ ] Нет `unwrap()` в продакшен коде
- [ ] Async функции используют `tokio::` вместо `std::` для I/O
- [ ] Slint interpreter используется для динамических Аур

---

## Антипаттерны (чего избегать)

### ❌ God Object в UseCase

```rust
// ❌ ПЛОХО
pub struct UseCase {
    repo1: Arc<Repository1>,
    repo2: Arc<Repository2>,
    repo3: Arc<Repository3>,
    api1: ApiClient1,
    api2: ApiClient2,
    // ... ещё 10 зависимостей
}
```

**Решение**: Разбить на несколько UseCase или вынести часть логики в Entity.

### ❌ Бизнес-логика в Controller

```rust
// ❌ ПЛОХО
#[tauri::command]
async fn create_aura(code: String) -> Result<String, String> {
    // ❌ Валидация в контроллере
    if code.is_empty() {
        return Err("Code is required".into());
    }
    
    // ❌ Бизнес-логика в контроллере
    let processed = code.to_uppercase();
}
```

**Решение**: Вся логика должна быть в UseCase и Entity.

### ❌ UseCase зависит от конкретной реализации

```rust
// ❌ ПЛОХО
pub struct UseCase {
    repo: FileAuraRepository,  // ❌ Конкретный тип
}
```

**Решение**: Работать только через trait.

```rust
// ✅ ПРАВИЛЬНО
pub struct UseCase<R: AuraRepository> {
    repo: R,
}
```

### ❌ Blocking в async контексте

```rust
// ❌ ПЛОХО
async fn read_file(&self) -> Result<String, Error> {
    std::fs::read_to_string("file.txt")  // ❌ Блокирует runtime
}

// ✅ ПРАВИЛЬНО
async fn read_file(&self) -> Result<String, Error> {
    tokio::fs::read_to_string("file.txt").await  // ✅ Async
}
```

---

## Логирование

### Принципы логирования

1. **Использовать `tracing`** - асинхронно-безопасное логирование
2. **Контекст через `Span`** - данные передаются через tracing span
3. **Уровни логирования** - DEBUG, INFO, WARN, ERROR
4. **Единое логирование ошибок** - ошибка логируется один раз на верхнем уровне

### Пример использования

```rust
// packages/daemon/src/usecase/aura.rs
use tracing::{info, error, instrument};

pub struct AuraUseCase<R: AuraRepository> {
    repo: R,
}

impl<R: AuraRepository> AuraUseCase<R> {
    #[instrument(skip(self, code), fields(aura_code_length = code.len()))]
    pub async fn import_aura(&self, code: &str) -> Result<Aura, AuraError> {
        info!("Importing aura");
        
        let aura = Aura::from_import_string(code)?;
        aura.validate()?;
        
        self.repo.save(&aura).await?;
        
        info!(aura_id = %aura.id, "Aura imported successfully");
        Ok(aura)
    }
}
```

### Инициализация в app.rs

```rust
// packages/daemon/src/app.rs
use tracing_subscriber::{fmt, EnvFilter};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация логгера
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .init();

    // ... остальная инициализация
}
```

### Уровни логирования

| Окружение | Формат | Уровень |
|-----------|--------|---------|
| `local`   | Pretty | `DEBUG` |
| `dev`     | JSON   | `DEBUG` |
| `prod`    | JSON   | `INFO`  |

### Запрещено логировать

- ❌ Пароли
- ❌ Токены
- ❌ Персональные данные пользователей
- ❌ Полные пути к файлам в продакшене

---

## Заключение

Чистая архитектура на Rust - это набор принципов, которые помогают создавать гибкий, тестируемый и поддерживаемый код с учётом особенностей языка (ownership, traits, async).

### Ключевые моменты

1. **Инверсия зависимостей** - главный принцип (traits в UseCase)
2. **Изоляция слоёв** - каждый слой выполняет свою роль
3. **Traits** - для разрыва зависимостей
4. **DI** - через конструкторы, без глобальных переменных
5. **Простота UseCase** - вся сложность в Entity
6. **Изоляция фреймворков** - только во внешних слоях
7. **Тестируемость** - основной показатель качества архитектуры
8. **Async безопасность** - tokio для всех I/O операций
9. **Slint interpreter** - для динамических Аур

### Rust-специфичные компромиссы

- Можно использовать `Clone` для маленьких структур
- Можно использовать `Arc<Mutex<>>` для shared state
- Можно использовать `#[derive]` макросы для Entity
- Можно использовать `serde` теги в Entity

### Недопустимые компромиссы

- ❌ Нарушение направления зависимостей
- ❌ Фреймворки в бизнес-логике
- ❌ Глобальные переменные
- ❌ Бизнес-логика в контроллерах
- ❌ UseCase зависят от конкретных реализаций
- ❌ `unwrap()` в продакшен коде
- ❌ Blocking I/O в async контексте

---

**Помните**: Цель архитектуры - сделать код гибким, тестируемым и поддерживаемым на долгие годы. Все решения должны приниматься с учётом этих целей и особенностей Rust.
