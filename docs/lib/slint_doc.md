# Документация slint v1.15.1

> Источник: [https://docs.rs/slint/1.15.1/slint/](https://docs.rs/slint/1.15.1/slint/)
> Сгенерировано: 2026-04-03 13:15:28

> Slint — фреймворк для декларативного создания графических интерфейсов на Rust.

---

## 📑 Оглавление

### Модули
- [`android/`](#android)
- [`docs/`](#docs)
- [`fontique_07/`](#fontique-07)
- [`language/`](#language)
- [`platform/`](#platform)
- [`wgpu_28/`](#wgpu-28)
- [`winit_030/`](#winit-030)

### Макросы
- [`format`](#format)
- [`include_modules`](#include-modules)
- [`init_translations`](#init-translations)
- [`slint`](#slint)

### Структуры
- [`BackendSelector`](#backendselector)
- [`BorrowedOpenGLTextureBuilder`](#borrowedopengltexturebuilder)
- [`Color`](#color)
- [`FilterModel`](#filtermodel)
- [`Image`](#image)
- [`JoinHandle`](#joinhandle)
- [`LoadImageError`](#loadimageerror)
- [`LogicalPosition`](#logicalposition)
- [`LogicalSize`](#logicalsize)
- [`MapModel`](#mapmodel)
- [`ModelNotify`](#modelnotify)
- [`ModelPeer`](#modelpeer)
- [`ModelRc`](#modelrc)
- [`OklchColor`](#oklchcolor)
- [`PhysicalPosition`](#physicalposition)
- [`PhysicalSize`](#physicalsize)
- [`ReverseModel`](#reversemodel)
- [`RgbaColor`](#rgbacolor)
- [`SharedPixelBuffer`](#sharedpixelbuffer)
- [`SharedString`](#sharedstring)
- [`SharedVector`](#sharedvector)
- [`SortModel`](#sortmodel)
- [`StandardListViewItem`](#standardlistviewitem)
- [`TableColumn`](#tablecolumn)
- [`Timer`](#timer)
- [`VecModel`](#vecmodel)
- [`Weak`](#weak)
- [`Window`](#window)
- [`WindowHandle`](#windowhandle)

### Перечисления
- [`BorrowedOpenGLTextureOrigin`](#borrowedopengltextureorigin)
- [`Brush`](#brush)
- [`CloseRequestResponse`](#closerequestresponse)
- [`EventLoopError`](#eventlooperror)
- [`GraphicsAPI`](#graphicsapi)
- [`PlatformError`](#platformerror)
- [`RenderingState`](#renderingstate)
- [`SelectBundledTranslationError`](#selectbundledtranslationerror)
- [`SetRenderingNotifierError`](#setrenderingnotifiererror)
- [`TimerMode`](#timermode)
- [`WindowPosition`](#windowposition)
- [`WindowSize`](#windowsize)

### Трейты
- [`ComponentHandle`](#componenthandle)
- [`Global`](#global)
- [`Model`](#model)
- [`ModelExt`](#modelext)
- [`ModelTracker`](#modeltracker)
- [`ToSharedString`](#tosharedstring)

### Функции
- [`invoke_from_event_loop`](#invoke-from-event-loop)
- [`quit_event_loop`](#quit-event-loop)
- [`run_event_loop`](#run-event-loop)
- [`run_event_loop_until_quit`](#run-event-loop-until-quit)
- [`select_bundled_translation`](#select-bundled-translation)
- [`set_xdg_app_id`](#set-xdg-app-id)
- [`spawn_local`](#spawn-local)

### Type Aliases
- [`Rgb8Pixel`](#rgb8pixel)
- [`Rgba8Pixel`](#rgba8pixel)

---
<a id="moduleandroidcopy-item-path"></a>

## `ModuleandroidCopy item path`
*Not (Android and (backend-android-activity-05orbackend-android-activity-06))*

---
Available on
**Android and (crate featuresbackend-android-activity-05orbackend-android-activity-06)**only.

## Modules

- **`android_activityNot (Android and (backend-android-activity-05orbackend-android-activity-06))`**: Re-export of theandroid-activitycrate.

## Functions

- **`init`**: Initializes the Android backend.
- **`init_with_event_listener`**: Similar toinit(), which allow to listen to android-activity’s event

[🔝 Наверх](#-оглавление)
<a id="moduledocscopy-item-path"></a>

## `ModuledocsCopy item path`
---
## Modules

- **`cargo_features`**: Feature flags and backend selection.
- **`generated_code`**: This module exists only to explain the API of the code generated from.slintdesign markup. Its described structure
is not really contained in the compiled crate.
- **`mcu`**: Slint on Microcontrollers
- **`type_mappings`**: Type Mappings

[🔝 Наверх](#-оглавление)
<a id="modulefontique-07copy-item-path"></a>

## `Modulefontique_07Copy item path`
---
Available on
**crate featureunstable-fontique-07**only.

## Re-exports

- **`pub use i_slint_common::sharedfontique::fontique;`**

## Functions

- **`shared_collection`**: Returns a clone offontique::Collectionthat’s used by Slint for text rendering. It’s set up
with shared storage, so fonts registered with the returned collection or additionally configured font
fallbacks apply to the entire process.

[🔝 Наверх](#-оглавление)
<a id="modulelanguagecopy-item-path"></a>

## `ModulelanguageCopy item path`
---
## Enums

- **`ColorScheme`**: This enum indicates the color scheme used by the widget style. Use this to explicitly switch
between dark and light schemes, or choose Unknown to fall back to the system default.

[🔝 Наверх](#-оглавление)
<a id="moduleplatformcopy-item-path"></a>

## `ModuleplatformCopy item path`
*renderer-femtovgand non-Android* | *renderer-software*

---
## Modules

- **`femtovg_rendererrenderer-femtovgand non-Android`**: This module contains thefemtovg_renderer::FemtoVGRendererand related types.
- **`software_rendererrenderer-software`**: This module contains thesoftware_renderer::SoftwareRendererand related types.

## Structs

- **`LayoutConstraints`**: This struct describes layout constraints of a resizable element, such as a window.
- **`WindowProperties`**: This struct contains getters that provide access to properties of theWindowelement, and is used withWindowAdapter::update_window_properties.

## Enums

- **`Clipboard`**: The clip board, used inPlatform::clipboard_textandPlatform::set_clipboard_text`
- **`Key`**: TheKeyenum is used to map a specific key by name e.g.Key::Controlto an
internal used unicode representation. The enum is convertible tostd::charandslint::SharedString.
Use this withslint::platform::WindowEventto supply key events to Slint’s platform abstraction.
- **`PlatformError`**: The platform encountered a fatal error.
- **`PointerEventButton`**: This enum describes the different types of buttons for a pointer event,
typically on a mouse or a pencil.
- **`SetPlatformError`**: This enum describes the different error scenarios that may occur whenset_platformfails.
- **`WindowEvent`**: A event that describes user input or windowing system events.

## Traits

- **`EventLoopProxy`**: Trait that is returned by thePlatform::new_event_loop_proxy
- **`Platform`**: This trait defines the interface between Slint and platform APIs typically provided by operating and windowing systems.
- **`Renderer`**: This trait represents a Renderer that can render a slint scene.
- **`WindowAdapter`**: This trait represents the adaptation layer between theWindowAPI and then
windowing specific window representation, such as a Win32HWNDhandle or awayland_surface_t.

## Functions

- **`duration_until_next_timer_update`**: Returns the duration before the next timer is expected to be activated. This is the
largest amount of time that you can wait before callingupdate_timers_and_animations().
- **`set_platform`**: Set the Slint platform abstraction.
- **`update_timers_and_animations`**: Call this function to update and potentially activate any pending timers, as well
as advance the state of any active animations.

[🔝 Наверх](#-оглавление)
<a id="modulewgpu-28copy-item-path"></a>

## `Modulewgpu_28Copy item path`
---
Available on
**crate featureunstable-wgpu-28**only.

## Modules

- **`wgpu`**: wgpuis a cross-platform, safe, pure-Rust graphics API. It runs natively on
Vulkan, Metal, D3D12, and OpenGL; and on top of WebGL2 and WebGPU on wasm.

## Structs

- **`WGPUSettings`**: This data structure provides settings for initializing WGPU renderers.

## Enums

- **`TextureImportError`**: This enum describes the possible errors that can occur when importing a WGPU texture,
viaImage::try_from().
- **`WGPUConfiguration`**: This enum describes the different ways to configure WGPU for rendering.

[🔝 Наверх](#-оглавление)
<a id="modulewinit-030copy-item-path"></a>

## `Modulewinit_030Copy item path`
*Deprecated*

---
Available on
**crate featureunstable-winit-030**only.

## Re-exports

- **`pub use i_slint_backend_winit::winit;`**

## Structs

- **`SlintEvent`**: Internal type used by the winit backend for thread communication and window system updates.

## Enums

- **`EventResult`**: Returned by callbacks passed toWindow::on_winit_window_eventto determine if winit events should propagate to the Slint event loop.

## Traits

- **`CustomApplicationHandler`**: Use this trait to intercept events from winit.
- **`WinitWindowAccessor`**: This helper trait can be used to obtain access to thewinit::window::Windowfor a givenslint::Window.

## Type Aliases

- **`EventLoopBuilder`**: Convenience alias for the event loop builder used by Slint.
- **`WinitWindowEventResultDeprecated`**: Deprecated alias toEventResult

[🔝 Наверх](#-оглавление)
<a id="macroformatcopy-item-path"></a>

## `MacroformatCopy item path`
---
```rust
macro_rules! format {
    ($($arg:tt)*) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macroinclude-modulescopy-item-path"></a>

## `Macroinclude_modulesCopy item path`
---
```rust
macro_rules! include_modules {
    () => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macroinit-translationscopy-item-path"></a>

## `Macroinit_translationsCopy item path`
---
```rust
macro_rules! init_translations {
    ($dirname:expr) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macroslintcopy-item-path"></a>

## `MacroslintCopy item path`
---
```rust
slint!() { /* proc-macro */ }
```

[🔝 Наверх](#-оглавление)
<a id="structbackendselectorcopy-item-path"></a>

## `StructBackendSelectorCopy item path`
---
```rust
pub struct BackendSelector { /* private fields */ }
```

## Implementations

## Trait Implementations

## Auto Trait Implementations

§
### implFreezeforBackendSelector

§
### impl !RefUnwindSafeforBackendSelector

§
### impl !SendforBackendSelector

§
### impl !SyncforBackendSelector

§
### implUnpinforBackendSelector

§
### implUnsafeUnpinforBackendSelector

§
### impl !UnwindSafeforBackendSelector

## Blanket Implementations

[🔝 Наверх](#-оглавление)
<a id="structborrowedopengltexturebuildercopy-item-path"></a>

## `StructBorrowedOpenGLTextureBuilderCopy item path`
---
```rust
pub struct BorrowedOpenGLTextureBuilder(/* private fields */);
```

Available on
**non-WebAssembly**only.

## Implementations

## Auto Trait Implementations

§
### implFreezeforBorrowedOpenGLTextureBuilder

§
### implRefUnwindSafeforBorrowedOpenGLTextureBuilder

§
### implSendforBorrowedOpenGLTextureBuilder

§
### implSyncforBorrowedOpenGLTextureBuilder

§
### implUnpinforBorrowedOpenGLTextureBuilder

§
### implUnsafeUnpinforBorrowedOpenGLTextureBuilder

§
### implUnwindSafeforBorrowedOpenGLTextureBuilder

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structcolorcopy-item-path"></a>

## `StructColorCopy item path`
---
```rust
pub struct Color { /* private fields */ }
```

## Implementations

## Trait Implementations

§
### implCopyforColor

§
### implStructuralPartialEqforColor

## Auto Trait Implementations

§
### implFreezeforColor

§
### implRefUnwindSafeforColor

§
### implSendforColor

§
### implSyncforColor

§
### implUnpinforColor

§
### implUnsafeUnpinforColor

§
### implUnwindSafeforColor

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structfiltermodelcopy-item-path"></a>

## `StructFilterModelCopy item path`
---
```rust
pub struct FilterModel<M, F>(/* private fields */)
where
    M: Model + 'static,
    F: Fn(&<M as Model>::Data) -> bool + 'static;
```

## Implementations

## Trait Implementations

## Auto Trait Implementations

§
### impl<M, F>FreezeforFilterModel<M, F>

§
### impl<M, F> !RefUnwindSafeforFilterModel<M, F>

§
### impl<M, F> !SendforFilterModel<M, F>

§
### impl<M, F> !SyncforFilterModel<M, F>

§
### impl<M, F>UnpinforFilterModel<M, F>

§
### impl<M, F>UnsafeUnpinforFilterModel<M, F>

§
### impl<M, F> !UnwindSafeforFilterModel<M, F>

## Blanket Implementations

[🔝 Наверх](#-оглавление)
<a id="structimagecopy-item-path"></a>

## `StructImageCopy item path`
---
```rust
pub struct Image(/* private fields */);
```

## Implementations

## Trait Implementations

§
### implStructuralPartialEqforImage

## Auto Trait Implementations

§
### implFreezeforImage

§
### impl !RefUnwindSafeforImage

§
### impl !SendforImage

§
### impl !SyncforImage

§
### implUnpinforImage

§
### implUnsafeUnpinforImage

§
### impl !UnwindSafeforImage

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#-оглавление)
<a id="structjoinhandlecopy-item-path"></a>

## `StructJoinHandleCopy item path`
---
```rust
pub struct JoinHandle<T>(/* private fields */);
```

## Implementations

## Trait Implementations

§
### impl<T>SendforJoinHandle<T>where
    T:Send,

Available on
**crate featurestd**only.

## Auto Trait Implementations

§
### impl<T>FreezeforJoinHandle<T>

§
### impl<T> !RefUnwindSafeforJoinHandle<T>

§
### impl<T>SyncforJoinHandle<T>

§
### impl<T>UnpinforJoinHandle<T>

§
### impl<T>UnsafeUnpinforJoinHandle<T>

§
### impl<T> !UnwindSafeforJoinHandle<T>

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structloadimageerrorcopy-item-path"></a>

## `StructLoadImageErrorCopy item path`
---
```rust
pub struct LoadImageError(/* private fields */);
```

## Trait Implementations

§
### implStructuralPartialEqforLoadImageError

## Auto Trait Implementations

§
### implFreezeforLoadImageError

§
### implRefUnwindSafeforLoadImageError

§
### implSendforLoadImageError

§
### implSyncforLoadImageError

§
### implUnpinforLoadImageError

§
### implUnsafeUnpinforLoadImageError

§
### implUnwindSafeforLoadImageError

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structlogicalpositioncopy-item-path"></a>

## `StructLogicalPositionCopy item path`
---
```rust
#[repr(C)]pub struct LogicalPosition {
    pub x: f32,
    pub y: f32,
}
```

## Fields

§`x:f32`The x coordinate.

§`y:f32`The y coordinate.

## Implementations

## Trait Implementations

§
### implCopyforLogicalPosition

§
### implStructuralPartialEqforLogicalPosition

## Auto Trait Implementations

§
### implFreezeforLogicalPosition

§
### implRefUnwindSafeforLogicalPosition

§
### implSendforLogicalPosition

§
### implSyncforLogicalPosition

§
### implUnpinforLogicalPosition

§
### implUnsafeUnpinforLogicalPosition

§
### implUnwindSafeforLogicalPosition

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structlogicalsizecopy-item-path"></a>

## `StructLogicalSizeCopy item path`
---
```rust
#[repr(C)]pub struct LogicalSize {
    pub width: f32,
    pub height: f32,
}
```

## Fields

§`width:f32`The width in logical pixels.

§`height:f32`The height in logical.

## Implementations

## Trait Implementations

§
### implCopyforLogicalSize

§
### implStructuralPartialEqforLogicalSize

## Auto Trait Implementations

§
### implFreezeforLogicalSize

§
### implRefUnwindSafeforLogicalSize

§
### implSendforLogicalSize

§
### implSyncforLogicalSize

§
### implUnpinforLogicalSize

§
### implUnsafeUnpinforLogicalSize

§
### implUnwindSafeforLogicalSize

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structmapmodelcopy-item-path"></a>

## `StructMapModelCopy item path`
---
```rust
pub struct MapModel<M, F> { /* private fields */ }
```

## Implementations

## Trait Implementations

## Auto Trait Implementations

§
### impl<M, F>FreezeforMapModel<M, F>where
    M:Freeze,
    F:Freeze,

§
### impl<M, F>RefUnwindSafeforMapModel<M, F>where
    M:RefUnwindSafe,
    F:RefUnwindSafe,

§
### impl<M, F>SendforMapModel<M, F>where
    M:Send,
    F:Send,

§
### impl<M, F>SyncforMapModel<M, F>where
    M:Sync,
    F:Sync,

§
### impl<M, F>UnpinforMapModel<M, F>where
    M:Unpin,
    F:Unpin,

§
### impl<M, F>UnsafeUnpinforMapModel<M, F>where
    M:UnsafeUnpin,
    F:UnsafeUnpin,

§
### impl<M, F>UnwindSafeforMapModel<M, F>where
    M:UnwindSafe,
    F:UnwindSafe,

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structmodelnotifycopy-item-path"></a>

## `StructModelNotifyCopy item path`
---
```rust
pub struct ModelNotify { /* private fields */ }
```

## Implementations

## Trait Implementations

## Auto Trait Implementations

§
### impl !FreezeforModelNotify

§
### impl !RefUnwindSafeforModelNotify

§
### impl !SendforModelNotify

§
### impl !SyncforModelNotify

§
### implUnpinforModelNotify

§
### implUnsafeUnpinforModelNotify

§
### impl !UnwindSafeforModelNotify

## Blanket Implementations

[🔝 Наверх](#-оглавление)
<a id="structmodelpeercopy-item-path"></a>

## `StructModelPeerCopy item path`
---
```rust
pub struct ModelPeer<'a> { /* private fields */ }
```

## Trait Implementations

## Auto Trait Implementations

§
### impl<'a>FreezeforModelPeer<'a>

§
### impl<'a> !RefUnwindSafeforModelPeer<'a>

§
### impl<'a> !SendforModelPeer<'a>

§
### impl<'a> !SyncforModelPeer<'a>

§
### impl<'a>UnpinforModelPeer<'a>

§
### impl<'a>UnsafeUnpinforModelPeer<'a>

§
### impl<'a> !UnwindSafeforModelPeer<'a>

## Blanket Implementations

[🔝 Наверх](#-оглавление)
<a id="structmodelrccopy-item-path"></a>

## `StructModelRcCopy item path`
---
```rust
pub struct ModelRc<T>(/* private fields */);
```

## Implementations

## Trait Implementations

## Auto Trait Implementations

§
### impl<T>FreezeforModelRc<T>

§
### impl<T> !RefUnwindSafeforModelRc<T>

§
### impl<T> !SendforModelRc<T>

§
### impl<T> !SyncforModelRc<T>

§
### impl<T>UnpinforModelRc<T>

§
### impl<T>UnsafeUnpinforModelRc<T>

§
### impl<T> !UnwindSafeforModelRc<T>

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#-оглавление)
<a id="structoklchcolorcopy-item-path"></a>

## `StructOklchColorCopy item path`
---
```rust
pub struct OklchColor {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
    pub alpha: f32,
}
```

## Fields

§`lightness:f32`The lightness component, between 0 (black) and 1 (white).

§`chroma:f32`The chroma component (color intensity), typically between 0 and about 0.4.

§`hue:f32`The hue component in degrees between 0 and 360.

§`alpha:f32`The alpha component, between 0 and 1.

## Trait Implementations

§
### implCopyforOklchColor

## Auto Trait Implementations

§
### implFreezeforOklchColor

§
### implRefUnwindSafeforOklchColor

§
### implSendforOklchColor

§
### implSyncforOklchColor

§
### implUnpinforOklchColor

§
### implUnsafeUnpinforOklchColor

§
### implUnwindSafeforOklchColor

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structphysicalpositioncopy-item-path"></a>

## `StructPhysicalPositionCopy item path`
---
```rust
pub struct PhysicalPosition {
    pub x: i32,
    pub y: i32,
}
```

## Fields

§`x:i32`The x coordinate.

§`y:i32`The y coordinate.

## Implementations

## Trait Implementations

§
### implCopyforPhysicalPosition

§
### implEqforPhysicalPosition

§
### implStructuralPartialEqforPhysicalPosition

## Auto Trait Implementations

§
### implFreezeforPhysicalPosition

§
### implRefUnwindSafeforPhysicalPosition

§
### implSendforPhysicalPosition

§
### implSyncforPhysicalPosition

§
### implUnpinforPhysicalPosition

§
### implUnsafeUnpinforPhysicalPosition

§
### implUnwindSafeforPhysicalPosition

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structphysicalsizecopy-item-path"></a>

## `StructPhysicalSizeCopy item path`
---
```rust
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}
```

## Fields

§`width:u32`The width in physical pixels.

§`height:u32`The height in physical pixels;

## Implementations

## Trait Implementations

§
### implCopyforPhysicalSize

§
### implEqforPhysicalSize

§
### implStructuralPartialEqforPhysicalSize

## Auto Trait Implementations

§
### implFreezeforPhysicalSize

§
### implRefUnwindSafeforPhysicalSize

§
### implSendforPhysicalSize

§
### implSyncforPhysicalSize

§
### implUnpinforPhysicalSize

§
### implUnsafeUnpinforPhysicalSize

§
### implUnwindSafeforPhysicalSize

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structreversemodelcopy-item-path"></a>

## `StructReverseModelCopy item path`
---
```rust
pub struct ReverseModel<M>(/* private fields */)
where
    M: Model + 'static;
```

## Implementations

## Trait Implementations

## Auto Trait Implementations

§
### impl<M>FreezeforReverseModel<M>

§
### impl<M> !RefUnwindSafeforReverseModel<M>

§
### impl<M> !SendforReverseModel<M>

§
### impl<M> !SyncforReverseModel<M>

§
### impl<M>UnpinforReverseModel<M>

§
### impl<M>UnsafeUnpinforReverseModel<M>

§
### impl<M> !UnwindSafeforReverseModel<M>

## Blanket Implementations

[🔝 Наверх](#-оглавление)
<a id="structrgbacolorcopy-item-path"></a>

## `StructRgbaColorCopy item path`
---
```rust
pub struct RgbaColor<T> {
    pub alpha: T,
    pub red: T,
    pub green: T,
    pub blue: T,
}
```

## Fields

§`alpha: T`The alpha component.

§`red: T`The red channel.

§`green: T`The green channel.

§`blue: T`The blue channel.

## Trait Implementations

§
### impl<T>CopyforRgbaColor<T>where
    T:Copy,

§
### impl<T>StructuralPartialEqforRgbaColor<T>

## Auto Trait Implementations

§
### impl<T>FreezeforRgbaColor<T>where
    T:Freeze,

§
### impl<T>RefUnwindSafeforRgbaColor<T>where
    T:RefUnwindSafe,

§
### impl<T>SendforRgbaColor<T>where
    T:Send,

§
### impl<T>SyncforRgbaColor<T>where
    T:Sync,

§
### impl<T>UnpinforRgbaColor<T>where
    T:Unpin,

§
### impl<T>UnsafeUnpinforRgbaColor<T>where
    T:UnsafeUnpin,

§
### impl<T>UnwindSafeforRgbaColor<T>where
    T:UnwindSafe,

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structsharedpixelbuffercopy-item-path"></a>

## `StructSharedPixelBufferCopy item path`
---
```rust
pub struct SharedPixelBuffer<Pixel> { /* private fields */ }
```

## Implementations

## Trait Implementations

## Auto Trait Implementations

§
### impl<Pixel>FreezeforSharedPixelBuffer<Pixel>

§
### impl<Pixel>RefUnwindSafeforSharedPixelBuffer<Pixel>where
    Pixel:RefUnwindSafe,

§
### impl<Pixel>SendforSharedPixelBuffer<Pixel>where
    Pixel:Send+Sync,

§
### impl<Pixel>SyncforSharedPixelBuffer<Pixel>where
    Pixel:Send+Sync,

§
### impl<Pixel>UnpinforSharedPixelBuffer<Pixel>

§
### impl<Pixel>UnsafeUnpinforSharedPixelBuffer<Pixel>

§
### impl<Pixel>UnwindSafeforSharedPixelBuffer<Pixel>where
    Pixel:RefUnwindSafe,

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structsharedstringcopy-item-path"></a>

## `StructSharedStringCopy item path`
---
```rust
pub struct SharedString { /* private fields */ }
```

## Implementations

## Trait Implementations

§
### implEqforSharedString

## Auto Trait Implementations

§
### implFreezeforSharedString

§
### implRefUnwindSafeforSharedString

§
### implSendforSharedString

§
### implSyncforSharedString

§
### implUnpinforSharedString

§
### implUnsafeUnpinforSharedString

§
### implUnwindSafeforSharedString

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structsharedvectorcopy-item-path"></a>

## `StructSharedVectorCopy item path`
---
```rust
pub struct SharedVector<T> { /* private fields */ }
```

## Implementations

## Trait Implementations

§
### impl<T>EqforSharedVector<T>where
    T:Eq,

§
### impl<T>SendforSharedVector<T>where
    T:Send+Sync,

§
### impl<T>SyncforSharedVector<T>where
    T:Send+Sync,

## Auto Trait Implementations

§
### impl<T>FreezeforSharedVector<T>

§
### impl<T>RefUnwindSafeforSharedVector<T>where
    T:RefUnwindSafe,

§
### impl<T>UnpinforSharedVector<T>

§
### impl<T>UnsafeUnpinforSharedVector<T>

§
### impl<T>UnwindSafeforSharedVector<T>where
    T:RefUnwindSafe,

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structsortmodelcopy-item-path"></a>

## `StructSortModelCopy item path`
---
```rust
pub struct SortModel<M, F>(/* private fields */)
where
    M: Model + 'static,
    F: SortHelper<<M as Model>::Data> + 'static;
```

## Implementations

## Trait Implementations

## Auto Trait Implementations

§
### impl<M, F>FreezeforSortModel<M, F>

§
### impl<M, F> !RefUnwindSafeforSortModel<M, F>

§
### impl<M, F> !SendforSortModel<M, F>

§
### impl<M, F> !SyncforSortModel<M, F>

§
### impl<M, F>UnpinforSortModel<M, F>

§
### impl<M, F>UnsafeUnpinforSortModel<M, F>

§
### impl<M, F> !UnwindSafeforSortModel<M, F>

## Blanket Implementations

[🔝 Наверх](#-оглавление)
<a id="structstandardlistviewitemcopy-item-path"></a>

## `StructStandardListViewItemCopy item path`
---
```rust
#[non_exhaustive]#[repr(C)]pub struct StandardListViewItem {
    pub text: SharedString,
}
```

## Fields (Non-exhaustive)

§`text:SharedString`The text content of the item

## Trait Implementations

§
### implStructuralPartialEqforStandardListViewItem

## Auto Trait Implementations

§
### implFreezeforStandardListViewItem

§
### implRefUnwindSafeforStandardListViewItem

§
### implSendforStandardListViewItem

§
### implSyncforStandardListViewItem

§
### implUnpinforStandardListViewItem

§
### implUnsafeUnpinforStandardListViewItem

§
### implUnwindSafeforStandardListViewItem

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structtablecolumncopy-item-path"></a>

## `StructTableColumnCopy item path`
---
```rust
#[non_exhaustive]#[repr(C)]pub struct TableColumn {
    pub title: SharedString,
    pub min_width: f32,
    pub horizontal_stretch: f32,
    pub sort_order: SortOrder,
    pub width: f32,
}
```

## Fields (Non-exhaustive)

§`title:SharedString`The title of the column header

§`min_width:f32`The minimum column width (logical length)

§`horizontal_stretch:f32`The horizontal column stretch

§`sort_order:SortOrder`Sorts the column

§`width:f32`the actual width of the column (logical length)

## Trait Implementations

§
### implStructuralPartialEqforTableColumn

## Auto Trait Implementations

§
### implFreezeforTableColumn

§
### implRefUnwindSafeforTableColumn

§
### implSendforTableColumn

§
### implSyncforTableColumn

§
### implUnpinforTableColumn

§
### implUnsafeUnpinforTableColumn

§
### implUnwindSafeforTableColumn

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structtimercopy-item-path"></a>

## `StructTimerCopy item path`
---
```rust
pub struct Timer { /* private fields */ }
```

## Implementations

## Trait Implementations

## Auto Trait Implementations

§
### impl !FreezeforTimer

§
### impl !RefUnwindSafeforTimer

§
### impl !SendforTimer

§
### impl !SyncforTimer

§
### implUnpinforTimer

§
### implUnsafeUnpinforTimer

§
### implUnwindSafeforTimer

## Blanket Implementations

[🔝 Наверх](#-оглавление)
<a id="structvecmodelcopy-item-path"></a>

## `StructVecModelCopy item path`
---
```rust
pub struct VecModel<T> { /* private fields */ }
```

## Implementations

## Trait Implementations

## Auto Trait Implementations

§
### impl<T> !FreezeforVecModel<T>

§
### impl<T> !RefUnwindSafeforVecModel<T>

§
### impl<T> !SendforVecModel<T>

§
### impl<T> !SyncforVecModel<T>

§
### impl<T>UnpinforVecModel<T>where
    T:Unpin,

§
### impl<T>UnsafeUnpinforVecModel<T>

§
### impl<T> !UnwindSafeforVecModel<T>

## Blanket Implementations

[🔝 Наверх](#-оглавление)
<a id="structweakcopy-item-path"></a>

## `StructWeakCopy item path`
---
```rust
pub struct Weak<T>where
    T: ComponentHandle,{ /* private fields */ }
```

## Implementations

## Trait Implementations

§
### impl<T>SendforWeak<T>where
    T:ComponentHandle,

Available on
**crate featuresstdorunsafe-single-threaded**only.
§
### impl<T>SyncforWeak<T>where
    T:ComponentHandle,

Available on
**crate featuresstdorunsafe-single-threaded**only.

## Auto Trait Implementations

§
### impl<T>FreezeforWeak<T>where
    <T asComponentHandle>::WeakInner:Freeze,

§
### impl<T>RefUnwindSafeforWeak<T>where
    <T asComponentHandle>::WeakInner:RefUnwindSafe,

§
### impl<T>UnpinforWeak<T>where
    <T asComponentHandle>::WeakInner:Unpin,

§
### impl<T>UnsafeUnpinforWeak<T>where
    <T asComponentHandle>::WeakInner:UnsafeUnpin,

§
### impl<T>UnwindSafeforWeak<T>where
    <T asComponentHandle>::WeakInner:UnwindSafe,

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="structwindowcopy-item-path"></a>

## `StructWindowCopy item path`
---
```rust
pub struct Window(/* private fields */);
```

## Implementations

## Trait Implementations

## Auto Trait Implementations

§
### impl !FreezeforWindow

§
### impl !RefUnwindSafeforWindow

§
### impl !SendforWindow

§
### impl !SyncforWindow

§
### implUnpinforWindow

§
### implUnsafeUnpinforWindow

§
### impl !UnwindSafeforWindow

## Blanket Implementations

[🔝 Наверх](#-оглавление)
<a id="structwindowhandlecopy-item-path"></a>

## `StructWindowHandleCopy item path`
---
```rust
pub struct WindowHandle { /* private fields */ }
```

Available on
**crate featureraw-window-handle-06**only.

## Trait Implementations

## Auto Trait Implementations

§
### implFreezeforWindowHandle

§
### impl !RefUnwindSafeforWindowHandle

§
### impl !SendforWindowHandle

§
### impl !SyncforWindowHandle

§
### implUnpinforWindowHandle

§
### implUnsafeUnpinforWindowHandle

§
### impl !UnwindSafeforWindowHandle

## Blanket Implementations

[🔝 Наверх](#-оглавление)
<a id="enumborrowedopengltextureorigincopy-item-path"></a>

## `EnumBorrowedOpenGLTextureOriginCopy item path`
---
```rust
#[non_exhaustive]#[repr(u8)]pub enum BorrowedOpenGLTextureOrigin {
    TopLeft = 0,
    BottomLeft = 1,
}
```

Available on
**non-WebAssembly**only.

## Variants (Non-exhaustive)

§
### TopLeft = 0

The top-left of the texture is the top-left of the texture drawn on the screen.

§
### BottomLeft = 1

The bottom-left of the texture is the top-left of the texture draw on the screen,
flipping it vertically.

## Trait Implementations

§
### implCopyforBorrowedOpenGLTextureOrigin

§
### implStructuralPartialEqforBorrowedOpenGLTextureOrigin

## Auto Trait Implementations

§
### implFreezeforBorrowedOpenGLTextureOrigin

§
### implRefUnwindSafeforBorrowedOpenGLTextureOrigin

§
### implSendforBorrowedOpenGLTextureOrigin

§
### implSyncforBorrowedOpenGLTextureOrigin

§
### implUnpinforBorrowedOpenGLTextureOrigin

§
### implUnsafeUnpinforBorrowedOpenGLTextureOrigin

§
### implUnwindSafeforBorrowedOpenGLTextureOrigin

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="enumbrushcopy-item-path"></a>

## `EnumBrushCopy item path`
---
```rust
#[non_exhaustive]#[repr(C)]pub enum Brush {
    SolidColor(Color),
    LinearGradient(LinearGradientBrush),
    RadialGradient(RadialGradientBrush),
    ConicGradient(ConicGradientBrush),
}
```

## Variants (Non-exhaustive)

§
### SolidColor(Color)

The color variant of brush is a plain color that is to be used for the fill.

§
### LinearGradient(LinearGradientBrush)

The linear gradient variant of a brush describes the gradient stops for a fill
where all color stops are along a line that’s rotated by the specified angle.

§
### RadialGradient(RadialGradientBrush)

The radial gradient variant of a brush describes a circle variant centered
in the middle

§
### ConicGradient(ConicGradientBrush)

The conical gradient variant of a brush describes a gradient that rotates around
a center point, like the hands of a clock

## Implementations

## Trait Implementations

§
### implStructuralPartialEqforBrush

## Auto Trait Implementations

§
### implFreezeforBrush

§
### implRefUnwindSafeforBrush

§
### implSendforBrush

§
### implSyncforBrush

§
### implUnpinforBrush

§
### implUnsafeUnpinforBrush

§
### implUnwindSafeforBrush

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="enumcloserequestresponsecopy-item-path"></a>

## `EnumCloseRequestResponseCopy item path`
---
```rust
#[repr(u8)]pub enum CloseRequestResponse {
    HideWindow = 0,
    KeepWindowShown = 1,
}
```

## Variants

§
### HideWindow = 0

The Window will be hidden (default action)

§
### KeepWindowShown = 1

The close request is rejected and the window will be kept shown.

## Trait Implementations

§
### implCopyforCloseRequestResponse

§
### implStructuralPartialEqforCloseRequestResponse

## Auto Trait Implementations

§
### implFreezeforCloseRequestResponse

§
### implRefUnwindSafeforCloseRequestResponse

§
### implSendforCloseRequestResponse

§
### implSyncforCloseRequestResponse

§
### implUnpinforCloseRequestResponse

§
### implUnsafeUnpinforCloseRequestResponse

§
### implUnwindSafeforCloseRequestResponse

## Blanket Implementations

§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="enumeventlooperrorcopy-item-path"></a>

## `EnumEventLoopErrorCopy item path`
---
```rust
#[non_exhaustive]pub enum EventLoopError {
    EventLoopTerminated,
    NoEventLoopProvider,
}
```

## Variants (Non-exhaustive)

§
### EventLoopTerminated

The event could not be sent because the event loop was terminated already

§
### NoEventLoopProvider

The event could not be sent because the Slint platform abstraction was not yet initialized,
or the platform does not support event loop.

## Trait Implementations

§
### implEqforEventLoopError

§
### implStructuralPartialEqforEventLoopError

## Auto Trait Implementations

§
### implFreezeforEventLoopError

§
### implRefUnwindSafeforEventLoopError

§
### implSendforEventLoopError

§
### implSyncforEventLoopError

§
### implUnpinforEventLoopError

§
### implUnsafeUnpinforEventLoopError

§
### implUnwindSafeforEventLoopError

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="enumgraphicsapicopy-item-path"></a>

## `EnumGraphicsAPICopy item path`
---
```rust
#[non_exhaustive]pub enum GraphicsAPI<'a> {
    NativeOpenGL {
        get_proc_address: &'a dyn Fn(&CStr) -> *const c_void,
    },
    WebGL {
        canvas_element_id: &'a str,
        context_type: &'a str,
    },
    #[non_exhaustive]    WGPU28 {
        instance: Instance,
        device: Device,
        queue: Queue,
    },
}
```

## Variants (Non-exhaustive)

§
### NativeOpenGL

The rendering is done using OpenGL.

#### Fields

§`get_proc_address: &'a dynFn(&CStr) ->*constc_void`Use this function pointer to obtain access to the OpenGL implementation - similar toeglGetProcAddress.

§
### WebGL

The rendering is done on a HTML Canvas element using WebGL.

#### Fields

§`canvas_element_id: &'astr`The DOM element id of the HTML Canvas element used for rendering.

§`context_type: &'astr`The drawing context type used on the HTML Canvas element for rendering. This is the argument to thegetContextfunction on the HTML Canvas element.

§
### #[non_exhaustive]WGPU28

Available on
**crate featureunstable-wgpu-28**only.
The rendering is based on WGPU 28.x. Use the provided fields to submit commits to the provided
WGPU command queue.

Note: This function is behind theunstable-wgpu-28feature flagand may be removed or changed in future minor releases, as new major WGPU releases become available.

See also theslint::wgpu_28module.

#### Fields

§`instance:Instance`The WGPU instance used for rendering.

§`device:Device`The WGPU device used for rendering.

§`queue:Queue`The WGPU queue for used for command submission.

## Trait Implementations

## Auto Trait Implementations

§
### impl<'a>FreezeforGraphicsAPI<'a>

§
### impl<'a> !RefUnwindSafeforGraphicsAPI<'a>

§
### impl<'a> !SendforGraphicsAPI<'a>

§
### impl<'a> !SyncforGraphicsAPI<'a>

§
### impl<'a>UnpinforGraphicsAPI<'a>

§
### impl<'a>UnsafeUnpinforGraphicsAPI<'a>

§
### impl<'a> !UnwindSafeforGraphicsAPI<'a>

## Blanket Implementations

[🔝 Наверх](#-оглавление)
<a id="enumplatformerrorcopy-item-path"></a>

## `EnumPlatformErrorCopy item path`
---
```rust
#[non_exhaustive]pub enum PlatformError {
    NoPlatform,
    NoEventLoopProvider,
    SetPlatformError(SetPlatformError),
    Other(String),
    OtherError(Box<dyn Error + Send + Sync>),
}
```

## Variants (Non-exhaustive)

§
### NoPlatform

No default platform was selected, or no platform could be initialized.

If you encounter this error, make sure to either selected trough thebackend-*cargo features flags,
or callplatform::set_platform()before running the event loop

§
### NoEventLoopProvider

The Slint Platform does not provide an event loop.

ThePlatform::run_event_loopis not implemented for the current platform.

§
### SetPlatformError(SetPlatformError)

There is already a platform set from another thread.

§
### Other(String)

Another platform-specific error occurred

§
### OtherError(Box<dynError+Send+Sync>)

Available on
**crate featurestd**only.
Another platform-specific error occurred.

## Trait Implementations

## Auto Trait Implementations

§
### implFreezeforPlatformError

§
### impl !RefUnwindSafeforPlatformError

§
### implSendforPlatformError

§
### implSyncforPlatformError

§
### implUnpinforPlatformError

§
### implUnsafeUnpinforPlatformError

§
### impl !UnwindSafeforPlatformError

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="enumrenderingstatecopy-item-path"></a>

## `EnumRenderingStateCopy item path`
---
```rust
#[non_exhaustive]#[repr(u8)]pub enum RenderingState {
    RenderingSetup = 0,
    BeforeRendering = 1,
    AfterRendering = 2,
    RenderingTeardown = 3,
}
```

## Variants (Non-exhaustive)

§
### RenderingSetup = 0

The window has been created and the graphics adapter/context initialized.

§
### BeforeRendering = 1

The scene of items is about to be rendered.

§
### AfterRendering = 2

The scene of items was rendered, but the back buffer was not sent for display presentation
yet (for example GL swap buffers).

§
### RenderingTeardown = 3

The window will be destroyed and/or graphics resources need to be released due to other
constraints.

## Trait Implementations

## Auto Trait Implementations

§
### implFreezeforRenderingState

§
### implRefUnwindSafeforRenderingState

§
### implSendforRenderingState

§
### implSyncforRenderingState

§
### implUnpinforRenderingState

§
### implUnsafeUnpinforRenderingState

§
### implUnwindSafeforRenderingState

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="enumselectbundledtranslationerrorcopy-item-path"></a>

## `EnumSelectBundledTranslationErrorCopy item path`
---
```rust
pub enum SelectBundledTranslationError {
    LanguageNotFound {
        available_languages: SharedVector<SharedString>,
    },
    NoTranslationsBundled,
}
```

## Variants

§
### LanguageNotFound

The language was not found. The list of available languages is included in this error variant.

#### Fields

§`available_languages:SharedVector<SharedString>`§
### NoTranslationsBundled

There are no bundled translations. Eitherselect_bundled_translationwas called before creating a component,
or the application’s.slintfile was compiled without the bundle translation option.

## Trait Implementations

## Auto Trait Implementations

§
### implFreezeforSelectBundledTranslationError

§
### implRefUnwindSafeforSelectBundledTranslationError

§
### implSendforSelectBundledTranslationError

§
### implSyncforSelectBundledTranslationError

§
### implUnpinforSelectBundledTranslationError

§
### implUnsafeUnpinforSelectBundledTranslationError

§
### implUnwindSafeforSelectBundledTranslationError

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="enumsetrenderingnotifiererrorcopy-item-path"></a>

## `EnumSetRenderingNotifierErrorCopy item path`
---
```rust
#[non_exhaustive]#[repr(u8)]pub enum SetRenderingNotifierError {
    Unsupported = 0,
    AlreadySet = 1,
}
```

## Variants (Non-exhaustive)

§
### Unsupported = 0

The rendering backend does not support rendering notifiers.

§
### AlreadySet = 1

There is already a rendering notifier set, multiple notifiers are not supported.

## Trait Implementations

## Auto Trait Implementations

§
### implFreezeforSetRenderingNotifierError

§
### implRefUnwindSafeforSetRenderingNotifierError

§
### implSendforSetRenderingNotifierError

§
### implSyncforSetRenderingNotifierError

§
### implUnpinforSetRenderingNotifierError

§
### implUnsafeUnpinforSetRenderingNotifierError

§
### implUnwindSafeforSetRenderingNotifierError

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="enumtimermodecopy-item-path"></a>

## `EnumTimerModeCopy item path`
---
```rust
#[non_exhaustive]#[repr(u8)]pub enum TimerMode {
    SingleShot = 0,
    Repeated = 1,
}
```

## Variants (Non-exhaustive)

§
### SingleShot = 0

A SingleShot timer is fired only once.

§
### Repeated = 1

A Repeated timer is fired repeatedly until it is stopped or dropped.

## Trait Implementations

§
### implCopyforTimerMode

## Auto Trait Implementations

§
### implFreezeforTimerMode

§
### implRefUnwindSafeforTimerMode

§
### implSendforTimerMode

§
### implSyncforTimerMode

§
### implUnpinforTimerMode

§
### implUnsafeUnpinforTimerMode

§
### implUnwindSafeforTimerMode

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="enumwindowpositioncopy-item-path"></a>

## `EnumWindowPositionCopy item path`
---
```rust
pub enum WindowPosition {
    Physical(PhysicalPosition),
    Logical(LogicalPosition),
}
```

## Variants

§
### Physical(PhysicalPosition)

The position in physical pixels.

§
### Logical(LogicalPosition)

The position in logical pixels.

## Implementations

## Trait Implementations

§
### implStructuralPartialEqforWindowPosition

## Auto Trait Implementations

§
### implFreezeforWindowPosition

§
### implRefUnwindSafeforWindowPosition

§
### implSendforWindowPosition

§
### implSyncforWindowPosition

§
### implUnpinforWindowPosition

§
### implUnsafeUnpinforWindowPosition

§
### implUnwindSafeforWindowPosition

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="enumwindowsizecopy-item-path"></a>

## `EnumWindowSizeCopy item path`
---
```rust
pub enum WindowSize {
    Physical(PhysicalSize),
    Logical(LogicalSize),
}
```

## Variants

§
### Physical(PhysicalSize)

The size in physical pixels.

§
### Logical(LogicalSize)

The size in logical screen pixels.

## Implementations

## Trait Implementations

§
### implStructuralPartialEqforWindowSize

## Auto Trait Implementations

§
### implFreezeforWindowSize

§
### implRefUnwindSafeforWindowSize

§
### implSendforWindowSize

§
### implSyncforWindowSize

§
### implUnpinforWindowSize

§
### implUnsafeUnpinforWindowSize

§
### implUnwindSafeforWindowSize

## Blanket Implementations

§
### impl<T>WasmNotSendfor Twhere
    T:Send,

§
### impl<T>WasmNotSendSyncfor Twhere
    T:WasmNotSend+WasmNotSync,

§
### impl<T>WasmNotSyncfor Twhere
    T:Sync,

[🔝 Наверх](#-оглавление)
<a id="traitcomponenthandlecopy-item-path"></a>

## `TraitComponentHandleCopy item path`
---
```rust
pub trait ComponentHandle {
    // Required methods
    fn as_weak(&self) -> Weak<Self>
       where Self: Sized;
    fn clone_strong(&self) -> Self;
    fn show(&self) -> Result<(), PlatformError>;
    fn hide(&self) -> Result<(), PlatformError>;
    fn window(&self) -> &Window;
    fn run(&self) -> Result<(), PlatformError>;
    fn global<'a, T>(&'a self) -> T
       where T: Global<'a, Self>,
             Self: Sized;
}
```

## Required Methods

## Dyn Compatibility

This trait isnotdyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

## Implementors

§
### implComponentHandleforSampleComponent

[🔝 Наверх](#-оглавление)
<a id="traitglobalcopy-item-path"></a>

## `TraitGlobalCopy item path`
---
```rust
pub trait Global<'a, Component> {
    // Required method
    fn get(component: &'a Component) -> Self;
}
```

## Required Methods

## Dyn Compatibility

This trait isnotdyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

## Implementors

[🔝 Наверх](#-оглавление)
<a id="traitmodelcopy-item-path"></a>

## `TraitModelCopy item path`
---
```rust
pub trait Model {
    type Data;

    // Required methods
    fn row_count(&self) -> usize;
    fn row_data(&self, row: usize) -> Option<Self::Data>;
    fn model_tracker(&self) -> &dyn ModelTracker;

    // Provided methods
    fn set_row_data(&self, _row: usize, _data: Self::Data) { ... }
    fn iter(&self) -> ModelIterator<'_, Self::Data> ⓘ
       where Self: Sized { ... }
    fn as_any(&self) -> &(dyn Any + 'static) { ... }
}
```

## Required Associated Types

## Required Methods

## Provided Methods

## Implementations on Foreign Types

## Implementors

[🔝 Наверх](#-оглавление)
<a id="traitmodelextcopy-item-path"></a>

## `TraitModelExtCopy item path`
---
```rust
pub trait ModelExt: Model {
    // Provided methods
    fn row_data_tracked(&self, row: usize) -> Option<Self::Data> { ... }
    fn map<F, U>(self, map_function: F) -> MapModel<Self, F>
       where Self: Sized + 'static,
             F: Fn(Self::Data) -> U + 'static { ... }
    fn filter<F>(self, filter_function: F) -> FilterModel<Self, F>
       where Self: Sized + 'static,
             F: Fn(&Self::Data) -> bool + 'static { ... }
    fn sort(self) -> SortModel<Self, AscendingSortHelper>
       where Self: Sized + 'static,
             Self::Data: Ord { ... }
    fn sort_by<F>(self, sort_function: F) -> SortModel<Self, F>
       where Self: Sized + 'static,
             F: FnMut(&Self::Data, &Self::Data) -> Ordering + 'static { ... }
    fn reverse(self) -> ReverseModel<Self>
       where Self: Sized + 'static { ... }
}
```

## Provided Methods

## Implementors

§
### impl<T>ModelExtfor Twhere
    T:Model,

[🔝 Наверх](#-оглавление)
<a id="traitmodeltrackercopy-item-path"></a>

## `TraitModelTrackerCopy item path`
---
```rust
pub trait ModelTracker {
    // Required methods
    fn attach_peer(&self, peer: ModelPeer<'_>);
    fn track_row_count_changes(&self);
    fn track_row_data_changes(&self, row: usize);
}
```

## Required Methods

## Implementations on Foreign Types

## Implementors

§
### implModelTrackerforModelNotify

[🔝 Наверх](#-оглавление)
<a id="traittosharedstringcopy-item-path"></a>

## `TraitToSharedStringCopy item path`
---
```rust
pub trait ToSharedString {
    // Required method
    fn to_shared_string(&self) -> SharedString;
}
```

## Required Methods

## Implementors

§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[🔝 Наверх](#-оглавление)
<a id="functioninvoke-from-event-loopcopy-item-path"></a>

## `Functioninvoke_from_event_loopCopy item path`
---
```rust
pub fn invoke_from_event_loop(
    func: impl FnOnce() + Send + 'static,
) -> Result<(), EventLoopError>
```

[🔝 Наверх](#-оглавление)
<a id="functionquit-event-loopcopy-item-path"></a>

## `Functionquit_event_loopCopy item path`
---
```rust
pub fn quit_event_loop() -> Result<(), EventLoopError>
```

[🔝 Наверх](#-оглавление)
<a id="functionrun-event-loopcopy-item-path"></a>

## `Functionrun_event_loopCopy item path`
---
```rust
pub fn run_event_loop() -> Result<(), PlatformError>
```

[🔝 Наверх](#-оглавление)
<a id="functionrun-event-loop-until-quitcopy-item-path"></a>

## `Functionrun_event_loop_until_quitCopy item path`
---
```rust
pub fn run_event_loop_until_quit() -> Result<(), PlatformError>
```

[🔝 Наверх](#-оглавление)
<a id="functionselect-bundled-translationcopy-item-path"></a>

## `Functionselect_bundled_translationCopy item path`
---
```rust
pub fn select_bundled_translation(
    language: &str,
) -> Result<(), SelectBundledTranslationError>
```

[🔝 Наверх](#-оглавление)
<a id="functionset-xdg-app-idcopy-item-path"></a>

## `Functionset_xdg_app_idCopy item path`
---
```rust
pub fn set_xdg_app_id(
    app_id: impl Into<SharedString>,
) -> Result<(), PlatformError>
```

[🔝 Наверх](#-оглавление)
<a id="functionspawn-localcopy-item-path"></a>

## `Functionspawn_localCopy item path`
---
```rust
pub fn spawn_local<F: Future + 'static>(
    fut: F,
) -> Result<JoinHandle<F::Output>, EventLoopError>
```

Available on
**target_has_atomic=ptr**only.

[🔝 Наверх](#-оглавление)
<a id="type-aliasrgb8pixelcopy-item-path"></a>

## `Type AliasRgb8PixelCopy item path`
---
```rust
pub type Rgb8Pixel = Rgb<u8>;
```

## Aliased Type

```rust
#[repr(C)]pub struct Rgb8Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
```

## Fields

§`r:u8`Red Component

§`g:u8`Green Component

§`b:u8`Blue Component

[🔝 Наверх](#-оглавление)
<a id="type-aliasrgba8pixelcopy-item-path"></a>

## `Type AliasRgba8PixelCopy item path`
---
```rust
pub type Rgba8Pixel = Rgba<u8>;
```

## Aliased Type

```rust
#[repr(C)]pub struct Rgba8Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
```

## Fields

§`r:u8`Red Component

§`g:u8`Green Component

§`b:u8`Blue Component

§`a:u8`Alpha Component

[🔝 Наверх](#-оглавление)

---
> ✅ Скачано: **67/67** страниц
