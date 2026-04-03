# Документация smithay_client_toolkit v0.20.0

> Источник: [https://docs.rs/smithay-client-toolkit/0.20.0/smithay_client_toolkit/](https://docs.rs/smithay-client-toolkit/0.20.0/smithay_client_toolkit/)
> Сгенерировано: 2026-04-03 12:20:08

---

## 📑 Оглавление
### Модули
- [`activation`](#activation)
- [`compositor`](#compositor)
- [`data_device_manager`](#data-device-manager)
- [`dmabuf`](#dmabuf)
- [`error`](#error)
- [`foreign_toplevel_list`](#foreign-toplevel-list)
- [`globals`](#globals)
- [`output`](#output)
- [`presentation_time`](#presentation-time)
- [`primary_selection`](#primary-selection)
- [`reexports`](#reexports)
- [`registry`](#registry)
- [`seat`](#seat)
- [`session_lock`](#session-lock)
- [`shell`](#shell)
- [`shm`](#shm)
- [`subcompositor`](#subcompositor)

### Макросы
- [`delegate_activation`](#delegate-activation)
- [`delegate_compositor`](#delegate-compositor)
- [`delegate_data_device`](#delegate-data-device)
- [`delegate_dmabuf`](#delegate-dmabuf)
- [`delegate_foreign_toplevel_list`](#delegate-foreign-toplevel-list)
- [`delegate_input_method`](#delegate-input-method)
- [`delegate_input_method_v3`](#delegate-input-method-v3)
- [`delegate_keyboard`](#delegate-keyboard)
- [`delegate_layer`](#delegate-layer)
- [`delegate_output`](#delegate-output)
- [`delegate_pointer`](#delegate-pointer)
- [`delegate_pointer_constraints`](#delegate-pointer-constraints)
- [`delegate_presentation_time`](#delegate-presentation-time)
- [`delegate_primary_selection`](#delegate-primary-selection)
- [`delegate_registry`](#delegate-registry)
- [`delegate_relative_pointer`](#delegate-relative-pointer)
- [`delegate_seat`](#delegate-seat)
- [`delegate_session_lock`](#delegate-session-lock)
- [`delegate_shm`](#delegate-shm)
- [`delegate_simple`](#delegate-simple)
- [`delegate_subcompositor`](#delegate-subcompositor)
- [`delegate_touch`](#delegate-touch)
- [`delegate_xdg_popup`](#delegate-xdg-popup)
- [`delegate_xdg_shell`](#delegate-xdg-shell)
- [`delegate_xdg_window`](#delegate-xdg-window)
- [`registry_handlers`](#registry-handlers)

---
<a id="moduleactivationcopy-item-path"></a>

## `ModuleactivationCopy item path`
---
## Structs

- **`ActivationState`**: State for xdg-activation
- **`RequestData`**: Minimal implementation ofRequestDataExt.

## Traits

- **`ActivationHandler`**: Handler for xdg-activation
- **`RequestDataExt`**: Data attached to a token request

[🔝 Наверх](#-оглавление)
<a id="modulecompositorcopy-item-path"></a>

## `ModulecompositorCopy item path`
---
## Structs

- **`CompositorState`**: A trivial wrapper around aWlRegion.
- **`Region`**: A trivial wrapper around aWlRegion.
- **`Surface`**: An ownedWlSurface.
- **`SurfaceData`**: Data associated with aWlSurface.

## Traits

- **`CompositorHandler`**
- **`SurfaceDataExt`**

[🔝 Наверх](#-оглавление)
<a id="moduledata-device-managercopy-item-path"></a>

## `Moduledata_device_managerCopy item path`
---
## Modules

- **`data_device`**
- **`data_offer`**
- **`data_source`**

## Structs

- **`DataDeviceManagerState`**: If thecalloopcargo feature is enabled, this can be used
as anEventSourcein a calloop event loop.
- **`ReadPipe`**: If thecalloopcargo feature is enabled, this can be used
as anEventSourcein a calloop event loop.
- **`WritePipe`**: If thecalloopcargo feature is enabled, this can be used
as anEventSourcein a calloop event loop.

[🔝 Наверх](#-оглавление)
<a id="moduledmabufcopy-item-path"></a>

## `ModuledmabufCopy item path`
---
## Structs

- **`DmabufFeedback`**: Description of supported and preferred dmabuf formats
- **`DmabufFeedbackTranche`**: A preference tranche of dmabuf formats
- **`DmabufFormat`**: A single dmabuf format/modifier pair
- **`DmabufParams`**: Builder for a dmabuf backed buffer
- **`DmabufState`**: A handler forzwp_linux_dmabuf_v1::ZwpLinuxDmabufV1

## Traits

- **`DmabufHandler`**

[🔝 Наверх](#-оглавление)
<a id="moduleerrorcopy-item-path"></a>

## `ModuleerrorCopy item path`
---
## Enums

- **`GlobalError`**: An error that may occur when creating objects using a global.

[🔝 Наверх](#-оглавление)
<a id="moduleforeign-toplevel-listcopy-item-path"></a>

## `Moduleforeign_toplevel_listCopy item path`
---
## Structs

- **`ForeignToplevelInfo`**: Information about a toplevel.
- **`ForeignToplevelList`**

## Traits

- **`ForeignToplevelListHandler`**: Handler trait for foreign toplevel list protocol.

[🔝 Наверх](#-оглавление)
<a id="moduleglobalscopy-item-path"></a>

## `ModuleglobalsCopy item path`
---
## Structs

- **`GlobalData`**: A struct used as the UserData field for globals bound by SCTK.

## Traits

- **`ProvidesBoundGlobal`**: A trait implemented by types that provide access to capability globals.

[🔝 Наверх](#-оглавление)
<a id="moduleoutputcopy-item-path"></a>

## `ModuleoutputCopy item path`
---
## Structs

- **`Mode`**: Information about an output.
- **`OutputData`**: Information about an output.
- **`OutputInfo`**: Information about an output.
- **`OutputState`**: A handler for delegatingwl_output::WlOutput.
- **`ScaleWatcherHandle`**

## Traits

- **`OutputHandler`**: Simplified event handler forwl_output::WlOutput.
SeeOutputState.

[🔝 Наверх](#-оглавление)
<a id="modulepresentation-timecopy-item-path"></a>

## `Modulepresentation_timeCopy item path`
---
## Structs

- **`PresentTime`**
- **`PresentationTimeState`**

## Traits

- **`PresentationTimeHandler`**

[🔝 Наверх](#-оглавление)
<a id="moduleprimary-selectioncopy-item-path"></a>

## `Moduleprimary_selectionCopy item path`
---
## Modules

- **`device`**
- **`offer`**
- **`selection`**

## Structs

- **`PrimarySelectionManagerState`**

[🔝 Наверх](#-оглавление)
<a id="modulereexportscopy-item-path"></a>

## `ModulereexportsCopy item path`
---
Expand description
Re-exports of some crates, for convenience.

## Re-exports

- **`pub usecalloop;calloop`**
- **`pub usecalloop_wayland_source;calloop`**
- **`pub usewayland_clientas client;`**
- **`pub usewayland_csd_frameas csd_frame;`**
- **`pub usewayland_protocolsas protocols;`**
- **`pub usewayland_protocols_experimentalas protocols_experimental;`**
- **`pub usewayland_protocols_miscas protocols_misc;`**
- **`pub usewayland_protocols_wlras protocols_wlr;`**

[🔝 Наверх](#-оглавление)
<a id="moduleregistrycopy-item-path"></a>

## `ModuleregistryCopy item path`
---
Expand description
Utilities for binding globals withwl_registryin delegates.

This module is based around theRegistryHandlertrait andRegistryState.

RegistryStateprovides an interface to bind globals regularly, creating an object with each new
instantiation or caching bound globals to prevent duplicate object instances from being created. Binding
a global regularly is accomplished throughRegistryState::bind_one.

Thedelegate_registrymacro is used to implement handling forwl_registry.

### Sample implementation ofRegistryHandler

```
use smithay_client_toolkit::reexports::client::{
    Connection, Dispatch, QueueHandle,
    delegate_dispatch,
    globals::GlobalList,
    protocol::wl_output,
};

use smithay_client_toolkit::registry::{
    GlobalProxy, ProvidesRegistryState, RegistryHandler, RegistryState,
};

struct ExampleApp {
    /// The registry state is needed to use the global abstractions.
    registry_state: RegistryState,
    /// This is a type we want to delegate global handling to.
    delegate_that_wants_registry: Delegate,
}

/// The delegate a global should be provided to.
struct Delegate {
    outputs: Vec<wl_output::WlOutput>,
}

// When implementing RegistryHandler, you must be able to dispatch any type you could bind using the registry state.
impl<D> RegistryHandler<D> for Delegate
where
    // In order to bind a global, you must statically assert the global may be handled with the data type.
    D: Dispatch<wl_output::WlOutput, ()>
        // ProvidesRegistryState provides a function to access the RegistryState within the impl.
        + ProvidesRegistryState
        // We need some way to access our part of the application's state.  This uses AsMut,
        // but you may prefer to create your own trait to avoid making .as_mut() ambiguous.
        + AsMut<Delegate>
        + 'static,
{
  /// New global added after initial enumeration.
   fn new_global(
       data: &mut D,
       conn: &Connection,
       qh: &QueueHandle<D>,
       name: u32,
       interface: &str,
       version: u32,
   ) {
        if interface == "wl_output" {
            // Bind `wl_output` with newest version from 1 to 4 the compositor supports
            let output = data.registry().bind_specific(qh, name, 1..=4, ()).unwrap();
            data.as_mut().outputs.push(output);
        }

        // You could either handle errors here or when attempting to use the interface.  Most
        // Wayland protocols are optional, so if your application can function without a
        // protocol it should try to do so; the From impl of GlobalProxy is written to make
        // this straightforward.
    }
}
```

## Structs

- **`RegistryState`**: State object associated with the registry handling for smithay’s client toolkit.
- **`SimpleGlobal`**

## Enums

- **`GlobalProxy`**: A helper for storing a bound global.

## Traits

- **`ProvidesRegistryState`**: Trait which asserts a data type may provide a mutable reference to the registry state.
- **`RegistryHandler`**: A trait implemented by modular parts of a smithay’s client toolkit and protocol delegates that may be used
to receive notification of a global being created or destroyed.

[🔝 Наверх](#-оглавление)
<a id="moduleseatcopy-item-path"></a>

## `ModuleseatCopy item path`
---
## Modules

- **`input_method`**: Implementation of theinput-method-unstable-v2protocol.
- **`input_method_v3`**: This implements support for the experimental xx-input-method-v2 protocol.
That protocol will hopefully become -v3 without changing the API at some point.
- **`keyboardxkbcommon`**
- **`pointer`**
- **`pointer_constraints`**
- **`relative_pointer`**
- **`touch`**

## Structs

- **`SeatData`**: Description of a seat.
- **`SeatInfo`**: Description of a seat.
- **`SeatState`**

## Enums

- **`Capability`**
- **`SeatError`**

## Traits

- **`SeatHandler`**

[🔝 Наверх](#-оглавление)
<a id="modulesession-lockcopy-item-path"></a>

## `Modulesession_lockCopy item path`
---
## Structs

- **`SessionLock`**: A session lock
- **`SessionLockData`**: A handler forext_session_lock_manager_v1::ExtSessionLockManagerV1
- **`SessionLockInner`**: A handler forext_session_lock_manager_v1::ExtSessionLockManagerV1
- **`SessionLockState`**: A handler forext_session_lock_manager_v1::ExtSessionLockManagerV1
- **`SessionLockSurface`**
- **`SessionLockSurfaceConfigure`**
- **`SessionLockSurfaceData`**

## Traits

- **`SessionLockHandler`**: Handler trait for session lock protocol.

[🔝 Наверх](#-оглавление)
<a id="moduleshellcopy-item-path"></a>

## `ModuleshellCopy item path`
---
Expand description

## Shell abstractions

A shell describes a set of wayland protocol extensions which define the capabilities of a surface and how
the surface is displayed.

### Cross desktop group (XDG) shell

The XDG shell describes the semantics of desktop application windows.

The XDG shell defines two types of surfaces:

- Window- An application window1.
- Popup- A child surface positioned relative to a window.

#### Why use the XDG shell

The XDG shell is the primary protocol through which application windows are created. You can be near
certain every desktop compositor will implement this shell so that applications may create windows.

See theXDG shell module documentationfor more information about creating application windows.

### Layer shell

The layer shell is a protocol which allows the creation of “layers”. A layer refers to a surface rendered
at some specific z-depth relative to other layers. A layer may also be anchored to some edge and corner of
the screen.

The layer shell defines one type of surface: thewlr_layer::LayerSurface.

There is no guarantee that the layer shell will be available in every compositor.

#### Why use the layer shell

The layer shell may be used to implement many desktop shell components, such as backgrounds, docks and
launchers.

1. The XDG shell protocol actually refers to a window as a toplevel surface, but we use the more
familiar term “window” for the sake of clarity.↩

## Modules

- **`wlr_layer`**: Cross desktop group (XDG) shell
- **`xdg`**: Cross desktop group (XDG) shell

## Structs

- **`Unsupported`**: An unsupported operation, often due to the version of the protocol.

## Traits

- **`WaylandSurface`**: Functionality shared by allwl_surface::WlSurfacebacked shell role objects.

[🔝 Наверх](#-оглавление)
<a id="moduleshmcopy-item-path"></a>

## `ModuleshmCopy item path`
---
## Modules

- **`multi`**: A pool implementation which automatically manage buffers.
- **`raw`**: A raw shared memory pool handler.
- **`slot`**: A pool implementation based on buffer slots

## Structs

- **`Shm`**

## Enums

- **`CreatePoolError`**: An error that may occur when creating a pool.

## Traits

- **`ShmHandler`**

[🔝 Наверх](#-оглавление)
<a id="modulesubcompositorcopy-item-path"></a>

## `ModulesubcompositorCopy item path`
---
## Structs

- **`SubcompositorState`**: The data assoctiated with the subsurface.
- **`SubsurfaceData`**: The data assoctiated with the subsurface.

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-activationcopy-item-path"></a>

## `Macrodelegate_activationCopy item path`
---
```
macro_rules! delegate_activation {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty, $data: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-compositorcopy-item-path"></a>

## `Macrodelegate_compositorCopy item path`
---
```
macro_rules! delegate_compositor {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty, surface: [$($surface: ty),*$(,)?]) => { ... };
    (@{$($ty:tt)*}; surface: []) => { ... };
    (@{$($ty:tt)*}; surface-only: $surface:ty) => { ... };
    (@$ty:tt; surface: [ $($surface:ty),+ ]) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-data-devicecopy-item-path"></a>

## `Macrodelegate_data_deviceCopy item path`
---
```
macro_rules! delegate_data_device {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-dmabufcopy-item-path"></a>

## `Macrodelegate_dmabufCopy item path`
---
```
macro_rules! delegate_dmabuf {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-foreign-toplevel-listcopy-item-path"></a>

## `Macrodelegate_foreign_toplevel_listCopy item path`
---
```
macro_rules! delegate_foreign_toplevel_list {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-input-methodcopy-item-path"></a>

## `Macrodelegate_input_methodCopy item path`
---
```
macro_rules! delegate_input_method {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-input-method-v3copy-item-path"></a>

## `Macrodelegate_input_method_v3Copy item path`
---
```
macro_rules! delegate_input_method_v3 {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-keyboardcopy-item-path"></a>

## `Macrodelegate_keyboardCopy item path`
---
```
macro_rules! delegate_keyboard {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty, keyboard: [$($udata:ty),* $(,)?]) => { ... };
}
```

Available on
**crate featurexkbcommon**only.

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-layercopy-item-path"></a>

## `Macrodelegate_layerCopy item path`
---
```
macro_rules! delegate_layer {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-outputcopy-item-path"></a>

## `Macrodelegate_outputCopy item path`
---
```
macro_rules! delegate_output {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-pointercopy-item-path"></a>

## `Macrodelegate_pointerCopy item path`
---
```
macro_rules! delegate_pointer {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty, pointer: [$($pointer_data:ty),* $(,)?]) => { ... };
    (@{$($ty:tt)*}; pointer: []) => { ... };
    (@{$($ty:tt)*}; pointer-only: $pointer_data:ty) => { ... };
    (@$ty:tt; pointer: [$($pointer:ty),*]) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-pointer-constraintscopy-item-path"></a>

## `Macrodelegate_pointer_constraintsCopy item path`
---
```
macro_rules! delegate_pointer_constraints {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-presentation-timecopy-item-path"></a>

## `Macrodelegate_presentation_timeCopy item path`
---
```
macro_rules! delegate_presentation_time {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-primary-selectioncopy-item-path"></a>

## `Macrodelegate_primary_selectionCopy item path`
---
```
macro_rules! delegate_primary_selection {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-registrycopy-item-path"></a>

## `Macrodelegate_registryCopy item path`
---
```
macro_rules! delegate_registry {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

Expand description
Delegates the handling ofwl_registry.

Anything which implementsRegistryHandlermay be used in the delegate.

### Usage

```
use smithay_client_toolkit::{
    delegate_registry, delegate_shm, registry_handlers,
    shm::{ShmHandler, Shm},
};

struct ExampleApp {
    shm_state: Shm,
}

// Here is the implementation of wl_shm to compile:
delegate_shm!(ExampleApp);

impl ShmHandler for ExampleApp {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm_state
    }
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-relative-pointercopy-item-path"></a>

## `Macrodelegate_relative_pointerCopy item path`
---
```
macro_rules! delegate_relative_pointer {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-seatcopy-item-path"></a>

## `Macrodelegate_seatCopy item path`
---
```
macro_rules! delegate_seat {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-session-lockcopy-item-path"></a>

## `Macrodelegate_session_lockCopy item path`
---
```
macro_rules! delegate_session_lock {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-shmcopy-item-path"></a>

## `Macrodelegate_shmCopy item path`
---
```
macro_rules! delegate_shm {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

Expand description
Delegates the handling ofwl_shmto someShm.

This macro requires two things, the type that will delegate toShmand a closure specifying how
to obtain the state object.

```
use smithay_client_toolkit::shm::{ShmHandler, Shm};
use smithay_client_toolkit::delegate_shm;

struct ExampleApp {
    /// The state object that will be our delegate.
    shm: Shm,
}

// Use the macro to delegate wl_shm to Shm.
delegate_shm!(ExampleApp);

// You must implement the ShmHandler trait to provide a way to access the Shm from your data type.
impl ShmHandler for ExampleApp {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-simplecopy-item-path"></a>

## `Macrodelegate_simpleCopy item path`
---
```
macro_rules! delegate_simple {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty:ty, $iface:ty, $max:expr) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-subcompositorcopy-item-path"></a>

## `Macrodelegate_subcompositorCopy item path`
---
```
macro_rules! delegate_subcompositor {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty, subsurface: [$($subsurface: ty),*$(,)?]) => { ... };
    (@{$($ty:tt)*}; subsurface: []) => { ... };
    (@{$($ty:tt)*}; subsurface-only: $subsurface:ty) => { ... };
    (@$ty:tt; subsurface: [ $($subsurface:ty),+ ]) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-touchcopy-item-path"></a>

## `Macrodelegate_touchCopy item path`
---
```
macro_rules! delegate_touch {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty, touch: [$($td:ty),* $(,)?]) => { ... };
    (@{$($ty:tt)*}; touch: $td:ty) => { ... };
    (@$ty:tt; [$($td:ty),*] ) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-xdg-popupcopy-item-path"></a>

## `Macrodelegate_xdg_popupCopy item path`
---
```
macro_rules! delegate_xdg_popup {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-xdg-shellcopy-item-path"></a>

## `Macrodelegate_xdg_shellCopy item path`
---
```
macro_rules! delegate_xdg_shell {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macrodelegate-xdg-windowcopy-item-path"></a>

## `Macrodelegate_xdg_windowCopy item path`
---
```
macro_rules! delegate_xdg_window {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => { ... };
}
```

[🔝 Наверх](#-оглавление)
<a id="macroregistry-handlerscopy-item-path"></a>

## `Macroregistry_handlersCopy item path`
---
```
macro_rules! registry_handlers {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $($ty:ty),* $(,)?) => { ... };
}
```

Expand description
A helper macro for implementingProvidesRegistryState.

Seedelegate_registryfor an example.

[🔝 Наверх](#-оглавление)

---
> ✅ 43/43 страниц скачано успешно
