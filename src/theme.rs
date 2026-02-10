use ratatui::style::Color;

pub struct Theme {
    pub name: &'static str,
    pub border: Color,
    pub bpm_text: Color,
    pub time_sig_text: Color,
    pub beat_accent: Color,
    pub beat_current: Color,
    pub beat_inactive: Color,
    pub status_playing: Color,
    pub status_stopped: Color,
    pub key_hint: Color,
    pub desc_text: Color,
}

pub const THEMES: [Theme; 5] = [
    // Default
    Theme {
        name: "Default",
        border: Color::Cyan,
        bpm_text: Color::Yellow,
        time_sig_text: Color::White,
        beat_accent: Color::Red,
        beat_current: Color::Cyan,
        beat_inactive: Color::DarkGray,
        status_playing: Color::Green,
        status_stopped: Color::Red,
        key_hint: Color::Yellow,
        desc_text: Color::White,
    },
    // Ocean
    Theme {
        name: "Ocean",
        border: Color::Blue,
        bpm_text: Color::Cyan,
        time_sig_text: Color::White,
        beat_accent: Color::Yellow,
        beat_current: Color::Blue,
        beat_inactive: Color::DarkGray,
        status_playing: Color::Cyan,
        status_stopped: Color::LightRed,
        key_hint: Color::LightCyan,
        desc_text: Color::White,
    },
    // Ember
    Theme {
        name: "Ember",
        border: Color::Red,
        bpm_text: Color::LightRed,
        time_sig_text: Color::White,
        beat_accent: Color::Yellow,
        beat_current: Color::Red,
        beat_inactive: Color::DarkGray,
        status_playing: Color::Yellow,
        status_stopped: Color::Magenta,
        key_hint: Color::LightRed,
        desc_text: Color::White,
    },
    // Forest
    Theme {
        name: "Forest",
        border: Color::Green,
        bpm_text: Color::LightGreen,
        time_sig_text: Color::White,
        beat_accent: Color::Yellow,
        beat_current: Color::Green,
        beat_inactive: Color::DarkGray,
        status_playing: Color::LightGreen,
        status_stopped: Color::Red,
        key_hint: Color::LightGreen,
        desc_text: Color::White,
    },
    // Neon
    Theme {
        name: "Neon",
        border: Color::Magenta,
        bpm_text: Color::LightMagenta,
        time_sig_text: Color::White,
        beat_accent: Color::LightCyan,
        beat_current: Color::Magenta,
        beat_inactive: Color::DarkGray,
        status_playing: Color::LightCyan,
        status_stopped: Color::LightRed,
        key_hint: Color::LightMagenta,
        desc_text: Color::White,
    },
];
