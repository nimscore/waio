<!-- LOGO -->
<h1>
<p align="center">
  <!-- <img src="https://github.com/waio/waio/assets/logo.png" alt="Logo" width="128"> -->
  <br>Waio
</h1>
  <p align="center">
    Native Wayland widgets and panels. One platform. No Electron.
    <br />
    Declarative UI via Slint, scripted with Lua, rendered through SHM.
    <br />
    <a href="#about">About</a>
    ·
    <a href="#download">Download</a>
    ·
    <a href="#documentation">Documentation</a>
    ·
    <a href="#contributing">Contributing</a>
    ·
    <a href="#roadmap">Roadmap</a>
    <br />
    <a href="README_RU.md">🇷🇺 Читать на русском</a>
  </p>
</p>

<br>

> [!WARNING]
>
> Waio is in **early alpha** and is **not ready** for daily use. APIs, file
> formats, and architecture are subject to breaking changes without notice.
> Crashes, rendering artifacts, and data loss are expected behavior at this
> stage. Use at your own risk.

## About

Waio is a platform for building native Wayland widgets and panels without
Electron, WebView, or any browser technology. While there are many ways to
put things on your desktop — conky, waybar, polybar, eww — they all force
you to choose between flexibility, performance, or simplicity. Waio provides
all three.

A single `.wa` file describes everything: declarative UI in Slint, logic
in Lua 5.4, configuration in YAML. The daemon compiles, renders, and
manages widgets through wlr-layer-shell and Wayland SHM buffers.

**`waio-daemon`** is a background process that owns the Wayland connection,
compiles Slint components at runtime, executes Lua scripts in a sandboxed
environment with a permission system, and communicates via JSON-RPC over a
Unix socket.

**`waio-cli`** is the command-line interface for loading, unloading,
updating, and inspecting widgets.

**`waio-shared`** is the shared Rust library with entity types, `.wa` file
parser, and IPC protocol. Used by both the daemon and CLI.

## Download

Build from source:

```bash
git clone https://github.com/nimscore/waio.git
cd waio
cargo build --release
```

Binaries: `target/release/waio-daemon`, `target/release/waio-cli`.

## Documentation

See the [documentation](https://github.com/nimscore/waio) (this repository) for
getting started guides, `.wa` file format reference, and architecture
details.

### Quick Start

```bash
# Start the daemon (in the background)
WAYLAND_DISPLAY=wayland-1 target/release/waio-daemon &

# Load a widget
target/release/waio-cli load examples/clock-bar/aura.wa
```

The widget appears on your screen. Edit `examples/clock-bar/aura.wa` and
use `waio-cli update` to reload it.

## Contributing and Developing

If you have any ideas, issues, etc. regarding Waio, or would like to
contribute through pull requests, please open an issue or submit a PR.
Those who would like to get involved with Waio's development should start
by reading the project structure and running the test suite.

```bash
cargo test --workspace      # run tests
cargo clippy --workspace    # lint
cargo fmt --all -- --check  # format check
```

## Roadmap and Status

Waio is in early alpha. The high-level plan for the project, in order:

|  #  | Step                                                    | Status |
| :-: | ------------------------------------------------------- | :----: |
|  1  | Core: Wayland lifecycle, per-widget rendering           |   ✅   |
|  2  | Lua scripting with calloop-based timers                 |   ✅   |
|  3  | Multi-monitor support, output auto-detection            |   ✅   |
|  4  | Security: Lua sandbox, permissions (fs, http)           |   ✅   |
|  5  | Persistence: save/restore after restart                 |   ✅   |
|  6  | Sub-component rendering (layered compositing)           |   ✅   |
|  7  | Production: integration tests, signal handling          |   ❌   |
|  8  | Desktop GUI app (Tauri + React) for widget management   |   ❌   |
|  9  | Ecosystem: package registry, themes, widget gallery     |   ❌   |

Additional details for each step in the big roadmap below:

#### Core: Wayland Lifecycle, Per-Widget Rendering

Waio implements the wlr-layer-shell protocol for creating panels and bars
that sit on desktop edges, and SHM (shared memory) buffers for rendering
pixels without GPU compositing. The surface lifecycle follows the correct
wlr-layer-shell sequence: create surface → commit (no buffer) → compositor
sends configure → ack configure → render first frame → frame callback loop.

Each widget gets its own Wayland surface and Slint software renderer.
Multi-layer widgets are supported: the daemon composites multiple Slint
sub-windows into a single ARGB8888 buffer and sends it to the surface via
`draw_precomposited()`.

#### Lua Scripting with Calloop-Based Timers

Lua 5.4 scripts run inside a sandboxed environment using mlua. Dangerous
functions (`os.execute`, `io.popen`, `dofile`, `loadfile`, `debug.*`) are
blocked at construction and at the globals level. Each widget runs in a
restricted `_ENV` that only exposes language primitives and explicitly
registered modules.

Timers are implemented through `calloop::channel`: timer threads sleep and
send `TimerFire` events through a channel, which the main event loop
receives and dispatches to Lua callbacks — all in the main thread, safely.
No polling, no `try_recv()` in the hot path.

#### Multi-Monitor Support, Output Auto-Detection

Widgets can be bound to a specific output (monitor) via the `output` field
in the `.wa` config. If not specified, the daemon auto-detects the first
connected output. Output tracking handles connect/disconnect events
gracefully.

#### Security: Lua Sandbox, Permissions (fs, http)

The Lua sandbox has two layers of defense:

1. `StdLib::ALL_SAFE` — blocks `debug` and C modules at construction
2. `sanitize_globals()` — nils dangerous functions (`os.execute`, `io.popen`, etc.)
3. Restricted `_ENV` — per-widget environment with only safe modules

Additional modules are gated behind explicit permissions:

- `waio.fs` — read-only file access with path traversal protection
  (`canonicalize()` + `starts_with()`). Requires `fs_read` permission.
- `waio.http` — HTTP GET/POST via ureq with URL validation, 10s timeout,
  10MB max response. Requires `http` permission.

#### Persistence: Save/Restore After Restart

Widgets are persisted to `data_dir/{uuid}.json` on load and automatically
restored when the daemon restarts. The configuration file at
`~/.config/waio/config.yaml` controls the data directory path and is shared
between the daemon and CLI.

#### Sub-Component Rendering (Layered Compositing)

Complex widgets can be built from multiple Slint components, each rendered
to its own buffer. The daemon composites these layers into a final buffer:
background first, then dynamic layers on top. Static layers (like
backgrounds) are only rendered once, while dynamic layers update when their
properties change. This approach eliminates ghosting artifacts that occur
with Slint's `ReusedBuffer` partial rendering.

#### Production: Integration Tests, Signal Handling

Currently in progress. The project has 21 unit tests covering timer
management, property store isolation, sandbox enforcement, and repository
CRUD operations. Integration tests and proper signal handling (SIGTERM from
systemd, SIGINT from Ctrl+C) are planned next.

#### Desktop GUI App (Tauri + React)

The long-term goal is a desktop application for managing widgets visually:
create, edit, position, preview, and load widgets without touching the
command line. Built with Tauri + React, communicating with the daemon over
the existing JSON-RPC IPC.

#### Ecosystem: Package Registry, Themes, Widget Gallery

The dream is a community where people share widgets, themes, and
configurations. A package registry would make it easy to discover and
install widgets from other users.

## License

MIT
