# SynthPlayer

A minimal, dark-themed local music player built with Rust, egui, and rodio.

## Features

- **Local music scanning** — recursively scans directories for MP3, FLAC, WAV, OGG, AAC, M4A, WMA, APE, ALAC
- **Metadata display** — title, artist, duration via lofty (ID3, Vorbis, FLAC tags)
- **Dark custom theme** — hand-drawn icons, no external assets
- **Custom progress bar** — click/drag to seek, themed to match
- **Format badges** — color-coded per file type
- **Search/filter** — by title or artist
- **Four play modes** — Normal, Shuffle, Repeat One, Repeat All
- **Session persistence** — remembers last music directory and volume

## Keyboard Shortcuts

| Key | Action |
|---|---|
| `Space` | Play / Pause |
| `←` `→` | Seek backward / forward 5s |
| `↑` `↓` | Select previous / next track |
| `Enter` | Play selected track |
| `Esc` | Stop |
| `Ctrl`+`←` | Previous track |
| `Ctrl`+`→` | Next track |

Shortcuts are disabled when a text field has focus.

## Build

```bash
# Requires Rust 1.85+ (edition 2024)
git clone https://github.com/<your-username>/synthplayer.git
cd synthplayer
cargo run --release
```

## Dependencies

| crate | purpose |
|---|---|
| eframe 0.31 | Window management, event loop |
| egui 0.31 | Immediate-mode GUI |
| rodio 0.20 | Audio playback |
| lofty 0.22 | Metadata parsing |
| walkdir 2 | Recursive directory scan |

## Architecture

```
src/
├── main.rs           # Entry point, window setup
├── core/
│   ├── scanner.rs    # File system scanning
│   ├── metadata.rs   # Audio tag reading
│   └── player.rs     # Playback control (rodio)
└── ui/
    └── app.rs        # egui interface (~900 lines)
```

Core modules have zero UI dependencies — swappable to TUI or Tauri.

## License

MIT
