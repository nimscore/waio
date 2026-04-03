# Документация slint_interpreter v1.15.1

> Источник: [https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/)
> Сгенерировано: 2026-04-03 10:27:50

---

## Оглавление
- [struct] `struct.BackendSelector` — [`struct.BackendSelector.html`](#struct.backendselector)
- [struct] `struct.Color` — [`struct.Color.html`](#struct.color)
- [struct] `struct.CompilationResult` — [`struct.CompilationResult.html`](#struct.compilationresult)
- [struct] `struct.Compiler` — [`struct.Compiler.html`](#struct.compiler)
- [struct] `struct.ComponentCompiler` — [`struct.ComponentCompiler.html`](#struct.componentcompiler)
- [struct] `struct.ComponentDefinition` — [`struct.ComponentDefinition.html`](#struct.componentdefinition)
- [struct] `struct.ComponentInstance` — [`struct.ComponentInstance.html`](#struct.componentinstance)
- [struct] `struct.Diagnostic` — [`struct.Diagnostic.html`](#struct.diagnostic)
- [struct] `struct.Image` — [`struct.Image.html`](#struct.image)
- [struct] `struct.JoinHandle` — [`struct.JoinHandle.html`](#struct.joinhandle)
- [struct] `struct.LoadImageError` — [`struct.LoadImageError.html`](#struct.loadimageerror)
- [struct] `struct.LogicalPosition` — [`struct.LogicalPosition.html`](#struct.logicalposition)
- [struct] `struct.LogicalSize` — [`struct.LogicalSize.html`](#struct.logicalsize)
- [struct] `struct.OklchColor` — [`struct.OklchColor.html`](#struct.oklchcolor)
- [struct] `struct.PhysicalPosition` — [`struct.PhysicalPosition.html`](#struct.physicalposition)
- [struct] `struct.PhysicalSize` — [`struct.PhysicalSize.html`](#struct.physicalsize)
- [struct] `struct.RgbaColor` — [`struct.RgbaColor.html`](#struct.rgbacolor)
- [struct] `struct.SharedPixelBuffer` — [`struct.SharedPixelBuffer.html`](#struct.sharedpixelbuffer)
- [struct] `struct.SharedString` — [`struct.SharedString.html`](#struct.sharedstring)
- [struct] `struct.SharedVector` — [`struct.SharedVector.html`](#struct.sharedvector)
- [struct] `struct.Struct` — [`struct.Struct.html`](#struct.struct)
- [struct] `struct.Weak` — [`struct.Weak.html`](#struct.weak)
- [struct] `struct.Window` — [`struct.Window.html`](#struct.window)
- [struct] `struct.WindowHandle` — [`struct.WindowHandle.html`](#struct.windowhandle)
- [enum] `enum.Brush` — [`enum.Brush.html`](#enum.brush)
- [enum] `enum.CloseRequestResponse` — [`enum.CloseRequestResponse.html`](#enum.closerequestresponse)
- [enum] `enum.DefaultTranslationContext` — [`enum.DefaultTranslationContext.html`](#enum.defaulttranslationcontext)
- [enum] `enum.DiagnosticLevel` — [`enum.DiagnosticLevel.html`](#enum.diagnosticlevel)
- [enum] `enum.EventLoopError` — [`enum.EventLoopError.html`](#enum.eventlooperror)
- [enum] `enum.GetPropertyError` — [`enum.GetPropertyError.html`](#enum.getpropertyerror)
- [enum] `enum.GraphicsAPI` — [`enum.GraphicsAPI.html`](#enum.graphicsapi)
- [enum] `enum.InvokeError` — [`enum.InvokeError.html`](#enum.invokeerror)
- [enum] `enum.PlatformError` — [`enum.PlatformError.html`](#enum.platformerror)
- [enum] `enum.RenderingState` — [`enum.RenderingState.html`](#enum.renderingstate)
- [enum] `enum.SetCallbackError` — [`enum.SetCallbackError.html`](#enum.setcallbackerror)
- [enum] `enum.SetPropertyError` — [`enum.SetPropertyError.html`](#enum.setpropertyerror)
- [enum] `enum.SetRenderingNotifierError` — [`enum.SetRenderingNotifierError.html`](#enum.setrenderingnotifiererror)
- [enum] `enum.Value` — [`enum.Value.html`](#enum.value)
- [enum] `enum.ValueType` — [`enum.ValueType.html`](#enum.valuetype)
- [enum] `enum.WindowPosition` — [`enum.WindowPosition.html`](#enum.windowposition)
- [enum] `enum.WindowSize` — [`enum.WindowSize.html`](#enum.windowsize)
- [trait] `trait.ComponentHandle` — [`trait.ComponentHandle.html`](#trait.componenthandle)
- [trait] `trait.Global` — [`trait.Global.html`](#trait.global)
- [trait] `trait.ToSharedString` — [`trait.ToSharedString.html`](#trait.tosharedstring)
- [fn] `fn.invoke_from_event_loop` — [`fn.invoke_from_event_loop.html`](#fn.invoke-from-event-loop)
- [fn] `fn.print_diagnostics` — [`fn.print_diagnostics.html`](#fn.print-diagnostics)
- [fn] `fn.quit_event_loop` — [`fn.quit_event_loop.html`](#fn.quit-event-loop)
- [fn] `fn.run_event_loop` — [`fn.run_event_loop.html`](#fn.run-event-loop)
- [fn] `fn.set_xdg_app_id` — [`fn.set_xdg_app_id.html`](#fn.set-xdg-app-id)
- [fn] `fn.spawn_local` — [`fn.spawn_local.html`](#fn.spawn-local)
- [macro] `macro.format` — [`macro.format.html`](#macro.format)
- [testing/index] `index` — [`testing/index.html`](#index)
- [type] `type.Rgb8Pixel` — [`type.Rgb8Pixel.html`](#type.rgb8pixel)
- [type] `type.Rgba8Pixel` — [`type.Rgba8Pixel.html`](#type.rgba8pixel)

---
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructBackendSelectorCopy item path

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#33)
```
pub struct BackendSelector { /* private fields */ }
```

Expand description
Use the BackendSelector to configure one of Slint’s built-inbackends with a rendererto accommodate specific needs of your application. This is a programmatic substitute for
theSLINT_BACKENDenvironment variable.

For example, to configure Slint to use a renderer that supports OpenGL ES 3.0, configure
theBackendSelectoras follows:

```
let selector = BackendSelector::new().require_opengl_es_with_version(3, 0);
if let Err(err) = selector.select() {
    eprintln!("Error selecting backend with OpenGL ES support: {err}");
}
```

## Implementations§

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#55)§
### implBackendSelector

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#58)
#### pub fnnew() ->BackendSelector

Creates a new BackendSelector.

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#65)
#### pub fnrequire_opengl_es_with_version(
    self,
    major:u8,
    minor:u8,
) ->BackendSelector

Adds the requirement to the selector that the backend must render with OpenGL ES
and the specified major and minor version.

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#73)
#### pub fnrequire_opengl_es(self) ->BackendSelector

Adds the requirement to the selector that the backend must render with OpenGL ES.

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#80)
#### pub fnrequire_opengl(self) ->BackendSelector

Adds the requirement to the selector that the backend must render with OpenGL.

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#88)
#### pub fnrequire_opengl_with_version(
    self,
    major:u8,
    minor:u8,
) ->BackendSelector

Adds the requirement to the selector that the backend must render with OpenGL
and the specified major and minor version.

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#96)
#### pub fnrequire_metal(self) ->BackendSelector

Adds the requirement to the selector that the backend must render with Apple’s Metal framework.

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#103)
#### pub fnrequire_vulkan(self) ->BackendSelector

Adds the requirement to the selector that the backend must render with Vulkan.

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#110)
#### pub fnrequire_d3d(self) ->BackendSelector

Adds the requirement to the selector that the backend must render with Direct 3D.

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#242)
#### pub fnrenderer_name(self, name:String) ->BackendSelector

Adds the requirement that the selected renderer must match the given name. This is
equivalent to setting theSLINT_BACKEND=nameenvironment variable and requires
that the corresponding renderer feature is enabled. For example, to select the Skia renderer,
enable therenderer-skiafeature and call this function withskiaas argument.

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#252)
#### pub fnbackend_name(self, name:String) ->BackendSelector

Adds the requirement that the selected backend must match the given name. This is
equivalent to setting theSLINT_BACKEND=nameenvironment variable and requires
that the corresponding backend feature is enabled. For example, to select the winit backend,
enable thebackend-winitfeature and call this function withwinitas argument.

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#265)
#### pub fnselect(self) ->Result<(),PlatformError>

Completes the backend selection process and tries to combine with specified requirements
with the different backends and renderers enabled at compile time. On success, the selected
backend is automatically set to be active. Returns an error if the requirements could not be met.

## Trait Implementations§

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#32)§
### implDefaultforBackendSelector

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#32)§
#### fndefault() ->BackendSelector

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#408)§
### implDropforBackendSelector

[Source](https://docs.rs/i-slint-backend-selector/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_selector/api.rs.html#409)§
#### fndrop(&mut self)

Executes the destructor for this type.
[Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)
## Auto Trait Implementations§

§
### implFreezeforBackendSelector

§
### implRefUnwindSafeforBackendSelector

§
### implSendforBackendSelector

§
### implSyncforBackendSelector

§
### implUnpinforBackendSelector

§
### implUnsafeUnpinforBackendSelector

§
### implUnwindSafeforBackendSelector

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructColorCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#54)
```
pub struct Color { /* private fields */ }
```

Expand description
Color represents a color in the Slint run-time, represented using 8-bit channels for
red, green, blue and the alpha (opacity).
It can be conveniently converted using theto_andfrom_(a)rgb helper functions:

```
let col = some_color.to_argb_f32();
do_something_with_red_and_green(col.red, col.green);

let RgbaColor { red, blue, green, .. } = some_color.to_argb_u8();
do_something_with_red(red);

let new_col = Color::from(RgbaColor{ red: 0.5, green: 0.65, blue: 0.32, alpha: 1.});
```

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#169)§
### implColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#171)
#### pub const fnfrom_argb_encoded(encoded:u32) ->Color

Construct a color from an integer encoded as0xAARRGGBB

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#181)
#### pub fnas_argb_encoded(&self) ->u32

Returns(alpha, red, green, blue)encoded as u32

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#190)
#### pub const fnfrom_argb_u8(alpha:u8, red:u8, green:u8, blue:u8) ->Color

Construct a color from the alpha, red, green and blue color channel parameters.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#208)
#### pub const fnfrom_rgb_u8(red:u8, green:u8, blue:u8) ->Color

Construct a color from the red, green and blue color channel parameters. The alpha
channel will have the value 255.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#213)
#### pub fnfrom_argb_f32(alpha:f32, red:f32, green:f32, blue:f32) ->Color

Construct a color from the alpha, red, green and blue color channel parameters.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#219)
#### pub fnfrom_rgb_f32(red:f32, green:f32, blue:f32) ->Color

Construct a color from the red, green and blue color channel parameters. The alpha
channel will have the value 255.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#224)
#### pub fnto_argb_u8(&self) ->RgbaColor<u8>

Converts this color to an RgbaColor struct for easy destructuring.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#229)
#### pub fnto_argb_f32(&self) ->RgbaColor<f32>

Converts this color to an RgbaColor struct for easy destructuring.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#234)
#### pub fnto_hsva(&self) ->HsvaColor

Converts this color to the HSV color space.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#242)
#### pub fnfrom_hsva(hue:f32, saturation:f32, value:f32, alpha:f32) ->Color

Construct a color from the hue, saturation, and value HSV color space parameters.

Hue is between 0 and 360, the others parameters between 0 and 1.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#253)
#### pub fnto_oklch(&self) ->OklchColor

Converts this color to the Oklch color space.

Oklch is a perceptually uniform color space with:

- Lightness (L): 0 to 1
- Chroma (C): typically 0 to ~0.4
- Hue (h): 0 to 360 degrees

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#264)
#### pub fnfrom_oklch(lightness:f32, chroma:f32, hue:f32, alpha:f32) ->Color

Construct a color from the Oklch color space parameters.

- lightness: 0 to 1 (black to white)
- chroma: typically 0 to ~0.4 (grayscale to vivid)
- hue: 0 to 360 degrees
- alpha: 0 to 1

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#271)
#### pub fnred(self) ->u8

Returns the red channel of the color as u8 in the range 0..255.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#277)
#### pub fngreen(self) ->u8

Returns the green channel of the color as u8 in the range 0..255.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#283)
#### pub fnblue(self) ->u8

Returns the blue channel of the color as u8 in the range 0..255.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#289)
#### pub fnalpha(self) ->u8

Returns the alpha channel of the color as u8 in the range 0..255.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#300)
#### pub fnbrighter(&self, factor:f32) ->Color

Returns a new version of this color that has the brightness increased
by the specified factor. This is done by converting the color to the HSV
color space and multiplying the brightness (value) with (1 + factor).
The result is converted back to RGB and the alpha channel is unchanged.
So for examplebrighter(0.2)will increase the brightness by 20%, and
callingbrighter(-0.5)will return a color that’s 50% darker.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#314)
#### pub fndarker(&self, factor:f32) ->Color

Returns a new version of this color that has the brightness decreased
by the specified factor. This is done by converting the color to the HSV
color space and dividing the brightness (value) by (1 + factor). The
result is converted back to RGB and the alpha channel is unchanged.
So for exampledarker(0.3)will decrease the brightness by 30%.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#349)
#### pub fntransparentize(&self, factor:f32) ->Color

Returns a new version of this color with the opacity decreased byfactor.

The transparency is obtained by multiplying the alpha channel by(1 - factor).

##### §Examples

Decreasing the opacity of a red color by half:

```
let red = Color::from_argb_f32(1.0, 1.0, 0.0, 0.0);
assert_eq!(red.transparentize(0.5), Color::from_argb_f32(0.5, 1.0, 0.0, 0.0));
```

Decreasing the opacity of a blue color by 20%:

```
let blue = Color::from_argb_f32(1.0, 0.0, 0.0, 1.0);
assert_eq!(blue.transparentize(0.2), Color::from_argb_f32(0.8, 0.0, 0.0, 1.0));
```

Negative values increase the opacity

```
let blue = Color::from_argb_f32(0.5, 0.0, 0.0, 1.0);
assert_eq!(blue.transparentize(-0.1), Color::from_argb_f32(0.55, 0.0, 0.0, 1.0));
```

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#376)
#### pub fnmix(&self, other: &Color, factor:f32) ->Color

Returns a new color that is a mix of this color andother. The specified factor is
clamped to be between0.0and1.0and then applied to this color, while1.0 - factoris applied toother.

##### §Examples

Mix red with black half-and-half:

```
let red = Color::from_rgb_f32(1.0, 0.0, 0.0);
let black = Color::from_rgb_f32(0.0, 0.0, 0.0);
assert_eq!(red.mix(&black, 0.5), Color::from_rgb_f32(0.5, 0.0, 0.0));
```

Mix Purple with OrangeRed,  with75%purpe and25%orange red ratio:

```
let purple = Color::from_rgb_u8(128, 0, 128);
let orange_red = Color::from_rgb_u8(255, 69, 0);
assert_eq!(purple.mix(&orange_red, 0.75), Color::from_rgb_f32(0.6264706, 0.06764706, 0.37647063));
```

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#416)
#### pub fnwith_alpha(&self, alpha:f32) ->Color

Returns a new version of this color with the opacity set toalpha.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#51)§
### implCloneforColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#51)§
#### fnclone(&self) ->Color

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#51)§
### implDebugforColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#51)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#51)§
### implDefaultforColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#51)§
#### fndefault() ->Color

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#429)§
### implDisplayforColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#430)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
### implFrom<Color> forBrush

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
#### fnfrom(value:Color) ->Brush

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#676)§
### implFrom<Color> forOklchColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#677)§
#### fnfrom(value:Color) ->OklchColor

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#109)§
### implFrom<Color> forRgbaColor<f32>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#111)§
#### fnfrom(col:Color) ->RgbaColor<f32>

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#154)§
### implFrom<Color> forRgbaColor<u8>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#156)§
#### fnfrom(col:Color) ->RgbaColor<u8>

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#410-415)§
### implFrom<Color> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#412-414)§
#### fnfrom(c:Color) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#518)§
### implFrom<HsvaColor> forColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#519)§
#### fnfrom(value:HsvaColor) ->Color

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#670)§
### implFrom<OklchColor> forColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#671)§
#### fnfrom(value:OklchColor) ->Color

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#124)§
### implFrom<RgbaColor<f32>> forColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#126)§
#### fnfrom(col:RgbaColor<f32>) ->Color

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#139)§
### implFrom<RgbaColor<u8>> forColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#141)§
#### fnfrom(col:RgbaColor<u8>) ->Color

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#423)§
### implInterpolatedPropertyValueforColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#424)§
#### fninterpolate(&self, target_value: &Color, t:f32) ->Color

Returns the interpolated value between self and target_value according to the progress parameter t that’s usually between 0 and 1. With certain animation easing curves it may over- or undershoot though.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#51)§
### implPartialEqforColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#51)§
#### fneq(&self, other: &Color) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#51)§
### implPartialOrdforColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#51)§
#### fnpartial_cmp(&self, other: &Color) ->Option<Ordering>

This method returns an ordering between
`self`and
`other`values if one exists.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1402)§
#### fnlt(&self, other:&Rhs) ->bool

Tests less than (for
`self`and
`other`) and is used by the
`<`operator.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1420)§
#### fnle(&self, other:&Rhs) ->bool

Tests less than or equal to (for
`self`and
`other`) and is used by the
`<=`operator.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1438)§
#### fngt(&self, other:&Rhs) ->bool

Tests greater than (for
`self`and
`other`) and is used by the
`>`operator.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1456)§
#### fnge(&self, other:&Rhs) ->bool

Tests greater than or equal to (for
`self`and
`other`) and is used by the
`>=`operator.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#416-425)§
### implTryFrom<Value> forColor

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#417)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#419-424)§
#### fntry_from(v:Value) ->Result<Color, Self::Error>

Performs the conversion.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#51)§
### implCopyforColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#51)§
### implStructuralPartialEqforColor

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#322-324)§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#326)§
#### fnto_shared_string(&self) ->SharedString

Converts the given value to a
[SharedString](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.SharedString.html).
[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#678-680)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#682)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2891)§
### impl<T>ToStringfor Twhere
    T:Display+ ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893)§
#### fnto_string(&self) ->String

Converts the given value to a
`String`.
[Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructCompilationResultCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1009-1017)
```
pub struct CompilationResult { /* private fields */ }
```

Expand description
The result of a compilation

IfSelf::has_errors()is true, then the compilation failed.
TheSelf::diagnostics()function can be used to retrieve the diagnostics (errors and/or warnings)
orSelf::print_diagnostics()can be used to print them to stderr.
The components can be retrieved usingSelf::components()

## Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1028-1088)§
### implCompilationResult

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1031-1033)
#### pub fnhas_errors(&self) ->bool

Returns true if the compilation failed.
The errors can be retrieved using theSelf::diagnostics()function.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1038-1040)
#### pub fndiagnostics(&self) -> implIterator<Item =Diagnostic> + '_

Return an iterator over the diagnostics.

You can also callSelf::print_diagnostics()to output the diagnostics to stderr

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1048-1050)
#### pub fnprint_diagnostics(&self)

Available on
**crate featuredisplay-diagnostics**only.
Print the diagnostics to stderr

The diagnostics are printed in the same style as rustc errors

This function is available when thedisplay-diagnosticsis enabled.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1053-1055)
#### pub fncomponents(&self) -> implIterator<Item =ComponentDefinition> + '_

Returns an iterator over the compiled components.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1058-1060)
#### pub fncomponent_names(&self) -> implIterator<Item = &str> + '_

Returns the names of the components that were compiled.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1064-1066)
#### pub fncomponent(&self, name: &str) ->Option<ComponentDefinition>

Return the component definition for the given name.
If the component does not exist, thenNoneis returned.

## Trait Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1008)§
### implCloneforCompilationResult

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1008)§
#### fnclone(&self) ->CompilationResult

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1019-1026)§
### implDebugforCompilationResult

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1020-1025)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)
## Auto Trait Implementations§

§
### implFreezeforCompilationResult

§
### impl !RefUnwindSafeforCompilationResult

§
### impl !SendforCompilationResult

§
### impl !SyncforCompilationResult

§
### implUnpinforCompilationResult

§
### implUnsafeUnpinforCompilationResult

§
### impl !UnwindSafeforCompilationResult

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructCompilerCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#840-842)
```
pub struct Compiler { /* private fields */ }
```

Expand description
This is the entry point of the crate, it can be used to load a.slintfile and
compile it into aCompilationResult.

## Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#853-1000)§
### implCompiler

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#855-857)
#### pub fnnew() -> Self

Returns a new Compiler.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#872-874)
#### pub fnset_include_paths(&mut self, include_paths:Vec<PathBuf>)

Sets the include paths used for looking up.slintimports to the specified vector of paths.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#877-879)
#### pub fninclude_paths(&self) -> &Vec<PathBuf>

Returns the include paths the component compiler is currently configured with.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#882-884)
#### pub fnset_library_paths(&mut self, library_paths:HashMap<String,PathBuf>)

Sets the library paths used for looking up@libraryimports to the specified map of library names to paths.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#887-889)
#### pub fnlibrary_paths(&self) -> &HashMap<String,PathBuf>

Returns the library paths the component compiler is currently configured with.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#901-903)
#### pub fnset_style(&mut self, style:String)

Sets the style to be used for widgets.

Use the “material” style as widget style when compiling:

```
use slint_interpreter::{ComponentDefinition, Compiler, ComponentHandle};

let mut compiler = Compiler::default();
compiler.set_style("material".into());
let result = spin_on::spin_on(compiler.build_from_path("hello.slint"));
```

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#906-908)
#### pub fnstyle(&self) ->Option<&String>

Returns the widget style the compiler is currently using when compiling .slint files.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#911-913)
#### pub fnset_translation_domain(&mut self, domain:String)

The domain used for translations

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#920-925)
#### pub fnset_default_translation_context(
    &mut self,
    default_translation_context:DefaultTranslationContext,
)

Unless explicitly specified with the@tr("context" => ...), the default translation context is the component name.
Use this option withDefaultTranslationContext::Noneto disable the default translation context.

The translation file must also not have context
(--no-default-translation-contextargument ofslint-tr-extractor)

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#934-944)
#### pub fnset_file_loader(
    &mut self,
    file_loader_fallback: implFn(&Path) ->Pin<Box<dynFuture<Output =Option<Result<String>>>>> + 'static,
)

Sets the callback that will be invoked when loading imported .slint files. The specifiedfile_loader_callbackparameter will be called with a canonical file path as argument
and is expected to return a future that, when resolved, provides the source code of the
.slint file to be imported as a string.
If an error is returned, then the build will abort with that error.
If None is returned, it means the normal resolution algorithm will proceed as if the hook
was not in place (i.e: load from the file system following the include paths)

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#964-983)
#### pub async fnbuild_from_path<P:AsRef<Path>>(
    &self,
    path: P,
) ->CompilationResult

Compile a .slint file

Returns a structure that holds the diagnostics and the compiled components.

Any diagnostics produced during the compilation, such as warnings or errors, can be retrieved
after the call usingCompilationResult::diagnostics().

If the file was compiled without error, the list of component names can be obtained withCompilationResult::component_names, and the compiled components themselves withCompilationResult::component().

If the path is"-", the file will be read from stdin.
If the extension of the file .rs, the firstslint!macro from a rust file will be extracted

This function isasyncbut in practice, this is only asynchronous ifSelf::set_file_loaderwas called and its future is actually asynchronous.
If that is not used, then it is fine to use a very simple executor, such as the one
provided by thespin_oncrate

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#997-999)
#### pub async fnbuild_from_source(
    &self,
    source_code:String,
    path:PathBuf,
) ->CompilationResult

Compile some .slint code

Thepathargument will be used for diagnostics and to compute relative
paths while importing.

Any diagnostics produced during the compilation, such as warnings or errors, can be retrieved
after the call usingCompilationResult::diagnostics().

This function isasyncbut in practice, this is only asynchronous ifSelf::set_file_loaderis set and its future is actually asynchronous.
If that is not used, then it is fine to use a very simple executor, such as the one
provided by thespin_oncrate

## Trait Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#844-851)§
### implDefaultforCompiler

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#845-850)§
#### fndefault() -> Self

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)
## Auto Trait Implementations§

§
### implFreezeforCompiler

§
### impl !RefUnwindSafeforCompiler

§
### impl !SendforCompiler

§
### impl !SyncforCompiler

§
### implUnpinforCompiler

§
### implUnsafeUnpinforCompiler

§
### impl !UnwindSafeforCompiler

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructComponentCompilerCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#683-686)
```
pub struct ComponentCompiler { /* private fields */ }
```

👎
Deprecated: Use slint_interpreter::Compiler instead
Expand description
ComponentCompiler is deprecated, useCompilerinstead

## Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#700-836)§
### implComponentCompiler

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#702-704)
#### pub fnnew() -> Self

Returns a new ComponentCompiler.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#707-709)
#### pub fnset_include_paths(&mut self, include_paths:Vec<PathBuf>)

Sets the include paths used for looking up.slintimports to the specified vector of paths.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#712-714)
#### pub fninclude_paths(&self) -> &Vec<PathBuf>

Returns the include paths the component compiler is currently configured with.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#717-719)
#### pub fnset_library_paths(&mut self, library_paths:HashMap<String,PathBuf>)

Sets the library paths used for looking up@libraryimports to the specified map of library names to paths.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#722-724)
#### pub fnlibrary_paths(&self) -> &HashMap<String,PathBuf>

Returns the library paths the component compiler is currently configured with.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#737-739)
#### pub fnset_style(&mut self, style:String)

Sets the style to be used for widgets.

Use the “material” style as widget style when compiling:

```
use slint_interpreter::{ComponentDefinition, ComponentCompiler, ComponentHandle};

let mut compiler = ComponentCompiler::default();
compiler.set_style("material".into());
let definition =
    spin_on::spin_on(compiler.build_from_path("hello.slint"));
```

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#742-744)
#### pub fnstyle(&self) ->Option<&String>

Returns the widget style the compiler is currently using when compiling .slint files.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#747-749)
#### pub fnset_translation_domain(&mut self, domain:String)

The domain used for translations

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#758-768)
#### pub fnset_file_loader(
    &mut self,
    file_loader_fallback: implFn(&Path) ->Pin<Box<dynFuture<Output =Option<Result<String>>>>> + 'static,
)

Sets the callback that will be invoked when loading imported .slint files. The specifiedfile_loader_callbackparameter will be called with a canonical file path as argument
and is expected to return a future that, when resolved, provides the source code of the
.slint file to be imported as a string.
If an error is returned, then the build will abort with that error.
If None is returned, it means the normal resolution algorithm will proceed as if the hook
was not in place (i.e: load from the file system following the include paths)

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#771-773)
#### pub fndiagnostics(&self) -> &Vec<Diagnostic>

Returns the diagnostics that were produced in the last call toSelf::build_from_pathorSelf::build_from_source.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#793-809)
#### pub async fnbuild_from_path<P:AsRef<Path>>(
    &mut self,
    path: P,
) ->Option<ComponentDefinition>

Compile a .slint file into a ComponentDefinition

Returns the compiledComponentDefinitionif there were no errors.

Any diagnostics produced during the compilation, such as warnings or errors, are collected
in this ComponentCompiler and can be retrieved after the call using theSelf::diagnostics()function. Theprint_diagnosticsfunction can be used to display the diagnostics
to the users.

Diagnostics from previous calls are cleared when calling this function.

If the path is"-", the file will be read from stdin.
If the extension of the file .rs, the firstslint!macro from a rust file will be extracted

This function isasyncbut in practice, this is only asynchronous ifSelf::set_file_loaderwas called and its future is actually asynchronous.
If that is not used, then it is fine to use a very simple executor, such as the one
provided by thespin_oncrate

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#827-835)
#### pub async fnbuild_from_source(
    &mut self,
    source_code:String,
    path:PathBuf,
) ->Option<ComponentDefinition>

Compile some .slint code into a ComponentDefinition

Thepathargument will be used for diagnostics and to compute relative
paths while importing.

Any diagnostics produced during the compilation, such as warnings or errors, are collected
in this ComponentCompiler and can be retrieved after the call using theSelf::diagnostics()function. Theprint_diagnosticsfunction can be used to display the diagnostics
to the users.

Diagnostics from previous calls are cleared when calling this function.

This function isasyncbut in practice, this is only asynchronous ifSelf::set_file_loaderis set and its future is actually asynchronous.
If that is not used, then it is fine to use a very simple executor, such as the one
provided by thespin_oncrate

## Trait Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#689-697)§
### implDefaultforComponentCompiler

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#690-696)§
#### fndefault() -> Self

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)
## Auto Trait Implementations§

§
### implFreezeforComponentCompiler

§
### impl !RefUnwindSafeforComponentCompiler

§
### impl !SendforComponentCompiler

§
### impl !SyncforComponentCompiler

§
### implUnpinforComponentCompiler

§
### implUnsafeUnpinforComponentCompiler

§
### impl !UnwindSafeforComponentCompiler

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructComponentDefinitionCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1098-1100)
```
pub struct ComponentDefinition { /* private fields */ }
```

Expand description
ComponentDefinition is a representation of a compiled component from .slint markup.

It can be constructed from a .slint file using theCompiler::build_from_pathorCompiler::build_from_sourcefunctions.
And then it can be instantiated with theSelf::createfunction.

The ComponentDefinition acts as a factory to create new instances. When you’ve finished
creating the instances it is safe to drop the ComponentDefinition.

## Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1102-1350)§
### implComponentDefinition

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1117-1122)
#### pub fncreate(&self) ->Result<ComponentInstance,PlatformError>

Creates a new instance of the component and returns a shared handle to it.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1176-1187)
#### pub fnproperties(&self) -> implIterator<Item = (String,ValueType)> + '_

Returns an iterator over all publicly declared properties. Each iterator item is a tuple of property name
and property type for each of them.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1190-1201)
#### pub fncallbacks(&self) -> implIterator<Item =String> + '_

Returns the names of all publicly declared callbacks.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1204-1215)
#### pub fnfunctions(&self) -> implIterator<Item =String> + '_

Returns the names of all publicly declared functions.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1221-1226)
#### pub fnglobals(&self) -> implIterator<Item =String> + '_

Returns the names of all exported global singletons

Note:Only globals that are exported or re-exported from the main .slint file will
be exposed in the API

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1257-1273)
#### pub fnglobal_properties(
    &self,
    global_name: &str,
) ->Option<implIterator<Item = (String,ValueType)> + '_>

List of publicly declared properties in the exported global singleton specified by its name.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1276-1289)
#### pub fnglobal_callbacks(
    &self,
    global_name: &str,
) ->Option<implIterator<Item =String> + '_>

List of publicly declared callbacks in the exported global singleton specified by its name.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1292-1305)
#### pub fnglobal_functions(
    &self,
    global_name: &str,
) ->Option<implIterator<Item =String> + '_>

List of publicly declared functions in the exported global singleton specified by its name.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1308-1313)
#### pub fnname(&self) -> &str

The name of this Component as written in the .slint file

## Trait Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1097)§
### implCloneforComponentDefinition

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1097)§
#### fnclone(&self) ->ComponentDefinition

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)
## Auto Trait Implementations§

§
### implFreezeforComponentDefinition

§
### impl !RefUnwindSafeforComponentDefinition

§
### impl !SendforComponentDefinition

§
### impl !SyncforComponentDefinition

§
### implUnpinforComponentDefinition

§
### implUnsafeUnpinforComponentDefinition

§
### impl !UnwindSafeforComponentDefinition

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructComponentInstanceCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1374-1376)
```
pub struct ComponentInstance { /* private fields */ }
```

Expand description
This represents an instance of a dynamic component

You can create an instance with theComponentDefinition::createfunction.

Properties and callback can be accessed using the associated functions.

An instance can be put on screen with theComponentInstance::runfunction.

## Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1378-1681)§
### implComponentInstance

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1380-1383)
#### pub fndefinition(&self) ->ComponentDefinition

Return theComponentDefinitionthat was used to create this instance.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1404-1424)
#### pub fnget_property(&self, name: &str) ->Result<Value,GetPropertyError>

Return the value for a public property of this component.

###### §Examples

```
use slint_interpreter::{ComponentDefinition, Compiler, Value, SharedString};
let code = r#"
    export component MyWin inherits Window {
        in-out property <int> my_property: 42;
    }
"#;
let mut compiler = Compiler::default();
let result = spin_on::spin_on(
    compiler.build_from_source(code.into(), Default::default()));
assert_eq!(result.diagnostics().count(), 0, "{:?}", result.diagnostics().collect::<Vec<_>>());
let instance = result.component("MyWin").unwrap().create().unwrap();
assert_eq!(instance.get_property("my_property").unwrap(), Value::from(42));
```

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1427-1442)
#### pub fnset_property(
    &self,
    name: &str,
    value:Value,
) ->Result<(),SetPropertyError>

Set the value for a public property of this component.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1478-1488)
#### pub fnset_callback(
    &self,
    name: &str,
    callback: implFn(&[Value]) ->Value+ 'static,
) ->Result<(),SetCallbackError>

Set a handler for the callback with the given name. A callback with that
name must be defined in the document otherwise an error will be returned.

Note: Since theComponentInstanceholds the handler, the handler itself should not
contain a strong reference to the instance. So if you need to capture the instance,
you should useSelf::as_weakto create a weak reference.

###### §Examples

```
use slint_interpreter::{Compiler, Value, SharedString, ComponentHandle};
use core::convert::TryInto;
let code = r#"
    export component MyWin inherits Window {
        callback foo(int) -> int;
        in-out property <int> my_prop: 12;
    }
"#;
let result = spin_on::spin_on(
    Compiler::default().build_from_source(code.into(), Default::default()));
assert_eq!(result.diagnostics().count(), 0, "{:?}", result.diagnostics().collect::<Vec<_>>());
let instance = result.component("MyWin").unwrap().create().unwrap();
let instance_weak = instance.as_weak();
instance.set_callback("foo", move |args: &[Value]| -> Value {
    let arg: u32 = args[0].clone().try_into().unwrap();
    let my_prop = instance_weak.unwrap().get_property("my_prop").unwrap();
    let my_prop : u32 = my_prop.try_into().unwrap();
    Value::from(arg + my_prop)
}).unwrap();

let res = instance.invoke("foo", &[Value::from(500)]).unwrap();
assert_eq!(res, Value::from(500+12));
```

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1494-1500)
#### pub fninvoke(&self, name: &str, args: &[Value]) ->Result<Value,InvokeError>

Call the given callback or function with the arguments

###### §Examples

See the documentation ofSelf::set_callbackfor an example

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1526-1539)
#### pub fnget_global_property(
    &self,
    global: &str,
    property: &str,
) ->Result<Value,GetPropertyError>

Return the value for a property within an exported global singleton used by this component.

Theglobalparameter is the exported name of the global singleton. Thepropertyargument
is the name of the property

###### §Examples

```
use slint_interpreter::{Compiler, Value, SharedString};
let code = r#"
    global Glob {
        in-out property <int> my_property: 42;
    }
    export { Glob as TheGlobal }
    export component MyWin inherits Window {
    }
"#;
let mut compiler = Compiler::default();
let result = spin_on::spin_on(compiler.build_from_source(code.into(), Default::default()));
assert_eq!(result.diagnostics().count(), 0, "{:?}", result.diagnostics().collect::<Vec<_>>());
let instance = result.component("MyWin").unwrap().create().unwrap();
assert_eq!(instance.get_global_property("TheGlobal", "my_property").unwrap(), Value::from(42));
```

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1542-1555)
#### pub fnset_global_property(
    &self,
    global: &str,
    property: &str,
    value:Value,
) ->Result<(),SetPropertyError>

Set the value for a property within an exported global singleton used by this component.

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1591-1605)
#### pub fnset_global_callback(
    &self,
    global: &str,
    name: &str,
    callback: implFn(&[Value]) ->Value+ 'static,
) ->Result<(),SetCallbackError>

Set a handler for the callback in the exported global singleton. A callback with that
name must be defined in the specified global and the global must be exported from the
main document otherwise an error will be returned.

###### §Examples

```
use slint_interpreter::{Compiler, Value, SharedString};
use core::convert::TryInto;
let code = r#"
    export global Logic {
        pure callback to_uppercase(string) -> string;
    }
    export component MyWin inherits Window {
        out property <string> hello: Logic.to_uppercase("world");
    }
"#;
let result = spin_on::spin_on(
    Compiler::default().build_from_source(code.into(), Default::default()));
let instance = result.component("MyWin").unwrap().create().unwrap();
instance.set_global_callback("Logic", "to_uppercase", |args: &[Value]| -> Value {
    let arg: SharedString = args[0].clone().try_into().unwrap();
    Value::from(SharedString::from(arg.to_uppercase()))
}).unwrap();

let res = instance.get_property("hello").unwrap();
assert_eq!(res, Value::from(SharedString::from("WORLD")));

let abc = instance.invoke_global("Logic", "to_uppercase", &[
    SharedString::from("abc").into()
]).unwrap();
assert_eq!(abc, Value::from(SharedString::from("ABC")));
```

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1611-1641)
#### pub fninvoke_global(
    &self,
    global: &str,
    callable_name: &str,
    args: &[Value],
) ->Result<Value,InvokeError>

Call the given callback or function within a global singleton with the arguments

###### §Examples

See the documentation ofSelf::set_global_callbackfor an example

## Trait Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1683-1725)§
### implComponentHandleforComponentInstance

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1686-1691)§
#### fnas_weak(&self) ->Weak<Self>where
    Self:Sized,

Returns a new weak pointer.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1693-1695)§
#### fnclone_strong(&self) -> Self

Returns a clone of this handle that’s a strong reference.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1701-1703)§
#### fnshow(&self) ->Result<(),PlatformError>

Convenience function for
[crate::Window::show()](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.Window.html#method.show). This shows the window on the screen and maintains an extra strong reference while the window is visible. To react to events from the windowing system, such as draw requests or mouse/touch input, it is still necessary to spin the event loop, using
[crate::run_event_loop](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/fn.run_event_loop.html).
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1705-1707)§
#### fnhide(&self) ->Result<(),PlatformError>

Convenience function for
[crate::Window::hide()](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.Window.html#method.hide). Hides the window, so that it is not visible anymore. The additional strong reference on the associated component, that was created when show() was called, is dropped.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1709-1713)§
#### fnrun(&self) ->Result<(),PlatformError>

This is a convenience function that first calls
[Self::show](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/trait.ComponentHandle.html#tymethod.show), followed by
[crate::run_event_loop()](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/fn.run_event_loop.html)and
[Self::hide](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/trait.ComponentHandle.html#tymethod.hide).
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1715-1717)§
#### fnwindow(&self) -> &Window

Returns the Window associated with this component. The window API can be used to control different aspects of the integration into the windowing system, such as the position on the screen.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1719-1724)§
#### fnglobal<'a, T:Global<'a, Self>>(&'a self) -> Twhere
    Self:Sized,

This function provides access to instances of global singletons exported in
`.slint`. See
[Global](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/trait.Global.html)for an example how to export and access globals from
`.slint`markup.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1727-1733)§
### implFrom<ComponentInstance> forVRc<ItemTreeVTable, ErasedItemTreeBox>

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1730-1732)§
#### fnfrom(value:ComponentInstance) -> Self

Converts to this type from the input type.

## Auto Trait Implementations§

§
### implFreezeforComponentInstance

§
### impl !RefUnwindSafeforComponentInstance

§
### impl !SendforComponentInstance

§
### impl !SyncforComponentInstance

§
### implUnpinforComponentInstance

§
### implUnsafeUnpinforComponentInstance

§
### impl !UnwindSafeforComponentInstance

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructDiagnosticCopy item path

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#269)
```
pub struct Diagnostic { /* private fields */ }
```

Expand description
This structure represent a diagnostic emitted while compiling .slint code.

It is basically a message, a level (warning or error), attached to a
position in the code

## Implementations§

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#276)§
### implDiagnostic

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#278)
#### pub fnlevel(&self) ->DiagnosticLevel

Return the level for this diagnostic

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#283)
#### pub fnmessage(&self) -> &str

Return a message for this diagnostic

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#290)
#### pub fnline_column(&self) -> (usize,usize)

Returns a tuple with the line (starting at 1) and column number (starting at 1)

Can also return (0, 0) if the span is invalid

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#303)
#### pub fnlength(&self) ->usize

Return the length of this diagnostic in UTF-8 encoded bytes.

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#311)
#### pub fnsource_file(&self) ->Option<&Path>

return the path of the source file where this error is attached

## Trait Implementations§

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#268)§
### implCloneforDiagnostic

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#268)§
#### fnclone(&self) ->Diagnostic

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#268)§
### implDebugforDiagnostic

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#268)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#316)§
### implDisplayforDiagnostic

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#317)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)
## Auto Trait Implementations§

§
### implFreezeforDiagnostic

§
### impl !RefUnwindSafeforDiagnostic

§
### impl !SendforDiagnostic

§
### impl !SyncforDiagnostic

§
### implUnpinforDiagnostic

§
### implUnsafeUnpinforDiagnostic

§
### impl !UnwindSafeforDiagnostic

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#322-324)§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#326)§
#### fnto_shared_string(&self) ->SharedString

Converts the given value to a
[SharedString](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.SharedString.html).
[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#678-680)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#682)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2891)§
### impl<T>ToStringfor Twhere
    T:Display+ ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893)§
#### fnto_string(&self) ->String

Converts the given value to a
`String`.
[Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructImageCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#699)
```
pub struct Image(/* private fields */);
```

Expand description
An image type that can be displayed by the Image element. You can construct
Image objects from a path to an image file on disk, usingSelf::load_from_path.

Another typical use-case is to render the image content with Rust code.
For this it’s most efficient to create a new SharedPixelBuffer with the known dimensions
and pass the mutable slice to your rendering function. Afterwards you can create an
Image.

The following example creates a 320x200 RGB pixel buffer and calls an external
low_level_render() function to draw a shape into it. Finally the result is
stored in an Image withSelf::from_rgb8():

```

fn low_level_render(width: u32, height: u32, buffer: &mut [u8]) {
    // render beautiful circle or other shapes here
}

let mut pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::new(320, 200);

low_level_render(pixel_buffer.width(), pixel_buffer.height(),
                 pixel_buffer.make_mut_bytes());

let image = Image::from_rgb8(pixel_buffer);
```

Another use-case is to import existing image data into Slint, by
creating a new Image through cloning of another image type.

The following example uses the popularimage crateto
load a.pngfile from disk, apply brightening filter on it and then import
it into anImage:

```
let mut cat_image = image::open("cat.png").expect("Error loading cat image").into_rgba8();

image::imageops::colorops::brighten_in_place(&mut cat_image, 20);

let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
    cat_image.as_raw(),
    cat_image.width(),
    cat_image.height(),
);
let image = Image::from_rgba8(buffer);
```

A popular software (CPU) rendering library in Rust is tiny-skia. The following example shows
how to use tiny-skia to render into aSharedPixelBuffer:

```
let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(640, 480);
let width = pixel_buffer.width();
let height = pixel_buffer.height();
let mut pixmap = tiny_skia::PixmapMut::from_bytes(
    pixel_buffer.make_mut_bytes(), width, height
).unwrap();
pixmap.fill(tiny_skia::Color::TRANSPARENT);

let circle = tiny_skia::PathBuilder::from_circle(320., 240., 150.).unwrap();

let mut paint = tiny_skia::Paint::default();
paint.shader = tiny_skia::LinearGradient::new(
    tiny_skia::Point::from_xy(100.0, 100.0),
    tiny_skia::Point::from_xy(400.0, 400.0),
    vec![
        tiny_skia::GradientStop::new(0.0, tiny_skia::Color::from_rgba8(50, 127, 150, 200)),
        tiny_skia::GradientStop::new(1.0, tiny_skia::Color::from_rgba8(220, 140, 75, 180)),
    ],
    tiny_skia::SpreadMode::Pad,
    tiny_skia::Transform::identity(),
).unwrap();

pixmap.fill_path(&circle, &paint, tiny_skia::FillRule::Winding, Default::default(), None);

let image = Image::from_rgba8_premultiplied(pixel_buffer);
```

#### §Sending Image to a thread

Imageis notSend, because it uses internal cache that are local to the Slint thread.
If you want to create image data in a thread and send that to slint, construct theSharedPixelBufferin a thread, and send that to Slint’s UI thread.

```
std::thread::spawn(move || {
    let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(640, 480);
    // ... fill the pixel_buffer with data as shown in the previous example ...
    slint::invoke_from_event_loop(move || {
        // this will run in the Slint's UI thread
        let image = Image::from_rgba8_premultiplied(pixel_buffer);
        // ... use the image, eg:
        // my_ui_handle.upgrade().unwrap().set_image(image);
    });
});
```

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#701)§
### implImage

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#709)
#### pub fnload_from_path(path: &Path) ->Result<Image,LoadImageError>

Available on
**crate featureimage-decoders**only.
Load an Image from a path to a file containing an image.

Supported formats are SVG, PNG and JPEG.
Enable support for additional formats supported by theimagecrate(
AVIF, BMP, DDS, Farbfeld, GIF, HDR, ICO, JPEG, EXR, PNG, PNM, QOI, TGA, TIFF, WebP)
by enabling theimage-default-formatscargo feature.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#718)
#### pub fnfrom_rgb8(buffer:SharedPixelBuffer<Rgb<u8>>) ->Image

Creates a new Image from the specified shared pixel buffer, where each pixel has three color
channels (red, green and blue) encoded as u8.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#727)
#### pub fnfrom_rgba8(buffer:SharedPixelBuffer<Rgba<u8>>) ->Image

Creates a new Image from the specified shared pixel buffer, where each pixel has four color
channels (red, green, blue and alpha) encoded as u8.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#739)
#### pub fnfrom_rgba8_premultiplied(buffer:SharedPixelBuffer<Rgba<u8>>) ->Image

Creates a new Image from the specified shared pixel buffer, where each pixel has four color
channels (red, green, blue and alpha) encoded as u8 and, in contrast toSelf::from_rgba8,
the alpha channel is also assumed to be multiplied to the red, green and blue channels.

Only construct an Image with this function if you know that your pixels are encoded this way.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#748)
#### pub fnto_rgb8(&self) ->Option<SharedPixelBuffer<Rgb<u8>>>

Returns the pixel buffer for the Image if available in RGB format without alpha.
Returns None if the pixels cannot be obtained, for example when the image was created from borrowed OpenGL textures.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#757)
#### pub fnto_rgba8(&self) ->Option<SharedPixelBuffer<Rgba<u8>>>

Returns the pixel buffer for the Image if available in RGBA format.
Returns None if the pixels cannot be obtained, for example when the image was created from borrowed OpenGL textures.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#776)
#### pub fnto_rgba8_premultiplied(&self) ->Option<SharedPixelBuffer<Rgba<u8>>>

Returns the pixel buffer for the Image if available in RGBA format, with the alpha channel pre-multiplied
to the red, green, and blue channels.
Returns None if the pixels cannot be obtained, for example when the image was created from borrowed OpenGL textures.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#871-874)
#### pub unsafe fnfrom_borrowed_gl_2d_rgba_texture(
    texture_id:NonZero<u32>,
    size:Size2D<u32,UnknownUnit>,
) ->Image

👎
Deprecated since 1.2.0: Use BorrowedOpenGLTextureBuilder
Available on
**non-WebAssembly**only.
Creates a new Image from an existing OpenGL texture. The texture remains borrowed by Slint
for the duration of being used for rendering, such as when assigned as source property to
anImageelement. It’s the application’s responsibility to delete the texture when it is
not used anymore.

The texture must be bindable against theGL_TEXTURE_2Dtarget, haveGL_RGBAas format
for the pixel data.

When Slint renders the texture, it assumes that the origin of the texture is at the top-left.
This is different from the default OpenGL coordinate system.

##### §Safety

This function is unsafe because invalid texture ids may lead to undefined behavior in OpenGL
drivers. A valid texture id is one that was created by the same OpenGL context that is
current during any of the invocations of the callback set onWindow::set_rendering_notifier().
OpenGL contexts between instances ofslint::Windoware not sharing resources. Consequentlyslint::Imageobjects created from borrowed OpenGL textures cannot be shared between
different windows.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#880)
#### pub fnload_from_svg_data(buffer: &[u8]) ->Result<Image,LoadImageError>

Available on
**crate featuresvg**only.
Creates a new Image from the specified buffer, which contains SVG raw data.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#892)
#### pub fnset_nine_slice_edges(
    &mut self,
    top:u16,
    right:u16,
    bottom:u16,
    left:u16,
)

Sets the nine-slice edges of the image.

Nine-slice scalingis a method for scaling
images in such a way that the corners are not distorted.
The arguments define the pixel sizes of the edges that cut the image into 9 slices.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#909)
#### pub fnsize(&self) ->Size2D<u32,UnknownUnit>

Returns the size of the Image in pixels.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#925)
#### pub fnpath(&self) ->Option<&Path>

Available on
**crate featurestd**only.
Returns the path of the image on disk, if it was constructed viaSelf::load_from_path.

For example:

```
let path_buf = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../../demos/printerdemo/ui/images/cat.jpg");
let image = Image::load_from_path(&path_buf).unwrap();
assert_eq!(image.path(), Some(path_buf.as_path()));
```

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#698)§
### implCloneforImage

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#698)§
#### fnclone(&self) ->Image

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#698)§
### implDebugforImage

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#698)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#698)§
### implDefaultforImage

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#698)§
#### fndefault() ->Image

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#252)§
### implFrom<Image> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#252)§
#### fnfrom(v:Image) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#698)§
### implFrom<ImageInner> forImage

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#698)§
#### fnfrom(value:ImageInner) ->Image

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#698)§
### implPartialEqforImage

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#698)§
#### fneq(&self, other: &Image) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#252)§
### implTryFrom<Value> forImage

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#252)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#252)§
#### fntry_from(v:Value) ->Result<Image, Self::Error>

Performs the conversion.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#698)§
### implStructuralPartialEqforImage

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructJoinHandleCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/future.rs.html#93)
```
pub struct JoinHandle<T>(/* private fields */);
```

Expand description
The return value of thespawn_local()function

Can be used to abort the future, or to get the value from a different thread with.await

This trait implements future. Polling it after it finished or aborted may result in a panic.

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/future.rs.html#115)§
### impl<T>JoinHandle<T>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/future.rs.html#119)
#### pub fnabort(self)

If the future hasn’t completed yet, this will make the event loop stop polling the corresponding future and it will be dropped

Once this handle has been aborted, it can no longer be polled

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/future.rs.html#123)
#### pub fnis_finished(&self) ->bool

Checks if the task associated with thisJoinHandlehas finished.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/future.rs.html#95)§
### impl<T>FutureforJoinHandle<T>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/future.rs.html#96)§
#### typeOutput= T

The type of value produced on completion.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/future.rs.html#98)§
#### fnpoll(
    self:Pin<&mutJoinHandle<T>>,
    cx: &mutContext<'_>,
) ->Poll<<JoinHandle<T> asFuture>::Output>

Attempts to resolve the future to a final value, registering the current task for wakeup if the value is not yet available.
[Read more](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#tymethod.poll)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/future.rs.html#131)§
### impl<T>SendforJoinHandle<T>where
    T:Send,

Available on
**crate featurestd**only.

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#830)§
### impl<F>FutureExtfor Fwhere
    F:Future+ ?Sized,

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#688-690)§
#### fnpoll(&mut self, cx: &mutContext<'_>) ->Poll<Self::Output>where
    Self:Unpin,

A convenience for calling
[Future::poll()](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#tymethod.poll)on
`!`[Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html)types.
[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#713-716)§
#### fnor<F>(self, other: F) ->Or<Self, F>where
    Self:Sized,
    F:Future<Output = Self::Output>,

Returns the result of
`self`or
`other`future, preferring
`self`if both are ready.
[Read more](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/futures_lite/future/trait.FutureExt.html#method.or)[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#746-749)§
#### fnrace<F>(self, other: F) ->Race<Self, F>where
    Self:Sized,
    F:Future<Output = Self::Output>,

Available on
**crate featuresstdandrace**only.
Returns the result of
`self`or
`other`future, with no preference if both are ready.
[Read more](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/futures_lite/future/trait.FutureExt.html#method.race)[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#774-776)§
#### fncatch_unwind(self) ->CatchUnwind<Self>where
    Self:Sized+UnwindSafe,

Available on
**crate featurestd**only.
Catches panics while polling the future.
[Read more](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/futures_lite/future/trait.FutureExt.html#method.catch_unwind)[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#798-800)§
#### fnboxed<'a>(self) ->Pin<Box<dynFuture<Output = Self::Output> +Send+ 'a>>where
    Self:Sized+Send+ 'a,

Available on
**crate featurealloc**only.
Boxes the future and changes its type to
`dyn Future + Send + 'a`.
[Read more](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/futures_lite/future/trait.FutureExt.html#method.boxed)[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#822-824)§
#### fnboxed_local<'a>(self) ->Pin<Box<dynFuture<Output = Self::Output> + 'a>>where
    Self:Sized+ 'a,

Available on
**crate featurealloc**only.
Boxes the future and changes its type to
`dyn Future + 'a`.
[Read more](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/futures_lite/future/trait.FutureExt.html#method.boxed_local)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#119)§
### impl<T>FutureExtfor Twhere
    T:Future+ ?Sized,

[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#146-149)§
#### fnmap<U, F>(self, f: F) ->Map<Self, F>where
    F:FnOnce(Self::Output) -> U,
    Self:Sized,

Map this future’s output to a different type, returning a new future of the resulting type.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.map)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#159-162)§
#### fnmap_into<U>(self) ->MapInto<Self, U>where
    Self::Output:Into<U>,
    Self:Sized,

Map this future’s output to a different type, returning a new future of the resulting type.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.map_into)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#191-195)§
#### fnthen<Fut, F>(self, f: F) ->Then<Self, Fut, F>where
    F:FnOnce(Self::Output) -> Fut,
    Fut:Future,
    Self:Sized,

Chain on a computation for when a future finished, passing the result of the future to the provided closure
`f`.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.then)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#222-225)§
#### fnleft_future<B>(self) ->Either<Self, B>where
    B:Future<Output = Self::Output>,
    Self:Sized,

Wrap this future in an
`Either`future, making it the left-hand variant of that
`Either`.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.left_future)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#252-255)§
#### fnright_future<A>(self) ->Either<A, Self>where
    A:Future<Output = Self::Output>,
    Self:Sized,

Wrap this future in an
`Either`future, making it the right-hand variant of that
`Either`.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.right_future)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#278-280)§
#### fninto_stream(self) ->IntoStream<Self>where
    Self:Sized,

Convert this future into a single element stream.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.into_stream)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#307-310)§
#### fnflatten(self) ->Flatten<Self>where
    Self::Output:Future,
    Self:Sized,

Flatten the execution of this future when the output of this future is itself another future.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.flatten)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#341-344)§
#### fnflatten_stream(self) ->FlattenStream<Self>where
    Self::Output:Stream,
    Self:Sized,

Flatten the execution of this future when the successful result of this future is a stream.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.flatten_stream)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#365-367)§
#### fnfuse(self) ->Fuse<Self>where
    Self:Sized,

Fuse a future such that
`poll`will never again be called once it has completed. This method can be used to turn any
`Future`into a
`FusedFuture`.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.fuse)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#391-394)§
#### fninspect<F>(self, f: F) ->Inspect<Self, F>where
    F:FnOnce(&Self::Output),
    Self:Sized,

Do something with the output of a future before passing it on.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.inspect)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#431-433)§
#### fncatch_unwind(self) ->CatchUnwind<Self>where
    Self:Sized+UnwindSafe,

Available on
**crate featurestd**only.
Catches unwinding panics while polling the future.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.catch_unwind)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#482-485)§
#### fnshared(self) ->Shared<Self>where
    Self:Sized,
    Self::Output:Clone,

Available on
**crate featurestd**only.
Create a cloneable handle to this future where all handles will resolve to the same result.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.shared)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#501-503)§
#### fnremote_handle(self) -> (Remote<Self>,RemoteHandle<Self::Output>)where
    Self:Sized,

Available on
**crate featureschannelandstd**only.
Turn this future into a future that yields
`()`on completion and sends its output to another future on a separate task.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.remote_handle)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#514-516)§
#### fnboxed<'a>(self) ->Pin<Box<dynFuture<Output = Self::Output> +Send+ 'a>>where
    Self:Sized+Send+ 'a,

Available on
**crate featurealloc**only.
Wrap the future in a Box, pinning it.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.boxed)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#528-530)§
#### fnboxed_local<'a>(self) ->Pin<Box<dynFuture<Output = Self::Output> + 'a>>where
    Self:Sized+ 'a,

Available on
**crate featurealloc**only.
Wrap the future in a Box, pinning it.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.boxed_local)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#537-539)§
#### fnunit_error(self) ->UnitError<Self>where
    Self:Sized,

Turns a
[Future<Output = T>](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html)into a
[TryFuture<Ok = T, Error = ()>](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html).
[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#546-548)§
#### fnnever_error(self) ->NeverError<Self>where
    Self:Sized,

Turns a
[Future<Output = T>](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html)into a
[TryFuture<Ok = T, Error = Never>](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html).
[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#554-556)§
#### fnpoll_unpin(&mut self, cx: &mutContext<'_>) ->Poll<Self::Output>where
    Self:Unpin,

A convenience for calling
`Future::poll`on
`Unpin`future types.
[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/future/mod.rs.html#592-594)§
#### fnnow_or_never(self) ->Option<Self::Output>where
    Self:Sized,

Evaluates and consumes the future, returning the resulting output if the future is ready after the first call to
`Future::poll`.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/future/trait.FutureExt.html#method.now_or_never)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#138)§
### impl<F>IntoFuturefor Fwhere
    F:Future,

[Source](https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#139)§
#### typeOutput= <F asFuture>::Output

The output that the future will produce on completion.
[Source](https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#140)§
#### typeIntoFuture= F

Which kind of future are we turning this into?
[Source](https://doc.rust-lang.org/nightly/src/core/future/into_future.rs.html#142)§
#### fninto_future(self) -> <F asIntoFuture>::IntoFuture

Creates a future from a value.
[Read more](https://doc.rust-lang.org/nightly/core/future/into_future/trait.IntoFuture.html#tymethod.into_future)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/src/futures_core/future.rs.html#83-85)§
### impl<F, T, E>TryFuturefor Fwhere
    F:Future<Output =Result<T, E>> + ?Sized,

[Source](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/src/futures_core/future.rs.html#87)§
#### typeOk= T

The type of successful values yielded by this future
[Source](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/src/futures_core/future.rs.html#88)§
#### typeError= E

The type of failures yielded by this future
[Source](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/src/futures_core/future.rs.html#91)§
#### fntry_poll(
    self:Pin<&mut F>,
    cx: &mutContext<'_>,
) ->Poll<<F asFuture>::Output>

Poll this
`TryFuture`as if it were a
`Future`.
[Read more](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#tymethod.try_poll)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#134)§
### impl<Fut>TryFutureExtfor Futwhere
    Fut:TryFuture+ ?Sized,

[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#171-174)§
#### fnflatten_sink<Item>(self) ->FlattenSink<Self, Self::Ok>where
    Self::Ok:Sink<Item, Error = Self::Error>,
    Self:Sized,

Available on
**crate featuresink**only.
Flattens the execution of this future when the successful result of this future is a
[Sink](https://docs.rs/futures-sink/0.3.31/x86_64-unknown-linux-gnu/futures_sink/trait.Sink.html).
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.flatten_sink)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#217-220)§
#### fnmap_ok<T, F>(self, f: F) ->MapOk<Self, F>where
    F:FnOnce(Self::Ok) -> T,
    Self:Sized,

Maps this future’s success value to a different value.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.map_ok)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#257-261)§
#### fnmap_ok_or_else<T, E, F>(self, e: E, f: F) ->MapOkOrElse<Self, F, E>where
    F:FnOnce(Self::Ok) -> T,
    E:FnOnce(Self::Error) -> T,
    Self:Sized,

Maps this future’s success value to a different value, and permits for error handling resulting in the same type.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.map_ok_or_else)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#308-311)§
#### fnmap_err<E, F>(self, f: F) ->MapErr<Self, F>where
    F:FnOnce(Self::Error) -> E,
    Self:Sized,

Maps this future’s error value to a different value.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.map_err)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#341-344)§
#### fnerr_into<E>(self) ->ErrInto<Self, E>where
    Self:Sized,
    Self::Error:Into<E>,

Maps this future’s
[Error](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Error)to a new error type using the
[Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html)trait.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.err_into)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#351-354)§
#### fnok_into<U>(self) ->OkInto<Self, U>where
    Self:Sized,
    Self::Ok:Into<U>,

Maps this future’s
[Ok](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok)to a new type using the
[Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html)trait.
[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#395-399)§
#### fnand_then<Fut, F>(self, f: F) ->AndThen<Self, Fut, F>where
    F:FnOnce(Self::Ok) -> Fut,
    Fut:TryFuture<Error = Self::Error>,
    Self:Sized,

Executes another future after this one resolves successfully. The success value is passed to a closure to create this subsequent future.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.and_then)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#440-444)§
#### fnor_else<Fut, F>(self, f: F) ->OrElse<Self, Fut, F>where
    F:FnOnce(Self::Error) -> Fut,
    Fut:TryFuture<Ok = Self::Ok>,
    Self:Sized,

Executes another future if this one resolves to an error. The error value is passed to a closure to create this subsequent future.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.or_else)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#467-470)§
#### fninspect_ok<F>(self, f: F) ->InspectOk<Self, F>where
    F:FnOnce(&Self::Ok),
    Self:Sized,

Do something with the success value of a future before passing it on.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.inspect_ok)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#493-496)§
#### fninspect_err<F>(self, f: F) ->InspectErr<Self, F>where
    F:FnOnce(&Self::Error),
    Self:Sized,

Do something with the error value of a future before passing it on.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.inspect_err)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#505-508)§
#### fntry_flatten(self) ->TryFlatten<Self, Self::Ok>where
    Self::Ok:TryFuture<Error = Self::Error>,
    Self:Sized,

Flatten the execution of this future when the successful result of this future is another future.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.try_flatten)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#538-541)§
#### fntry_flatten_stream(self) ->TryFlattenStream<Self>where
    Self::Ok:TryStream<Error = Self::Error>,
    Self:Sized,

Flatten the execution of this future when the successful result of this future is a stream.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.try_flatten_stream)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#570-573)§
#### fnunwrap_or_else<F>(self, f: F) ->UnwrapOrElse<Self, F>where
    Self:Sized,
    F:FnOnce(Self::Error) -> Self::Ok,

Unwraps this future’s output, producing a future with this future’s
[Ok](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#associatedtype.Ok)type as its
[Output](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html#associatedtype.Output)type.
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.unwrap_or_else)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#610-612)§
#### fninto_future(self) ->IntoFuture<Self>where
    Self:Sized,

Wraps a
[TryFuture](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html)into a type that implements
[Future](https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html).
[Read more](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/futures_util/future/try_future/trait.TryFutureExt.html#method.into_future)[Source](https://docs.rs/futures-util/0.3.31/x86_64-unknown-linux-gnu/src/futures_util/future/try_future/mod.rs.html#619-621)§
#### fntry_poll_unpin(
    &mut self,
    cx: &mutContext<'_>,
) ->Poll<Result<Self::Ok, Self::Error>>where
    Self:Unpin,

A convenience method for calling
[TryFuture::try_poll](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/future/trait.TryFuture.html#tymethod.try_poll)on
[Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html)future types.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructLoadImageErrorCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#590)
```
pub struct LoadImageError(/* private fields */);
```

Expand description
Error generated if an image cannot be loaded for any reasons.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#589)§
### implDebugforLoadImageError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#589)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#589)§
### implDefaultforLoadImageError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#589)§
#### fndefault() ->LoadImageError

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#592)§
### implDisplayforLoadImageError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#593)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#599)§
### implErrorforLoadImageError

Available on
**crate featurestd**only.
1.30.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#111)§
#### fnsource(&self) ->Option<&(dynError+ 'static)>

Returns the lower-level source of this error, if any.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#137)§
#### fndescription(&self) -> &str

👎
Deprecated since 1.42.0: use the Display impl or to_string()
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#147)§
#### fncause(&self) ->Option<&dynError>

👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#260)§
#### fnprovide<'a>(&'a self, request: &mutRequest<'a>)

🔬
This is a nightly-only experimental API. (
`error_generic_member_access`)
Provides type-based access to context intended for error reports.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#589)§
### implPartialEqforLoadImageError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#589)§
#### fneq(&self, other: &LoadImageError) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#589)§
### implStructuralPartialEqforLoadImageError

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#322-324)§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#326)§
#### fnto_shared_string(&self) ->SharedString

Converts the given value to a
[SharedString](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.SharedString.html).
[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#678-680)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#682)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2891)§
### impl<T>ToStringfor Twhere
    T:Display+ ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893)§
#### fnto_string(&self) ->String

Converts the given value to a
`String`.
[Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructLogicalPositionCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#29)
```
#[repr(C)]pub struct LogicalPosition {
    pub x: f32,
    pub y: f32,
}
```

Expand description
A position represented in the coordinate space of logical pixels. That is the space before applying
a display device specific scale factor.

## Fields§

§`x:f32`The x coordinate.

§`y:f32`The y coordinate.

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#36)§
### implLogicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#39)
#### pub const fnnew(x:f32, y:f32) ->LogicalPosition

Construct a new logical position from the given x and y coordinates, that are assumed to be
in the logical coordinate space.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#45)
#### pub fnfrom_physical(
    physical_pos:PhysicalPosition,
    scale_factor:f32,
) ->LogicalPosition

Convert a given physical position to a logical position by dividing the coordinates with the
specified scale factor.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#51)
#### pub fnto_physical(&self, scale_factor:f32) ->PhysicalPosition

Convert this logical position to a physical position by multiplying the coordinates with the
specified scale factor.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#26)§
### implCloneforLogicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#26)§
#### fnclone(&self) ->LogicalPosition

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#26)§
### implDebugforLogicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#26)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#26)§
### implDefaultforLogicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#26)§
#### fndefault() ->LogicalPosition

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#346)§
### implFrom<LogicalPosition> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#346)§
#### fnfrom(_:LogicalPosition) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
### implFrom<LogicalPosition> forWindowPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
#### fnfrom(value:LogicalPosition) ->WindowPosition

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#26)§
### implPartialEqforLogicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#26)§
#### fneq(&self, other: &LogicalPosition) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#346)§
### implTryFrom<Value> forLogicalPosition

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#346)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#346)§
#### fntry_from(v:Value) ->Result<LogicalPosition, Self::Error>

Performs the conversion.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#26)§
### implCopyforLogicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#26)§
### implStructuralPartialEqforLogicalPosition

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructLogicalSizeCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#129)
```
#[repr(C)]pub struct LogicalSize {
    pub width: f32,
    pub height: f32,
}
```

Expand description
A size represented in the coordinate space of logical pixels. That is the space before applying
a display device specific scale factor.

## Fields§

§`width:f32`The width in logical pixels.

§`height:f32`The height in logical.

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#136)§
### implLogicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#139)
#### pub const fnnew(width:f32, height:f32) ->LogicalSize

Construct a new logical size from the given width and height values, that are assumed to be
in the logical coordinate space.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#145)
#### pub fnfrom_physical(
    physical_size:PhysicalSize,
    scale_factor:f32,
) ->LogicalSize

Convert a given physical size to a logical size by dividing width and height by the
specified scale factor.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#154)
#### pub fnto_physical(&self, scale_factor:f32) ->PhysicalSize

Convert this logical size to a physical size by multiplying width and height with the
specified scale factor.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#127)§
### implCloneforLogicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#127)§
#### fnclone(&self) ->LogicalSize

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#127)§
### implDebugforLogicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#127)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#127)§
### implDefaultforLogicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#127)§
#### fndefault() ->LogicalSize

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
### implFrom<LogicalSize> forWindowSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
#### fnfrom(value:LogicalSize) ->WindowSize

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#127)§
### implPartialEqforLogicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#127)§
#### fneq(&self, other: &LogicalSize) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#127)§
### implCopyforLogicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#127)§
### implStructuralPartialEqforLogicalSize

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructOklchColorCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#538)
```
pub struct OklchColor {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
    pub alpha: f32,
}
```

Expand description
OklchColor stores the lightness, chroma, hue and alpha components of a color
in the Oklch color space asf32fields.
Oklch is a perceptually uniform color space, useful for color manipulation.
This is merely a helper struct for use withColor.

Reference:https://bottosson.github.io/posts/oklab/

## Fields§

§`lightness:f32`The lightness component, between 0 (black) and 1 (white).

§`chroma:f32`The chroma component (color intensity), typically between 0 and about 0.4.

§`hue:f32`The hue component in degrees between 0 and 360.

§`alpha:f32`The alpha component, between 0 and 1.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#536)§
### implCloneforOklchColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#536)§
#### fnclone(&self) ->OklchColor

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#536)§
### implDebugforOklchColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#536)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#536)§
### implDefaultforOklchColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#536)§
#### fndefault() ->OklchColor

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#676)§
### implFrom<Color> forOklchColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#677)§
#### fnfrom(value:Color) ->OklchColor

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#579)§
### implFrom<OklabColor> forOklchColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#580)§
#### fnfrom(oklab: OklabColor) ->OklchColor

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#670)§
### implFrom<OklchColor> forColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#671)§
#### fnfrom(value:OklchColor) ->Color

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#656)§
### implFrom<OklchColor> forRgbaColor<f32>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#657)§
#### fnfrom(oklch:OklchColor) ->RgbaColor<f32>

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#663)§
### implFrom<RgbaColor<f32>> forOklchColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#664)§
#### fnfrom(col:RgbaColor<f32>) ->OklchColor

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#549)§
### implPartialEqforOklchColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#550)§
#### fneq(&self, other: &OklchColor) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#536)§
### implPartialOrdforOklchColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#536)§
#### fnpartial_cmp(&self, other: &OklchColor) ->Option<Ordering>

This method returns an ordering between
`self`and
`other`values if one exists.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1402)§
#### fnlt(&self, other:&Rhs) ->bool

Tests less than (for
`self`and
`other`) and is used by the
`<`operator.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1420)§
#### fnle(&self, other:&Rhs) ->bool

Tests less than or equal to (for
`self`and
`other`) and is used by the
`<=`operator.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1438)§
#### fngt(&self, other:&Rhs) ->bool

Tests greater than (for
`self`and
`other`) and is used by the
`>`operator.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1456)§
#### fnge(&self, other:&Rhs) ->bool

Tests greater than or equal to (for
`self`and
`other`) and is used by the
`>=`operator.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#536)§
### implCopyforOklchColor

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructPhysicalPositionCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#67)
```
pub struct PhysicalPosition {
    pub x: i32,
    pub y: i32,
}
```

Expand description
A position represented in the coordinate space of physical device pixels. That is the space after applying
a display device specific scale factor to pixels from the logical coordinate space.

## Fields§

§`x:i32`The x coordinate.

§`y:i32`The y coordinate.

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#74)§
### implPhysicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#77)
#### pub const fnnew(x:i32, y:i32) ->PhysicalPosition

Construct a new physical position from the given x and y coordinates, that are assumed to be
in the physical coordinate space.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#83)
#### pub fnfrom_logical(
    logical_pos:LogicalPosition,
    scale_factor:f32,
) ->PhysicalPosition

Convert a given logical position to a physical position by multiplying the coordinates with the
specified scale factor.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#89)
#### pub fnto_logical(&self, scale_factor:f32) ->LogicalPosition

Convert this physical position to a logical position by dividing the coordinates with the
specified scale factor.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#65)§
### implCloneforPhysicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#65)§
#### fnclone(&self) ->PhysicalPosition

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#65)§
### implDebugforPhysicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#65)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#65)§
### implDefaultforPhysicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#65)§
#### fndefault() ->PhysicalPosition

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
### implFrom<PhysicalPosition> forWindowPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
#### fnfrom(value:PhysicalPosition) ->WindowPosition

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#65)§
### implPartialEqforPhysicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#65)§
#### fneq(&self, other: &PhysicalPosition) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#65)§
### implCopyforPhysicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#65)§
### implEqforPhysicalPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#65)§
### implStructuralPartialEqforPhysicalPosition

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)§
#### fnequivalent(&self, key:&K) ->bool

Compare self to
`key`and return
`true`if they are equal.
[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)§
#### fnequivalent(&self, key:&K) ->bool

Checks if this value is equivalent to the given key.
[Read more](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructPhysicalSizeCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#171)
```
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}
```

Expand description
A size represented in the coordinate space of physical device pixels. That is the space after applying
a display device specific scale factor to pixels from the logical coordinate space.

## Fields§

§`width:u32`The width in physical pixels.

§`height:u32`The height in physical pixels;

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#178)§
### implPhysicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#181)
#### pub const fnnew(width:u32, height:u32) ->PhysicalSize

Construct a new physical size from the width and height values, that are assumed to be
in the physical coordinate space.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#187)
#### pub fnfrom_logical(
    logical_size:LogicalSize,
    scale_factor:f32,
) ->PhysicalSize

Convert a given logical size to a physical size by multiplying width and height with the
specified scale factor.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#196)
#### pub fnto_logical(&self, scale_factor:f32) ->LogicalSize

Convert this physical size to a logical size by dividing width and height by the
specified scale factor.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#169)§
### implCloneforPhysicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#169)§
#### fnclone(&self) ->PhysicalSize

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#169)§
### implDebugforPhysicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#169)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#169)§
### implDefaultforPhysicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#169)§
#### fndefault() ->PhysicalSize

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
### implFrom<PhysicalSize> forWindowSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
#### fnfrom(value:PhysicalSize) ->WindowSize

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#169)§
### implPartialEqforPhysicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#169)§
#### fneq(&self, other: &PhysicalSize) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#169)§
### implCopyforPhysicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#169)§
### implEqforPhysicalSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#169)§
### implStructuralPartialEqforPhysicalSize

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)§
#### fnequivalent(&self, key:&K) ->bool

Compare self to
`key`and return
`true`if they are equal.
[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)§
#### fnequivalent(&self, key:&K) ->bool

Checks if this value is equivalent to the given key.
[Read more](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructRgbaColorCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#19)
```
pub struct RgbaColor<T> {
    pub alpha: T,
    pub red: T,
    pub green: T,
    pub blue: T,
}
```

Expand description
RgbaColor stores the red, green, blue and alpha components of a color
with the precision of the generic parameter T. For example if T is f32,
the values are normalized between 0 and 1. If T is u8, they values range
is 0 to 255.
This is merely a helper class for use withColor.

## Fields§

§`alpha: T`The alpha component.

§`red: T`The red channel.

§`green: T`The green channel.

§`blue: T`The blue channel.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#18)§
### impl<T>CloneforRgbaColor<T>where
    T:Clone,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#18)§
#### fnclone(&self) ->RgbaColor<T>

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#18)§
### impl<T>DebugforRgbaColor<T>where
    T:Debug,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#18)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#18)§
### impl<T>DefaultforRgbaColor<T>where
    T:Default,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#18)§
#### fndefault() ->RgbaColor<T>

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#109)§
### implFrom<Color> forRgbaColor<f32>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#111)§
#### fnfrom(col:Color) ->RgbaColor<f32>

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#154)§
### implFrom<Color> forRgbaColor<u8>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#156)§
#### fnfrom(col:Color) ->RgbaColor<u8>

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#492)§
### implFrom<HsvaColor> forRgbaColor<f32>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#493)§
#### fnfrom(col:HsvaColor) ->RgbaColor<f32>

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#629)§
### implFrom<OklabColor> forRgbaColor<f32>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#630)§
#### fnfrom(oklab: OklabColor) ->RgbaColor<f32>

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#656)§
### implFrom<OklchColor> forRgbaColor<f32>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#657)§
#### fnfrom(oklch:OklchColor) ->RgbaColor<f32>

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#124)§
### implFrom<RgbaColor<f32>> forColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#126)§
#### fnfrom(col:RgbaColor<f32>) ->Color

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#663)§
### implFrom<RgbaColor<f32>> forOklchColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#664)§
#### fnfrom(col:RgbaColor<f32>) ->OklchColor

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#97)§
### implFrom<RgbaColor<f32>> forRgbaColor<u8>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#99)§
#### fnfrom(col:RgbaColor<f32>) ->RgbaColor<u8>

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#139)§
### implFrom<RgbaColor<u8>> forColor

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#141)§
#### fnfrom(col:RgbaColor<u8>) ->Color

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#85)§
### implFrom<RgbaColor<u8>> forRgbaColor<f32>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#87)§
#### fnfrom(col:RgbaColor<u8>) ->RgbaColor<f32>

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#18)§
### impl<T>PartialEqforRgbaColor<T>where
    T:PartialEq,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#18)§
#### fneq(&self, other: &RgbaColor<T>) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#18)§
### impl<T>CopyforRgbaColor<T>where
    T:Copy,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/color.rs.html#18)§
### impl<T>StructuralPartialEqforRgbaColor<T>

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructSharedPixelBufferCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#64)
```
pub struct SharedPixelBuffer<Pixel> { /* private fields */ }
```

Expand description
SharedPixelBuffer is a container for storing image data as pixels. It is
internally reference counted and cheap to clone.

You can construct a new empty shared pixel buffer withSharedPixelBuffer::new,
or you can clone it from an existing contiguous buffer that you might already have, usingSharedPixelBuffer::clone_from_slice.

See the documentation forImagefor examples how to use this type to integrate
Slint with external rendering functions.

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#70)§
### impl<Pixel>SharedPixelBuffer<Pixel>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#72)
#### pub fnwidth(&self) ->u32

Returns the width of the image in pixels.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#77)
#### pub fnheight(&self) ->u32

Returns the height of the image in pixels.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#82)
#### pub fnsize(&self) ->Size2D<u32,UnknownUnit>

Returns the size of the image in pixels.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#87)§
### impl<Pixel>SharedPixelBuffer<Pixel>where
    Pixel:Clone,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#89)
#### pub fnmake_mut_slice(&mut self) -> &mut[Pixel]

Return a mutable slice to the pixel data. If the SharedPixelBuffer was shared, this will make a copy of the buffer.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#94-96)§
### impl<Pixel>SharedPixelBuffer<Pixel>where
    Pixel:Clone+Pod,[Pixel]:ComponentBytes<u8>,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#99)
#### pub fnas_bytes(&self) -> &[u8]ⓘ

Returns the pixels interpreted as raw bytes.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#105)
#### pub fnmake_mut_bytes(&mut self) -> &mut [u8]ⓘ

Returns the pixels interpreted as raw bytes.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#111)§
### impl<Pixel>SharedPixelBuffer<Pixel>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#113)
#### pub fnas_slice(&self) -> &[Pixel]

Return a slice to the pixel data.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#118)§
### impl<Pixel>SharedPixelBuffer<Pixel>where
    Pixel:Clone+Default,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#121)
#### pub fnnew(width:u32, height:u32) ->SharedPixelBuffer<Pixel>

Creates a new SharedPixelBuffer with the given width and height. Each pixel will be initialized with the value
thatDefault::default()returns for the Pixel type.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#131)§
### impl<Pixel>SharedPixelBuffer<Pixel>where
    Pixel:Clone,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#135-141)
#### pub fnclone_from_slice<SourcePixelType>(
    pixel_slice: &[SourcePixelType],
    width:u32,
    height:u32,
) ->SharedPixelBuffer<Pixel>where[SourcePixelType]:AsPixels<Pixel>,

Creates a new SharedPixelBuffer by cloning and converting pixels from an existing
slice. This function is useful when another crate was used to allocate an image
and you would like to convert it for use in Slint.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#62)§
### impl<Pixel>CloneforSharedPixelBuffer<Pixel>where
    Pixel:Clone,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#62)§
#### fnclone(&self) ->SharedPixelBuffer<Pixel>

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#62)§
### impl<Pixel>DebugforSharedPixelBuffer<Pixel>where
    Pixel:Debug,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#62)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)
## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructSharedStringCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#43)
```
pub struct SharedString { /* private fields */ }
```

Expand description
A string type used by the Slint run-time.

SharedString uses implicit data sharing to make it efficient to pass around copies. When
cloning, a reference to the data is cloned, not the data itself. The data itself is only copied
when modifying it, for example usingpush_str. This is also called copy-on-write.

Under the hood the string data is UTF-8 encoded and it is always terminated with a null character.

SharedStringimplementsDeref<Target=str>so it can be easily passed to any function taking a&str.
It also implementFromsuch that it an easily be converted to and from the typical rust String type with.into()

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#48)§
### implSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#52)
#### pub fnnew() ->SharedString

Creates a new empty string

Same asSharedString::default()

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#61)
#### pub fnlen(&self) ->usize

Size of the string, in bytes. This excludes the terminating null character.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#66)
#### pub fnis_empty(&self) ->bool

Return true if the String is empty

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#71)
#### pub fnas_str(&self) -> &str

Return a slice to the string

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#88)
#### pub fnpush_str(&mut self, x: &str)

Append a string to this string

```
let mut hello = SharedString::from("Hello");
hello.push_str(", ");
hello.push_str("World");
hello.push_str("!");
assert_eq!(hello, "Hello, World!");
```

## Methods fromDeref<Target =str>§

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#157)
#### pub fnlen(&self) ->usize

Returns the length ofself.

This length is in bytes, notchars or graphemes. In other words,
it might not be what a human considers the length of the string.

##### §Examples

```
let len = "foo".len();
assert_eq!(3, len);

assert_eq!("ƒoo".len(), 4); // fancy f!
assert_eq!("ƒoo".chars().count(), 3);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#177)
#### pub fnis_empty(&self) ->bool

Returnstrueifselfhas a length of zero bytes.

##### §Examples

```
let s = "";
assert!(s.is_empty());

let s = "not empty";
assert!(!s.is_empty());
```

1.9.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#377)
#### pub fnis_char_boundary(&self, index:usize) ->bool

Checks thatindex-th byte is the first byte in a UTF-8 code point
sequence or the end of the string.

The start and end of the string (whenindex == self.len()) are
considered to be boundaries.

Returnsfalseifindexis greater thanself.len().

##### §Examples

```
let s = "Löwe 老虎 Léopard";
assert!(s.is_char_boundary(0));
// start of `老`
assert!(s.is_char_boundary(6));
assert!(s.is_char_boundary(s.len()));

// second byte of `ö`
assert!(!s.is_char_boundary(2));

// third byte of `老`
assert!(!s.is_char_boundary(8));
```

1.91.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#426)
#### pub fnfloor_char_boundary(&self, index:usize) ->usize

Finds the closestxnot exceedingindexwhereis_char_boundary(x)istrue.

This method can help you truncate a string so that it’s still valid UTF-8, but doesn’t
exceed a given number of bytes. Note that this is done purely at the character level
and can still visually split graphemes, even though the underlying characters aren’t
split. For example, the emoji 🧑‍🔬 (scientist) could be split so that the string only
includes 🧑 (person) instead.

##### §Examples

```
let s = "❤️🧡💛💚💙💜";
assert_eq!(s.len(), 26);
assert!(!s.is_char_boundary(13));

let closest = s.floor_char_boundary(13);
assert_eq!(closest, 10);
assert_eq!(&s[..closest], "❤️🧡");
```

1.91.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#469)
#### pub fnceil_char_boundary(&self, index:usize) ->usize

Finds the closestxnot belowindexwhereis_char_boundary(x)istrue.

Ifindexis greater than the length of the string, this returns the length of the string.

This method is the natural complement tofloor_char_boundary. See that method
for more details.

##### §Examples

```
let s = "❤️🧡💛💚💙💜";
assert_eq!(s.len(), 26);
assert!(!s.is_char_boundary(13));

let closest = s.ceil_char_boundary(13);
assert_eq!(closest, 14);
assert_eq!(&s[..closest], "❤️🧡💛");
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#502)
#### pub fnas_bytes(&self) -> &[u8]ⓘ

Converts a string slice to a byte slice. To convert the byte slice back
into a string slice, use thefrom_utf8function.

##### §Examples

```
let bytes = "bors".as_bytes();
assert_eq!(b"bors", bytes);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#578)
#### pub fnas_ptr(&self) ->*constu8

Converts a string slice to a raw pointer.

As string slices are a slice of bytes, the raw pointer points to au8. This pointer will be pointing to the first byte of the string
slice.

The caller must ensure that the returned pointer is never written to.
If you need to mutate the contents of the string slice, useas_mut_ptr.

##### §Examples

```
let s = "Hello";
let ptr = s.as_ptr();
```

1.20.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#622)
#### pub fnget<I>(&self, i: I) ->Option<&<I asSliceIndex<str>>::Output>where
    I:SliceIndex<str>,

Returns a subslice ofstr.

This is the non-panicking alternative to indexing thestr. ReturnsNonewhenever equivalent indexing operation would panic.

##### §Examples

```
let v = String::from("🗻∈🌏");

assert_eq!(Some("🗻"), v.get(0..4));

// indices not on UTF-8 sequence boundaries
assert!(v.get(1..).is_none());
assert!(v.get(..8).is_none());

// out of bounds
assert!(v.get(..42).is_none());
```

1.20.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#687)
#### pub unsafe fnget_unchecked<I>(&self, i: I) -> &<I asSliceIndex<str>>::Outputwhere
    I:SliceIndex<str>,

Returns an unchecked subslice ofstr.

This is the unchecked alternative to indexing thestr.

##### §Safety

Callers of this function are responsible that these preconditions are
satisfied:

- The starting index must not exceed the ending index;
- Indexes must be within bounds of the original slice;
- Indexes must lie on UTF-8 sequence boundaries.

Failing that, the returned string slice may reference invalid memory or
violate the invariants communicated by thestrtype.

##### §Examples

```
let v = "🗻∈🌏";
unsafe {
    assert_eq!("🗻", v.get_unchecked(0..4));
    assert_eq!("∈", v.get_unchecked(4..7));
    assert_eq!("🌏", v.get_unchecked(7..11));
}
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#773)
#### pub unsafe fnslice_unchecked(&self, begin:usize, end:usize) -> &str

👎
Deprecated since 1.29.0: use
`get_unchecked(begin..end)`instead
Creates a string slice from another string slice, bypassing safety
checks.

This is generally not recommended, use with caution! For a safe
alternative seestrandIndex.

This new slice goes frombegintoend, includingbeginbut
excludingend.

To get a mutable string slice instead, see theslice_mut_uncheckedmethod.

##### §Safety

Callers of this function are responsible that three preconditions are
satisfied:

- beginmust not exceedend.
- beginandendmust be byte positions within the string slice.
- beginandendmust lie on UTF-8 sequence boundaries.

##### §Examples

```
let s = "Löwe 老虎 Léopard";

unsafe {
    assert_eq!("Löwe 老虎 Léopard", s.slice_unchecked(0, 21));
}

let s = "Hello, world!";

unsafe {
    assert_eq!("world", s.slice_unchecked(7, 12));
}
```

1.4.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#847)
#### pub fnsplit_at(&self, mid:usize) -> (&str, &str)

Divides one string slice into two at an index.

The argument,mid, should be a byte offset from the start of the
string. It must also be on the boundary of a UTF-8 code point.

The two slices returned go from the start of the string slice tomid,
and frommidto the end of the string slice.

To get mutable string slices instead, see thesplit_at_mutmethod.

##### §Panics

Panics ifmidis not on a UTF-8 code point boundary, or if it is past
the end of the last code point of the string slice.  For a non-panicking
alternative seesplit_at_checked.

##### §Examples

```
let s = "Per Martin-Löf";

let (first, last) = s.split_at(3);

assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);
```

1.80.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#928)
#### pub fnsplit_at_checked(&self, mid:usize) ->Option<(&str, &str)>

Divides one string slice into two at an index.

The argument,mid, should be a valid byte offset from the start of the
string. It must also be on the boundary of a UTF-8 code point. The
method returnsNoneif that’s not the case.

The two slices returned go from the start of the string slice tomid,
and frommidto the end of the string slice.

To get mutable string slices instead, see thesplit_at_mut_checkedmethod.

##### §Examples

```
let s = "Per Martin-Löf";

let (first, last) = s.split_at_checked(3).unwrap();
assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);

assert_eq!(None, s.split_at_checked(13));  // Inside “ö”
assert_eq!(None, s.split_at_checked(16));  // Beyond the string length
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1066)
#### pub fnchars(&self) ->Chars<'_>

Returns an iterator over thechars of a string slice.

As a string slice consists of valid UTF-8, we can iterate through a
string slice bychar. This method returns such an iterator.

It’s important to remember thatcharrepresents a Unicode Scalar
Value, and might not match your idea of what a ‘character’ is. Iteration
over grapheme clusters may be what you actually want. This functionality
is not provided by Rust’s standard library, check crates.io instead.

##### §Examples

Basic usage:

```
let word = "goodbye";

let count = word.chars().count();
assert_eq!(7, count);

let mut chars = word.chars();

assert_eq!(Some('g'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('d'), chars.next());
assert_eq!(Some('b'), chars.next());
assert_eq!(Some('y'), chars.next());
assert_eq!(Some('e'), chars.next());

assert_eq!(None, chars.next());
```

Remember,chars might not match your intuition about characters:

```
let y = "y̆";

let mut chars = y.chars();

assert_eq!(Some('y'), chars.next()); // not 'y̆'
assert_eq!(Some('\u{0306}'), chars.next());

assert_eq!(None, chars.next());
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1123)
#### pub fnchar_indices(&self) ->CharIndices<'_>

Returns an iterator over thechars of a string slice, and their
positions.

As a string slice consists of valid UTF-8, we can iterate through a
string slice bychar. This method returns an iterator of both
thesechars, as well as their byte positions.

The iterator yields tuples. The position is first, thecharis
second.

##### §Examples

Basic usage:

```
let word = "goodbye";

let count = word.char_indices().count();
assert_eq!(7, count);

let mut char_indices = word.char_indices();

assert_eq!(Some((0, 'g')), char_indices.next());
assert_eq!(Some((1, 'o')), char_indices.next());
assert_eq!(Some((2, 'o')), char_indices.next());
assert_eq!(Some((3, 'd')), char_indices.next());
assert_eq!(Some((4, 'b')), char_indices.next());
assert_eq!(Some((5, 'y')), char_indices.next());
assert_eq!(Some((6, 'e')), char_indices.next());

assert_eq!(None, char_indices.next());
```

Remember,chars might not match your intuition about characters:

```
let yes = "y̆es";

let mut char_indices = yes.char_indices();

assert_eq!(Some((0, 'y')), char_indices.next()); // not (0, 'y̆')
assert_eq!(Some((1, '\u{0306}')), char_indices.next());

// note the 3 here - the previous character took up two bytes
assert_eq!(Some((3, 'e')), char_indices.next());
assert_eq!(Some((4, 's')), char_indices.next());

assert_eq!(None, char_indices.next());
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1146)
#### pub fnbytes(&self) ->Bytes<'_>

Returns an iterator over the bytes of a string slice.

As a string slice consists of a sequence of bytes, we can iterate
through a string slice by byte. This method returns such an iterator.

##### §Examples

```
let mut bytes = "bors".bytes();

assert_eq!(Some(b'b'), bytes.next());
assert_eq!(Some(b'o'), bytes.next());
assert_eq!(Some(b'r'), bytes.next());
assert_eq!(Some(b's'), bytes.next());

assert_eq!(None, bytes.next());
```

1.1.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1198)
#### pub fnsplit_whitespace(&self) ->SplitWhitespace<'_>

Splits a string slice by whitespace.

The iterator returned will return string slices that are sub-slices of
the original string slice, separated by any amount of whitespace.

‘Whitespace’ is defined according to the terms of the Unicode Derived
Core PropertyWhite_Space. If you only want to split on ASCII whitespace
instead, usesplit_ascii_whitespace.

##### §Examples

Basic usage:

```
let mut iter = "A few words".split_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

All kinds of whitespace are considered:

```
let mut iter = " Mary   had\ta\u{2009}little  \n\t lamb".split_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

If the string is empty or all whitespace, the iterator yields no string slices:

```
assert_eq!("".split_whitespace().next(), None);
assert_eq!("   ".split_whitespace().next(), None);
```

1.34.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1249)
#### pub fnsplit_ascii_whitespace(&self) ->SplitAsciiWhitespace<'_>

Splits a string slice by ASCII whitespace.

The iterator returned will return string slices that are sub-slices of
the original string slice, separated by any amount of ASCII whitespace.

This uses the same definition aschar::is_ascii_whitespace.
To split by UnicodeWhitespaceinstead, usesplit_whitespace.

##### §Examples

Basic usage:

```
let mut iter = "A few words".split_ascii_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

Various kinds of ASCII whitespace are considered
(seechar::is_ascii_whitespace):

```
let mut iter = " Mary   had\ta little  \n\t lamb".split_ascii_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

If the string is empty or all ASCII whitespace, the iterator yields no string slices:

```
assert_eq!("".split_ascii_whitespace().next(), None);
assert_eq!("   ".split_ascii_whitespace().next(), None);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1313)
#### pub fnlines(&self) ->Lines<'_>

Returns an iterator over the lines of a string, as string slices.

Lines are split at line endings that are either newlines (\n) or
sequences of a carriage return followed by a line feed (\r\n).

Line terminators are not included in the lines returned by the iterator.

Note that any carriage return (\r) not immediately followed by a
line feed (\n) does not split a line. These carriage returns are
thereby included in the produced lines.

The final line ending is optional. A string that ends with a final line
ending will return the same lines as an otherwise identical string
without a final line ending.

An empty string returns an empty iterator.

##### §Examples

Basic usage:

```
let text = "foo\r\nbar\n\nbaz\r";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
// Trailing carriage return is included in the last line
assert_eq!(Some("baz\r"), lines.next());

assert_eq!(None, lines.next());
```

The final line does not require any ending:

```
let text = "foo\nbar\n\r\nbaz";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
assert_eq!(Some("baz"), lines.next());

assert_eq!(None, lines.next());
```

An empty string returns an empty iterator:

```
let text = "";
let mut lines = text.lines();

assert_eq!(lines.next(), None);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1322)
#### pub fnlines_any(&self) ->LinesAny<'_>

👎
Deprecated since 1.4.0: use lines() instead now
Returns an iterator over the lines of a string.

1.8.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1342)
#### pub fnencode_utf16(&self) ->EncodeUtf16<'_>

Returns an iterator ofu16over the string encoded
as native endian UTF-16 (without byte-order mark).

##### §Examples

```
let text = "Zażółć gęślą jaźń";

let utf8_len = text.len();
let utf16_len = text.encode_utf16().count();

assert!(utf16_len <= utf8_len);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1367)
#### pub fncontains<P>(&self, pat: P) ->boolwhere
    P:Pattern,

Returnstrueif the given pattern matches a sub-slice of
this string slice.

Returnsfalseif it does not.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Examples

```
let bananas = "bananas";

assert!(bananas.contains("nana"));
assert!(!bananas.contains("apples"));
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1405)
#### pub fnstarts_with<P>(&self, pat: P) ->boolwhere
    P:Pattern,

Returnstrueif the given pattern matches a prefix of this
string slice.

Returnsfalseif it does not.

Thepatterncan be a&str, in which case this function will return true if
the&stris a prefix of this string slice.

Thepatterncan also be achar, a slice ofchars, or a
function or closure that determines if a character matches.
These will only be checked against the first character of this string slice.
Look at the second example below regarding behavior for slices ofchars.

##### §Examples

```
let bananas = "bananas";

assert!(bananas.starts_with("bana"));
assert!(!bananas.starts_with("nana"));
```

```
let bananas = "bananas";

// Note that both of these assert successfully.
assert!(bananas.starts_with(&['b', 'a', 'n', 'a']));
assert!(bananas.starts_with(&['a', 'b', 'c', 'd']));
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1430-1432)
#### pub fnends_with<P>(&self, pat: P) ->boolwhere
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

Returnstrueif the given pattern matches a suffix of this
string slice.

Returnsfalseif it does not.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Examples

```
let bananas = "bananas";

assert!(bananas.ends_with("anas"));
assert!(!bananas.ends_with("nana"));
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1481)
#### pub fnfind<P>(&self, pat: P) ->Option<usize>where
    P:Pattern,

Returns the byte index of the first character of this string slice that
matches the pattern.

ReturnsNoneif the pattern doesn’t match.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Examples

Simple patterns:

```
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.find('L'), Some(0));
assert_eq!(s.find('é'), Some(14));
assert_eq!(s.find("pard"), Some(17));
```

More complex patterns using point-free style and closures:

```
let s = "Löwe 老虎 Léopard";

assert_eq!(s.find(char::is_whitespace), Some(5));
assert_eq!(s.find(char::is_lowercase), Some(1));
assert_eq!(s.find(|c: char| c.is_whitespace() || c.is_lowercase()), Some(1));
assert_eq!(s.find(|c: char| (c < 'o') && (c > 'a')), Some(4));
```

Not finding the pattern:

```
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.find(x), None);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1527-1529)
#### pub fnrfind<P>(&self, pat: P) ->Option<usize>where
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

Returns the byte index for the first character of the last match of the pattern in
this string slice.

ReturnsNoneif the pattern doesn’t match.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Examples

Simple patterns:

```
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.rfind('L'), Some(13));
assert_eq!(s.rfind('é'), Some(14));
assert_eq!(s.rfind("pard"), Some(24));
```

More complex patterns with closures:

```
let s = "Löwe 老虎 Léopard";

assert_eq!(s.rfind(char::is_whitespace), Some(12));
assert_eq!(s.rfind(char::is_lowercase), Some(20));
```

Not finding the pattern:

```
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.rfind(x), None);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1655)
#### pub fnsplit<P>(&self, pat: P) ->Split<'_, P>where
    P:Pattern,

Returns an iterator over substrings of this string slice, separated by
characters matched by a pattern.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

If there are no matches the full string slice is returned as the only
item in the iterator.

##### §Iterator behavior

The returned iterator will be aDoubleEndedIteratorif the pattern
allows a reverse search and forward/reverse search yields the same
elements. This is true for, e.g.,char, but not for&str.

If the pattern allows a reverse search but its results might differ
from a forward search, thersplitmethod can be used.

##### §Examples

Simple patterns:

```
let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

let v: Vec<&str> = "".split('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
assert_eq!(v, ["lion", "", "tiger", "leopard"]);

let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);

let v: Vec<&str> = "AABBCC".split("DD").collect();
assert_eq!(v, ["AABBCC"]);

let v: Vec<&str> = "abc1def2ghi".split(char::is_numeric).collect();
assert_eq!(v, ["abc", "def", "ghi"]);

let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);
```

If the pattern is a slice of chars, split on each occurrence of any of the characters:

```
let v: Vec<&str> = "2020-11-03 23:59".split(&['-', ' ', ':', '@'][..]).collect();
assert_eq!(v, ["2020", "11", "03", "23", "59"]);
```

A more complex pattern, using a closure:

```
let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "def", "ghi"]);
```

If a string contains multiple contiguous separators, you will end up
with empty strings in the output:

```
let x = "||||a||b|c".to_string();
let d: Vec<_> = x.split('|').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

Contiguous separators are separated by the empty string.

```
let x = "(///)".to_string();
let d: Vec<_> = x.split('/').collect();

assert_eq!(d, &["(", "", "", ")"]);
```

Separators at the start or end of a string are neighbored
by empty strings.

```
let d: Vec<_> = "010".split("0").collect();
assert_eq!(d, &["", "1", ""]);
```

When the empty string is used as a separator, it separates
every character in the string, along with the beginning
and end of the string.

```
let f: Vec<_> = "rust".split("").collect();
assert_eq!(f, &["", "r", "u", "s", "t", ""]);
```

Contiguous separators can lead to possibly surprising behavior
when whitespace is used as the separator. This code is correct:

```
let x = "    a  b c".to_string();
let d: Vec<_> = x.split(' ').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

It doesnotgive you:

ⓘ
```
assert_eq!(d, &["a", "b", "c"]);
```

Usesplit_whitespacefor this behavior.

1.51.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1696)
#### pub fnsplit_inclusive<P>(&self, pat: P) ->SplitInclusive<'_, P>where
    P:Pattern,

Returns an iterator over substrings of this string slice, separated by
characters matched by a pattern.

Differs from the iterator produced bysplitin thatsplit_inclusiveleaves the matched part as the terminator of the substring.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Examples

```
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb."
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb."]);
```

If the last element of the string is matched,
that element will be considered the terminator of the preceding substring.
That substring will be the last item returned by the iterator.

```
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb.\n"
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb.\n"]);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1751-1753)
#### pub fnrsplit<P>(&self, pat: P) ->RSplit<'_, P>where
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

Returns an iterator over substrings of the given string slice, separated
by characters matched by a pattern and yielded in reverse order.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator requires that the pattern supports a reverse
search, and it will be aDoubleEndedIteratorif a forward/reverse
search yields the same elements.

For iterating from the front, thesplitmethod can be used.

##### §Examples

Simple patterns:

```
let v: Vec<&str> = "Mary had a little lamb".rsplit(' ').collect();
assert_eq!(v, ["lamb", "little", "a", "had", "Mary"]);

let v: Vec<&str> = "".rsplit('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplit('X').collect();
assert_eq!(v, ["leopard", "tiger", "", "lion"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplit("::").collect();
assert_eq!(v, ["leopard", "tiger", "lion"]);
```

A more complex pattern, using a closure:

```
let v: Vec<&str> = "abc1defXghi".rsplit(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "def", "abc"]);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1800)
#### pub fnsplit_terminator<P>(&self, pat: P) ->SplitTerminator<'_, P>where
    P:Pattern,

Returns an iterator over substrings of the given string slice, separated
by characters matched by a pattern.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

Equivalent tosplit, except that the trailing substring
is skipped if empty.

This method can be used for string data that isterminated,
rather thanseparatedby a pattern.

##### §Iterator behavior

The returned iterator will be aDoubleEndedIteratorif the pattern
allows a reverse search and forward/reverse search yields the same
elements. This is true for, e.g.,char, but not for&str.

If the pattern allows a reverse search but its results might differ
from a forward search, thersplit_terminatormethod can be used.

##### §Examples

```
let v: Vec<&str> = "A.B.".split_terminator('.').collect();
assert_eq!(v, ["A", "B"]);

let v: Vec<&str> = "A..B..".split_terminator(".").collect();
assert_eq!(v, ["A", "", "B", ""]);

let v: Vec<&str> = "A.B:C.D".split_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["A", "B", "C", "D"]);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1846-1848)
#### pub fnrsplit_terminator<P>(&self, pat: P) ->RSplitTerminator<'_, P>where
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

Returns an iterator over substrings ofself, separated by characters
matched by a pattern and yielded in reverse order.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

Equivalent tosplit, except that the trailing substring is
skipped if empty.

This method can be used for string data that isterminated,
rather thanseparatedby a pattern.

##### §Iterator behavior

The returned iterator requires that the pattern supports a
reverse search, and it will be double ended if a forward/reverse
search yields the same elements.

For iterating from the front, thesplit_terminatormethod can be
used.

##### §Examples

```
let v: Vec<&str> = "A.B.".rsplit_terminator('.').collect();
assert_eq!(v, ["B", "A"]);

let v: Vec<&str> = "A..B..".rsplit_terminator(".").collect();
assert_eq!(v, ["", "B", "", "A"]);

let v: Vec<&str> = "A.B:C.D".rsplit_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["D", "C", "B", "A"]);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1901)
#### pub fnsplitn<P>(&self, n:usize, pat: P) ->SplitN<'_, P>where
    P:Pattern,

Returns an iterator over substrings of the given string slice, separated
by a pattern, restricted to returning at mostnitems.

Ifnsubstrings are returned, the last substring (thenth substring)
will contain the remainder of the string.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator will not be double ended, because it is
not efficient to support.

If the pattern allows a reverse search, thersplitnmethod can be
used.

##### §Examples

Simple patterns:

```
let v: Vec<&str> = "Mary had a little lambda".splitn(3, ' ').collect();
assert_eq!(v, ["Mary", "had", "a little lambda"]);

let v: Vec<&str> = "lionXXtigerXleopard".splitn(3, "X").collect();
assert_eq!(v, ["lion", "", "tigerXleopard"]);

let v: Vec<&str> = "abcXdef".splitn(1, 'X').collect();
assert_eq!(v, ["abcXdef"]);

let v: Vec<&str> = "".splitn(1, 'X').collect();
assert_eq!(v, [""]);
```

A more complex pattern, using a closure:

```
let v: Vec<&str> = "abc1defXghi".splitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "defXghi"]);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1950-1952)
#### pub fnrsplitn<P>(&self, n:usize, pat: P) ->RSplitN<'_, P>where
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

Returns an iterator over substrings of this string slice, separated by a
pattern, starting from the end of the string, restricted to returning at
mostnitems.

Ifnsubstrings are returned, the last substring (thenth substring)
will contain the remainder of the string.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator will not be double ended, because it is not
efficient to support.

For splitting from the front, thesplitnmethod can be used.

##### §Examples

Simple patterns:

```
let v: Vec<&str> = "Mary had a little lamb".rsplitn(3, ' ').collect();
assert_eq!(v, ["lamb", "little", "Mary had a"]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplitn(3, 'X').collect();
assert_eq!(v, ["leopard", "tiger", "lionX"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplitn(2, "::").collect();
assert_eq!(v, ["leopard", "lion::tiger"]);
```

A more complex pattern, using a closure:

```
let v: Vec<&str> = "abc1defXghi".rsplitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "abc1def"]);
```

1.52.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1970)
#### pub fnsplit_once<P>(&self, delimiter: P) ->Option<(&str, &str)>where
    P:Pattern,

Splits the string on the first occurrence of the specified delimiter and
returns prefix before delimiter and suffix after delimiter.

##### §Examples

```
assert_eq!("cfg".split_once('='), None);
assert_eq!("cfg=".split_once('='), Some(("cfg", "")));
assert_eq!("cfg=foo".split_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".split_once('='), Some(("cfg", "foo=bar")));
```

1.52.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1989-1991)
#### pub fnrsplit_once<P>(&self, delimiter: P) ->Option<(&str, &str)>where
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

Splits the string on the last occurrence of the specified delimiter and
returns prefix before delimiter and suffix after delimiter.

##### §Examples

```
assert_eq!("cfg".rsplit_once('='), None);
assert_eq!("cfg=".rsplit_once('='), Some(("cfg", "")));
assert_eq!("cfg=foo".rsplit_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".rsplit_once('='), Some(("cfg=foo", "bar")));
```

1.2.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2029)
#### pub fnmatches<P>(&self, pat: P) ->Matches<'_, P>where
    P:Pattern,

Returns an iterator over the disjoint matches of a pattern within the
given string slice.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator will be aDoubleEndedIteratorif the pattern
allows a reverse search and forward/reverse search yields the same
elements. This is true for, e.g.,char, but not for&str.

If the pattern allows a reverse search but its results might differ
from a forward search, thermatchesmethod can be used.

##### §Examples

```
let v: Vec<&str> = "abcXXXabcYYYabc".matches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".matches(char::is_numeric).collect();
assert_eq!(v, ["1", "2", "3"]);
```

1.2.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2063-2065)
#### pub fnrmatches<P>(&self, pat: P) ->RMatches<'_, P>where
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

Returns an iterator over the disjoint matches of a pattern within this
string slice, yielded in reverse order.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator requires that the pattern supports a reverse
search, and it will be aDoubleEndedIteratorif a forward/reverse
search yields the same elements.

For iterating from the front, thematchesmethod can be used.

##### §Examples

```
let v: Vec<&str> = "abcXXXabcYYYabc".rmatches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".rmatches(char::is_numeric).collect();
assert_eq!(v, ["3", "2", "1"]);
```

1.5.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2107)
#### pub fnmatch_indices<P>(&self, pat: P) ->MatchIndices<'_, P>where
    P:Pattern,

Returns an iterator over the disjoint matches of a pattern within this string
slice as well as the index that the match starts at.

For matches ofpatwithinselfthat overlap, only the indices
corresponding to the first match are returned.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator will be aDoubleEndedIteratorif the pattern
allows a reverse search and forward/reverse search yields the same
elements. This is true for, e.g.,char, but not for&str.

If the pattern allows a reverse search but its results might differ
from a forward search, thermatch_indicesmethod can be used.

##### §Examples

```
let v: Vec<_> = "abcXXXabcYYYabc".match_indices("abc").collect();
assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);

let v: Vec<_> = "1abcabc2".match_indices("abc").collect();
assert_eq!(v, [(1, "abc"), (4, "abc")]);

let v: Vec<_> = "ababa".match_indices("aba").collect();
assert_eq!(v, [(0, "aba")]); // only the first `aba`
```

1.5.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2147-2149)
#### pub fnrmatch_indices<P>(&self, pat: P) ->RMatchIndices<'_, P>where
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

Returns an iterator over the disjoint matches of a pattern withinself,
yielded in reverse order along with the index of the match.

For matches ofpatwithinselfthat overlap, only the indices
corresponding to the last match are returned.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Iterator behavior

The returned iterator requires that the pattern supports a reverse
search, and it will be aDoubleEndedIteratorif a forward/reverse
search yields the same elements.

For iterating from the front, thematch_indicesmethod can be used.

##### §Examples

```
let v: Vec<_> = "abcXXXabcYYYabc".rmatch_indices("abc").collect();
assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);

let v: Vec<_> = "1abcabc2".rmatch_indices("abc").collect();
assert_eq!(v, [(4, "abc"), (1, "abc")]);

let v: Vec<_> = "ababa".rmatch_indices("aba").collect();
assert_eq!(v, [(2, "aba")]); // only the last `aba`
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2171)
#### pub fntrim(&self) -> &str

Returns a string slice with leading and trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived
Core PropertyWhite_Space, which includes newlines.

##### §Examples

```
let s = "\n Hello\tworld\t\n";

assert_eq!("Hello\tworld", s.trim());
```

1.30.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2210)
#### pub fntrim_start(&self) -> &str

Returns a string slice with leading whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived
Core PropertyWhite_Space, which includes newlines.

##### §Text directionality

A string is a sequence of bytes.startin this context means the first
position of that byte string; for a left-to-right language like English or
Russian, this will be left side, and for right-to-left languages like
Arabic or Hebrew, this will be the right side.

##### §Examples

Basic usage:

```
let s = "\n Hello\tworld\t\n";
assert_eq!("Hello\tworld\t\n", s.trim_start());
```

Directionality:

```
let s = "  English  ";
assert!(Some('E') == s.trim_start().chars().next());

let s = "  עברית  ";
assert!(Some('ע') == s.trim_start().chars().next());
```

1.30.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2249)
#### pub fntrim_end(&self) -> &str

Returns a string slice with trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived
Core PropertyWhite_Space, which includes newlines.

##### §Text directionality

A string is a sequence of bytes.endin this context means the last
position of that byte string; for a left-to-right language like English or
Russian, this will be right side, and for right-to-left languages like
Arabic or Hebrew, this will be the left side.

##### §Examples

Basic usage:

```
let s = "\n Hello\tworld\t\n";
assert_eq!("\n Hello\tworld", s.trim_end());
```

Directionality:

```
let s = "  English  ";
assert!(Some('h') == s.trim_end().chars().rev().next());

let s = "  עברית  ";
assert!(Some('ת') == s.trim_end().chars().rev().next());
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2289)
#### pub fntrim_left(&self) -> &str

👎
Deprecated since 1.33.0: superseded by
`trim_start`Returns a string slice with leading whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived
Core PropertyWhite_Space.

##### §Text directionality

A string is a sequence of bytes. ‘Left’ in this context means the first
position of that byte string; for a language like Arabic or Hebrew
which are ‘right to left’ rather than ‘left to right’, this will be
therightside, not the left.

##### §Examples

Basic usage:

```
let s = " Hello\tworld\t";

assert_eq!("Hello\tworld\t", s.trim_left());
```

Directionality:

```
let s = "  English";
assert!(Some('E') == s.trim_left().chars().next());

let s = "  עברית";
assert!(Some('ע') == s.trim_left().chars().next());
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2329)
#### pub fntrim_right(&self) -> &str

👎
Deprecated since 1.33.0: superseded by
`trim_end`Returns a string slice with trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived
Core PropertyWhite_Space.

##### §Text directionality

A string is a sequence of bytes. ‘Right’ in this context means the last
position of that byte string; for a language like Arabic or Hebrew
which are ‘right to left’ rather than ‘left to right’, this will be
theleftside, not the right.

##### §Examples

Basic usage:

```
let s = " Hello\tworld\t";

assert_eq!(" Hello\tworld", s.trim_right());
```

Directionality:

```
let s = "English  ";
assert!(Some('h') == s.trim_right().chars().rev().next());

let s = "עברית  ";
assert!(Some('ת') == s.trim_right().chars().rev().next());
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2362-2364)
#### pub fntrim_matches<P>(&self, pat: P) -> &strwhere
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>DoubleEndedSearcher<'a>,

Returns a string slice with all prefixes and suffixes that match a
pattern repeatedly removed.

Thepatterncan be achar, a slice ofchars, or a function
or closure that determines if a character matches.

##### §Examples

Simple patterns:

```
assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
```

A more complex pattern, using a closure:

```
assert_eq!("1foo1barXX".trim_matches(|c| c == '1' || c == 'X'), "foo1bar");
```

1.30.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2409)
#### pub fntrim_start_matches<P>(&self, pat: P) -> &strwhere
    P:Pattern,

Returns a string slice with all prefixes that match a pattern
repeatedly removed.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Text directionality

A string is a sequence of bytes.startin this context means the first
position of that byte string; for a left-to-right language like English or
Russian, this will be left side, and for right-to-left languages like
Arabic or Hebrew, this will be the right side.

##### §Examples

```
assert_eq!("11foo1bar11".trim_start_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_start_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_start_matches(x), "foo1bar12");
```

1.45.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2443)
#### pub fnstrip_prefix<P>(&self, prefix: P) ->Option<&str>where
    P:Pattern,

Returns a string slice with the prefix removed.

If the string starts with the patternprefix, returns the substring after the prefix,
wrapped inSome. Unliketrim_start_matches, this method removes the prefix exactly once.

If the string does not start withprefix, returnsNone.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Examples

```
assert_eq!("foo:bar".strip_prefix("foo:"), Some("bar"));
assert_eq!("foo:bar".strip_prefix("bar"), None);
assert_eq!("foofoo".strip_prefix("foo"), Some("foo"));
```

1.45.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2471-2473)
#### pub fnstrip_suffix<P>(&self, suffix: P) ->Option<&str>where
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

Returns a string slice with the suffix removed.

If the string ends with the patternsuffix, returns the substring before the suffix,
wrapped inSome.  Unliketrim_end_matches, this method removes the suffix exactly once.

If the string does not end withsuffix, returnsNone.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Examples

```
assert_eq!("bar:foo".strip_suffix(":foo"), Some("bar"));
assert_eq!("bar:foo".strip_suffix("bar"), None);
assert_eq!("foofoo".strip_suffix("foo"), Some("foo"));
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2507-2509)
#### pub fnstrip_circumfix<P, S>(&self, prefix: P, suffix: S) ->Option<&str>where
    P:Pattern,
    S:Pattern,
    <S asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

🔬
This is a nightly-only experimental API. (
`strip_circumfix`)
Returns a string slice with the prefix and suffix removed.

If the string starts with the patternprefixand ends with the patternsuffix, returns
the substring after the prefix and before the suffix, wrapped inSome.
Unliketrim_start_matchesandtrim_end_matches, this method removes both the prefix
and suffix exactly once.

If the string does not start withprefixor does not end withsuffix, returnsNone.

Eachpatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Examples

```
#![feature(strip_circumfix)]

assert_eq!("bar:hello:foo".strip_circumfix("bar:", ":foo"), Some("hello"));
assert_eq!("bar:foo".strip_circumfix("foo", "foo"), None);
assert_eq!("foo:bar;".strip_circumfix("foo:", ';'), Some("bar"));
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2547)
#### pub fntrim_prefix<P>(&self, prefix: P) -> &strwhere
    P:Pattern,

🔬
This is a nightly-only experimental API. (
`trim_prefix_suffix`)
Returns a string slice with the optional prefix removed.

If the string starts with the patternprefix, returns the substring after the prefix.
Unlikestrip_prefix, this method always returns&strfor easy method chaining,
instead of returningOption<&str>.

If the string does not start withprefix, returns the original string unchanged.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Examples

```
#![feature(trim_prefix_suffix)]

// Prefix present - removes it
assert_eq!("foo:bar".trim_prefix("foo:"), "bar");
assert_eq!("foofoo".trim_prefix("foo"), "foo");

// Prefix absent - returns original string
assert_eq!("foo:bar".trim_prefix("bar"), "foo:bar");

// Method chaining example
assert_eq!("<https://example.com/>".trim_prefix('<').trim_suffix('>'), "https://example.com/");
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2584-2586)
#### pub fntrim_suffix<P>(&self, suffix: P) -> &strwhere
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

🔬
This is a nightly-only experimental API. (
`trim_prefix_suffix`)
Returns a string slice with the optional suffix removed.

If the string ends with the patternsuffix, returns the substring before the suffix.
Unlikestrip_suffix, this method always returns&strfor easy method chaining,
instead of returningOption<&str>.

If the string does not end withsuffix, returns the original string unchanged.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Examples

```
#![feature(trim_prefix_suffix)]

// Suffix present - removes it
assert_eq!("bar:foo".trim_suffix(":foo"), "bar");
assert_eq!("foofoo".trim_suffix("foo"), "foo");

// Suffix absent - returns original string
assert_eq!("bar:foo".trim_suffix("bar"), "bar:foo");

// Method chaining example
assert_eq!("<https://example.com/>".trim_prefix('<').trim_suffix('>'), "https://example.com/");
```

1.30.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2627-2629)
#### pub fntrim_end_matches<P>(&self, pat: P) -> &strwhere
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

Returns a string slice with all suffixes that match a pattern
repeatedly removed.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Text directionality

A string is a sequence of bytes.endin this context means the last
position of that byte string; for a left-to-right language like English or
Russian, this will be right side, and for right-to-left languages like
Arabic or Hebrew, this will be the left side.

##### §Examples

Simple patterns:

```
assert_eq!("11foo1bar11".trim_end_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_end_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_end_matches(x), "12foo1bar");
```

A more complex pattern, using a closure:

```
assert_eq!("1fooX".trim_end_matches(|c| c == '1' || c == 'X'), "1foo");
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2671)
#### pub fntrim_left_matches<P>(&self, pat: P) -> &strwhere
    P:Pattern,

👎
Deprecated since 1.33.0: superseded by
`trim_start_matches`Returns a string slice with all prefixes that match a pattern
repeatedly removed.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Text directionality

A string is a sequence of bytes. ‘Left’ in this context means the first
position of that byte string; for a language like Arabic or Hebrew
which are ‘right to left’ rather than ‘left to right’, this will be
therightside, not the left.

##### §Examples

```
assert_eq!("11foo1bar11".trim_left_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_left_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_left_matches(x), "foo1bar12");
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2714-2716)
#### pub fntrim_right_matches<P>(&self, pat: P) -> &strwhere
    P:Pattern,
    <P asPattern>::Searcher<'a>: for<'a>ReverseSearcher<'a>,

👎
Deprecated since 1.33.0: superseded by
`trim_end_matches`Returns a string slice with all suffixes that match a pattern
repeatedly removed.

Thepatterncan be a&str,char, a slice ofchars, or a
function or closure that determines if a character matches.

##### §Text directionality

A string is a sequence of bytes. ‘Right’ in this context means the last
position of that byte string; for a language like Arabic or Hebrew
which are ‘right to left’ rather than ‘left to right’, this will be
theleftside, not the right.

##### §Examples

Simple patterns:

```
assert_eq!("11foo1bar11".trim_right_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_right_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_right_matches(x), "12foo1bar");
```

A more complex pattern, using a closure:

```
assert_eq!("1fooX".trim_right_matches(|c| c == '1' || c == 'X'), "1foo");
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2765)
#### pub fnparse<F>(&self) ->Result<F, <F asFromStr>::Err>where
    F:FromStr,

Parses this string slice into another type.

Becauseparseis so general, it can cause problems with type
inference. As such,parseis one of the few times you’ll see
the syntax affectionately known as the ‘turbofish’:::<>. This
helps the inference algorithm understand specifically which type
you’re trying to parse into.

parsecan parse into any type that implements theFromStrtrait.

##### §Errors

Will returnErrif it’s not possible to parse this string slice into
the desired type.

##### §Examples

Basic usage:

```
let four: u32 = "4".parse().unwrap();

assert_eq!(4, four);
```

Using the ‘turbofish’ instead of annotatingfour:

```
let four = "4".parse::<u32>();

assert_eq!(Ok(4), four);
```

Failing to parse:

```
let nope = "j".parse::<u32>();

assert!(nope.is_err());
```

1.23.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2786)
#### pub fnis_ascii(&self) ->bool

Checks if all characters in this string are within the ASCII range.

An empty string returnstrue.

##### §Examples

```
let ascii = "hello!\n";
let non_ascii = "Grüße, Jürgen ❤";

assert!(ascii.is_ascii());
assert!(!non_ascii.is_ascii());
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2798)
#### pub fnas_ascii(&self) ->Option<&[AsciiChar]>

🔬
This is a nightly-only experimental API. (
`ascii_char`)
If this string sliceis_ascii, returns it as a slice
ofASCII characters, otherwise returnsNone.

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2812)
#### pub unsafe fnas_ascii_unchecked(&self) -> &[AsciiChar]

🔬
This is a nightly-only experimental API. (
`ascii_char`)
Converts this string slice into a slice ofASCII characters,
without checking whether they are valid.

##### §Safety

Every character in this string must be ASCII, or else this is UB.

1.23.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2840)
#### pub fneq_ignore_ascii_case(&self, other: &str) ->bool

Checks that two strings are an ASCII case-insensitive match.

Same asto_ascii_lowercase(a) == to_ascii_lowercase(b),
but without allocating and copying temporaries.

##### §Examples

```
assert!("Ferris".eq_ignore_ascii_case("FERRIS"));
assert!("Ferrös".eq_ignore_ascii_case("FERRöS"));
assert!(!"Ferrös".eq_ignore_ascii_case("FERRÖS"));
```

1.80.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2919)
#### pub fntrim_ascii_start(&self) -> &str

Returns a string slice with leading ASCII whitespace removed.

‘Whitespace’ refers to the definition used byu8::is_ascii_whitespace.

##### §Examples

```
assert_eq!(" \t \u{3000}hello world\n".trim_ascii_start(), "\u{3000}hello world\n");
assert_eq!("  ".trim_ascii_start(), "");
assert_eq!("".trim_ascii_start(), "");
```

1.80.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2944)
#### pub fntrim_ascii_end(&self) -> &str

Returns a string slice with trailing ASCII whitespace removed.

‘Whitespace’ refers to the definition used byu8::is_ascii_whitespace.

##### §Examples

```
assert_eq!("\r hello world\u{3000}\n ".trim_ascii_end(), "\r hello world\u{3000}");
assert_eq!("  ".trim_ascii_end(), "");
assert_eq!("".trim_ascii_end(), "");
```

1.80.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2970)
#### pub fntrim_ascii(&self) -> &str

Returns a string slice with leading and trailing ASCII whitespace
removed.

‘Whitespace’ refers to the definition used byu8::is_ascii_whitespace.

##### §Examples

```
assert_eq!("\r hello world\n ".trim_ascii(), "hello world");
assert_eq!("  ".trim_ascii(), "");
assert_eq!("".trim_ascii(), "");
```

1.34.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3013)
#### pub fnescape_debug(&self) ->EscapeDebug<'_>

Returns an iterator that escapes each char inselfwithchar::escape_debug.

Note: only extended grapheme codepoints that begin the string will be
escaped.

##### §Examples

As an iterator:

```
for c in "❤\n!".escape_debug() {
    print!("{c}");
}
println!();
```

Usingprintln!directly:

```
println!("{}", "❤\n!".escape_debug());
```

Both are equivalent to:

```
println!("❤\\n!");
```

Usingto_string:

```
assert_eq!("❤\n!".escape_debug().to_string(), "❤\\n!");
```

1.34.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3059)
#### pub fnescape_default(&self) ->EscapeDefault<'_>

Returns an iterator that escapes each char inselfwithchar::escape_default.

##### §Examples

As an iterator:

```
for c in "❤\n!".escape_default() {
    print!("{c}");
}
println!();
```

Usingprintln!directly:

```
println!("{}", "❤\n!".escape_default());
```

Both are equivalent to:

```
println!("\\u{{2764}}\\n!");
```

Usingto_string:

```
assert_eq!("❤\n!".escape_default().to_string(), "\\u{2764}\\n!");
```

1.34.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3097)
#### pub fnescape_unicode(&self) ->EscapeUnicode<'_>

Returns an iterator that escapes each char inselfwithchar::escape_unicode.

##### §Examples

As an iterator:

```
for c in "❤\n!".escape_unicode() {
    print!("{c}");
}
println!();
```

Usingprintln!directly:

```
println!("{}", "❤\n!".escape_unicode());
```

Both are equivalent to:

```
println!("\\u{{2764}}\\u{{a}}\\u{{21}}");
```

Usingto_string:

```
assert_eq!("❤\n!".escape_unicode().to_string(), "\\u{2764}\\u{a}\\u{21}");
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3130)
#### pub fnsubstr_range(&self, substr: &str) ->Option<Range<usize>>

🔬
This is a nightly-only experimental API. (
`substr_range`)
Returns the range that a substring points to.

ReturnsNoneifsubstrdoes not point withinself.

Unlikestr::find,this does not search through the string.
Instead, it uses pointer arithmetic to find where in the stringsubstris derived from.

This is useful for extendingstr::splitand similar methods.

Note that this method may return false positives (typically eitherSome(0..0)orSome(self.len()..self.len())) ifsubstris a
zero-lengthstrthat points at the beginning or end of another,
independent,str.

##### §Examples

```
#![feature(substr_range)]

let data = "a, b, b, a";
let mut iter = data.split(", ").map(|s| data.substr_range(s).unwrap());

assert_eq!(iter.next(), Some(0..1));
assert_eq!(iter.next(), Some(3..4));
assert_eq!(iter.next(), Some(6..7));
assert_eq!(iter.next(), Some(9..10));
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3141)
#### pub fnas_str(&self) -> &str

🔬
This is a nightly-only experimental API. (
`str_as_str`)
Returns the same string as a string slice&str.

This method is redundant when used directly on&str, but
it helps dereferencing other string-like types to string slices,
for example references toBox<str>orArc<str>.

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#268)
#### pub fnreplace<P>(&self, from: P, to: &str) ->Stringwhere
    P:Pattern,

Available on
**non-no_global_oom_handling**only.
Replaces all matches of a pattern with another string.

replacecreates a newString, and copies the data from this string slice into it.
While doing so, it attempts to find matches of a pattern. If it finds any, it
replaces them with the replacement string slice.

##### §Examples

```
let s = "this is old";

assert_eq!("this is new", s.replace("old", "new"));
assert_eq!("than an old", s.replace("is", "an"));
```

When the pattern doesn’t match, it returns this string slice asString:

```
let s = "this is old";
assert_eq!(s, s.replace("cookie monster", "little lamb"));
```

1.16.0
·
[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#323)
#### pub fnreplacen<P>(&self, pat: P, to: &str, count:usize) ->Stringwhere
    P:Pattern,

Available on
**non-no_global_oom_handling**only.
Replaces first N matches of a pattern with another string.

replacencreates a newString, and copies the data from this string slice into it.
While doing so, it attempts to find matches of a pattern. If it finds any, it
replaces them with the replacement string slice at mostcounttimes.

##### §Examples

```
let s = "foo foo 123 foo";
assert_eq!("new new 123 foo", s.replacen("foo", "new", 2));
assert_eq!("faa fao 123 foo", s.replacen('o', "a", 3));
assert_eq!("foo foo new23 foo", s.replacen(char::is_numeric, "new", 1));
```

When the pattern doesn’t match, it returns this string slice asString:

```
let s = "this is old";
assert_eq!(s, s.replacen("cookie monster", "little lamb", 10));
```

1.2.0
·
[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#380)
#### pub fnto_lowercase(&self) ->String

Available on
**non-no_global_oom_handling**only.
Returns the lowercase equivalent of this string slice, as a newString.

‘Lowercase’ is defined according to the terms of the Unicode Derived Core PropertyLowercase.

Since some characters can expand into multiple characters when changing
the case, this function returns aStringinstead of modifying the
parameter in-place.

##### §Examples

Basic usage:

```
let s = "HELLO";

assert_eq!("hello", s.to_lowercase());
```

A tricky example, with sigma:

```
let sigma = "Σ";

assert_eq!("σ", sigma.to_lowercase());

// but at the end of a word, it's ς, not σ:
let odysseus = "ὈΔΥΣΣΕΎΣ";

assert_eq!("ὀδυσσεύς", odysseus.to_lowercase());
```

Languages without case are not changed:

```
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_lowercase());
```

1.2.0
·
[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#465)
#### pub fnto_uppercase(&self) ->String

Available on
**non-no_global_oom_handling**only.
Returns the uppercase equivalent of this string slice, as a newString.

‘Uppercase’ is defined according to the terms of the Unicode Derived Core PropertyUppercase.

Since some characters can expand into multiple characters when changing
the case, this function returns aStringinstead of modifying the
parameter in-place.

##### §Examples

Basic usage:

```
let s = "hello";

assert_eq!("HELLO", s.to_uppercase());
```

Scripts without case are not changed:

```
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_uppercase());
```

One character can become multiple:

```
let s = "tschüß";

assert_eq!("TSCHÜSS", s.to_uppercase());
```

1.16.0
·
[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#529)
#### pub fnrepeat(&self, n:usize) ->String

Available on
**non-no_global_oom_handling**only.
Creates a newStringby repeating a stringntimes.

##### §Panics

This function will panic if the capacity would overflow.

##### §Examples

Basic usage:

```
assert_eq!("abc".repeat(4), String::from("abcabcabcabc"));
```

A panic upon overflow:

ⓘ
```
// this will panic at runtime
let huge = "0123456789abcdef".repeat(usize::MAX);
```

1.23.0
·
[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#559)
#### pub fnto_ascii_uppercase(&self) ->String

Available on
**non-no_global_oom_handling**only.
Returns a copy of this string where each character is mapped to its
ASCII upper case equivalent.

ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’,
but non-ASCII letters are unchanged.

To uppercase the value in-place, usemake_ascii_uppercase.

To uppercase ASCII characters in addition to non-ASCII characters, useto_uppercase.

##### §Examples

```
let s = "Grüße, Jürgen ❤";

assert_eq!("GRüßE, JüRGEN ❤", s.to_ascii_uppercase());
```

1.23.0
·
[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#591)
#### pub fnto_ascii_lowercase(&self) ->String

Available on
**non-no_global_oom_handling**only.
Returns a copy of this string where each character is mapped to its
ASCII lower case equivalent.

ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’,
but non-ASCII letters are unchanged.

To lowercase the value in-place, usemake_ascii_lowercase.

To lowercase ASCII characters in addition to non-ASCII characters, useto_lowercase.

##### §Examples

```
let s = "Grüße, Jürgen ❤";

assert_eq!("grüße, jürgen ❤", s.to_ascii_lowercase());
```

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#257)§
### implAdd<&str> forSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#258)§
#### typeOutput=SharedString

The resulting type after applying the
`+`operator.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#259)§
#### fnadd(self, other: &str) ->SharedString

Performs the
`+`operation.
[Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#251)§
### implAddAssign<&str> forSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#252)§
#### fnadd_assign(&mut self, other: &str)

Performs the
`+=`operation.
[Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#190)§
### implAsRef<[u8]> forSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#192)§
#### fnas_ref(&self) -> &[u8]ⓘ

Converts this type into a shared reference of the (usually inferred) input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#162)§
### implAsRef<CStr> forSharedString

Available on
**crate featurestd**only.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#164)§
#### fnas_ref(&self) -> &CStr

Converts this type into a shared reference of the (usually inferred) input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#183)§
### implAsRef<OsStr> forSharedString

Available on
**crate featurestd**only.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#185)§
#### fnas_ref(&self) -> &OsStr

Converts this type into a shared reference of the (usually inferred) input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#175)§
### implAsRef<Path> forSharedString

Available on
**crate featurestd**only.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#177)§
#### fnas_ref(&self) -> &Path

Converts this type into a shared reference of the (usually inferred) input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#132)§
### implAsRef<str> forSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#134)§
#### fnas_ref(&self) -> &str

Converts this type into a shared reference of the (usually inferred) input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#278)§
### implBorrow<str> forSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#279)§
#### fnborrow(&self) -> &str

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#41)§
### implCloneforSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#41)§
#### fnclone(&self) ->SharedString

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#120)§
### implDebugforSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#121)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#41)§
### implDefaultforSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#41)§
#### fndefault() ->SharedString

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#103)§
### implDerefforSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#104)§
#### typeTarget=str

The resulting type after dereferencing.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#105)§
#### fnderef(&self) -> &<SharedStringasDeref>::Target

Dereferences the value.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#126)§
### implDisplayforSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#127)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#284)§
### implExtend<char> forSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#285)§
#### fnextend<X>(&mut self, iter: X)where
    X:IntoIterator<Item =char>,

Extends a collection with the contents of an iterator.
[Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#420)§
#### fnextend_one(&mut self, item: A)

🔬
This is a nightly-only experimental API. (
`extend_one`)
Extends a collection with exactly one element.
[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)§
#### fnextend_reserve(&mut self, additional:usize)

🔬
This is a nightly-only experimental API. (
`extend_one`)
Reserves capacity in a collection for the given number of additional elements.
[Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#227)§
### implFrom<&String> forSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#228)§
#### fnfrom(s: &String) ->SharedString

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#110)§
### implFrom<&str> forSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#111)§
#### fnfrom(value: &str) ->SharedString

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/input.rs.html#230)§
### implFrom<Key> forSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/input.rs.html#230)§
#### fnfrom(k:Key) ->SharedString

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#250)§
### implFrom<SharedString> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#250)§
#### fnfrom(v:SharedString) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#221)§
### implFrom<String> forSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#222)§
#### fnfrom(s:String) ->SharedString

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#233)§
### implFrom<char> forSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#234)§
#### fnfrom(c:char) ->SharedString

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#296)§
### implFromIterator<char> forSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#297)§
#### fnfrom_iter<T>(iter: T) ->SharedStringwhere
    T:IntoIterator<Item =char>,

Creates a value from an iterator.
[Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#265)§
### implHashforSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#266)§
#### fnhash<H>(&self, state:&mut H)where
    H:Hasher,

Feeds this value into the given
[Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html).
[Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)1.3.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)§
#### fnhash_slice<H>(data: &[Self], state:&mut H)where
    H:Hasher,
    Self:Sized,

Feeds a slice of this type into the given
[Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html).
[Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#215)§
### implOrdforSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#216)§
#### fncmp(&self, other: &SharedString) ->Ordering

This method returns an
[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html)between
`self`and
`other`.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)1.21.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1025-1027)§
#### fnmax(self, other: Self) -> Selfwhere
    Self:Sized,

Compares and returns the maximum of two values.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)1.21.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1064-1066)§
#### fnmin(self, other: Self) -> Selfwhere
    Self:Sized,

Compares and returns the minimum of two values.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)1.50.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1090-1092)§
#### fnclamp(self, min: Self, max: Self) -> Selfwhere
    Self:Sized,

Restrict a value to a certain interval.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#197-199)§
### impl<T>PartialEq<T> forSharedStringwhere
    T:AsRef<str> + ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#201)§
#### fneq(&self, other:&T) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#207-209)§
### impl<T>PartialOrd<T> forSharedStringwhere
    T:AsRef<str> + ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#211)§
#### fnpartial_cmp(&self, other:&T) ->Option<Ordering>

This method returns an ordering between
`self`and
`other`values if one exists.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1402)§
#### fnlt(&self, other:&Rhs) ->bool

Tests less than (for
`self`and
`other`) and is used by the
`<`operator.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1420)§
#### fnle(&self, other:&Rhs) ->bool

Tests less than or equal to (for
`self`and
`other`) and is used by the
`<=`operator.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1438)§
#### fngt(&self, other:&Rhs) ->bool

Tests greater than (for
`self`and
`other`) and is used by the
`>`operator.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1456)§
#### fnge(&self, other:&Rhs) ->bool

Tests greater than or equal to (for
`self`and
`other`) and is used by the
`>=`operator.
[Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#250)§
### implTryFrom<Value> forSharedString

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#250)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#250)§
#### fntry_from(v:Value) ->Result<SharedString, Self::Error>

Performs the conversion.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#271)§
### implWriteforSharedString

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#272)§
#### fnwrite_str(&mut self, s: &str) ->Result<(),Error>

Writes a string slice into this writer, returning whether the write succeeded.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#tymethod.write_str)1.1.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/fmt/mod.rs.html#183)§
#### fnwrite_char(&mut self, c:char) ->Result<(),Error>

Writes a
[char](https://doc.rust-lang.org/nightly/std/primitive.char.html)into this writer, returning whether the write succeeded.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_char)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/fmt/mod.rs.html#212)§
#### fnwrite_fmt(&mut self, args:Arguments<'_>) ->Result<(),Error>

Glue for usage of the
[write!](https://doc.rust-lang.org/nightly/core/macro.write.html)macro with implementors of this trait.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html#method.write_fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#205)§
### implEqforSharedString

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#104-107)§
### impl<Q, K>Comparable<K> for Qwhere
    Q:Ord+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#110)§
#### fncompare(&self, key:&K) ->Ordering

Compare self to
`key`and return their ordering.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)§
#### fnequivalent(&self, key:&K) ->bool

Compare self to
`key`and return
`true`if they are equal.
[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)§
#### fnequivalent(&self, key:&K) ->bool

Checks if this value is equivalent to the given key.
[Read more](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380)§
### impl<P, T>Receiverfor Pwhere
    P:Deref<Target = T> + ?Sized,
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382)§
#### typeTarget= T

🔬
This is a nightly-only experimental API. (
`arbitrary_self_types`)
The target type on which the method may be called.
[Source](https://docs.rs/hex/0.4.3/x86_64-unknown-linux-gnu/src/hex/lib.rs.html#137)§
### impl<T>ToHexfor Twhere
    T:AsRef<[u8]>,

[Source](https://docs.rs/hex/0.4.3/x86_64-unknown-linux-gnu/src/hex/lib.rs.html#138)§
#### fnencode_hex<U>(&self) -> Uwhere
    U:FromIterator<char>,

Encode the hex strict representing
`self`into the result. Lower case letters are used (e.g.
`f9b4ca`)
[Source](https://docs.rs/hex/0.4.3/x86_64-unknown-linux-gnu/src/hex/lib.rs.html#142)§
#### fnencode_hex_upper<U>(&self) -> Uwhere
    U:FromIterator<char>,

Encode the hex strict representing
`self`into the result. Upper case letters are used (e.g.
`F9B4CA`)
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#322-324)§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#326)§
#### fnto_shared_string(&self) ->SharedString

Converts the given value to a
[SharedString](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.SharedString.html).
[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#678-680)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#682)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2891)§
### impl<T>ToStringfor Twhere
    T:Display+ ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893)§
#### fnto_string(&self) ->String

Converts the given value to a
`String`.
[Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructSharedVectorCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#80)
```
pub struct SharedVector<T> { /* private fields */ }
```

Expand description
SharedVector holds a reference-counted read-only copy of[T].

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#127)§
### impl<T>SharedVector<T>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#129)
#### pub fnwith_capacity(capacity:usize) ->SharedVector<T>

Create a new empty array with a pre-allocated capacity in number of items

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#138)
#### pub fnlen(&self) ->usize

Number of elements in the array

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#143)
#### pub fnis_empty(&self) ->bool

Return true if the SharedVector is empty

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#148)
#### pub fnas_slice(&self) -> &[T]

Return a slice to the array

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#163)§
### impl<T>SharedVector<T>where
    T:Clone,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#165)
#### pub fnfrom_slice(slice: &[T]) ->SharedVector<T>

Create a SharedVector from a slice

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#194)
#### pub fnmake_mut_slice(&mut self) -> &mut[T]

Return a mutable slice to the array. If the array was shared, this will make a copy of the array.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#200)
#### pub fnpush(&mut self, value: T)

Add an element to the array. If the array was shared, this will make a copy of the array.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#213)
#### pub fnpop(&mut self) ->Option<T>

Removes last element from the array and returns it.
If the array was shared, this will make a copy of the array.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#237)
#### pub fnresize(&mut self, new_len:usize, value: T)

Resize the array to the given size.
If the array was smaller new elements will be initialized with the value.
If the array was bigger, extra elements will be discarded

```
use i_slint_core::SharedVector;
let mut shared_vector = SharedVector::<u32>::from_slice(&[1, 2, 3]);
shared_vector.resize(5, 8);
assert_eq!(shared_vector.as_slice(), &[1, 2, 3, 8, 8]);
shared_vector.resize(2, 0);
assert_eq!(shared_vector.as_slice(), &[1, 2]);
```

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#279)
#### pub fnclear(&mut self)

Clears the vector and removes all elements.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#290)
#### pub fnreserve(&mut self, additional:usize)

Reserves capacity for at leastadditionalbytes more than the current vector’s length.

## Methods fromDeref<Target =[T]>§

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#116)
#### pub fnlen(&self) ->usize

Returns the number of elements in the slice.

##### §Examples

```
let a = [1, 2, 3];
assert_eq!(a.len(), 3);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#136)
#### pub fnis_empty(&self) ->bool

Returnstrueif the slice has a length of 0.

##### §Examples

```
let a = [1, 2, 3];
assert!(!a.is_empty());

let b: &[i32] = &[];
assert!(b.is_empty());
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#155)
#### pub fnfirst(&self) ->Option<&T>

Returns the first element of the slice, orNoneif it is empty.

##### §Examples

```
let v = [10, 40, 30];
assert_eq!(Some(&10), v.first());

let w: &[i32] = &[];
assert_eq!(None, w.first());
```

1.5.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#198)
#### pub fnsplit_first(&self) ->Option<(&T, &[T])>

Returns the first and all the rest of the elements of the slice, orNoneif it is empty.

##### §Examples

```
let x = &[0, 1, 2];

if let Some((first, elements)) = x.split_first() {
    assert_eq!(first, &0);
    assert_eq!(elements, &[1, 2]);
}
```

1.5.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#240)
#### pub fnsplit_last(&self) ->Option<(&T, &[T])>

Returns the last and all the rest of the elements of the slice, orNoneif it is empty.

##### §Examples

```
let x = &[0, 1, 2];

if let Some((last, elements)) = x.split_last() {
    assert_eq!(last, &2);
    assert_eq!(elements, &[0, 1]);
}
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#281)
#### pub fnlast(&self) ->Option<&T>

Returns the last element of the slice, orNoneif it is empty.

##### §Examples

```
let v = [10, 40, 30];
assert_eq!(Some(&30), v.last());

let w: &[i32] = &[];
assert_eq!(None, w.last());
```

1.77.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#327)
#### pub fnfirst_chunk<const N:usize>(&self) ->Option<&[T; N]>

Returns an array reference to the firstNitems in the slice.

If the slice is not at leastNin length, this will returnNone.

##### §Examples

```
let u = [10, 40, 30];
assert_eq!(Some(&[10, 40]), u.first_chunk::<2>());

let v: &[i32] = &[10];
assert_eq!(None, v.first_chunk::<2>());

let w: &[i32] = &[];
assert_eq!(Some(&[]), w.first_chunk::<0>());
```

1.77.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#387)
#### pub fnsplit_first_chunk<const N:usize>(&self) ->Option<(&[T; N], &[T])>

Returns an array reference to the firstNitems in the slice and the remaining slice.

If the slice is not at leastNin length, this will returnNone.

##### §Examples

```
let x = &[0, 1, 2];

if let Some((first, elements)) = x.split_first_chunk::<2>() {
    assert_eq!(first, &[0, 1]);
    assert_eq!(elements, &[2]);
}

assert_eq!(None, x.split_first_chunk::<4>());
```

1.77.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#447)
#### pub fnsplit_last_chunk<const N:usize>(&self) ->Option<(&[T], &[T; N])>

Returns an array reference to the lastNitems in the slice and the remaining slice.

If the slice is not at leastNin length, this will returnNone.

##### §Examples

```
let x = &[0, 1, 2];

if let Some((elements, last)) = x.split_last_chunk::<2>() {
    assert_eq!(elements, &[0]);
    assert_eq!(last, &[1, 2]);
}

assert_eq!(None, x.split_last_chunk::<4>());
```

1.77.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#509)
#### pub fnlast_chunk<const N:usize>(&self) ->Option<&[T; N]>

Returns an array reference to the lastNitems in the slice.

If the slice is not at leastNin length, this will returnNone.

##### §Examples

```
let u = [10, 40, 30];
assert_eq!(Some(&[40, 30]), u.last_chunk::<2>());

let v: &[i32] = &[10];
assert_eq!(None, v.last_chunk::<2>());

let w: &[i32] = &[];
assert_eq!(Some(&[]), w.last_chunk::<0>());
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#572-574)
#### pub fnget<I>(&self, index: I) ->Option<&<I asSliceIndex<[T]>>::Output>where
    I:SliceIndex<[T]>,

Returns a reference to an element or subslice depending on the type of
index.

- If given a position, returns a reference to the element at that
position orNoneif out of bounds.
- If given a range, returns the subslice corresponding to that range,
orNoneif out of bounds.

##### §Examples

```
let v = [10, 40, 30];
assert_eq!(Some(&40), v.get(1));
assert_eq!(Some(&[10, 40][..]), v.get(0..2));
assert_eq!(None, v.get(3));
assert_eq!(None, v.get(0..4));
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#639-641)
#### pub unsafe fnget_unchecked<I>(
    &self,
    index: I,
) -> &<I asSliceIndex<[T]>>::Outputwhere
    I:SliceIndex<[T]>,

Returns a reference to an element or subslice, without doing bounds
checking.

For a safe alternative seeget.

##### §Safety

Calling this method with an out-of-bounds index isundefined behavioreven if the resulting reference is not used.

You can think of this like.get(index).unwrap_unchecked().  It’s UB
to call.get_unchecked(len), even if you immediately convert to a
pointer.  And it’s UB to call.get_unchecked(..len + 1),.get_unchecked(..=len), or similar.

##### §Examples

```
let x = &[1, 2, 4];

unsafe {
    assert_eq!(x.get_unchecked(1), &2);
}
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#726)
#### pub fnas_ptr(&self) ->*const T

Returns a raw pointer to the slice’s buffer.

The caller must ensure that the slice outlives the pointer this
function returns, or else it will end up dangling.

The caller must also ensure that the memory the pointer (non-transitively) points to
is never written to (except inside anUnsafeCell) using this pointer or any pointer
derived from it. If you need to mutate the contents of the slice, useas_mut_ptr.

Modifying the container referenced by this slice may cause its buffer
to be reallocated, which would also make any pointers to it invalid.

##### §Examples

```
let x = &[1, 2, 4];
let x_ptr = x.as_ptr();

unsafe {
    for i in 0..x.len() {
        assert_eq!(x.get_unchecked(i), &*x_ptr.add(i));
    }
}
```

1.48.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#793)
#### pub fnas_ptr_range(&self) ->Range<*const T>

Returns the two raw pointers spanning the slice.

The returned range is half-open, which means that the end pointer
pointsone pastthe last element of the slice. This way, an empty
slice is represented by two equal pointers, and the difference between
the two pointers represents the size of the slice.

Seeas_ptrfor warnings on using these pointers. The end pointer
requires extra caution, as it does not point to a valid element in the
slice.

This function is useful for interacting with foreign interfaces which
use two pointers to refer to a range of elements in memory, as is
common in C++.

It can also be useful to check if a pointer to an element refers to an
element of this slice:

```
let a = [1, 2, 3];
let x = &a[1] as *const _;
let y = &5 as *const _;

assert!(a.as_ptr_range().contains(&x));
assert!(!a.as_ptr_range().contains(&y));
```

1.93.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#850)
#### pub fnas_array<const N:usize>(&self) ->Option<&[T; N]>

Gets a reference to the underlying array.

IfNis not exactly equal to the length ofself, then this method returnsNone.

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1040)
#### pub fniter(&self) ->Iter<'_, T>

Returns an iterator over the slice.

The iterator yields all items from start to end.

##### §Examples

```
let x = &[1, 2, 4];
let mut iterator = x.iter();

assert_eq!(iterator.next(), Some(&1));
assert_eq!(iterator.next(), Some(&2));
assert_eq!(iterator.next(), Some(&4));
assert_eq!(iterator.next(), None);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1115)
#### pub fnwindows(&self, size:usize) ->Windows<'_, T>

Returns an iterator over all contiguous windows of lengthsize. The windows overlap. If the slice is shorter thansize, the iterator returns no values.

##### §Panics

Panics ifsizeis zero.

##### §Examples

```
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.windows(3);
assert_eq!(iter.next().unwrap(), &['l', 'o', 'r']);
assert_eq!(iter.next().unwrap(), &['o', 'r', 'e']);
assert_eq!(iter.next().unwrap(), &['r', 'e', 'm']);
assert!(iter.next().is_none());
```

If the slice is shorter thansize:

```
let slice = ['f', 'o', 'o'];
let mut iter = slice.windows(4);
assert!(iter.next().is_none());
```

Because theIteratortrait cannot represent the required lifetimes,
there is nowindows_mutanalog towindows;[0,1,2].windows_mut(2).collect()would violatethe rules of references(though aLendingIteratoranalog is possible). You can sometimes useCell::as_slice_of_cellsin
conjunction withwindowsinstead:

```
use std::cell::Cell;

let mut array = ['R', 'u', 's', 't', ' ', '2', '0', '1', '5'];
let slice = &mut array[..];
let slice_of_cells: &[Cell<char>] = Cell::from_mut(slice).as_slice_of_cells();
for w in slice_of_cells.windows(3) {
    Cell::swap(&w[0], &w[2]);
}
assert_eq!(array, ['s', 't', ' ', '2', '0', '1', '5', 'u', 'R']);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1155)
#### pub fnchunks(&self, chunk_size:usize) ->Chunks<'_, T>

Returns an iterator overchunk_sizeelements of the slice at a time, starting at the
beginning of the slice.

The chunks are slices and do not overlap. Ifchunk_sizedoes not divide the length of the
slice, then the last chunk will not have lengthchunk_size.

Seechunks_exactfor a variant of this iterator that returns chunks of always exactlychunk_sizeelements, andrchunksfor the same iterator but starting at the end of the
slice.

If yourchunk_sizeis a constant, consider usingas_chunksinstead, which will
give references to arrays of exactly that length, rather than slices.

##### §Panics

Panics ifchunk_sizeis zero.

##### §Examples

```
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.chunks(2);
assert_eq!(iter.next().unwrap(), &['l', 'o']);
assert_eq!(iter.next().unwrap(), &['r', 'e']);
assert_eq!(iter.next().unwrap(), &['m']);
assert!(iter.next().is_none());
```

1.31.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1242)
#### pub fnchunks_exact(&self, chunk_size:usize) ->ChunksExact<'_, T>

Returns an iterator overchunk_sizeelements of the slice at a time, starting at the
beginning of the slice.

The chunks are slices and do not overlap. Ifchunk_sizedoes not divide the length of the
slice, then the last up tochunk_size-1elements will be omitted and can be retrieved
from theremainderfunction of the iterator.

Due to each chunk having exactlychunk_sizeelements, the compiler can often optimize the
resulting code better than in the case ofchunks.

Seechunksfor a variant of this iterator that also returns the remainder as a smaller
chunk, andrchunks_exactfor the same iterator but starting at the end of the slice.

If yourchunk_sizeis a constant, consider usingas_chunksinstead, which will
give references to arrays of exactly that length, rather than slices.

##### §Panics

Panics ifchunk_sizeis zero.

##### §Examples

```
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.chunks_exact(2);
assert_eq!(iter.next().unwrap(), &['l', 'o']);
assert_eq!(iter.next().unwrap(), &['r', 'e']);
assert!(iter.next().is_none());
assert_eq!(iter.remainder(), &['m']);
```

1.88.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1338)
#### pub unsafe fnas_chunks_unchecked<const N:usize>(&self) -> &[[T; N]]

Splits the slice into a slice ofN-element arrays,
assuming that there’s no remainder.

This is the inverse operation toas_flattened.

As this isunsafe, consider whether you could useas_chunksoras_rchunksinstead, perhaps via something likeif let (chunks, []) = slice.as_chunks()orlet (chunks, []) = slice.as_chunks() else { unreachable!() };.

##### §Safety

This may only be called when

- The slice splits exactly intoN-element chunks (akaself.len() % N == 0).
- N != 0.

##### §Examples

```
let slice: &[char] = &['l', 'o', 'r', 'e', 'm', '!'];
let chunks: &[[char; 1]] =
    // SAFETY: 1-element chunks never have remainder
    unsafe { slice.as_chunks_unchecked() };
assert_eq!(chunks, &[['l'], ['o'], ['r'], ['e'], ['m'], ['!']]);
let chunks: &[[char; 3]] =
    // SAFETY: The slice length (6) is a multiple of 3
    unsafe { slice.as_chunks_unchecked() };
assert_eq!(chunks, &[['l', 'o', 'r'], ['e', 'm', '!']]);

// These would be unsound:
// let chunks: &[[_; 5]] = slice.as_chunks_unchecked() // The slice length is not a multiple of 5
// let chunks: &[[_; 0]] = slice.as_chunks_unchecked() // Zero-length chunks are never allowed
```

1.88.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1396)
#### pub fnas_chunks<const N:usize>(&self) -> (&[[T; N]], &[T])

Splits the slice into a slice ofN-element arrays,
starting at the beginning of the slice,
and a remainder slice with length strictly less thanN.

The remainder is meaningful in the division sense.  Givenlet (chunks, remainder) = slice.as_chunks(), then:

- chunks.len()equalsslice.len() / N,
- remainder.len()equalsslice.len() % N, and
- slice.len()equalschunks.len() * N + remainder.len().

You can flatten the chunks back into a slice-of-Twithas_flattened.

##### §Panics

Panics ifNis zero.

Note that this check is against a const generic parameter, not a runtime
value, and thus a particular monomorphization will either always panic
or it will never panic.

##### §Examples

```
let slice = ['l', 'o', 'r', 'e', 'm'];
let (chunks, remainder) = slice.as_chunks();
assert_eq!(chunks, &[['l', 'o'], ['r', 'e']]);
assert_eq!(remainder, &['m']);
```

If you expect the slice to be an exact multiple, you can combinelet-elsewith an empty slice pattern:

```
let slice = ['R', 'u', 's', 't'];
let (chunks, []) = slice.as_chunks::<2>() else {
    panic!("slice didn't have even length")
};
assert_eq!(chunks, &[['R', 'u'], ['s', 't']]);
```

1.88.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1443)
#### pub fnas_rchunks<const N:usize>(&self) -> (&[T], &[[T; N]])

Splits the slice into a slice ofN-element arrays,
starting at the end of the slice,
and a remainder slice with length strictly less thanN.

The remainder is meaningful in the division sense.  Givenlet (remainder, chunks) = slice.as_rchunks(), then:

- remainder.len()equalsslice.len() % N,
- chunks.len()equalsslice.len() / N, and
- slice.len()equalschunks.len() * N + remainder.len().

You can flatten the chunks back into a slice-of-Twithas_flattened.

##### §Panics

Panics ifNis zero.

Note that this check is against a const generic parameter, not a runtime
value, and thus a particular monomorphization will either always panic
or it will never panic.

##### §Examples

```
let slice = ['l', 'o', 'r', 'e', 'm'];
let (remainder, chunks) = slice.as_rchunks();
assert_eq!(remainder, &['l']);
assert_eq!(chunks, &[['o', 'r'], ['e', 'm']]);
```

1.94.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1646)
#### pub fnarray_windows<const N:usize>(&self) ->ArrayWindows<'_, T, N>

Returns an iterator over overlapping windows ofNelements of a slice,
starting at the beginning of the slice.

This is the const generic equivalent ofwindows.

IfNis greater than the size of the slice, it will return no windows.

##### §Panics

Panics ifNis zero.

Note that this check is against a const generic parameter, not a runtime
value, and thus a particular monomorphization will either always panic
or it will never panic.

##### §Examples

```
let slice = [0, 1, 2, 3];
let mut iter = slice.array_windows();
assert_eq!(iter.next().unwrap(), &[0, 1]);
assert_eq!(iter.next().unwrap(), &[1, 2]);
assert_eq!(iter.next().unwrap(), &[2, 3]);
assert!(iter.next().is_none());
```

1.31.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1686)
#### pub fnrchunks(&self, chunk_size:usize) ->RChunks<'_, T>

Returns an iterator overchunk_sizeelements of the slice at a time, starting at the end
of the slice.

The chunks are slices and do not overlap. Ifchunk_sizedoes not divide the length of the
slice, then the last chunk will not have lengthchunk_size.

Seerchunks_exactfor a variant of this iterator that returns chunks of always exactlychunk_sizeelements, andchunksfor the same iterator but starting at the beginning
of the slice.

If yourchunk_sizeis a constant, consider usingas_rchunksinstead, which will
give references to arrays of exactly that length, rather than slices.

##### §Panics

Panics ifchunk_sizeis zero.

##### §Examples

```
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.rchunks(2);
assert_eq!(iter.next().unwrap(), &['e', 'm']);
assert_eq!(iter.next().unwrap(), &['o', 'r']);
assert_eq!(iter.next().unwrap(), &['l']);
assert!(iter.next().is_none());
```

1.31.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1775)
#### pub fnrchunks_exact(&self, chunk_size:usize) ->RChunksExact<'_, T>

Returns an iterator overchunk_sizeelements of the slice at a time, starting at the
end of the slice.

The chunks are slices and do not overlap. Ifchunk_sizedoes not divide the length of the
slice, then the last up tochunk_size-1elements will be omitted and can be retrieved
from theremainderfunction of the iterator.

Due to each chunk having exactlychunk_sizeelements, the compiler can often optimize the
resulting code better than in the case ofrchunks.

Seerchunksfor a variant of this iterator that also returns the remainder as a smaller
chunk, andchunks_exactfor the same iterator but starting at the beginning of the
slice.

If yourchunk_sizeis a constant, consider usingas_rchunksinstead, which will
give references to arrays of exactly that length, rather than slices.

##### §Panics

Panics ifchunk_sizeis zero.

##### §Examples

```
let slice = ['l', 'o', 'r', 'e', 'm'];
let mut iter = slice.rchunks_exact(2);
assert_eq!(iter.next().unwrap(), &['e', 'm']);
assert_eq!(iter.next().unwrap(), &['o', 'r']);
assert!(iter.next().is_none());
assert_eq!(iter.remainder(), &['l']);
```

1.77.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1864-1866)
#### pub fnchunk_by<F>(&self, pred: F) ->ChunkBy<'_, T, F>where
    F:FnMut(&T,&T) ->bool,

Returns an iterator over the slice producing non-overlapping runs
of elements using the predicate to separate them.

The predicate is called for every pair of consecutive elements,
meaning that it is called onslice[0]andslice[1],
followed byslice[1]andslice[2], and so on.

##### §Examples

```
let slice = &[1, 1, 1, 3, 3, 2, 2, 2];

let mut iter = slice.chunk_by(|a, b| a == b);

assert_eq!(iter.next(), Some(&[1, 1, 1][..]));
assert_eq!(iter.next(), Some(&[3, 3][..]));
assert_eq!(iter.next(), Some(&[2, 2, 2][..]));
assert_eq!(iter.next(), None);
```

This method can be used to extract the sorted subslices:

```
let slice = &[1, 1, 2, 3, 2, 3, 2, 3, 4];

let mut iter = slice.chunk_by(|a, b| a <= b);

assert_eq!(iter.next(), Some(&[1, 1, 2, 3][..]));
assert_eq!(iter.next(), Some(&[2, 3][..]));
assert_eq!(iter.next(), Some(&[2, 3, 4][..]));
assert_eq!(iter.next(), None);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1952)
#### pub fnsplit_at(&self, mid:usize) -> (&[T], &[T])

Divides one slice into two at an index.

The first will contain all indices from[0, mid)(excluding
the indexmiditself) and the second will contain all
indices from[mid, len)(excluding the indexlenitself).

##### §Panics

Panics ifmid > len.  For a non-panicking alternative seesplit_at_checked.

##### §Examples

```
let v = ['a', 'b', 'c'];

{
   let (left, right) = v.split_at(0);
   assert_eq!(left, []);
   assert_eq!(right, ['a', 'b', 'c']);
}

{
    let (left, right) = v.split_at(2);
    assert_eq!(left, ['a', 'b']);
    assert_eq!(right, ['c']);
}

{
    let (left, right) = v.split_at(3);
    assert_eq!(left, ['a', 'b', 'c']);
    assert_eq!(right, []);
}
```

1.79.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2038)
#### pub unsafe fnsplit_at_unchecked(&self, mid:usize) -> (&[T], &[T])

Divides one slice into two at an index, without doing bounds checking.

The first will contain all indices from[0, mid)(excluding
the indexmiditself) and the second will contain all
indices from[mid, len)(excluding the indexlenitself).

For a safe alternative seesplit_at.

##### §Safety

Calling this method with an out-of-bounds index isundefined behavioreven if the resulting reference is not used. The caller has to ensure that0 <= mid <= self.len().

##### §Examples

```
let v = ['a', 'b', 'c'];

unsafe {
   let (left, right) = v.split_at_unchecked(0);
   assert_eq!(left, []);
   assert_eq!(right, ['a', 'b', 'c']);
}

unsafe {
    let (left, right) = v.split_at_unchecked(2);
    assert_eq!(left, ['a', 'b']);
    assert_eq!(right, ['c']);
}

unsafe {
    let (left, right) = v.split_at_unchecked(3);
    assert_eq!(left, ['a', 'b', 'c']);
    assert_eq!(right, []);
}
```

1.80.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2153)
#### pub fnsplit_at_checked(&self, mid:usize) ->Option<(&[T], &[T])>

Divides one slice into two at an index, returningNoneif the slice is
too short.

Ifmid ≤ lenreturns a pair of slices where the first will contain all
indices from[0, mid)(excluding the indexmiditself) and the
second will contain all indices from[mid, len)(excluding the indexlenitself).

Otherwise, ifmid > len, returnsNone.

##### §Examples

```
let v = [1, -2, 3, -4, 5, -6];

{
   let (left, right) = v.split_at_checked(0).unwrap();
   assert_eq!(left, []);
   assert_eq!(right, [1, -2, 3, -4, 5, -6]);
}

{
    let (left, right) = v.split_at_checked(2).unwrap();
    assert_eq!(left, [1, -2]);
    assert_eq!(right, [3, -4, 5, -6]);
}

{
    let (left, right) = v.split_at_checked(6).unwrap();
    assert_eq!(left, [1, -2, 3, -4, 5, -6]);
    assert_eq!(right, []);
}

assert_eq!(None, v.split_at_checked(7));
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2244-2246)
#### pub fnsplit<F>(&self, pred: F) ->Split<'_, T, F>where
    F:FnMut(&T) ->bool,

Returns an iterator over subslices separated by elements that matchpred. The matched element is not contained in the subslices.

##### §Examples

```
let slice = [10, 40, 33, 20];
let mut iter = slice.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());
```

If the first element is matched, an empty slice will be the first item
returned by the iterator. Similarly, if the last element in the slice
is matched, an empty slice will be the last item returned by the
iterator:

```
let slice = [10, 40, 33];
let mut iter = slice.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40]);
assert_eq!(iter.next().unwrap(), &[]);
assert!(iter.next().is_none());
```

If two matched elements are directly adjacent, an empty slice will be
present between them:

```
let slice = [10, 6, 33, 20];
let mut iter = slice.split(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10]);
assert_eq!(iter.next().unwrap(), &[]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());
```

1.51.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2302-2304)
#### pub fnsplit_inclusive<F>(&self, pred: F) ->SplitInclusive<'_, T, F>where
    F:FnMut(&T) ->bool,

Returns an iterator over subslices separated by elements that matchpred. The matched element is contained in the end of the previous
subslice as a terminator.

##### §Examples

```
let slice = [10, 40, 33, 20];
let mut iter = slice.split_inclusive(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
assert_eq!(iter.next().unwrap(), &[20]);
assert!(iter.next().is_none());
```

If the last element of the slice is matched,
that element will be considered the terminator of the preceding slice.
That slice will be the last item returned by the iterator.

```
let slice = [3, 10, 40, 33];
let mut iter = slice.split_inclusive(|num| num % 3 == 0);

assert_eq!(iter.next().unwrap(), &[3]);
assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
assert!(iter.next().is_none());
```

1.27.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2362-2364)
#### pub fnrsplit<F>(&self, pred: F) ->RSplit<'_, T, F>where
    F:FnMut(&T) ->bool,

Returns an iterator over subslices separated by elements that matchpred, starting at the end of the slice and working backwards.
The matched element is not contained in the subslices.

##### §Examples

```
let slice = [11, 22, 33, 0, 44, 55];
let mut iter = slice.rsplit(|num| *num == 0);

assert_eq!(iter.next().unwrap(), &[44, 55]);
assert_eq!(iter.next().unwrap(), &[11, 22, 33]);
assert_eq!(iter.next(), None);
```

As withsplit(), if the first or last element is matched, an empty
slice will be the first (or last) item returned by the iterator.

```
let v = &[0, 1, 1, 2, 3, 5, 8];
let mut it = v.rsplit(|n| *n % 2 == 0);
assert_eq!(it.next().unwrap(), &[]);
assert_eq!(it.next().unwrap(), &[3, 5]);
assert_eq!(it.next().unwrap(), &[1, 1]);
assert_eq!(it.next().unwrap(), &[]);
assert_eq!(it.next(), None);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2416-2418)
#### pub fnsplitn<F>(&self, n:usize, pred: F) ->SplitN<'_, T, F>where
    F:FnMut(&T) ->bool,

Returns an iterator over subslices separated by elements that matchpred, limited to returning at mostnitems. The matched element is
not contained in the subslices.

The last element returned, if any, will contain the remainder of the
slice.

##### §Examples

Print the slice split once by numbers divisible by 3 (i.e.,[10, 40],[20, 60, 50]):

```
let v = [10, 40, 30, 20, 60, 50];

for group in v.splitn(2, |num| *num % 3 == 0) {
    println!("{group:?}");
}
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2471-2473)
#### pub fnrsplitn<F>(&self, n:usize, pred: F) ->RSplitN<'_, T, F>where
    F:FnMut(&T) ->bool,

Returns an iterator over subslices separated by elements that matchpredlimited to returning at mostnitems. This starts at the end of
the slice and works backwards. The matched element is not contained in
the subslices.

The last element returned, if any, will contain the remainder of the
slice.

##### §Examples

Print the slice split once, starting from the end, by numbers divisible
by 3 (i.e.,[50],[10, 40, 30, 20]):

```
let v = [10, 40, 30, 20, 60, 50];

for group in v.rsplitn(2, |num| *num % 3 == 0) {
    println!("{group:?}");
}
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2525-2527)
#### pub fnsplit_once<F>(&self, pred: F) ->Option<(&[T], &[T])>where
    F:FnMut(&T) ->bool,

🔬
This is a nightly-only experimental API. (
`slice_split_once`)
Splits the slice on the first element that matches the specified
predicate.

If any matching elements are present in the slice, returns the prefix
before the match and suffix after. The matching element itself is not
included. If no elements match, returnsNone.

##### §Examples

```
#![feature(slice_split_once)]
let s = [1, 2, 3, 2, 4];
assert_eq!(s.split_once(|&x| x == 2), Some((
    &[1][..],
    &[3, 2, 4][..]
)));
assert_eq!(s.split_once(|&x| x == 0), None);
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2553-2555)
#### pub fnrsplit_once<F>(&self, pred: F) ->Option<(&[T], &[T])>where
    F:FnMut(&T) ->bool,

🔬
This is a nightly-only experimental API. (
`slice_split_once`)
Splits the slice on the last element that matches the specified
predicate.

If any matching elements are present in the slice, returns the prefix
before the match and suffix after. The matching element itself is not
included. If no elements match, returnsNone.

##### §Examples

```
#![feature(slice_split_once)]
let s = [1, 2, 3, 2, 4];
assert_eq!(s.rsplit_once(|&x| x == 2), Some((
    &[1, 2, 3][..],
    &[4][..]
)));
assert_eq!(s.rsplit_once(|&x| x == 0), None);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2589-2591)
#### pub fncontains(&self, x:&T) ->boolwhere
    T:PartialEq,

Returnstrueif the slice contains an element with the given value.

This operation isO(n).

Note that if you have a sorted slice,binary_searchmay be faster.

##### §Examples

```
let v = [10, 40, 30];
assert!(v.contains(&30));
assert!(!v.contains(&50));
```

If you do not have a&T, but some other value that you can compare
with one (for example,StringimplementsPartialEq<str>), you can
useiter().any:

```
let v = [String::from("hello"), String::from("world")]; // slice of `String`
assert!(v.iter().any(|e| e == "hello")); // search with `&str`
assert!(!v.iter().any(|e| e == "hi"));
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2619-2621)
#### pub fnstarts_with(&self, needle: &[T]) ->boolwhere
    T:PartialEq,

Returnstrueifneedleis a prefix of the slice or equal to the slice.

##### §Examples

```
let v = [10, 40, 30];
assert!(v.starts_with(&[10]));
assert!(v.starts_with(&[10, 40]));
assert!(v.starts_with(&v));
assert!(!v.starts_with(&[50]));
assert!(!v.starts_with(&[10, 50]));
```

Always returnstrueifneedleis an empty slice:

```
let v = &[10, 40, 30];
assert!(v.starts_with(&[]));
let v: &[u8] = &[];
assert!(v.starts_with(&[]));
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2650-2652)
#### pub fnends_with(&self, needle: &[T]) ->boolwhere
    T:PartialEq,

Returnstrueifneedleis a suffix of the slice or equal to the slice.

##### §Examples

```
let v = [10, 40, 30];
assert!(v.ends_with(&[30]));
assert!(v.ends_with(&[40, 30]));
assert!(v.ends_with(&v));
assert!(!v.ends_with(&[50]));
assert!(!v.ends_with(&[50, 30]));
```

Always returnstrueifneedleis an empty slice:

```
let v = &[10, 40, 30];
assert!(v.ends_with(&[]));
let v: &[u8] = &[];
assert!(v.ends_with(&[]));
```

1.51.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2682-2684)
#### pub fnstrip_prefix<P>(&self, prefix:&P) ->Option<&[T]>where
    P:SlicePattern<Item = T> + ?Sized,
    T:PartialEq,

Returns a subslice with the prefix removed.

If the slice starts withprefix, returns the subslice after the prefix, wrapped inSome.
Ifprefixis empty, simply returns the original slice. Ifprefixis equal to the
original slice, returns an empty slice.

If the slice does not start withprefix, returnsNone.

##### §Examples

```
let v = &[10, 40, 30];
assert_eq!(v.strip_prefix(&[10]), Some(&[40, 30][..]));
assert_eq!(v.strip_prefix(&[10, 40]), Some(&[30][..]));
assert_eq!(v.strip_prefix(&[10, 40, 30]), Some(&[][..]));
assert_eq!(v.strip_prefix(&[50]), None);
assert_eq!(v.strip_prefix(&[10, 50]), None);

let prefix : &str = "he";
assert_eq!(b"hello".strip_prefix(prefix.as_bytes()),
           Some(b"llo".as_ref()));
```

1.51.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2718-2720)
#### pub fnstrip_suffix<P>(&self, suffix:&P) ->Option<&[T]>where
    P:SlicePattern<Item = T> + ?Sized,
    T:PartialEq,

Returns a subslice with the suffix removed.

If the slice ends withsuffix, returns the subslice before the suffix, wrapped inSome.
Ifsuffixis empty, simply returns the original slice. Ifsuffixis equal to the
original slice, returns an empty slice.

If the slice does not end withsuffix, returnsNone.

##### §Examples

```
let v = &[10, 40, 30];
assert_eq!(v.strip_suffix(&[30]), Some(&[10, 40][..]));
assert_eq!(v.strip_suffix(&[40, 30]), Some(&[10][..]));
assert_eq!(v.strip_suffix(&[10, 40, 30]), Some(&[][..]));
assert_eq!(v.strip_suffix(&[50]), None);
assert_eq!(v.strip_suffix(&[50, 30]), None);
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2757-2761)
#### pub fnstrip_circumfix<S, P>(&self, prefix:&P, suffix:&S) ->Option<&[T]>where
    T:PartialEq,
    S:SlicePattern<Item = T> + ?Sized,
    P:SlicePattern<Item = T> + ?Sized,

🔬
This is a nightly-only experimental API. (
`strip_circumfix`)
Returns a subslice with the prefix and suffix removed.

If the slice starts withprefixand ends withsuffix, returns the subslice after the
prefix and before the suffix, wrapped inSome.

If the slice does not start withprefixor does not end withsuffix, returnsNone.

##### §Examples

```
#![feature(strip_circumfix)]

let v = &[10, 50, 40, 30];
assert_eq!(v.strip_circumfix(&[10], &[30]), Some(&[50, 40][..]));
assert_eq!(v.strip_circumfix(&[10], &[40, 30]), Some(&[50][..]));
assert_eq!(v.strip_circumfix(&[10, 50], &[40, 30]), Some(&[][..]));
assert_eq!(v.strip_circumfix(&[50], &[30]), None);
assert_eq!(v.strip_circumfix(&[10], &[40]), None);
assert_eq!(v.strip_circumfix(&[], &[40, 30]), Some(&[10, 50][..]));
assert_eq!(v.strip_circumfix(&[10, 50], &[]), Some(&[40, 30][..]));
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2793-2795)
#### pub fntrim_prefix<P>(&self, prefix:&P) -> &[T]where
    P:SlicePattern<Item = T> + ?Sized,
    T:PartialEq,

🔬
This is a nightly-only experimental API. (
`trim_prefix_suffix`)
Returns a subslice with the optional prefix removed.

If the slice starts withprefix, returns the subslice after the prefix.  Ifprefixis empty or the slice does not start withprefix, simply returns the original slice.
Ifprefixis equal to the original slice, returns an empty slice.

##### §Examples

```
#![feature(trim_prefix_suffix)]

let v = &[10, 40, 30];

// Prefix present - removes it
assert_eq!(v.trim_prefix(&[10]), &[40, 30][..]);
assert_eq!(v.trim_prefix(&[10, 40]), &[30][..]);
assert_eq!(v.trim_prefix(&[10, 40, 30]), &[][..]);

// Prefix absent - returns original slice
assert_eq!(v.trim_prefix(&[50]), &[10, 40, 30][..]);
assert_eq!(v.trim_prefix(&[10, 50]), &[10, 40, 30][..]);

let prefix : &str = "he";
assert_eq!(b"hello".trim_prefix(prefix.as_bytes()), b"llo".as_ref());
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2833-2835)
#### pub fntrim_suffix<P>(&self, suffix:&P) -> &[T]where
    P:SlicePattern<Item = T> + ?Sized,
    T:PartialEq,

🔬
This is a nightly-only experimental API. (
`trim_prefix_suffix`)
Returns a subslice with the optional suffix removed.

If the slice ends withsuffix, returns the subslice before the suffix.  Ifsuffixis empty or the slice does not end withsuffix, simply returns the original slice.
Ifsuffixis equal to the original slice, returns an empty slice.

##### §Examples

```
#![feature(trim_prefix_suffix)]

let v = &[10, 40, 30];

// Suffix present - removes it
assert_eq!(v.trim_suffix(&[30]), &[10, 40][..]);
assert_eq!(v.trim_suffix(&[40, 30]), &[10][..]);
assert_eq!(v.trim_suffix(&[10, 40, 30]), &[][..]);

// Suffix absent - returns original slice
assert_eq!(v.trim_suffix(&[50]), &[10, 40, 30][..]);
assert_eq!(v.trim_suffix(&[50, 30]), &[10, 40, 30][..]);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2919-2921)
#### pub fnbinary_search(&self, x:&T) ->Result<usize,usize>where
    T:Ord,

Binary searches this slice for a given element.
If the slice is not sorted, the returned result is unspecified and
meaningless.

If the value is found thenResult::Okis returned, containing the
index of the matching element. If there are multiple matches, then any
one of the matches could be returned. The index is chosen
deterministically, but is subject to change in future versions of Rust.
If the value is not found thenResult::Erris returned, containing
the index where a matching element could be inserted while maintaining
sorted order.

See alsobinary_search_by,binary_search_by_key, andpartition_point.

##### §Examples

Looks up a series of four elements. The first is found, with a
uniquely determined position; the second and third are not
found; the fourth could match any position in[1, 4].

```
let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

assert_eq!(s.binary_search(&13),  Ok(9));
assert_eq!(s.binary_search(&4),   Err(7));
assert_eq!(s.binary_search(&100), Err(13));
let r = s.binary_search(&1);
assert!(match r { Ok(1..=4) => true, _ => false, });
```

If you want to find that wholerangeof matching items, rather than
an arbitrary matching one, that can be done usingpartition_point:

```
let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

let low = s.partition_point(|x| x < &1);
assert_eq!(low, 1);
let high = s.partition_point(|x| x <= &1);
assert_eq!(high, 5);
let r = s.binary_search(&1);
assert!((low..high).contains(&r.unwrap()));

assert!(s[..low].iter().all(|&x| x < 1));
assert!(s[low..high].iter().all(|&x| x == 1));
assert!(s[high..].iter().all(|&x| x > 1));

// For something not found, the "range" of equal items is empty
assert_eq!(s.partition_point(|x| x < &11), 9);
assert_eq!(s.partition_point(|x| x <= &11), 9);
assert_eq!(s.binary_search(&11), Err(9));
```

If you want to insert an item to a sorted vector, while maintaining
sort order, consider usingpartition_point:

```
let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
let num = 42;
let idx = s.partition_point(|&x| x <= num);
// If `num` is unique, `s.partition_point(|&x| x < num)` (with `<`) is equivalent to
// `s.binary_search(&num).unwrap_or_else(|x| x)`, but using `<=` will allow `insert`
// to shift less elements.
s.insert(idx, num);
assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2970-2972)
#### pub fnbinary_search_by<'a, F>(&'a self, f: F) ->Result<usize,usize>where
    F:FnMut(&'a T) ->Ordering,

Binary searches this slice with a comparator function.

The comparator function should return an order code that indicates
whether its argument isLess,EqualorGreaterthe desired
target.
If the slice is not sorted or if the comparator function does not
implement an order consistent with the sort order of the underlying
slice, the returned result is unspecified and meaningless.

If the value is found thenResult::Okis returned, containing the
index of the matching element. If there are multiple matches, then any
one of the matches could be returned. The index is chosen
deterministically, but is subject to change in future versions of Rust.
If the value is not found thenResult::Erris returned, containing
the index where a matching element could be inserted while maintaining
sorted order.

See alsobinary_search,binary_search_by_key, andpartition_point.

##### §Examples

Looks up a series of four elements. The first is found, with a
uniquely determined position; the second and third are not
found; the fourth could match any position in[1, 4].

```
let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

let seek = 13;
assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
let seek = 4;
assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
let seek = 100;
assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
let seek = 1;
let r = s.binary_search_by(|probe| probe.cmp(&seek));
assert!(match r { Ok(1..=4) => true, _ => false, });
```

1.10.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#3071-3074)
#### pub fnbinary_search_by_key<'a, B, F>(
    &'a self,
    b:&B,
    f: F,
) ->Result<usize,usize>where
    F:FnMut(&'a T) -> B,
    B:Ord,

Binary searches this slice with a key extraction function.

Assumes that the slice is sorted by the key, for instance withsort_by_keyusing the same key extraction function.
If the slice is not sorted by the key, the returned result is
unspecified and meaningless.

If the value is found thenResult::Okis returned, containing the
index of the matching element. If there are multiple matches, then any
one of the matches could be returned. The index is chosen
deterministically, but is subject to change in future versions of Rust.
If the value is not found thenResult::Erris returned, containing
the index where a matching element could be inserted while maintaining
sorted order.

See alsobinary_search,binary_search_by, andpartition_point.

##### §Examples

Looks up a series of four elements in a slice of pairs sorted by
their second elements. The first is found, with a uniquely
determined position; the second and third are not found; the
fourth could match any position in[1, 4].

```
let s = [(0, 0), (2, 1), (4, 1), (5, 1), (3, 1),
         (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
         (1, 21), (2, 34), (4, 55)];

assert_eq!(s.binary_search_by_key(&13, |&(a, b)| b),  Ok(9));
assert_eq!(s.binary_search_by_key(&4, |&(a, b)| b),   Err(7));
assert_eq!(s.binary_search_by_key(&100, |&(a, b)| b), Err(13));
let r = s.binary_search_by_key(&1, |&(a, b)| b);
assert!(match r { Ok(1..=4) => true, _ => false, });
```

1.30.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4499)
#### pub unsafe fnalign_to<U>(&self) -> (&[T], &[U], &[T])

Transmutes the slice to a slice of another type, ensuring alignment of the types is
maintained.

This method splits the slice into three distinct slices: prefix, correctly aligned middle
slice of a new type, and the suffix slice. The middle part will be as big as possible under
the given alignment constraint and element size.

This method has no purpose when either input elementTor output elementUare
zero-sized and will return the original slice without splitting anything.

##### §Safety

This method is essentially atransmutewith respect to the elements in the returned
middle slice, so all the usual caveats pertaining totransmute::<T, U>also apply here.

##### §Examples

Basic usage:

```
unsafe {
    let bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
    let (prefix, shorts, suffix) = bytes.align_to::<u16>();
    // less_efficient_algorithm_for_bytes(prefix);
    // more_efficient_algorithm_for_aligned_shorts(shorts);
    // less_efficient_algorithm_for_bytes(suffix);
}
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4655-4658)
#### pub fnas_simd<const LANES:usize>(&self) -> (&[T], &[Simd<T, LANES>], &[T])whereSimd<T, LANES>:AsRef<[T; LANES]>,
    T:SimdElement,

🔬
This is a nightly-only experimental API. (
`portable_simd`)
Splits a slice into a prefix, a middle of aligned SIMD types, and a suffix.

This is a safe wrapper aroundslice::align_to, so inherits the same
guarantees as that method.

##### §Panics

This will panic if the size of the SIMD type is different fromLANEStimes that of the scalar.

At the time of writing, the trait restrictions onSimd<T, LANES>keeps
that from ever happening, as only power-of-two numbers of lanes are
supported.  It’s possible that, in the future, those restrictions might
be lifted in a way that would make it possible to see panics from this
method for something likeLANES == 3.

##### §Examples

```
#![feature(portable_simd)]
use core::simd::prelude::*;

let short = &[1, 2, 3];
let (prefix, middle, suffix) = short.as_simd::<4>();
assert_eq!(middle, []); // Not enough elements for anything in the middle

// They might be split in any possible way between prefix and suffix
let it = prefix.iter().chain(suffix).copied();
assert_eq!(it.collect::<Vec<_>>(), vec![1, 2, 3]);

fn basic_simd_sum(x: &[f32]) -> f32 {
    use std::ops::Add;
    let (prefix, middle, suffix) = x.as_simd();
    let sums = f32x4::from_array([
        prefix.iter().copied().sum(),
        0.0,
        0.0,
        suffix.iter().copied().sum(),
    ]);
    let sums = middle.iter().copied().fold(sums, f32x4::add);
    sums.reduce_sum()
}

let numbers: Vec<f32> = (1..101).map(|x| x as _).collect();
assert_eq!(basic_simd_sum(&numbers[1..99]), 4949.0);
```

1.82.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4728-4730)
#### pub fnis_sorted(&self) ->boolwhere
    T:PartialOrd,

Checks if the elements of this slice are sorted.

That is, for each elementaand its following elementb,a <= bmust hold. If the
slice yields exactly zero or one element,trueis returned.

Note that ifSelf::Itemis onlyPartialOrd, but notOrd, the above definition
implies that this function returnsfalseif any two consecutive items are not
comparable.

##### §Examples

```
let empty: [i32; 0] = [];

assert!([1, 2, 2, 9].is_sorted());
assert!(![1, 3, 2, 4].is_sorted());
assert!([0].is_sorted());
assert!(empty.is_sorted());
assert!(![0.0, 1.0, f32::NAN].is_sorted());
```

1.82.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4771-4773)
#### pub fnis_sorted_by<'a, F>(&'a self, compare: F) ->boolwhere
    F:FnMut(&'a T,&'a T) ->bool,

Checks if the elements of this slice are sorted using the given comparator function.

Instead of usingPartialOrd::partial_cmp, this function uses the givencomparefunction to determine whether two elements are to be considered in sorted order.

##### §Examples

```
assert!([1, 2, 2, 9].is_sorted_by(|a, b| a <= b));
assert!(![1, 2, 2, 9].is_sorted_by(|a, b| a < b));

assert!([0].is_sorted_by(|a, b| true));
assert!([0].is_sorted_by(|a, b| false));

let empty: [i32; 0] = [];
assert!(empty.is_sorted_by(|a, b| false));
assert!(empty.is_sorted_by(|a, b| true));
```

1.82.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4795-4798)
#### pub fnis_sorted_by_key<'a, F, K>(&'a self, f: F) ->boolwhere
    F:FnMut(&'a T) -> K,
    K:PartialOrd,

Checks if the elements of this slice are sorted using the given key extraction function.

Instead of comparing the slice’s elements directly, this function compares the keys of the
elements, as determined byf. Apart from that, it’s equivalent tois_sorted; see its
documentation for more information.

##### §Examples

```
assert!(["c", "bb", "aaa"].is_sorted_by_key(|s| s.len()));
assert!(![-2i32, -1, 0, 3].is_sorted_by_key(|n| n.abs()));
```

1.52.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#4854-4856)
#### pub fnpartition_point<P>(&self, pred: P) ->usizewhere
    P:FnMut(&T) ->bool,

Returns the index of the partition point according to the given predicate
(the index of the first element of the second partition).

The slice is assumed to be partitioned according to the given predicate.
This means that all elements for which the predicate returns true are at the start of the slice
and all elements for which the predicate returns false are at the end.
For example,[7, 15, 3, 5, 4, 12, 6]is partitioned under the predicatex % 2 != 0(all odd numbers are at the start, all even at the end).

If this slice is not partitioned, the returned result is unspecified and meaningless,
as this method performs a kind of binary search.

See alsobinary_search,binary_search_by, andbinary_search_by_key.

##### §Examples

```
let v = [1, 2, 3, 3, 5, 6, 7];
let i = v.partition_point(|&x| x < 5);

assert_eq!(i, 4);
assert!(v[..i].iter().all(|&x| x < 5));
assert!(v[i..].iter().all(|&x| !(x < 5)));
```

If all elements of the slice match the predicate, including if the slice
is empty, then the length of the slice will be returned:

```
let a = [2, 4, 8];
assert_eq!(a.partition_point(|x| x < &100), a.len());
let a: [i32; 0] = [];
assert_eq!(a.partition_point(|x| x < &100), 0);
```

If you want to insert an item to a sorted vector, while maintaining
sort order:

```
let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
let num = 42;
let idx = s.partition_point(|&x| x <= num);
s.insert(idx, num);
assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
```

1.94.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5260)
#### pub fnelement_offset(&self, element:&T) ->Option<usize>

Returns the index that an element reference points to.

ReturnsNoneifelementdoes not point to the start of an element within the slice.

This method is useful for extending slice iterators likeslice::split.

Note that this uses pointer arithmetic anddoes not compare elements.
To find the index of an element via comparison, use.iter().position()instead.

##### §Panics

Panics ifTis zero-sized.

##### §Examples

Basic usage:

```
let nums: &[u32] = &[1, 7, 1, 1];
let num = &nums[2];

assert_eq!(num, &1);
assert_eq!(nums.element_offset(num), Some(2));
```

ReturningNonewith an unaligned element:

```
let arr: &[[u32; 2]] = &[[0, 1], [2, 3]];
let flat_arr: &[u32] = arr.as_flattened();

let ok_elm: &[u32; 2] = flat_arr[0..2].try_into().unwrap();
let weird_elm: &[u32; 2] = flat_arr[1..3].try_into().unwrap();

assert_eq!(ok_elm, &[0, 1]);
assert_eq!(weird_elm, &[1, 2]);

assert_eq!(arr.element_offset(ok_elm), Some(0)); // Points to element 0
assert_eq!(arr.element_offset(weird_elm), None); // Points between element 0 and 1
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5314)
#### pub fnsubslice_range(&self, subslice: &[T]) ->Option<Range<usize>>

🔬
This is a nightly-only experimental API. (
`substr_range`)
Returns the range of indices that a subslice points to.

ReturnsNoneifsubslicedoes not point within the slice or if it is not aligned with the
elements in the slice.

This methoddoes not compare elements. Instead, this method finds the location in the slice thatsubslicewas obtained from. To find the index of a subslice via comparison, instead use.windows().position().

This method is useful for extending slice iterators likeslice::split.

Note that this may return a false positive (eitherSome(0..0)orSome(self.len()..self.len()))
ifsubslicehas a length of zero and points to the beginning or end of another, separate, slice.

##### §Panics

Panics ifTis zero-sized.

##### §Examples

Basic usage:

```
#![feature(substr_range)]

let nums = &[0, 5, 10, 0, 0, 5];

let mut iter = nums
    .split(|t| *t == 0)
    .map(|n| nums.subslice_range(n).unwrap());

assert_eq!(iter.next(), Some(0..0));
assert_eq!(iter.next(), Some(1..3));
assert_eq!(iter.next(), Some(4..4));
assert_eq!(iter.next(), Some(5..6));
```

[Source](https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#5341)
#### pub fnas_slice(&self) -> &[T]

🔬
This is a nightly-only experimental API. (
`str_as_str`)
Returns the same slice&[T].

This method is redundant when used directly on&[T], but
it helps dereferencing other “container” types to slices,
for exampleBox<[T]>orArc<[T]>.

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#372-374)
#### pub fnto_vec(&self) ->Vec<T>where
    T:Clone,

Available on
**non-no_global_oom_handling**only.
Copiesselfinto a newVec.

##### §Examples

```
let s = [10, 40, 30];
let x = s.to_vec();
// Here, `s` and `x` can be modified independently.
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#396-398)
#### pub fnto_vec_in<A>(&self, alloc: A) ->Vec<T, A>where
    A:Allocator,
    T:Clone,

🔬
This is a nightly-only experimental API. (
`allocator_api`)
Available on
**non-no_global_oom_handling**only.
Copiesselfinto a newVecwith an allocator.

##### §Examples

```
#![feature(allocator_api)]

use std::alloc::System;

let s = [10, 40, 30];
let x = s.to_vec_in(System);
// Here, `s` and `x` can be modified independently.
```

1.40.0
·
[Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#510-512)
#### pub fnrepeat(&self, n:usize) ->Vec<T>where
    T:Copy,

Available on
**non-no_global_oom_handling**only.
Creates a vector by copying a slicentimes.

##### §Panics

This function will panic if the capacity would overflow.

##### §Examples

```
assert_eq!([1, 2].repeat(3), vec![1, 2, 1, 2, 1, 2]);
```

A panic upon overflow:

ⓘ
```
// this will panic at runtime
b"0123456789abcdef".repeat(usize::MAX);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#578-580)
#### pub fnconcat<Item>(&self) -> <[T]asConcat<Item>>::Outputⓘwhere[T]:Concat<Item>,
    Item: ?Sized,

Flattens a slice ofTinto a single valueSelf::Output.

##### §Examples

```
assert_eq!(["hello", "world"].concat(), "helloworld");
assert_eq!([[1, 2], [3, 4]].concat(), [1, 2, 3, 4]);
```

1.3.0
·
[Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#597-599)
#### pub fnjoin<Separator>(
    &self,
    sep: Separator,
) -> <[T]asJoin<Separator>>::Outputⓘwhere[T]:Join<Separator>,

Flattens a slice ofTinto a single valueSelf::Output, placing a
given separator between each.

##### §Examples

```
assert_eq!(["hello", "world"].join(" "), "hello world");
assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3, 4]);
assert_eq!([[1, 2], [3, 4]].join(&[0, 0][..]), [1, 2, 0, 0, 3, 4]);
```

1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/alloc/slice.rs.html#617-619)
#### pub fnconnect<Separator>(
    &self,
    sep: Separator,
) -> <[T]asJoin<Separator>>::Outputⓘwhere[T]:Join<Separator>,

👎
Deprecated since 1.3.0: renamed to join
Flattens a slice ofTinto a single valueSelf::Output, placing a
given separator between each.

##### §Examples

```
assert_eq!(["hello", "world"].connect(" "), "hello world");
assert_eq!([[1, 2], [3, 4]].connect(&0), [1, 2, 0, 3, 4]);
```

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#410)§
### impl<T>AsRef<[T]> forSharedVector<T>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#412)§
#### fnas_ref(&self) -> &[T]

Converts this type into a shared reference of the (usually inferred) input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#109)§
### impl<T>CloneforSharedVector<T>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#110)§
#### fnclone(&self) ->SharedVector<T>

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#404)§
### impl<T>DebugforSharedVector<T>where
    T:Debug,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#405)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#398)§
### impl<T>DefaultforSharedVector<T>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#399)§
#### fndefault() ->SharedVector<T>

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#295)§
### impl<T>DerefforSharedVector<T>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#296)§
#### typeTarget=[T]

The resulting type after dereferencing.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#297)§
#### fnderef(&self) -> &<SharedVector<T> asDeref>::Target

Dereferences the value.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#89)§
### impl<T>DropforSharedVector<T>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#90)§
#### fndrop(&mut self)

Executes the destructor for this type.
[Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#378)§
### impl<T>Extend<T> forSharedVector<T>where
    T:Clone,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#379)§
#### fnextend<X>(&mut self, iter: X)where
    X:IntoIterator<Item = T>,

Extends a collection with the contents of an iterator.
[Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#420)§
#### fnextend_one(&mut self, item: A)

🔬
This is a nightly-only experimental API. (
`extend_one`)
Extends a collection with exactly one element.
[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)§
#### fnextend_reserve(&mut self, additional:usize)

🔬
This is a nightly-only experimental API. (
`extend_one`)
Reserves capacity in a collection for the given number of additional elements.
[Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#309)§
### impl<T>From<&[T]> forSharedVector<T>where
    T:Clone,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#310)§
#### fnfrom(slice: &[T]) ->SharedVector<T>

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#326)§
### impl<T, const N:usize>From<[T; N]> forSharedVector<T>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#327)§
#### fnfrom(array:[T; N]) ->SharedVector<T>

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#257)§
### implFrom<SharedVector<f32>> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#257)§
#### fnfrom(v:SharedVector<f32>) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#260)§
### implFrom<SharedVector<u16>> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#260)§
#### fnfrom(v:SharedVector<u16>) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#332)§
### impl<T>FromIterator<T> forSharedVector<T>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#333)§
#### fnfrom_iter<I>(iter: I) ->SharedVector<T>where
    I:IntoIterator<Item = T>,

Creates a value from an iterator.
[Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#429)§
### impl<T>IntoIteratorforSharedVector<T>where
    T:Clone,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#430)§
#### typeItem= T

The type of the elements being iterated over.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#431)§
#### typeIntoIter=IntoIter<T>

Which kind of iterator are we turning this into?
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#432)§
#### fninto_iter(self) -> <SharedVector<T> asIntoIterator>::IntoIter

Creates an iterator from a value.
[Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#417-420)§
### impl<T, U>PartialEq<U> forSharedVector<T>where
    U:AsRef<[T]> + ?Sized,
    T:PartialEq,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#422)§
#### fneq(&self, other:&U) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#257)§
### implTryFrom<Value> forSharedVector<f32>

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#257)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#257)§
#### fntry_from(v:Value) ->Result<SharedVector<f32>, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#260)§
### implTryFrom<Value> forSharedVector<u16>

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#260)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#260)§
#### fntry_from(v:Value) ->Result<SharedVector<u16>, Self::Error>

Performs the conversion.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#427)§
### impl<T>EqforSharedVector<T>where
    T:Eq,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#85)§
### impl<T>SendforSharedVector<T>where
    T:Send+Sync,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/sharedvector.rs.html#87)§
### impl<T>SyncforSharedVector<T>where
    T:Send+Sync,

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)§
#### fnequivalent(&self, key:&K) ->bool

Compare self to
`key`and return
`true`if they are equal.
[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)§
#### fnequivalent(&self, key:&K) ->bool

Checks if this value is equivalent to the given key.
[Read more](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#378-380)§
### impl<P, T>Receiverfor Pwhere
    P:Deref<Target = T> + ?Sized,
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#382)§
#### typeTarget= T

🔬
This is a nightly-only experimental API. (
`arbitrary_self_types`)
The target type on which the method may be called.
[Source](https://docs.rs/hex/0.4.3/x86_64-unknown-linux-gnu/src/hex/lib.rs.html#137)§
### impl<T>ToHexfor Twhere
    T:AsRef<[u8]>,

[Source](https://docs.rs/hex/0.4.3/x86_64-unknown-linux-gnu/src/hex/lib.rs.html#138)§
#### fnencode_hex<U>(&self) -> Uwhere
    U:FromIterator<char>,

Encode the hex strict representing
`self`into the result. Lower case letters are used (e.g.
`f9b4ca`)
[Source](https://docs.rs/hex/0.4.3/x86_64-unknown-linux-gnu/src/hex/lib.rs.html#142)§
#### fnencode_hex_upper<U>(&self) -> Uwhere
    U:FromIterator<char>,

Encode the hex strict representing
`self`into the result. Upper case letters are used (e.g.
`F9B4CA`)
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructStructCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#658)
```
pub struct Struct(/* private fields */);
```

Expand description
This type represents a runtime instance of structure in.slint.

This can either be an instance of a name structure introduced
with thestructkeyword in the .slint file, or an anonymous struct
written with the{ key: value, }notation.

It can be constructed with theFromIteratortrait, and converted
into or from aValuewith theFrom,TryFromtrait

```
use core::convert::TryInto;
// Construct a value from a key/value iterator
let value : Value = [("foo".into(), 45u32.into()), ("bar".into(), true.into())]
    .iter().cloned().collect::<Struct>().into();

// get the properties of a `{ foo: 45, bar: true }`
let s : Struct = value.try_into().unwrap();
assert_eq!(s.get_field("foo").cloned().unwrap().try_into(), Ok(45u32));
```

## Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#659-673)§
### implStruct

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#661-663)
#### pub fnget_field(&self, name: &str) ->Option<&Value>

Get the value for a given struct field

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#665-667)
#### pub fnset_field(&mut self, name:String, value:Value)

Set the value of a given struct field

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#670-672)
#### pub fniter(&self) -> implIterator<Item = (&str, &Value)>

Iterate over all the fields in this struct

## Trait Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#657)§
### implCloneforStruct

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#657)§
#### fnclone(&self) ->Struct

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#657)§
### implDebugforStruct

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#657)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#657)§
### implDefaultforStruct

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#657)§
#### fndefault() ->Struct

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#253)§
### implFrom<Struct> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#253)§
#### fnfrom(v:Struct) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#675-679)§
### implFromIterator<(String,Value)> forStruct

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#676-678)§
#### fnfrom_iter<T:IntoIterator<Item = (String,Value)>>(iter: T) -> Self

Creates a value from an iterator.
[Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#657)§
### implPartialEqforStruct

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#657)§
#### fneq(&self, other: &Struct) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#253)§
### implTryFrom<Value> forStruct

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#253)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#253)§
#### fntry_from(v:Value) ->Result<Struct, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#657)§
### implStructuralPartialEqforStruct

## Auto Trait Implementations§

§
### implFreezeforStruct

§
### impl !RefUnwindSafeforStruct

§
### impl !SendforStruct

§
### impl !SyncforStruct

§
### implUnpinforStruct

§
### implUnsafeUnpinforStruct

§
### impl !UnwindSafeforStruct

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructWeakCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#880)
```
pub struct Weak<T>where
    T: ComponentHandle,{ /* private fields */ }
```

Expand description
Struct that’s used to hold weak references of aSlint component

In order to create a Weak, you should useComponentHandle::as_weak.

Strong references should not be captured by the functions given to a lambda,
as this would produce a reference loop and leak the component.
Instead, the callback function should capture a weak component.

The Weak component also implementSendand can be send to another thread.
but the upgrade function will only return a valid component from the same thread
as the one it has been created from.
This is useful to use withinvoke_from_event_loop()orSelf::upgrade_in_event_loop().

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#906)§
### impl<T>Weak<T>where
    T:ComponentHandle,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#921-923)
#### pub fnupgrade(&self) ->Option<T>where
    T:ComponentHandle,

Returns a new strongly referenced component if some other instance still
holds a strong reference. Otherwise, returns None.

This also returns None if the current thread is not the thread that created
the component

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#937)
#### pub fnunwrap(&self) -> T

Convenience function that returns a new strongly referenced component if
some other instance still holds a strong reference and the current thread
is the thread that created this component.
Otherwise, this function panics.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#980-985)
#### pub fnupgrade_in_event_loop(
    &self,
    func: implFnOnce(T) +Send+ 'static,
) ->Result<(),EventLoopError>where
    T: 'static,

Available on
**crate featuresstdorunsafe-single-threaded**only.
Convenience function that combinesinvoke_from_event_loop()withSelf::upgrade()

The given functor will be added to an internal queue and will wake the event loop.
On the next iteration of the event loop, the functor will be executed with aTas an argument.

If the component was dropped because there are no more strong reference to the component,
the functor will not be called.

##### §Example

```
slint::slint! { export component MyApp inherits Window { in property <int> foo; /* ... */ } }
let handle = MyApp::new().unwrap();
let handle_weak = handle.as_weak();
let thread = std::thread::spawn(move || {
    // ... Do some computation in the thread
    let foo = 42;
    // now forward the data to the main thread using upgrade_in_event_loop
    handle_weak.upgrade_in_event_loop(move |handle| handle.set_foo(foo));
});
handle.run().unwrap();
```

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#896)§
### impl<T>CloneforWeak<T>where
    T:ComponentHandle,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#897)§
#### fnclone(&self) ->Weak<T>

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#886)§
### impl<T>DefaultforWeak<T>where
    T:ComponentHandle,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#887)§
#### fndefault() ->Weak<T>

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1000)§
### impl<T>SendforWeak<T>where
    T:ComponentHandle,

Available on
**crate featuresstdorunsafe-single-threaded**only.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1003)§
### impl<T>SyncforWeak<T>where
    T:ComponentHandle,

Available on
**crate featuresstdorunsafe-single-threaded**only.

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructWindowCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#449)
```
pub struct Window(/* private fields */);
```

Expand description
This type represents a window towards the windowing system, that’s used to render the
scene of a component. It provides API to control windowing system specific aspects such
as the position on the screen.

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#463)§
### implWindow

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#496)
#### pub fnnew(window_adapter_weak:Weak<dynWindowAdapter>) ->Window

Create a new window from a window adapter

You only need to create the window yourself when you create aWindowAdapterfromPlatform::create_window_adapter

Since the window adapter must own the Window, this function is meant to be used withRc::new_cyclic

##### §Example

```
use std::rc::Rc;
use slint::platform::{WindowAdapter, Renderer};
use slint::{Window, PhysicalSize};
struct MyWindowAdapter {
    window: Window,
    //...
}
impl WindowAdapter for MyWindowAdapter {
   fn window(&self) -> &Window { &self.window }
   fn size(&self) -> PhysicalSize { unimplemented!() }
   fn renderer(&self) -> &dyn Renderer { unimplemented!() }
}

fn create_window_adapter() -> Rc<dyn WindowAdapter> {
   Rc::<MyWindowAdapter>::new_cyclic(|weak| {
       MyWindowAdapter {
          window: Window::new(weak.clone()),
          //...
       }
   })
}
```

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#505)
#### pub fnshow(&self) ->Result<(),PlatformError>

Shows the window on the screen. An additional strong reference on the
associated component is maintained while the window is visible.

CallSelf::hide()to make the window invisible again, and drop the additional
strong reference.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#512)
#### pub fnhide(&self) ->Result<(),PlatformError>

Hides the window, so that it is not visible anymore. The additional strong
reference on the associated component, that was created whenSelf::show()was called, is
dropped.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#518-521)
#### pub fnset_rendering_notifier(
    &self,
    callback: implFnMut(RenderingState, &GraphicsAPI<'_>) + 'static,
) ->Result<(),SetRenderingNotifierError>

This function allows registering a callback that’s invoked during the different phases of
rendering. This allows custom rendering on top or below of the scene.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#527)
#### pub fnon_close_requested(
    &self,
    callback: implFnMut() ->CloseRequestResponse+ 'static,
)

This function allows registering a callback that’s invoked when the user tries to close a window.
The callback has to return aCloseRequestResponse.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#532)
#### pub fnrequest_redraw(&self)

This function issues a request to the windowing system to redraw the contents of the window.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#538)
#### pub fnscale_factor(&self) ->f32

This function returns the scale factor that allows converting between logical and
physical pixels.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#544)
#### pub fnposition(&self) ->PhysicalPosition

Returns the position of the window on the screen, in physical screen coordinates and including
a window frame (if present).

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#551)
#### pub fnset_position(&self, position: implInto<WindowPosition>)

Sets the position of the window on the screen, in physical screen coordinates and including
a window frame (if present).
Note that on some windowing systems, such as Wayland, this functionality is not available.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#558)
#### pub fnsize(&self) ->PhysicalSize

Returns the size of the window on the screen, in physical screen coordinates and excluding
a window frame (if present).

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#564)
#### pub fnset_size(&self, size: implInto<WindowSize>)

Resizes the window to the specified size on the screen, in physical pixels and excluding
a window frame (if present).

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#570)
#### pub fnis_fullscreen(&self) ->bool

Returns if the window is currently fullscreen

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#575)
#### pub fnset_fullscreen(&self, fullscreen:bool)

Set or unset the window to display fullscreen.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#580)
#### pub fnis_maximized(&self) ->bool

Returns if the window is currently maximized

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#585)
#### pub fnset_maximized(&self, maximized:bool)

Maximize or unmaximize the window.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#590)
#### pub fnis_minimized(&self) ->bool

Returns if the window is currently minimized

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#595)
#### pub fnset_minimized(&self, minimized:bool)

Minimize or unminimze the window.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#630)
#### pub fndispatch_event(&self, event:WindowEvent)

Dispatch a window event to the scene.

Use this when you’re implementing your own backend and want to forward user input events.

Any position fields in the event must be in the logical pixel coordinate system relative to
the top left corner of the window.

This function panics if there is an error processing the event.
UseSelf::try_dispatch_event()to handle the error.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#640-643)
#### pub fntry_dispatch_event(
    &self,
    event:WindowEvent,
) ->Result<(),PlatformError>

Dispatch a window event to the scene.

Use this when you’re implementing your own backend and want to forward user input events.

Any position fields in the event must be in the logical pixel coordinate system relative to
the top left corner of the window.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#722)
#### pub fnhas_active_animations(&self) ->bool

Returns true if there is an animation currently active on any property in the Window; false otherwise.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#729)
#### pub fnis_visible(&self) ->bool

Returns the visibility state of the window. This function can return false even if you previously called show()
on it, for example if the user minimized the window.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#743)
#### pub fnwindow_handle(&self) ->WindowHandle

Available on
**crate featureraw-window-handle-06**only.
Returns a struct that implements the raw window handle traits to access the windowing system specific window
and display handles.

Note that the window handle may only become available after the window has been created by the window manager,
which typically occurs after at least one iteration of the event loop following a call toshow().

Support for this function depends on the platform backend.

This function is only accessible if you enable theraw-window-handle-06crate feature.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#765)
#### pub fntake_snapshot(
    &self,
) ->Result<SharedPixelBuffer<Rgba<u8>>,PlatformError>

Takes a snapshot of the window contents and returns it as RGBA8 encoded pixel buffer.

Note that this function may be slow to call as it may need to re-render the scene.

## Trait Implementations§

[Source](https://docs.rs/i-slint-backend-qt/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_qt/lib.rs.html#337)§
### implQtWidgetAccessorforWindow

Available on
**non-no_qt**only.
[Source](https://docs.rs/i-slint-backend-qt/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_qt/lib.rs.html#338)§
#### fnqt_widget_ptr(&self) ->Option<NonNull<()>>

[Source](https://docs.rs/i-slint-backend-winit/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_winit/lib.rs.html#965)§
### implWinitWindowAccessorforWindow

[Source](https://docs.rs/i-slint-backend-winit/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_winit/lib.rs.html#966)§
#### fnhas_winit_window(&self) ->bool

Returns true if a
[winit::window::Window](https://docs.rs/winit/0.30.12/x86_64-unknown-linux-gnu/winit/window/struct.Window.html)exists for this window. This is the case if the window is backed by this winit backend.
[Source](https://docs.rs/i-slint-backend-winit/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_winit/lib.rs.html#974-977)§
#### fnwith_winit_window<T>(&self, callback: implFnOnce(&Window) -> T) ->Option<T>

Invokes the specified callback with a reference to the
[winit::window::Window](https://docs.rs/winit/0.30.12/x86_64-unknown-linux-gnu/winit/window/struct.Window.html)that exists for this Slint window and returns
`Some(T)`; otherwise
`None`.
[Source](https://docs.rs/i-slint-backend-winit/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_winit/lib.rs.html#985-987)§
#### fnwinit_window(
    &self,
) -> implFuture<Output =Result<Arc<Window>,PlatformError>>

Returns a future that resolves to the
[winit::window::Window](https://docs.rs/winit/0.30.12/x86_64-unknown-linux-gnu/winit/window/struct.Window.html)for this Slint window. When the future is ready, the output it resolves to is either
`Ok(Arc<winit::window::Window>)`if the window exists, or an error if the window has been deleted in the meanwhile or isn’t backed by the winit backend.
[Read more](https://docs.rs/i-slint-backend-winit/1.15.1/x86_64-unknown-linux-gnu/i_slint_backend_winit/trait.WinitWindowAccessor.html#tymethod.winit_window)[Source](https://docs.rs/i-slint-backend-winit/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_backend_winit/lib.rs.html#1003-1007)§
#### fnon_winit_window_event(
    &self,
    callback: implFnMut(&Window, &WindowEvent) ->EventResult+ 'static,
)

Registers a window event filter callback for this Slint window.
[Read more](https://docs.rs/i-slint-backend-winit/1.15.1/x86_64-unknown-linux-gnu/i_slint_backend_winit/trait.WinitWindowAccessor.html#tymethod.on_winit_window_event)
## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# StructWindowHandleCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#411)
```
pub struct WindowHandle { /* private fields */ }
```

Available on
**crate featureraw-window-handle-06**only.
Expand description
This struct represents a persistent handle to a window and implements theraw_window_handle_06::HasWindowHandleandraw_window_handle_06::HasDisplayHandletraits for accessing exposing raw window and display handles.
Obtain an instance of this by callingWindow::window_handle().

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#410)§
### implCloneforWindowHandle

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#410)§
#### fnclone(&self) ->WindowHandle

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#431)§
### implHasDisplayHandleforWindowHandle

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#432-434)§
#### fndisplay_handle(&self) ->Result<DisplayHandle<'_>,HandleError>

Get a handle to the display controller of the windowing system.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#416)§
### implHasWindowHandleforWindowHandle

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#417-419)§
#### fnwindow_handle(&self) ->Result<WindowHandle<'_>,HandleError>

Get a handle to the window.

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://docs.rs/raw-window-handle/0.6.2/x86_64-unknown-linux-gnu/src/raw_window_handle/lib.rs.html#232)§
### impl<T>HasRawDisplayHandlefor Twhere
    T:HasDisplayHandle+ ?Sized,

[Source](https://docs.rs/raw-window-handle/0.6.2/x86_64-unknown-linux-gnu/src/raw_window_handle/lib.rs.html#233)§
#### fnraw_display_handle(&self) ->Result<RawDisplayHandle,HandleError>

👎
Deprecated: Use
`HasDisplayHandle`instead
[Source](https://docs.rs/raw-window-handle/0.6.2/x86_64-unknown-linux-gnu/src/raw_window_handle/lib.rs.html#85)§
### impl<T>HasRawWindowHandlefor Twhere
    T:HasWindowHandle+ ?Sized,

[Source](https://docs.rs/raw-window-handle/0.6.2/x86_64-unknown-linux-gnu/src/raw_window_handle/lib.rs.html#86)§
#### fnraw_window_handle(&self) ->Result<RawWindowHandle,HandleError>

👎
Deprecated: Use
`HasWindowHandle`instead
[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumBrushCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#25)
```
#[non_exhaustive]#[repr(C)]pub enum Brush {
    SolidColor(Color),
    LinearGradient(LinearGradientBrush),
    RadialGradient(RadialGradientBrush),
    ConicGradient(ConicGradientBrush),
}
```

Expand description
A brush is a data structure that is used to describe how
a shape, such as a rectangle, path or even text, shall be filled.
A brush can also be applied to the outline of a shape, that means
the fill of the outline itself.

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
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

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#46)§
### implBrush

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#49)
#### pub fncolor(&self) ->Color

If the brush is SolidColor, the contained color is returned.
If the brush is a LinearGradient, the color of the first stop is returned.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#72)
#### pub fnis_transparent(&self) ->bool

Returns true if this brush contains a fully transparent color (alpha value is zero)

```
assert!(Brush::default().is_transparent());
assert!(Brush::SolidColor(Color::from_argb_u8(0, 255, 128, 140)).is_transparent());
assert!(!Brush::SolidColor(Color::from_argb_u8(25, 128, 140, 210)).is_transparent());
```

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#89)
#### pub fnis_opaque(&self) ->bool

Returns true if this brush is fully opaque

```
assert!(!Brush::default().is_opaque());
assert!(!Brush::SolidColor(Color::from_argb_u8(25, 255, 128, 140)).is_opaque());
assert!(Brush::SolidColor(Color::from_rgb_u8(128, 140, 210)).is_opaque());
```

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#102)
#### pub fnbrighter(&self, factor:f32) ->Brush

Returns a new version of this brush that has the brightness increased
by the specified factor. This is done by callingColor::brighteron
all the colors of this brush.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#132)
#### pub fndarker(&self, factor:f32) ->Brush

Returns a new version of this brush that has the brightness decreased
by the specified factor. This is done by callingColor::darkeron
all the color of this brush.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#161)
#### pub fntransparentize(&self, amount:f32) ->Brush

Returns a new version of this brush with the opacity decreased byfactor.

The transparency is obtained by multiplying the alpha channel by(1 - factor).

See alsoColor::transparentize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#190)
#### pub fnwith_alpha(&self, alpha:f32) ->Brush

Returns a new version of this brush with the related color’s opacities
set toalpha.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
### implCloneforBrush

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
#### fnclone(&self) ->Brush

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
### implDebugforBrush

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#40)§
### implDefaultforBrush

Construct a brush with transparent color

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#41)§
#### fndefault() ->Brush

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#254)§
### implFrom<Brush> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#254)§
#### fnfrom(v:Brush) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
### implFrom<Color> forBrush

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
#### fnfrom(value:Color) ->Brush

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
### implFrom<ConicGradientBrush> forBrush

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
#### fnfrom(value:ConicGradientBrush) ->Brush

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
### implFrom<LinearGradientBrush> forBrush

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
#### fnfrom(value:LinearGradientBrush) ->Brush

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
### implFrom<RadialGradientBrush> forBrush

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
#### fnfrom(value:RadialGradientBrush) ->Brush

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#556)§
### implInterpolatedPropertyValueforBrush

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#557)§
#### fninterpolate(&self, target_value: &Brush, t:f32) ->Brush

Returns the interpolated value between self and target_value according to the progress parameter t that’s usually between 0 and 1. With certain animation easing curves it may over- or undershoot though.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
### implPartialEqforBrush

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
#### fneq(&self, other: &Brush) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#254)§
### implTryFrom<Value> forBrush

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#254)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#254)§
#### fntry_from(v:Value) ->Result<Brush, Self::Error>

Performs the conversion.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/brush.rs.html#22)§
### implStructuralPartialEqforBrush

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumCloseRequestResponseCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#455)
```
#[repr(u8)]pub enum CloseRequestResponse {
    HideWindow = 0,
    KeepWindowShown = 1,
}
```

Expand description
This enum describes whether a Window is allowed to be hidden when the user tries to close the window.
It is the return type of the callback provided toWindow::on_close_requested.

## Variants§

§
### HideWindow = 0

The Window will be hidden (default action)

§
### KeepWindowShown = 1

The close request is rejected and the window will be kept shown.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#453)§
### implCloneforCloseRequestResponse

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#453)§
#### fnclone(&self) ->CloseRequestResponse

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#453)§
### implDebugforCloseRequestResponse

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#453)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#453)§
### implDefaultforCloseRequestResponse

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#453)§
#### fndefault() ->CloseRequestResponse

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#453)§
### implPartialEqforCloseRequestResponse

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#453)§
#### fneq(&self, other: &CloseRequestResponse) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#453)§
### implCopyforCloseRequestResponse

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#453)§
### implStructuralPartialEqforCloseRequestResponse

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumDefaultTranslationContextCopy item path

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/lib.rs.html#68)
```
#[non_exhaustive]pub enum DefaultTranslationContext {
    ComponentName,
    None,
}
```

Expand description
Argument ofCompiler::set_default_translation_context()

This enum specifies the default translation context when no context is explicitly
specified in the@tr("context" => ...)macro.

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
### ComponentName

The default translation context is the component name in which the@tris written.

This is the default behavior ofslint-tr-extractor.

§
### None

Opt out of the default translation context.

When using this option, invokeslint-tr-extractorwith--no-default-translation-contextto make sure that the translation files have no context for strings which didn’t specify a context.

## Trait Implementations§

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/lib.rs.html#66)§
### implCloneforDefaultTranslationContext

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/lib.rs.html#66)§
#### fnclone(&self) ->DefaultTranslationContext

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/lib.rs.html#66)§
### implDebugforDefaultTranslationContext

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/lib.rs.html#66)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/lib.rs.html#66)§
### implPartialEqforDefaultTranslationContext

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/lib.rs.html#66)§
#### fneq(&self, other: &DefaultTranslationContext) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/lib.rs.html#66)§
### implEqforDefaultTranslationContext

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/lib.rs.html#66)§
### implStructuralPartialEqforDefaultTranslationContext

## Auto Trait Implementations§

§
### implFreezeforDefaultTranslationContext

§
### implRefUnwindSafeforDefaultTranslationContext

§
### implSendforDefaultTranslationContext

§
### implSyncforDefaultTranslationContext

§
### implUnpinforDefaultTranslationContext

§
### implUnsafeUnpinforDefaultTranslationContext

§
### implUnwindSafeforDefaultTranslationContext

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)§
#### fnequivalent(&self, key:&K) ->bool

Compare self to
`key`and return
`true`if they are equal.
[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)§
#### fnequivalent(&self, key:&K) ->bool

Checks if this value is equivalent to the given key.
[Read more](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumDiagnosticLevelCopy item path

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#256)
```
#[non_exhaustive]pub enum DiagnosticLevel {
    Error,
    Warning,
}
```

Expand description
This enum describes the level or severity of a diagnostic message produced by the compiler.

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
### Error

The diagnostic found is an error that prevents successful compilation.

§
### Warning

The diagnostic found is a warning.

## Trait Implementations§

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#254)§
### implCloneforDiagnosticLevel

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#254)§
#### fnclone(&self) ->DiagnosticLevel

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#254)§
### implDebugforDiagnosticLevel

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#254)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#254)§
### implDefaultforDiagnosticLevel

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#254)§
#### fndefault() ->DiagnosticLevel

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#254)§
### implPartialEqforDiagnosticLevel

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#254)§
#### fneq(&self, other: &DiagnosticLevel) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#254)§
### implCopyforDiagnosticLevel

[Source](https://docs.rs/i-slint-compiler/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_compiler/diagnostics.rs.html#254)§
### implStructuralPartialEqforDiagnosticLevel

## Auto Trait Implementations§

§
### implFreezeforDiagnosticLevel

§
### implRefUnwindSafeforDiagnosticLevel

§
### implSendforDiagnosticLevel

§
### implSyncforDiagnosticLevel

§
### implUnpinforDiagnosticLevel

§
### implUnsafeUnpinforDiagnosticLevel

§
### implUnwindSafeforDiagnosticLevel

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumEventLoopErrorCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1065)
```
#[non_exhaustive]pub enum EventLoopError {
    EventLoopTerminated,
    NoEventLoopProvider,
}
```

Expand description
Error returned from theinvoke_from_event_loop()andquit_event_loop()function

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
### EventLoopTerminated

The event could not be sent because the event loop was terminated already

§
### NoEventLoopProvider

The event could not be sent because the Slint platform abstraction was not yet initialized,
or the platform does not support event loop.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1062)§
### implCloneforEventLoopError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1062)§
#### fnclone(&self) ->EventLoopError

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1062)§
### implDebugforEventLoopError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1062)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1073)§
### implDisplayforEventLoopError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1074)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1087)§
### implErrorforEventLoopError

Available on
**crate featurestd**only.
1.30.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#111)§
#### fnsource(&self) ->Option<&(dynError+ 'static)>

Returns the lower-level source of this error, if any.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#137)§
#### fndescription(&self) -> &str

👎
Deprecated since 1.42.0: use the Display impl or to_string()
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#147)§
#### fncause(&self) ->Option<&dynError>

👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#260)§
#### fnprovide<'a>(&'a self, request: &mutRequest<'a>)

🔬
This is a nightly-only experimental API. (
`error_generic_member_access`)
Provides type-based access to context intended for error reports.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1062)§
### implPartialEqforEventLoopError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1062)§
#### fneq(&self, other: &EventLoopError) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1062)§
### implEqforEventLoopError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1062)§
### implStructuralPartialEqforEventLoopError

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)§
#### fnequivalent(&self, key:&K) ->bool

Compare self to
`key`and return
`true`if they are equal.
[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)§
#### fnequivalent(&self, key:&K) ->bool

Checks if this value is equivalent to the given key.
[Read more](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#322-324)§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#326)§
#### fnto_shared_string(&self) ->SharedString

Converts the given value to a
[SharedString](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.SharedString.html).
[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#678-680)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#682)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2891)§
### impl<T>ToStringfor Twhere
    T:Display+ ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893)§
#### fnto_string(&self) ->String

Converts the given value to a
`String`.
[Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumGetPropertyErrorCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1738-1742)
```
#[non_exhaustive]pub enum GetPropertyError {
    NoSuchProperty,
}
```

Expand description
Error returned byComponentInstance::get_property

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
### NoSuchProperty

There is no property with the given name

## Trait Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1736)§
### implCloneforGetPropertyError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1736)§
#### fnclone(&self) ->GetPropertyError

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1736)§
### implDebugforGetPropertyError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1736)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1736)§
### implDisplayforGetPropertyError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1736)§
#### fnfmt(&self, __derive_more_f: &mutFormatter<'_>) ->Result

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1736)§
### implErrorforGetPropertyError

1.30.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#111)§
#### fnsource(&self) ->Option<&(dynError+ 'static)>

Returns the lower-level source of this error, if any.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#137)§
#### fndescription(&self) -> &str

👎
Deprecated since 1.42.0: use the Display impl or to_string()
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#147)§
#### fncause(&self) ->Option<&dynError>

👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#260)§
#### fnprovide<'a>(&'a self, request: &mutRequest<'a>)

🔬
This is a nightly-only experimental API. (
`error_generic_member_access`)
Provides type-based access to context intended for error reports.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1736)§
### implPartialEqforGetPropertyError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1736)§
#### fneq(&self, other: &GetPropertyError) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1736)§
### implCopyforGetPropertyError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1736)§
### implEqforGetPropertyError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1736)§
### implStructuralPartialEqforGetPropertyError

## Auto Trait Implementations§

§
### implFreezeforGetPropertyError

§
### implRefUnwindSafeforGetPropertyError

§
### implSendforGetPropertyError

§
### implSyncforGetPropertyError

§
### implUnpinforGetPropertyError

§
### implUnsafeUnpinforGetPropertyError

§
### implUnwindSafeforGetPropertyError

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)§
#### fnequivalent(&self, key:&K) ->bool

Compare self to
`key`and return
`true`if they are equal.
[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)§
#### fnequivalent(&self, key:&K) ->bool

Checks if this value is equivalent to the given key.
[Read more](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#322-324)§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#326)§
#### fnto_shared_string(&self) ->SharedString

Converts the given value to a
[SharedString](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.SharedString.html).
[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#678-680)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#682)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2891)§
### impl<T>ToStringfor Twhere
    T:Display+ ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893)§
#### fnto_string(&self) ->String

Converts the given value to a
`String`.
[Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumGraphicsAPICopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#263)
```
#[non_exhaustive]pub enum GraphicsAPI<'a> {
    NativeOpenGL {
        get_proc_address: &'a dyn Fn(&CStr) -> *const c_void,
    },
    WebGL {
        canvas_element_id: &'a str,
        context_type: &'a str,
    },
}
```

Expand description
This enum describes a low-level access to specific graphics APIs used
by the renderer.

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
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

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#261)§
### impl<'a>CloneforGraphicsAPI<'a>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#261)§
#### fnclone(&self) ->GraphicsAPI<'a>

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#313)§
### implDebugforGraphicsAPI<'_>

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#314)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)
## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumInvokeErrorCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1775-1779)
```
#[non_exhaustive]pub enum InvokeError {
    NoSuchCallable,
}
```

Expand description
Error returned byComponentInstance::invoke

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
### NoSuchCallable

There is no callback or function with the given name

## Trait Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1773)§
### implCloneforInvokeError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1773)§
#### fnclone(&self) ->InvokeError

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1773)§
### implDebugforInvokeError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1773)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1773)§
### implDisplayforInvokeError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1773)§
#### fnfmt(&self, __derive_more_f: &mutFormatter<'_>) ->Result

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1773)§
### implErrorforInvokeError

1.30.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#111)§
#### fnsource(&self) ->Option<&(dynError+ 'static)>

Returns the lower-level source of this error, if any.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#137)§
#### fndescription(&self) -> &str

👎
Deprecated since 1.42.0: use the Display impl or to_string()
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#147)§
#### fncause(&self) ->Option<&dynError>

👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#260)§
#### fnprovide<'a>(&'a self, request: &mutRequest<'a>)

🔬
This is a nightly-only experimental API. (
`error_generic_member_access`)
Provides type-based access to context intended for error reports.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1773)§
### implPartialEqforInvokeError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1773)§
#### fneq(&self, other: &InvokeError) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1773)§
### implCopyforInvokeError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1773)§
### implEqforInvokeError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1773)§
### implStructuralPartialEqforInvokeError

## Auto Trait Implementations§

§
### implFreezeforInvokeError

§
### implRefUnwindSafeforInvokeError

§
### implSendforInvokeError

§
### implSyncforInvokeError

§
### implUnpinforInvokeError

§
### implUnsafeUnpinforInvokeError

§
### implUnwindSafeforInvokeError

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)§
#### fnequivalent(&self, key:&K) ->bool

Compare self to
`key`and return
`true`if they are equal.
[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)§
#### fnequivalent(&self, key:&K) ->bool

Checks if this value is equivalent to the given key.
[Read more](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#322-324)§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#326)§
#### fnto_shared_string(&self) ->SharedString

Converts the given value to a
[SharedString](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.SharedString.html).
[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#678-680)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#682)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2891)§
### impl<T>ToStringfor Twhere
    T:Display+ ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893)§
#### fnto_string(&self) ->String

Converts the given value to a
`String`.
[Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumPlatformErrorCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1099)
```
#[non_exhaustive]pub enum PlatformError {
    NoPlatform,
    NoEventLoopProvider,
    SetPlatformError(SetPlatformError),
    Other(String),
    OtherError(Box<dyn Error + Send + Sync>),
}
```

Expand description
The platform encountered a fatal error.

This error typically indicates an issue with initialization or connecting to the windowing system.

This can be constructed from aString:

```
use slint::platform::PlatformError;
PlatformError::from(format!("Could not load resource {}", 1234));
```

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
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

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1129)§
### implDebugforPlatformError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1130)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1135)§
### implDisplayforPlatformError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1136)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1173)§
### implErrorforPlatformError

Available on
**crate featurestd**only.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1174)§
#### fnsource(&self) ->Option<&(dynError+ 'static)>

Returns the lower-level source of this error, if any.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#137)§
#### fndescription(&self) -> &str

👎
Deprecated since 1.42.0: use the Display impl or to_string()
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#147)§
#### fncause(&self) ->Option<&dynError>

👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#260)§
#### fnprovide<'a>(&'a self, request: &mutRequest<'a>)

🔬
This is a nightly-only experimental API. (
`error_generic_member_access`)
Provides type-based access to context intended for error reports.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1159)§
### implFrom<&str> forPlatformError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1160)§
#### fnfrom(value: &str) ->PlatformError

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1166)§
### implFrom<Box<dynError+Send+Sync>> forPlatformError

Available on
**crate featurestd**only.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1167)§
#### fnfrom(error:Box<dynError+Send+Sync>) ->PlatformError

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1154)§
### implFrom<String> forPlatformError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1155)§
#### fnfrom(value:String) ->PlatformError

Converts to this type from the input type.

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#322-324)§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#326)§
#### fnto_shared_string(&self) ->SharedString

Converts the given value to a
[SharedString](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.SharedString.html).
[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#678-680)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#682)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2891)§
### impl<T>ToStringfor Twhere
    T:Display+ ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893)§
#### fnto_string(&self) ->String

Converts the given value to a
`String`.
[Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumRenderingStateCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#338)
```
#[non_exhaustive]#[repr(u8)]pub enum RenderingState {
    RenderingSetup = 0,
    BeforeRendering = 1,
    AfterRendering = 2,
    RenderingTeardown = 3,
}
```

Expand description
This enum describes the different rendering states, that will be provided
to the parameter of the callback forset_rendering_notifieron theslint::Window.

When OpenGL is used for rendering, the context will be current.
It’s safe to call OpenGL functions, but it is crucial that the state of the context is
preserved. So make sure to save and restore state such asTEXTURE_BINDING_2DorARRAY_BUFFER_BINDINGperfectly.

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
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

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#335)§
### implCloneforRenderingState

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#335)§
#### fnclone(&self) ->RenderingState

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#335)§
### implDebugforRenderingState

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#335)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)
## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumSetCallbackErrorCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1766-1770)
```
#[non_exhaustive]pub enum SetCallbackError {
    NoSuchCallback,
}
```

Expand description
Error returned byComponentInstance::set_callback

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
### NoSuchCallback

There is no callback with the given name

## Trait Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1764)§
### implCloneforSetCallbackError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1764)§
#### fnclone(&self) ->SetCallbackError

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1764)§
### implDebugforSetCallbackError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1764)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1764)§
### implDisplayforSetCallbackError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1764)§
#### fnfmt(&self, __derive_more_f: &mutFormatter<'_>) ->Result

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1764)§
### implErrorforSetCallbackError

1.30.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#111)§
#### fnsource(&self) ->Option<&(dynError+ 'static)>

Returns the lower-level source of this error, if any.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#137)§
#### fndescription(&self) -> &str

👎
Deprecated since 1.42.0: use the Display impl or to_string()
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#147)§
#### fncause(&self) ->Option<&dynError>

👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#260)§
#### fnprovide<'a>(&'a self, request: &mutRequest<'a>)

🔬
This is a nightly-only experimental API. (
`error_generic_member_access`)
Provides type-based access to context intended for error reports.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1764)§
### implPartialEqforSetCallbackError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1764)§
#### fneq(&self, other: &SetCallbackError) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1764)§
### implCopyforSetCallbackError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1764)§
### implEqforSetCallbackError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1764)§
### implStructuralPartialEqforSetCallbackError

## Auto Trait Implementations§

§
### implFreezeforSetCallbackError

§
### implRefUnwindSafeforSetCallbackError

§
### implSendforSetCallbackError

§
### implSyncforSetCallbackError

§
### implUnpinforSetCallbackError

§
### implUnsafeUnpinforSetCallbackError

§
### implUnwindSafeforSetCallbackError

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)§
#### fnequivalent(&self, key:&K) ->bool

Compare self to
`key`and return
`true`if they are equal.
[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)§
#### fnequivalent(&self, key:&K) ->bool

Checks if this value is equivalent to the given key.
[Read more](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#322-324)§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#326)§
#### fnto_shared_string(&self) ->SharedString

Converts the given value to a
[SharedString](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.SharedString.html).
[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#678-680)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#682)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2891)§
### impl<T>ToStringfor Twhere
    T:Display+ ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893)§
#### fnto_string(&self) ->String

Converts the given value to a
`String`.
[Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumSetPropertyErrorCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1747-1761)
```
#[non_exhaustive]pub enum SetPropertyError {
    NoSuchProperty,
    WrongType,
    AccessDenied,
}
```

Expand description
Error returned byComponentInstance::set_property

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
### NoSuchProperty

There is no property with the given name.

§
### WrongType

The property exists but does not have a type matching the dynamic value.

This happens for example when assigning a source struct value to a target
struct property, where the source doesn’t have all the fields the target struct
requires.

§
### AccessDenied

Attempt to set an output property.

## Trait Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1745)§
### implCloneforSetPropertyError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1745)§
#### fnclone(&self) ->SetPropertyError

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1745)§
### implDebugforSetPropertyError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1745)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1745)§
### implDisplayforSetPropertyError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1745)§
#### fnfmt(&self, __derive_more_f: &mutFormatter<'_>) ->Result

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1745)§
### implErrorforSetPropertyError

1.30.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#111)§
#### fnsource(&self) ->Option<&(dynError+ 'static)>

Returns the lower-level source of this error, if any.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#137)§
#### fndescription(&self) -> &str

👎
Deprecated since 1.42.0: use the Display impl or to_string()
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#147)§
#### fncause(&self) ->Option<&dynError>

👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#260)§
#### fnprovide<'a>(&'a self, request: &mutRequest<'a>)

🔬
This is a nightly-only experimental API. (
`error_generic_member_access`)
Provides type-based access to context intended for error reports.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1745)§
### implPartialEqforSetPropertyError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1745)§
#### fneq(&self, other: &SetPropertyError) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1745)§
### implCopyforSetPropertyError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1745)§
### implEqforSetPropertyError

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1745)§
### implStructuralPartialEqforSetPropertyError

## Auto Trait Implementations§

§
### implFreezeforSetPropertyError

§
### implRefUnwindSafeforSetPropertyError

§
### implSendforSetPropertyError

§
### implSyncforSetPropertyError

§
### implUnpinforSetPropertyError

§
### implUnsafeUnpinforSetPropertyError

§
### implUnwindSafeforSetPropertyError

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)§
#### fnequivalent(&self, key:&K) ->bool

Compare self to
`key`and return
`true`if they are equal.
[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)§
### impl<Q, K>Equivalent<K> for Qwhere
    Q:Eq+ ?Sized,
    K:Borrow<Q> + ?Sized,

[Source](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)§
#### fnequivalent(&self, key:&K) ->bool

Checks if this value is equivalent to the given key.
[Read more](https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#322-324)§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#326)§
#### fnto_shared_string(&self) ->SharedString

Converts the given value to a
[SharedString](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.SharedString.html).
[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#678-680)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#682)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2891)§
### impl<T>ToStringfor Twhere
    T:Display+ ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893)§
#### fnto_string(&self) ->String

Converts the given value to a
`String`.
[Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumSetRenderingNotifierErrorCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#371)
```
#[non_exhaustive]#[repr(u8)]pub enum SetRenderingNotifierError {
    Unsupported = 0,
    AlreadySet = 1,
}
```

Expand description
This enum describes the different error scenarios that may occur when the application
registers a rendering notifier on aslint::Window.

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
### Unsupported = 0

The rendering backend does not support rendering notifiers.

§
### AlreadySet = 1

There is already a rendering notifier set, multiple notifiers are not supported.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#368)§
### implCloneforSetRenderingNotifierError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#368)§
#### fnclone(&self) ->SetRenderingNotifierError

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#368)§
### implDebugforSetRenderingNotifierError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#368)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#378)§
### implDisplayforSetRenderingNotifierError

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#379)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#392)§
### implErrorforSetRenderingNotifierError

Available on
**crate featurestd**only.
1.30.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#111)§
#### fnsource(&self) ->Option<&(dynError+ 'static)>

Returns the lower-level source of this error, if any.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#137)§
#### fndescription(&self) -> &str

👎
Deprecated since 1.42.0: use the Display impl or to_string()
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#147)§
#### fncause(&self) ->Option<&dynError>

👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#260)§
#### fnprovide<'a>(&'a self, request: &mutRequest<'a>)

🔬
This is a nightly-only experimental API. (
`error_generic_member_access`)
Provides type-based access to context intended for error reports.
[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)
## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#322-324)§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#326)§
#### fnto_shared_string(&self) ->SharedString

Converts the given value to a
[SharedString](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/struct.SharedString.html).
[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#678-680)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#682)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)§
### impl<T>ToSmolStrfor Twhere
    T:Display+ ?Sized,

[Source](https://docs.rs/smol_str/0.3.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)§
#### fnto_smolstr(&self) ->SmolStr

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2891)§
### impl<T>ToStringfor Twhere
    T:Display+ ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2893)§
#### fnto_string(&self) ->String

Converts the given value to a
`String`.
[Read more](https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html#tymethod.to_string)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumValueCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#101-140)
```
#[non_exhaustive]pub enum Value {
    Void = 0,
    Number(f64),
    String(SharedString),
    Bool(bool),
    Image(Image),
    Model(ModelRc<Value>),
    Struct(Struct),
    Brush(Brush),
}
```

Expand description
This is a dynamically typed value used in the Slint interpreter.
It can hold a value of different types, and you should use theFromorTryFromtraits to access the value.

```
use core::convert::TryInto;
// create a value containing an integer
let v = Value::from(100u32);
assert_eq!(v.try_into(), Ok(100u32));
```

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
### Void = 0

There is nothing in this value. That’s the default.
For example, a function that does not return a result would return a Value::Void

§
### Number(f64)

Anintor afloat(this is also used for unit based type such aslengthorangle)

§
### String(SharedString)

Correspond to thestringtype in .slint

§
### Bool(bool)

Correspond to thebooltype in .slint

§
### Image(Image)

Correspond to theimagetype in .slint

§
### Model(ModelRc<Value>)

A model (that includes array in .slint)

§
### Struct(Struct)

An object

§
### Brush(Brush)

Correspond tobrushorcolortype in .slint.  For color, this is then aBrush::SolidColor

## Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#142-157)§
### implValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#144-156)
#### pub fnvalue_type(&self) ->ValueType

Returns the type variant that this value holds without the containing value.

## Trait Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#98)§
### implCloneforValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#98)§
#### fnclone(&self) ->Value

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#193-219)§
### implDebugforValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#194-218)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#98)§
### implDefaultforValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#98)§
#### fndefault() ->Value

Returns the “default value” for a type.
[Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#396-401)§
### implFrom<()> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#398-400)§
#### fnfrom(_:()) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<AccessibleRole> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:AccessibleRole) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<AnimationDirection> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:AnimationDirection) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#254)§
### implFrom<Brush> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#254)§
#### fnfrom(v:Brush) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#410-415)§
### implFrom<Color> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#412-414)§
#### fnfrom(c:Color) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<ColorScheme> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:ColorScheme) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#258)§
### implFrom<ComponentFactory> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#258)§
#### fnfrom(v:ComponentFactory) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<DialogButtonRole> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:DialogButtonRole) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implFrom<DropEvent> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fnfrom(item:DropEvent) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#256)§
### implFrom<EasingCurve> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#256)§
#### fnfrom(v:EasingCurve) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implFrom<Edges> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fnfrom(item:Edges) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<EventResult> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:EventResult) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<FillRule> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:FillRule) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<FocusReason> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:FocusReason) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implFrom<FontMetrics> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fnfrom(item:FontMetrics) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#252)§
### implFrom<Image> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#252)§
#### fnfrom(v:Image) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<ImageFit> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:ImageFit) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<ImageHorizontalAlignment> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:ImageHorizontalAlignment) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<ImageRendering> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:ImageRendering) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<ImageTiling> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:ImageTiling) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<ImageVerticalAlignment> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:ImageVerticalAlignment) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<InputType> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:InputType) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#381-385)§
### implFrom<Instant> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#382-384)§
#### fnfrom(value:Instant) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implFrom<KeyEvent> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fnfrom(item:KeyEvent) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implFrom<KeyboardModifiers> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fnfrom(item:KeyboardModifiers) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<LayoutAlignment> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:LayoutAlignment) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#344)§
### implFrom<LayoutInfo> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#344)§
#### fnfrom(_:LayoutInfo) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#427-432)§
### implFrom<Length<f32,LogicalPx>> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#429-431)§
#### fnfrom(l:LogicalLength) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<LineCap> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:LineCap) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<LineJoin> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:LineJoin) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#508-518)§
### implFrom<LogicalEdges> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#510-517)§
#### fnfrom(s:LogicalEdges) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#346)§
### implFrom<LogicalPosition> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#346)§
#### fnfrom(_:LogicalPosition) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implFrom<MenuEntry> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fnfrom(item:MenuEntry) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#552-560)§
### impl<T:Into<Value> +TryFrom<Value> + 'static>From<ModelRc<T>> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#553-559)§
#### fnfrom(m:ModelRc<T>) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<MouseCursor> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:MouseCursor) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<OperatingSystemType> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:OperatingSystemType) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<Orientation> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:Orientation) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#255)§
### implFrom<PathData> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#255)§
#### fnfrom(v:PathData) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<PathEvent> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:PathEvent) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#444-452)§
### implFrom<Point2D<f32,LogicalPx>> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#446-451)§
#### fnfrom(pt:LogicalPoint) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#345)§
### implFrom<Point2D<f32,UnknownUnit>> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#345)§
#### fnfrom(_:Point) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implFrom<PointerEvent> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fnfrom(item:PointerEvent) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<PointerEventButton> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:PointerEventButton) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<PointerEventKind> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:PointerEventKind) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implFrom<PointerScrollEvent> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fnfrom(item:PointerScrollEvent) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<PopupClosePolicy> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:PopupClosePolicy) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<ScrollBarPolicy> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:ScrollBarPolicy) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#250)§
### implFrom<SharedString> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#250)§
#### fnfrom(v:SharedString) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#257)§
### implFrom<SharedVector<f32>> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#257)§
#### fnfrom(v:SharedVector<f32>) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#260)§
### implFrom<SharedVector<u16>> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#260)§
#### fnfrom(v:SharedVector<u16>) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#476-484)§
### implFrom<Size2D<f32,LogicalPx>> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#478-483)§
#### fnfrom(s:LogicalSize) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<SortOrder> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:SortOrder) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<StandardButtonKind> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:StandardButtonKind) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implFrom<StandardListViewItem> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fnfrom(item:StandardListViewItem) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implFrom<StateInfo> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fnfrom(item:StateInfo) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#253)§
### implFrom<Struct> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#253)§
#### fnfrom(v:Struct) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#259)§
### implFrom<StyledText> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#259)§
#### fnfrom(v:StyledText) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implFrom<TableColumn> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fnfrom(item:TableColumn) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<TextHorizontalAlignment> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:TextHorizontalAlignment) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<TextOverflow> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:TextOverflow) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<TextStrokeStyle> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:TextStrokeStyle) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<TextVerticalAlignment> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:TextVerticalAlignment) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implFrom<TextWrap> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fnfrom(v:TextWrap) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#251)§
### implFrom<bool> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#251)§
#### fnfrom(v:bool) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implFrom<f32> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fnfrom(v:f32) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implFrom<f64> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fnfrom(v:f64) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implFrom<i32> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fnfrom(v:i32) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implFrom<i64> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fnfrom(v:i64) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implFrom<isize> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fnfrom(v:isize) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implFrom<u32> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fnfrom(v:u32) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implFrom<u64> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fnfrom(v:u64) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implFrom<usize> forValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fnfrom(v:usize) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#159-191)§
### implPartialEqforValue

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#160-190)§
#### fneq(&self, other: &Self) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#402-408)§
### implTryFrom<Value> for()

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#403)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#405-407)§
#### fntry_from(_:Value) ->Result<(), Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forAccessibleRole

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<AccessibleRole,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forAnimationDirection

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<AnimationDirection,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#254)§
### implTryFrom<Value> forBrush

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#254)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#254)§
#### fntry_from(v:Value) ->Result<Brush, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#416-425)§
### implTryFrom<Value> forColor

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#417)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#419-424)§
#### fntry_from(v:Value) ->Result<Color, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forColorScheme

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<ColorScheme,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#258)§
### implTryFrom<Value> forComponentFactory

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#258)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#258)§
#### fntry_from(v:Value) ->Result<ComponentFactory, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forDialogButtonRole

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<DialogButtonRole,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implTryFrom<Value> forDropEvent

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fntry_from(v:Value) ->Result<DropEvent, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#256)§
### implTryFrom<Value> forEasingCurve

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#256)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#256)§
#### fntry_from(v:Value) ->Result<EasingCurve, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implTryFrom<Value> forEdges

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fntry_from(v:Value) ->Result<Edges, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forEventResult

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<EventResult,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forFillRule

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<FillRule,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forFocusReason

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<FocusReason,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implTryFrom<Value> forFontMetrics

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fntry_from(v:Value) ->Result<FontMetrics, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#252)§
### implTryFrom<Value> forImage

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#252)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#252)§
#### fntry_from(v:Value) ->Result<Image, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forImageFit

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<ImageFit,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forImageHorizontalAlignment

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<ImageHorizontalAlignment,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forImageRendering

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<ImageRendering,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forImageTiling

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<ImageTiling,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forImageVerticalAlignment

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<ImageVerticalAlignment,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forInputType

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<InputType,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#386-394)§
### implTryFrom<Value> forInstant

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#387)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#388-393)§
#### fntry_from(v:Value) ->Result<Instant, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implTryFrom<Value> forKeyEvent

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fntry_from(v:Value) ->Result<KeyEvent, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implTryFrom<Value> forKeyboardModifiers

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fntry_from(v:Value) ->Result<KeyboardModifiers, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forLayoutAlignment

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<LayoutAlignment,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#344)§
### implTryFrom<Value> forLayoutInfo

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#344)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#344)§
#### fntry_from(v:Value) ->Result<LayoutInfo, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#433-442)§
### implTryFrom<Value> forLogicalLength

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#434)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#436-441)§
#### fntry_from(v:Value) ->Result<LogicalLength, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forLineCap

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<LineCap,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forLineJoin

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<LineJoin,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#519-550)§
### implTryFrom<Value> forLogicalEdges

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#520)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#522-549)§
#### fntry_from(v:Value) ->Result<LogicalEdges, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#346)§
### implTryFrom<Value> forLogicalPosition

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#346)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#346)§
#### fntry_from(v:Value) ->Result<LogicalPosition, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implTryFrom<Value> forMenuEntry

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fntry_from(v:Value) ->Result<MenuEntry, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#561-580)§
### impl<T:TryFrom<Value> +Default+ 'static>TryFrom<Value> forModelRc<T>

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#562)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#564-579)§
#### fntry_from(v:Value) ->Result<ModelRc<T>, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forMouseCursor

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<MouseCursor,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forOperatingSystemType

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<OperatingSystemType,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forOrientation

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<Orientation,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#255)§
### implTryFrom<Value> forPathData

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#255)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#255)§
#### fntry_from(v:Value) ->Result<PathData, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forPathEvent

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<PathEvent,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#453-474)§
### implTryFrom<Value> forLogicalPoint

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#454)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#456-473)§
#### fntry_from(v:Value) ->Result<LogicalPoint, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#345)§
### implTryFrom<Value> forPoint

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#345)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#345)§
#### fntry_from(v:Value) ->Result<Point, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implTryFrom<Value> forPointerEvent

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fntry_from(v:Value) ->Result<PointerEvent, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forPointerEventButton

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<PointerEventButton,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forPointerEventKind

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<PointerEventKind,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implTryFrom<Value> forPointerScrollEvent

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fntry_from(v:Value) ->Result<PointerScrollEvent, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forPopupClosePolicy

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<PopupClosePolicy,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forScrollBarPolicy

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<ScrollBarPolicy,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#250)§
### implTryFrom<Value> forSharedString

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#250)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#250)§
#### fntry_from(v:Value) ->Result<SharedString, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#257)§
### implTryFrom<Value> forSharedVector<f32>

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#257)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#257)§
#### fntry_from(v:Value) ->Result<SharedVector<f32>, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#260)§
### implTryFrom<Value> forSharedVector<u16>

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#260)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#260)§
#### fntry_from(v:Value) ->Result<SharedVector<u16>, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#485-506)§
### implTryFrom<Value> forLogicalSize

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#486)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#488-505)§
#### fntry_from(v:Value) ->Result<LogicalSize, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forSortOrder

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<SortOrder,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forStandardButtonKind

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<StandardButtonKind,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implTryFrom<Value> forStandardListViewItem

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fntry_from(v:Value) ->Result<StandardListViewItem, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implTryFrom<Value> forStateInfo

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fntry_from(v:Value) ->Result<StateInfo, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#253)§
### implTryFrom<Value> forStruct

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#253)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#253)§
#### fntry_from(v:Value) ->Result<Struct, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#259)§
### implTryFrom<Value> forStyledText

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#259)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#259)§
#### fntry_from(v:Value) ->Result<StyledText, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
### implTryFrom<Value> forTableColumn

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#348)§
#### fntry_from(v:Value) ->Result<TableColumn, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forTextHorizontalAlignment

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<TextHorizontalAlignment,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forTextOverflow

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<TextOverflow,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forTextStrokeStyle

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<TextStrokeStyle,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forTextVerticalAlignment

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<TextVerticalAlignment,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
### implTryFrom<Value> forTextWrap

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### typeError=()

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#379)§
#### fntry_from(v:Value) ->Result<TextWrap,()>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#251)§
### implTryFrom<Value> forbool

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#251)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#251)§
#### fntry_from(v:Value) ->Result<bool, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implTryFrom<Value> forf32

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fntry_from(v:Value) ->Result<f32, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implTryFrom<Value> forf64

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fntry_from(v:Value) ->Result<f64, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implTryFrom<Value> fori32

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fntry_from(v:Value) ->Result<i32, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implTryFrom<Value> fori64

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fntry_from(v:Value) ->Result<i64, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implTryFrom<Value> forisize

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fntry_from(v:Value) ->Result<isize, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implTryFrom<Value> foru32

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fntry_from(v:Value) ->Result<u32, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implTryFrom<Value> foru64

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fntry_from(v:Value) ->Result<u64, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
### implTryFrom<Value> forusize

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### typeError=Value

The type returned in the event of a conversion error.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#249)§
#### fntry_from(v:Value) ->Result<usize, Self::Error>

Performs the conversion.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/eval.rs.html#120)§
### implValueTypeforValue

## Auto Trait Implementations§

§
### implFreezeforValue

§
### impl !RefUnwindSafeforValue

§
### impl !SendforValue

§
### impl !SyncforValue

§
### implUnpinforValue

§
### implUnsafeUnpinforValue

§
### impl !UnwindSafeforValue

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)§
### impl<T>NoneValuefor Twhere
    T:Default,

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)§
#### typeNoneType= T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)§
#### fnnull_value() -> T

The none-equivalent value.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)[Source](https://docs.rs/parley/0.7.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)§
### impl<T>Brushfor Twhere
    T:Clone+PartialEq+Default+Debug,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumValueTypeCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#37-60)
```
#[non_exhaustive]pub enum ValueType {
    Void,
    Number,
    String,
    Bool,
    Model,
    Struct,
    Brush,
    Image,
}
```

Expand description
This enum represents the different public variants of theValueenum, without
the contained values.

## Variants (Non-exhaustive)§

This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
### Void

The variant that expresses the non-type. This is the default.

§
### Number

Anintor afloat(this is also used for unit based type such aslengthorangle)

§
### String

Correspond to thestringtype in .slint

§
### Bool

Correspond to thebooltype in .slint

§
### Model

A model (that includes array in .slint)

§
### Struct

An object

§
### Brush

Correspond tobrushorcolortype in .slint.  For color, this is then aBrush::SolidColor

§
### Image

Correspond toimagetype in .slint.

## Trait Implementations§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#34)§
### implCloneforValueType

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#34)§
#### fnclone(&self) ->ValueType

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#34)§
### implDebugforValueType

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#34)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#62-85)§
### implFrom<Type> forValueType

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#63-84)§
#### fnfrom(ty:LangType) -> Self

Converts to this type from the input type.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#34)§
### implPartialEqforValueType

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#34)§
#### fneq(&self, other: &ValueType) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#34)§
### implCopyforValueType

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#34)§
### implStructuralPartialEqforValueType

## Auto Trait Implementations§

§
### implFreezeforValueType

§
### implRefUnwindSafeforValueType

§
### implSendforValueType

§
### implSyncforValueType

§
### implUnpinforValueType

§
### implUnsafeUnpinforValueType

§
### implUnwindSafeforValueType

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumWindowPositionCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#107)
```
pub enum WindowPosition {
    Physical(PhysicalPosition),
    Logical(LogicalPosition),
}
```

Expand description
The position of the window in either physical or logical pixels. This is used
withWindow::set_position.

## Variants§

§
### Physical(PhysicalPosition)

The position in physical pixels.

§
### Logical(LogicalPosition)

The position in logical pixels.

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#114)§
### implWindowPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#116)
#### pub fnto_physical(&self, scale_factor:f32) ->PhysicalPosition

Turn theWindowPositioninto aPhysicalPosition.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
### implCloneforWindowPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
#### fnclone(&self) ->WindowPosition

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
### implDebugforWindowPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
### implFrom<LogicalPosition> forWindowPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
#### fnfrom(value:LogicalPosition) ->WindowPosition

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
### implFrom<PhysicalPosition> forWindowPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
#### fnfrom(value:PhysicalPosition) ->WindowPosition

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
### implPartialEqforWindowPosition

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
#### fneq(&self, other: &WindowPosition) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#106)§
### implStructuralPartialEqforWindowPosition

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# EnumWindowSizeCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#209)
```
pub enum WindowSize {
    Physical(PhysicalSize),
    Logical(LogicalSize),
}
```

Expand description
The size of a window represented in either physical or logical pixels. This is used
withWindow::set_size.

## Variants§

§
### Physical(PhysicalSize)

The size in physical pixels.

§
### Logical(LogicalSize)

The size in logical screen pixels.

## Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#216)§
### implWindowSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#218)
#### pub fnto_physical(&self, scale_factor:f32) ->PhysicalSize

Turn theWindowSizeinto aPhysicalSize.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#226)
#### pub fnto_logical(&self, scale_factor:f32) ->LogicalSize

Turn theWindowSizeinto aLogicalSize.

## Trait Implementations§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
### implCloneforWindowSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
#### fnclone(&self) ->WindowSize

Returns a duplicate of the value.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)§
#### fnclone_from(&mut self, source: &Self)

Performs copy-assignment from
`source`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
### implDebugforWindowSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
#### fnfmt(&self, f: &mutFormatter<'_>) ->Result<(),Error>

Formats the value using the given formatter.
[Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
### implFrom<LogicalSize> forWindowSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
#### fnfrom(value:LogicalSize) ->WindowSize

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
### implFrom<PhysicalSize> forWindowSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
#### fnfrom(value:PhysicalSize) ->WindowSize

Converts to this type from the input type.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
### implPartialEqforWindowSize

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
#### fneq(&self, other: &WindowSize) ->bool

Tests for
`self`and
`other`values to be equal, and is used by
`==`.
1.0.0
·
[Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)§
#### fnne(&self, other:&Rhs) ->bool

Tests for
`!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.
[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#208)§
### implStructuralPartialEqforWindowSize

## Auto Trait Implementations§

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

## Blanket Implementations§

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#138)§
### impl<T>Anyfor Twhere
    T: 'static + ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#139)§
#### fntype_id(&self) ->TypeId

Gets the
`TypeId`of
`self`.
[Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)§
### impl<T>Borrow<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)§
#### fnborrow(&self) ->&T

Immutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)§
### impl<T>BorrowMut<T> for Twhere
    T: ?Sized,

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)§
#### fnborrow_mut(&mut self) ->&mut T

Mutably borrows from an owned value.
[Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#547)§
### impl<T>CloneToUninitfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#549)§
#### unsafe fnclone_to_uninit(&self, dest:*mutu8)

🔬
This is a nightly-only experimental API. (
`clone_to_uninit`)
Performs copy-assignment from
`self`to
`dest`.
[Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)§
### impl<T>Downcastfor Twhere
    T:Any,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)§
#### fninto_any(self:Box<T>) ->Box<dynAny>

Convert
`Box<dyn Trait>`(where
`Trait: Downcast`) to
`Box<dyn Any>`.
`Box<dyn Any>`can then be further
`downcast`into
`Box<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)§
#### fninto_any_rc(self:Rc<T>) ->Rc<dynAny>

Convert
`Rc<Trait>`(where
`Trait: Downcast`) to
`Rc<Any>`.
`Rc<Any>`can then be further
`downcast`into
`Rc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)§
#### fnas_any(&self) -> &(dynAny+ 'static)

Convert
`&Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&Any`’s vtable from
`&Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)§
#### fnas_any_mut(&mut self) -> &mut (dynAny+ 'static)

Convert
`&mut Trait`(where
`Trait: Downcast`) to
`&Any`. This is needed since Rust cannot generate
`&mut Any`’s vtable from
`&mut Trait`’s.
[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)§
### impl<T>DowncastSyncfor Twhere
    T:Any+Send+Sync,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)§
#### fninto_any_arc(self:Arc<T>) ->Arc<dynAny+Send+Sync>

Convert
`Arc<Trait>`(where
`Trait: Downcast`) to
`Arc<Any>`.
`Arc<Any>`can then be further
`downcast`into
`Arc<ConcreteType>`where
`ConcreteType`implements
`Trait`.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#785)§
### impl<T>From<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#788)§
#### fnfrom(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)§
### impl<T>Instrumentfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)§
#### fninstrument(self, span:Span) ->Instrumented<Self>

Instruments this type with the provided
[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.instrument)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)§
#### fnin_current_span(self) ->Instrumented<Self>

Instruments this type with the
[current](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html#method.current)[Span](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/span/struct.Span.html), returning an
`Instrumented`wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.Instrument.html#method.in_current_span)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#767-769)§
### impl<T, U>Into<U> for Twhere
    U:From<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#777)§
#### fninto(self) -> U

CallsU::from(self).

That is, this conversion is whatever the implementation ofFrom<T> for Uchooses to do.

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)§
### impl<T>IntoEitherfor T

[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)§
#### fninto_either(self, into_left:bool) ->Either<Self, Self>

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left`is
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)[Source](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)§
#### fninto_either_with<F>(self, into_left: F) ->Either<Self, Self>where
    F:FnOnce(&Self) ->bool,

Converts
`self`into a
[Left](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)if
`into_left(&self)`returns
`true`. Converts
`self`into a
[Right](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right)variant of
[Either<Self, Self>](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html)otherwise.
[Read more](https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)§
### impl<T>ToOwnedfor Twhere
    T:Clone,

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)§
#### typeOwned= T

The resulting type after obtaining ownership.
[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)§
#### fnto_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)§
#### fnclone_into(&self, target:&mut T)

Uses borrowed data to replace owned data, usually by cloning.
[Read more](https://doc.rust-lang.org/nightly/alloc/borrow/trait.ToOwned.html#method.clone_into)[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#827-829)§
### impl<T, U>TryFrom<U> for Twhere
    U:Into<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#831)§
#### typeError=Infallible

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#834)§
#### fntry_from(value: U) ->Result<T, <T asTryFrom<U>>::Error>

Performs the conversion.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#811-813)§
### impl<T, U>TryInto<U> for Twhere
    U:TryFrom<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#815)§
#### typeError= <U asTryFrom<T>>::Error

The type returned in the event of a conversion error.
[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#818)§
#### fntry_into(self) ->Result<U, <U asTryFrom<T>>::Error>

Performs the conversion.
[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)§
### impl<T>WithSubscriberfor T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)§
#### fnwith_subscriber<S>(self, subscriber: S) ->WithDispatch<Self>where
    S:Into<Dispatch>,

Attaches the provided
[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)§
#### fnwith_current_subscriber(self) ->WithDispatch<Self>

Attaches the current
[default](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/dispatcher/index.html#setting-the-default-subscriber)[Subscriber](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/subscriber/trait.Subscriber.html)to this type, returning a
[WithDispatch](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/struct.WithDispatch.html)wrapper.
[Read more](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# TraitComponentHandleCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#817)
```
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

Expand description
This trait describes the common public API of a strongly referenced Slint component.
It allows creating strongly-referenced clones, a conversion into/ a weak pointer as well
as other convenience functions.

This trait is implemented by thegenerated component

## Required Methods§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#822-824)
#### fnas_weak(&self) ->Weak<Self>where
    Self:Sized,

Returns a new weak pointer.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#828)
#### fnclone_strong(&self) -> Self

Returns a clone of this handle that’s a strong reference.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#841)
#### fnshow(&self) ->Result<(),PlatformError>

Convenience function forcrate::Window::show().
This shows the window on the screen and maintains an extra strong reference while
the window is visible. To react to events from the windowing system, such as draw
requests or mouse/touch input, it is still necessary to spin the event loop,
usingcrate::run_event_loop.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#846)
#### fnhide(&self) ->Result<(),PlatformError>

Convenience function forcrate::Window::hide().
Hides the window, so that it is not visible anymore. The additional strong reference
on the associated component, that was created when show() was called, is dropped.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#851)
#### fnwindow(&self) -> &Window

Returns the Window associated with this component. The window API can be used
to control different aspects of the integration into the windowing system,
such as the position on the screen.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#855)
#### fnrun(&self) ->Result<(),PlatformError>

This is a convenience function that first callsSelf::show, followed bycrate::run_event_loop()andSelf::hide.

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#859-861)
#### fnglobal<'a, T>(&'a self) -> Twhere
    T:Global<'a, Self>,
    Self:Sized,

This function provides access to instances of global singletons exported in.slint.
SeeGlobalfor an example how to export and access globals from.slintmarkup.

## Dyn Compatibility§

This trait isnotdyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

## Implementors§

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1683-1725)§
### implComponentHandleforComponentInstance

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# TraitGlobalCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#807)
```
pub trait Global<'a, Component> {
    // Required method
    fn get(component: &'a Component) -> Self;
}
```

Expand description
This trait is used to obtain references to global singletons exported in.slintmarkup. Alternatively, you can useComponentHandle::globalto obtain access.

This trait is implemented by the compiler for each global singleton that’s exported.

## §Example

The following example of.slintmarkup defines a global singleton calledPalette, exports
it and modifies it from Rust code:

```
slint::slint!{
export global Palette {
    in property<color> foreground-color;
    in property<color> background-color;
}

export component App inherits Window {
   background: Palette.background-color;
   Text {
      text: "Hello";
      color: Palette.foreground-color;
   }
   // ...
}
}
let app = App::new().unwrap();
app.global::<Palette>().set_background_color(slint::Color::from_rgb_u8(0, 0, 0));

// alternate way to access the global singleton:
Palette::get(&app).set_foreground_color(slint::Color::from_rgb_u8(255, 255, 255));
```

See also thelanguage documentation for global singletonsfor more information.

Note:Only globals that are exported or re-exported from the main .slint file will
be exposed in the API

## Required Methods§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#809)
#### fnget(component:&'a Component) -> Self

Returns a reference that’s tied to the life time of the provided component.

## Dyn Compatibility§

This trait isnotdyn compatible.

In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.

## Implementors§

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# TraitToSharedStringCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#317)
```
pub trait ToSharedString {
    // Required method
    fn to_shared_string(&self) -> SharedString;
}
```

Expand description
A trait for converting a value to aSharedString.

This trait is automatically implemented for any type which implements theDisplaytrait as long as the trait is in scope.
As such,ToSharedStringshouldn’t be implemented directly:Displayshould be implemented instead, and you get theToSharedStringimplementation for free.

## Required Methods§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#319)
#### fnto_shared_string(&self) ->SharedString

Converts the given value to aSharedString.

## Implementors§

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#322-324)§
### impl<T>ToSharedStringfor Twhere
    T:Display+ ?Sized,

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# Functioninvoke_from_event_loopCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1039)
```
pub fn invoke_from_event_loop(
    func: impl FnOnce() + Send + 'static,
) -> Result<(), EventLoopError>
```

Expand description
Adds the specified function to an internal queue, notifies the event loop to wake up.
Once woken up, any queued up functors will be invoked.

This function is thread-safe and can be called from any thread, including the one
running the event loop. The provided functors will only be invoked from the thread
that started the event loop.

You can use this to set properties or use any other Slint APIs from other threads,
by collecting the code in a functor and queuing it up for invocation within the event loop.

If you want to capture non-Send types to run in the next event loop iteration,
you can use theslint::spawn_localfunction instead.

See alsoWeak::upgrade_in_event_loop.

## §Example

```
slint::slint! { export component MyApp inherits Window { in property <int> foo; /* ... */ } }
let handle = MyApp::new().unwrap();
let handle_weak = handle.as_weak();
let thread = std::thread::spawn(move || {
    // ... Do some computation in the thread
    let foo = 42;
     // now forward the data to the main thread using invoke_from_event_loop
    let handle_copy = handle_weak.clone();
    slint::invoke_from_event_loop(move || handle_copy.unwrap().set_foo(foo));
});
handle.run().unwrap();
```

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# Functionprint_diagnosticsCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1358-1364)
```
pub fn print_diagnostics(diagnostics: &[Diagnostic])
```

Available on
**crate featuredisplay-diagnostics**only.
Expand description
Print the diagnostics to stderr

The diagnostics are printed in the same style as rustc errors

This function is available when thedisplay-diagnosticsis enabled.

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# Functionquit_event_loopCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1056)
```
pub fn quit_event_loop() -> Result<(), EventLoopError>
```

Expand description
Schedules the main event loop for termination. This function is meant
to be called from callbacks triggered by the UI. After calling the function,
it will return immediately and once control is passed back to the event loop,
the initial call toslint::run_event_loop()will return.

This function can be called from any thread

Any previously queued events may or may not be processed before the loop terminates.
This is platform dependent behaviour.

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# Functionrun_event_loopCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1784-1786)
```
pub fn run_event_loop() -> Result<(), PlatformError>
```

Expand description
Enters the main event loop. This is necessary in order to receive
events from the windowing system in order to render to the screen
and react to user input.

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# Functionset_xdg_app_idCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/api.rs.html#1190)
```
pub fn set_xdg_app_id(
    app_id: impl Into<SharedString>,
) -> Result<(), PlatformError>
```

Expand description
Sets the application id for use on Wayland or X11 withxdgcompliant window managers. This must be set before the window is shown, and has only an effect on Wayland or X11.

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# Functionspawn_localCopy item path

[Source](https://docs.rs/slint-interpreter/1.15.1/src/slint_interpreter/api.rs.html#1791-1794)
```
pub fn spawn_local<F: Future + 'static>(
    fut: F,
) -> Result<JoinHandle<F::Output>, EventLoopError>
```

Expand description
Spawns aFutureto execute in the Slint event loop.

See the documentation ofslint::spawn_local()for more info

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# MacroformatCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/string.rs.html#25)
```
macro_rules! format {
    ($($arg:tt)*) => { ... };
}
```

Expand description
This macro is the same asstd::format!, but it returns aSharedStringinstead.

#### §Example

```
let s : slint::SharedString = slint::format!("Hello {}", "world");
assert_eq!(s, slint::SharedString::from("Hello world"));
```

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/index.html)
# ModuletestingCopy item path

[Source](https://docs.rs/slint-interpreter/src/slint_interpreter/api.rs.html#1798)Expand description
This module contains a few functions used by the tests

## Functions§

[send_keyboard_char](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/fn.send_keyboard_char.html)Wrapper around
[i_slint_core::tests::slint_send_keyboard_char](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/i_slint_core/tests/fn.slint_send_keyboard_char.html)[send_keyboard_string_sequence](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/fn.send_keyboard_string_sequence.html)Wrapper around
[i_slint_core::tests::send_keyboard_string_sequence](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/i_slint_core/tests/fn.send_keyboard_string_sequence.html)[send_mouse_click](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/fn.send_mouse_click.html)Wrapper around
[i_slint_core::tests::slint_send_mouse_click](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/i_slint_core/tests/fn.slint_send_mouse_click.html)

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# Type AliasRgb8PixelCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#150)
```
pub type Rgb8Pixel = Rgb<u8>;
```

Expand description
Convenience alias for a pixel with three color channels (red, green and blue), each
encoded as u8.

## Aliased Type§

```
#[repr(C)]pub struct Rgb8Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
```

## Fields§

§`r:u8`Red Component

§`g:u8`Green Component

§`b:u8`Blue Component

[🔝 Наверх](#оглавление)
[slint_interpreter](https://docs.rs/slint-interpreter/1.15.1/slint_interpreter/index.html)
# Type AliasRgba8PixelCopy item path

[Source](https://docs.rs/i-slint-core/1.15.1/x86_64-unknown-linux-gnu/src/i_slint_core/graphics/image.rs.html#153)
```
pub type Rgba8Pixel = Rgba<u8>;
```

Expand description
Convenience alias for a pixel with four color channels (red, green, blue and alpha), each
encoded as u8.

## Aliased Type§

```
#[repr(C)]pub struct Rgba8Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
```

## Fields§

§`r:u8`Red Component

§`g:u8`Green Component

§`b:u8`Blue Component

§`a:u8`Alpha Component

[🔝 Наверх](#оглавление)

---

> ✅ Успешно скачано: 54/54 страниц
