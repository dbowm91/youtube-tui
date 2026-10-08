# Installation

The YouTube TUI supports Linux, macOS, and Windows.

## Install from Crates.io (recommended)

Crates.io is a repository for programs written in Rust, and the YouTube TUI is available <a href="https://crates.io/crates/youtube-tui" target=_blank>*there*</a>.

To install using this method, you will need `rustc` and `cargo` present. <a href="https://www.rust-lang.org/tools/install" target=_blank>*Here*</a>'s a tutorial on how to get them.

Now, run the following command:

```sh
cargo install youtube-tui
```

> To check and update all programs installed from Crates.io, you can use CLI tools like <a href="https://crates.io/crates/cargo-update" target=_blank>*cargo-update*</a>.

## AUR for Arch Linux (recommended)

The YouTube TUI is available in the AUR <a href="https://aur.archlinux.org/packages/youtube-tui-git" target=_blank>*here*</a>.

Use an <a href="https://aur.archlinux.org" target=_blank>AUR</a> helper like <a href="https://aur.archlinux.org/packages/yay" target=_blank>`yay`</a> to install.

```sh
yay -S youtube-tui # lastest crates.io release, recommended
yay -S youtube-tui-git # latest git version, potentially untested
yay -S youtube-tui-full-bin # default binary (out of date)
yay -S youtube-tui-nodefaults-bin # minimal binary (out of date)
```
## Installation on NixOS  (recommended)

Add the following Nix code to your NixOS Configuration

```nix 
environment.systemPackages = [
  pkgs.youtube-tui
];
```

##  Installation on NixOS **with flakes** (recommended)

```nix
{
    inputs.youtube-tui.url = "github:Siriusmart/youtube-tui";

    ...
}
```

## Build from source

Use the `cargo` command:

```sh
cargo install --git https://github.com/siriusmart/youtube-tui
```

### Confirm YouTube TUI has been installed

Run the following command in terminal:

```sh
youtube-tui
```

If installed correctly, a TUI should be launched. Press `q` to close the TUI.

## Installation on macOS

Install using `cargo` exactly as you would on Linux:

```sh
cargo install youtube-tui
```

Prerequisites (via [Homebrew](https://brew.sh)):

```sh
brew install mpv yt-dlp pkg-config
```

- `mpv` provides the `libmpv` dev library the default `mpv` feature links against, and doubles as the default external `video-player`.
- `yt-dlp` must be installed and on `PATH`: the TUI shells out to the configured `youtube-downloader` (`yt-dlp` by default) both for library downloads and to resolve watch URLs for the embedded player (see [`mpv`](#mpv-default)).
- `pkg-config` lets the build scripts locate `libmpv` (and OpenSSL, where applicable).

If your Homebrew `libmpv` has the wrong architecture (e.g. an Intel-only `/usr/local` install under Rosetta while building native `arm64`), point the build at a matching `libmpv` prefix with `MPV_LIB_DIR` instead — the directory is added to the link search path and baked into the binary as an `rpath`, so no runtime environment variables are needed (but the folder must stay in place):

```sh
MPV_LIB_DIR=/path/to/libmpv cargo install youtube-tui
```

Prebuilt self-contained `arm64` bundles are available from <a href="https://github.com/media-kit/libmpv-darwin-build/releases" target=_blank>`media-kit/libmpv-darwin-build`</a> — verify the checksum before extracting.

To build without the embedded mpv audio player:

```sh
cargo install youtube-tui --no-default-features -F sixel -F halfblock -F clipboard -F rustypipe
```

### macOS configuration

The default config directory on macOS is `~/Library/Application Support/youtube-tui/`. Thumbnails and the download library live under the same folder.

Default tools on macOS (fresh installs only — existing `main.yml` files are untouched):
- **Browser**: `open` (opens URLs in the default browser)
- **Terminal emulator**: empty — downloads run headless in the background. macOS has no single command that opens a visible terminal running a given command, so set `terminal-emulator` to your terminal's launcher (e.g. `xterm -e`) if you want visible download progress.
- **Shell**: `sh`
- **Save path**: `~/Library/Application Support/youtube-tui/saved/`

These can be changed in `main.yml`.

## Installation on Linux

Install using `cargo` (or your distro packages, e.g. the AUR entries above):

```sh
cargo install youtube-tui
```

System dependencies (Debian/Ubuntu names — adapt for your distro):

```sh
sudo apt install pkg-config libssl-dev libmpv-dev libxcb1-dev libx11-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev mpv yt-dlp
```

- `libmpv-dev` for the default `mpv` feature (or disable it and use an external player via the `video-player` config).
- `libssl-dev` for TLS (pulled in via `reqwest`); `pkg-config` for the build scripts to find both.
- `libxcb1-dev` + `libx11-dev` for the default `clipboard` feature.
- `mpv` and `yt-dlp` as the default external player and downloader (both must be on `PATH`).

`MPV_LIB_DIR` works on Linux too (see macOS above) if your `libmpv` lives in a custom prefix.

The default `terminal-emulator` is `konsole -e` (KDE); GNOME users should set it to e.g. `gnome-terminal --` in `main.yml`.

## Installation on Windows

Install using `cargo` exactly as you would on Linux:

```sh
cargo install youtube-tui
```

All default features work on Windows. On the first build, the `mpv` feature will automatically download the required `libmpv` dev library (~30MB) and place `libmpv-2.dll` next to the installed binary — no manual steps needed.

Prerequisites:
- [Rust / cargo](https://www.rust-lang.org/tools/install)
- [MSVC build tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022) ("Desktop development with C++" workload) — required to compile Rust on Windows
- Internet access on the first build (to download `libmpv`)

To build without the embedded mpv audio player (skips the download):

```sh
cargo install youtube-tui --no-default-features -F sixel -F halfblock -F clipboard -F rustypipe
```

### Windows configuration

The default config directory on Windows is `%APPDATA%\youtube-tui\`. Data files (history, thumbnails, etc.) are stored in `%LOCALAPPDATA%\youtube-tui\`.

Default tools on Windows:
- **Browser**: `explorer` (opens URLs in default browser)
- **Terminal emulator**: `cmd /c start cmd /k`
- **Shell**: `cmd`

These can be changed in `main.yml`.

## Features

The TUI has features that can be enabled/disabled when compiling.

```sh
cargo install youtube-tui # install with all default features
cargo install youtube-tui --no-default-features -F feature1 -F feature2 # install with only the specified features
cargo install youtube-tui --all-features # install with all features (even if not included in default)
```

### `halfblock` (default)

Display images through HalfBlocks, work best in terminals with TrueColour support.

### `sixel` (default)

Display images with Sixels, allows the display of images at full definition.

This uses a pure Rust Sixel encoder on all platforms (no native dependencies, no `libsixel` required). If the terminal does not support Sixels, images automatically fall back to the iTerm, Kitty, or HalfBlock renderers.

### `clipboard` (default)

Allows clipboard pasting in commands and search bar.

On Linux, requires <a href="https://xcb.freedesktop.org/" target=_blank>`libxcb`</a>. On Windows, uses native clipboard APIs (no extra dependencies).

### `mpv` (default)

Embedded audio player.

On Linux, requires <a href="https://mpv.io/" target=_blank>`libmpv`</a> to be installed (`libmpv-dev` on Debian/Ubuntu, `mpv` on Arch).

On Windows, the build system downloads `libmpv` automatically on first build and copies `libmpv-2.dll` next to the installed binary. No manual steps are required.

On macOS (Apple Silicon), the Homebrew `libmpv` may be Intel-only and unable to link into native `arm64` builds. In that case, download a prebuilt `arm64` `libmpv` + dependency bundle (e.g. from <a href="https://github.com/media-kit/libmpv-darwin-build/releases" target=_blank>`media-kit/libmpv-darwin-build`</a>) into a private prefix such as `~/.local/lib`, verify its checksum, and point the build at it (replace `<dir>` with the extracted folder):

```sh
MPV_LIB_DIR=~/.local/lib/<dir> cargo build
```

The directory is added to the link search path and baked into the binary as an `rpath`, so no environment variables are needed at runtime — but the `libmpv` folder must stay where it was at build time. Alternatively, build with `--target x86_64-apple-darwin` to use the Intel Homebrew `libmpv` under Rosetta, or disable the `mpv` feature and use an external player via the `video-player` config instead.

Embedded libmpv builds cannot run mpv's own YouTube resolver (no lua), so the TUI resolves YouTube/Invidious watch URLs to direct stream URLs with the configured `youtube-downloader` (`yt-dlp` by default, must be installed and on `PATH`) before handing them to the embedded player. Local files and direct media URLs play as-is.

### rustypipe (default)

Rustypipe backend for fetching video information, Rustypipe runs on your local machine, it's a bit like yt-dlp.

### invidious

invidious backend for fetching video information, requires a working Invidious instance with API enabled set in [`main.yml`](./config/main.md).

> You *must* enable at least one of the two backends.
