<h1 align="center">rplayer</h1>

<br />

<img alt="rplayer preview" src="https://github.com/user-attachments/assets/bff25efc-c992-4f8a-a75e-d597c69410b5" />

<br />

`rplayer` is a minimal, fast, keyboard-driven terminal local music player. It is made for one thing: **playing your local music without getting in the way**

## Features

- Play local music
- Play / pause
- Previous / next track
- Seek forward / backward by 10 seconds
- Fully keyboard-driven controls
- Terminal UI built with [Ratatui](https://github.com/ratatui/ratatui)
- Audio playback with [Rodio](https://github.com/RustAudio/rodio)
- Linux MPRIS integration with [mpris-server](https://github.com/SeaDve/mpris-server)
- Pass your music directory directly through the CLI

## Usage

Pass your music directory as an argument:

```bash
rplayer ~/Music
```

That's it.

## Keybindings

`rplayer` is designed around keyboard-driven navigation.

| Key | Action |
| --- | --- |
| `Space` | Play / pause |
| `h` | Seek backward 10s |
| `l` | Seek forward 10s |
| `H` | Previous track |
| `L` | Next track |
| `q` | Quit |

The lowercase `h` / `l` keys are used for seeking, while uppercase `H` / `L` handle track navigation.

## Why?

I wanted a music player that is:

- simple
- local
- lightweight
- keyboard-driven
- no unnecessary features

## Building

As of now, you can only build it from source.

Clone the repository:

```bash
git clone https://github.com/wein1m/rplayer.git
cd rplayer
```

Build the release binary:

```bash
cargo build --release
```
## Customization

You can customize the TUI colors in [src/tui/theme.rs](https://github.com/wein1m/rplayer/blob/main/src/tui/theme.rs).

Keybindings can also be customized in [src/tui/keybindings.rs](https://github.com/wein1m/rplayer/blob/main/src/tui/keybindings.rs).

## Built With

- **Ratatui** (terminal UI)
- **Rodio** (audio playback)
- **Lofty** (audio metadata)
- **MPRIS / mpris-server** (Linux media player integration)




<p align="center">
  👻 EOF
</p>
