use crate::audio::AudioEngine;
use crate::theme::{Theme, THEMES};
use crossterm::event::{KeyCode, KeyEvent};
use std::time::{Duration, Instant};

const TIME_SIGNATURES: [(u8, u8); 6] = [(2, 4), (3, 4), (4, 4), (5, 4), (6, 8), (7, 8)];
const MIN_BPM: u16 = 20;
const MAX_BPM: u16 = 300;
const TAP_HISTORY: usize = 8;
/// Discard taps older than 3 seconds
const TAP_TIMEOUT_SECS: f64 = 3.0;

pub struct App {
    pub bpm: u16,
    pub time_signature: (u8, u8),
    pub playing: bool,
    pub current_beat: u8,
    last_beat_time: Instant,
    tap_times: Vec<Instant>,
    pub should_quit: bool,
    pub show_help: bool,
    ts_index: usize,
    pub theme_index: usize,
}

impl App {
    pub fn new(bpm: u16, time_signature: (u8, u8)) -> Self {
        let ts_index = TIME_SIGNATURES
            .iter()
            .position(|&ts| ts == time_signature)
            .unwrap_or(2); // default to 4/4

        Self {
            bpm: bpm.clamp(MIN_BPM, MAX_BPM),
            time_signature,
            playing: false,
            current_beat: 0,
            last_beat_time: Instant::now(),
            tap_times: Vec::new(),
            should_quit: false,
            show_help: false,
            ts_index,
            theme_index: 0,
        }
    }

    pub fn theme(&self) -> &'static Theme {
        &THEMES[self.theme_index]
    }

    pub fn beat_duration(&self) -> Duration {
        Duration::from_secs_f64(60.0 / self.bpm as f64)
    }

    pub fn tick(&mut self, audio: &AudioEngine) {
        if !self.playing {
            return;
        }

        let now = Instant::now();
        if now.duration_since(self.last_beat_time) >= self.beat_duration() {
            // Advance FIRST, then play — so current_beat matches the sounding beat
            self.current_beat = (self.current_beat + 1) % self.time_signature.0;
            let accent = self.current_beat == 0;
            audio.play_click(accent);
            self.last_beat_time = now;
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if self.show_help {
            match key.code {
                KeyCode::Esc | KeyCode::Char('?') => self.show_help = false,
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('?') => self.show_help = true,
            KeyCode::Char(' ') => self.toggle_play(),
            KeyCode::Up | KeyCode::Char('k') => self.adjust_bpm(1),
            KeyCode::Down | KeyCode::Char('j') => self.adjust_bpm(-1),
            KeyCode::Char('+') | KeyCode::Char('=') => self.adjust_bpm(10),
            KeyCode::Char('-') => self.adjust_bpm(-10),
            KeyCode::Char('t') => self.tap_tempo(),
            KeyCode::Char(']') => self.cycle_time_signature(1),
            KeyCode::Char('[') => self.cycle_time_signature(-1),
            KeyCode::Char('c') => self.cycle_theme(),
            _ => {}
        }
    }

    fn toggle_play(&mut self) {
        self.playing = !self.playing;
        if self.playing {
            // Set to last beat so the first advance in tick() wraps to beat 0
            self.current_beat = self.time_signature.0 - 1;
            // Set in the past so tick() fires on the very next loop iteration
            self.last_beat_time = Instant::now() - self.beat_duration();
        }
    }

    fn adjust_bpm(&mut self, delta: i16) {
        let new_bpm = (self.bpm as i16 + delta).clamp(MIN_BPM as i16, MAX_BPM as i16);
        self.bpm = new_bpm as u16;
    }

    fn tap_tempo(&mut self) {
        let now = Instant::now();

        // Discard old taps
        self.tap_times
            .retain(|&t| now.duration_since(t).as_secs_f64() < TAP_TIMEOUT_SECS);

        self.tap_times.push(now);

        // Keep only the last TAP_HISTORY taps
        if self.tap_times.len() > TAP_HISTORY {
            let drain_count = self.tap_times.len() - TAP_HISTORY;
            self.tap_times.drain(..drain_count);
        }

        // Need at least 2 taps to calculate BPM
        if self.tap_times.len() >= 2 {
            let count = (self.tap_times.len() - 1) as f64;
            let sum: f64 = self
                .tap_times
                .windows(2)
                .map(|w| w[1].duration_since(w[0]).as_secs_f64())
                .sum();

            let avg_interval = sum / count;
            if avg_interval > 0.0 {
                let new_bpm = (60.0 / avg_interval).round() as u16;
                self.bpm = new_bpm.clamp(MIN_BPM, MAX_BPM);
            }
        }
    }

    fn cycle_time_signature(&mut self, direction: i8) {
        let len = TIME_SIGNATURES.len() as i8;
        self.ts_index = ((self.ts_index as i8 + direction).rem_euclid(len)) as usize;
        self.time_signature = TIME_SIGNATURES[self.ts_index];
        // Set to last beat so the next tick() wraps to beat 0 (downbeat)
        self.current_beat = self.time_signature.0 - 1;
        if self.playing {
            // Force the next beat to fire immediately with the new time signature
            self.last_beat_time = Instant::now() - self.beat_duration();
        }
    }

    fn cycle_theme(&mut self) {
        self.theme_index = (self.theme_index + 1) % THEMES.len();
    }
}
