# Metronome

A real-time terminal metronome with audio feedback, built in Rust using [ratatui](https://github.com/ratatui/ratatui) and [rodio](https://github.com/RustAudio/rodio).

```
┌──────────────────────────────────────────────────────────┐
│                     ♩ METRONOME                          │
├──────────────────────────────────────────────────────────┤
│                                                          │
│                    120 BPM · Allegro                     │
│                    4/4  [Default]                        │
│                                                          │
│             [ ● ]  [ ○ ]  [ ○ ]  [ ○ ]                  │
│                                                          │
│                     ▶ Playing                            │
│                                                          │
│   Space: play/stop   ↑ ↓: BPM ±1   + -: BPM ±10       │
│   t: tap tempo  [ ]: time sig  c: theme  q: quit  ?: help│
└──────────────────────────────────────────────────────────┘
```

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or later)
- A working audio output device

### Build from source

```bash
git clone <repo-url>
cd metronome
cargo build --release
```

The binary will be at `target/release/metronome`.

### Run directly

```bash
cargo run
```

## Usage

```bash
# Default: 120 BPM in 4/4
metronome

# Set a custom BPM
metronome --bpm 140

# Set BPM and time signature
metronome --bpm 90 --time-signature 3/4

# Short flags
metronome -b 160 -t 6/8
```

### CLI Options

| Option | Short | Default | Description |
|---|---|---|---|
| `--bpm` | `-b` | 120 | Beats per minute (20-300) |
| `--time-signature` | `-t` | 4/4 | Time signature in N/D format |

## Controls

| Key | Action |
|---|---|
| `Space` | Play / stop |
| `Up` / `k` | Increase BPM by 1 |
| `Down` / `j` | Decrease BPM by 1 |
| `+` / `=` | Increase BPM by 10 |
| `-` | Decrease BPM by 10 |
| `t` | Tap tempo |
| `]` | Next time signature |
| `[` | Previous time signature |
| `c` | Cycle color theme |
| `?` | Toggle help overlay |
| `q` / `Esc` | Quit |

## Features

### Beat Visualization

Each beat in the current time signature is shown as a circle. The active beat displays as a filled circle (`●`) while inactive beats show as hollow (`○`). The downbeat (beat 1) is always highlighted in the accent color.

### Tempo Markings

The current BPM is shown alongside its classical tempo marking:

| BPM Range | Marking |
|---|---|
| < 40 | Grave |
| 40-59 | Largo |
| 60-72 | Adagio |
| 73-107 | Andante |
| 108-119 | Moderato |
| 120-155 | Allegro |
| 156-175 | Vivace |
| 176-199 | Presto |
| 200+ | Prestissimo |

### Tap Tempo

Press `t` repeatedly to set the BPM by tapping. The algorithm averages your last 8 taps within a 3-second window. Taps older than 3 seconds are discarded, so just keep tapping at the desired pace.

### Time Signatures

Six time signatures are available, cycled with `[` and `]`:

**2/4** &middot; **3/4** &middot; **4/4** &middot; **5/4** &middot; **6/8** &middot; **7/8**

### Audio

The metronome produces short sine-wave clicks:

- **Downbeat (accent):** 880 Hz
- **Other beats:** 440 Hz

Each click is 30 ms with a 5 ms fade-in/out envelope to avoid pops.

### Color Themes

Press `c` to cycle through 5 built-in themes:

- **Default** — Cyan and yellow
- **Ocean** — Blue and cyan
- **Ember** — Red and orange
- **Forest** — Green tones
- **Neon** — Magenta and cyan

## Dependencies

- [ratatui](https://crates.io/crates/ratatui) — Terminal UI framework
- [crossterm](https://crates.io/crates/crossterm) — Terminal event handling
- [rodio](https://crates.io/crates/rodio) — Audio playback
- [clap](https://crates.io/crates/clap) — CLI argument parsing
- [color-eyre](https://crates.io/crates/color-eyre) — Error reporting

## License

See [LICENSE](LICENSE) for details.
