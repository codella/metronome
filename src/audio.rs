use rodio::source::Source;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::time::Duration;

const ACCENT_FREQ: f32 = 880.0;
const NORMAL_FREQ: f32 = 440.0;
const CLICK_DURATION_MS: u64 = 30;
const FADE_MS: u64 = 5;
const SAMPLE_RATE: u32 = 44100;

/// A sine wave source with a simple fade-in/fade-out envelope.
struct EnvelopedSine {
    freq: f32,
    total_samples: usize,
    current_sample: usize,
    fade_samples: usize,
}

impl EnvelopedSine {
    fn new(freq: f32, duration_ms: u64, fade_ms: u64) -> Self {
        let total_samples = (SAMPLE_RATE as u64 * duration_ms / 1000) as usize;
        let fade_samples = (SAMPLE_RATE as u64 * fade_ms / 1000) as usize;
        Self {
            freq,
            total_samples,
            current_sample: 0,
            fade_samples,
        }
    }
}

impl Iterator for EnvelopedSine {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.current_sample >= self.total_samples {
            return None;
        }

        let t = self.current_sample as f32 / SAMPLE_RATE as f32;
        let sine = (2.0 * std::f32::consts::PI * self.freq * t).sin();

        // Envelope: fade in at start, fade out at end
        let envelope = if self.current_sample < self.fade_samples {
            self.current_sample as f32 / self.fade_samples as f32
        } else if self.current_sample > self.total_samples - self.fade_samples {
            (self.total_samples - self.current_sample) as f32 / self.fade_samples as f32
        } else {
            1.0
        };

        self.current_sample += 1;
        Some(sine * envelope * 0.5)
    }
}

impl Source for EnvelopedSine {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.total_samples - self.current_sample)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_millis(CLICK_DURATION_MS))
    }
}

pub struct AudioEngine {
    _stream: OutputStream,
    _handle: OutputStreamHandle,
    sink: Sink,
}

impl AudioEngine {
    pub fn new() -> color_eyre::Result<Self> {
        let (stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;
        Ok(Self {
            _stream: stream,
            _handle: handle,
            sink,
        })
    }

    pub fn play_click(&self, accent: bool) {
        let freq = if accent { ACCENT_FREQ } else { NORMAL_FREQ };
        let source = EnvelopedSine::new(freq, CLICK_DURATION_MS, FADE_MS);
        self.sink.append(source);
    }
}
