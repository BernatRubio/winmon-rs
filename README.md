# winmon-rs

A small Windows system monitor written in Rust. It renders live CPU and RAM
utilisation as icons in the notification area (system tray), drawing the
percentage digits onto each icon at runtime.

The application has no main window and no configuration file. It runs in the
background and communicates only through its tray icons.

## Features

- CPU usage sampled once per second from `GetSystemTimes` and displayed as a
  percentage on a tray icon labelled `CPU`.
- Physical memory load sampled once per second from `GlobalMemoryStatusEx` and
  displayed as a percentage on a tray icon labelled `RAM`.
- Icons are generated at runtime: a 32x32 RGBA bitmap is drawn with the metric
  label and the current value, then converted to an `HICON` through
  `CreateIconIndirect`. Generated icons are cached per value, so repeated
  percentages reuse the same icon handle.
- Optional launch at login, implemented through the per-user `Run` registry key.
- No runtime dependencies beyond the Windows API and the fonts noted below.

## Requirements

- Windows. The code calls Win32 APIs directly and declares
  `#![windows_subsystem = "windows"]`; it does not target other platforms.
- A Rust toolchain supporting edition 2024 (Rust 1.85 or newer).
- Segoe UI fonts present at `%SystemRoot%\Fonts\segoeui.ttf` and
  `%SystemRoot%\Fonts\segoeuib.ttf`. These two files are embedded into the
  binary at compile time with `include_bytes!`, so a build fails if they are
  missing.

## Building

```
git clone https://github.com/BernatRubio/winmon-rs.git
cd winmon-rs
cargo build --release
```

The resulting binary is `target/release/winmon-rs.exe`.

Because the fonts are embedded at compile time, cross-compiling from a
non-Windows host is not expected to work without supplying those files.

## Running

```
cargo run --release
```

Or launch the built executable directly. No window appears; two icons are added
to the notification area, one for CPU and one for RAM. Each icon is updated
once per second.

## Command-line arguments

| Argument           | Effect                                                                    |
| ------------------ | ------------------------------------------------------------------------- |
| *(none)*           | Starts the monitor                                                        |
| `add-autostart`    | Registers the executable for launch at login                              |
| `remove-autostart` | Removes the launch-at-login registration                                  |

Any unrecognised argument falls through to the default behaviour and starts the
monitor.

Both autostart commands write to
`HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run` under the
value name `winmon-rs`, so no administrator rights are required.

```
winmon-rs.exe add-autostart
winmon-rs.exe remove-autostart
```

## Project layout

```
src/
  main.rs              Entry point and command-line dispatch
  app.rs               Startup, thread orchestration, autostart registry helpers
  window.rs            Minimal Win32 window wrapper: class registration,
                       window creation, message loop, window procedure
  utils.rs             FILETIME to u64 conversion helper
  metrics/
    mod.rs             Metric trait and the generic spawn_metric runner
    cpu.rs             CPU sampling and percentage calculation
    ram.rs             Memory sampling
  tray/
    mod.rs             Module declarations
    tray.rs            Tray icon lifecycle (Shell_NotifyIconW) and metric IDs
    icon.rs            Runtime icon rendering and HICON construction
```

## How it works

Each metric implements the `Metric` trait, which supplies a window class name
and a sampling task. `spawn_metric` runs that metric on a dedicated thread: the
thread registers a window class, creates a window, starts the sampling loop on a
child thread, and then pumps messages. The window is never shown; it exists only
so the tray icon has an owner window to deliver messages to.

CPU usage is computed from the difference between two consecutive snapshots of
idle, kernel, and user time. Kernel time includes idle time on Windows, so total
time is kernel plus user and used time is total minus idle. The percentage is
calculated with integer arithmetic and rounded to the nearest whole number. The
first sample after startup is discarded, since a delta requires two readings.

Memory load is read directly from the `dwMemoryLoad` field of `MEMORYSTATUSEX`,
which is already expressed as a percentage.

Sampling intervals are fixed at one second in the source.

## Dependencies

| Crate       | Version | Purpose                                        |
| ----------- | ------- | ---------------------------------------------- |
| `windows`   | 0.62.2  | Win32 bindings (GDI, system information, threading, shell, windowing) |
| `winreg`    | 0.56.0  | Registry access for the autostart entry         |
| `ab_glyph`  | 0.2.32  | Font loading and glyph rasterisation            |
| `image`     | 0.25.10 | RGBA image buffer for icon rendering            |
| `imageproc` | 0.26.2 | Text drawing onto the icon bitmap               |

## Known limitations

- Fixed one-second sampling interval; there is no way to change it.
- The tray tooltip is set once to the metric label (`CPU`, `RAM`) and is not
  updated with the current value, so the percentage is visible only in the icon
  itself.
- No tray context menu and no quit command are present in the current source, so
  stopping the process requires Task Manager or a similar tool.
- No installer, no code signing, and no packaged release artifacts.
- Metrics are limited to CPU and RAM.
