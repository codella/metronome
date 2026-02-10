# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```bash
cargo build
cargo run
cargo run -- --bpm 140 --time-signature 3/4
```

No tests, linter config, or CI exist yet.

## Architecture

A real-time TUI metronome with audio feedback. Four modules, one data flow:

**main.rs** — Entry point. Parses CLI args (clap), creates `AudioEngine` and `App`, runs the event loop at ~62.5 FPS (16ms poll). Calls `app.tick(audio)` then `ui::render(frame, app)` each frame.

**app.rs** — All application state and logic. `App` struct owns BPM, time signature, beat position, play state, theme index. `tick()` checks elapsed time against `beat_duration()` and fires `audio.play_click()`. `handle_key()` dispatches all keyboard input. Tap tempo averages the last 8 taps within a 3-second window.

**audio.rs** — Sound generation via rodio. `AudioEngine::play_click(accent)` creates a short (30ms) sine wave: 880 Hz for the downbeat accent, 440 Hz for other beats. Uses a custom `EnvelopedSine` source with 5ms fade in/out at 44.1 kHz.

**ui.rs** — Renders the TUI with ratatui. Each render function (`render_bpm`, `render_beats`, `render_status`, `render_controls`, `render_help_modal`) reads from `App` and uses `app.theme()` for all colors.

**theme.rs** — Pure data. A `Theme` struct with 11 semantic color fields and a `THEMES` constant array of 5 themes (Default, Ocean, Ember, Forest, Neon). No logic.

## Key Patterns

- Colors are never hardcoded in ui.rs — always accessed via `app.theme().field_name`
- The downbeat (beat 0) always renders in `beat_accent` color with bold, even when inactive
- Time signatures are a fixed array of 6 options cycled with modular arithmetic
- `App` state is the single source of truth; ui.rs is a pure function of that state
