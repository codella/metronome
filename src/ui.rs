use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let theme = app.theme();

    // Main border
    let main_block = Block::default()
        .title(" ♩ METRONOME ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let inner = main_block.inner(area);
    frame.render_widget(main_block, area);

    // Vertically center the content
    let content_height: u16 = 2 + 1 + 1 + 1 + 1 + 1 + 2; // BPM + spacers + beats + status + controls
    let top_pad = inner.height.saturating_sub(content_height) / 2;

    let chunks = Layout::vertical([
        Constraint::Length(top_pad), // top padding
        Constraint::Length(2),       // BPM + time signature
        Constraint::Length(1),       // spacer
        Constraint::Length(1),       // beat indicators
        Constraint::Length(1),       // spacer
        Constraint::Length(1),       // status
        Constraint::Length(1),       // spacer
        Constraint::Length(2),       // controls
        Constraint::Min(0),          // bottom fill
    ])
    .split(inner);

    render_bpm(frame, chunks[1], app);
    render_beats(frame, chunks[3], app);
    render_status(frame, chunks[5], app);
    render_controls(frame, chunks[7], app);

    if app.show_help {
        render_help_modal(frame, area, app);
    }
}

fn tempo_marking(bpm: u16) -> &'static str {
    match bpm {
        ..=39 => "Grave",
        40..=59 => "Largo",
        60..=72 => "Adagio",
        73..=107 => "Andante",
        108..=119 => "Moderato",
        120..=155 => "Allegro",
        156..=175 => "Vivace",
        176..=199 => "Presto",
        200.. => "Prestissimo",
    }
}

fn render_bpm(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let bpm_text = format!("{} BPM · {}", app.bpm, tempo_marking(app.bpm));
    let ts_text = format!(
        "{}/{}  [{}]",
        app.time_signature.0, app.time_signature.1, theme.name
    );

    let lines = vec![
        Line::from(Span::styled(
            bpm_text,
            Style::default()
                .fg(theme.bpm_text)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            ts_text,
            Style::default().fg(theme.time_sig_text),
        )),
    ];

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}

fn render_beats(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let beats = app.time_signature.0;
    let mut spans = Vec::new();

    for i in 0..beats {
        if i > 0 {
            spans.push(Span::raw("  "));
        }

        let is_current = app.playing && i == app.current_beat;
        let is_downbeat = i == 0;

        let symbol = if is_current { "●" } else { "○" };
        let color = if is_downbeat {
            theme.beat_accent
        } else if is_current {
            theme.beat_current
        } else {
            theme.beat_inactive
        };

        let style = if is_current || is_downbeat {
            Style::default()
                .fg(color)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(color)
        };

        spans.push(Span::styled(format!("[ {} ]", symbol), style));
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}

fn render_status(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let (status_text, status_color) = if app.playing {
        ("▶ Playing", theme.status_playing)
    } else {
        ("■ Stopped", theme.status_stopped)
    };

    let paragraph = Paragraph::new(Line::from(Span::styled(
        status_text,
        Style::default()
            .fg(status_color)
            .add_modifier(Modifier::BOLD),
    )))
    .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_controls(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let key_style = Style::default().fg(theme.key_hint);
    let rows: &[&[(&str, &str)]] = &[
        &[("Space", ": play/stop   "), ("↑ ↓", ": BPM ±1   "), ("+ -", ": BPM ±10")],
        &[("t", ": tap tempo   "), ("[ ]", ": time sig   "), ("c", ": theme   "), ("q", ": quit   "), ("?", ": help")],
    ];

    let lines: Vec<Line> = rows
        .iter()
        .map(|row| {
            Line::from(
                row.iter()
                    .flat_map(|(key, desc)| [Span::styled(*key, key_style), Span::raw(*desc)])
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}

fn render_help_modal(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();

    let key_style = Style::default()
        .fg(theme.key_hint)
        .add_modifier(Modifier::BOLD);
    let desc_style = Style::default().fg(theme.desc_text);

    let controls: &[(&str, &str)] = &[
        ("Space", "Play / stop the metronome"),
        ("↑ / k", "Increase BPM by 1"),
        ("↓ / j", "Decrease BPM by 1"),
        ("+ / =", "Increase BPM by 10"),
        ("-", "Decrease BPM by 10"),
        ("t", "Tap tempo — set BPM by tapping"),
        ("]", "Next time signature"),
        ("[", "Previous time signature"),
        ("c", "Cycle color theme"),
        ("?", "Toggle this help screen"),
        ("q / Esc", "Quit"),
    ];

    let key_col_width = 9_u16;
    let gap = 2_u16;
    let desc_col_width = 30_u16;
    let modal_width = key_col_width + gap + desc_col_width + 2; // +2 for borders
    let modal_height = controls.len() as u16 + 2; // +2 for borders
    let x = area.x + area.width.saturating_sub(modal_width) / 2;
    let y = area.y + area.height.saturating_sub(modal_height) / 2;
    let modal_area = Rect::new(
        x,
        y,
        modal_width.min(area.width),
        modal_height.min(area.height),
    );

    frame.render_widget(Clear, modal_area);

    let block = Block::default()
        .title(" Help ")
        .title_alignment(Alignment::Center)
        .title_bottom(Line::from(" Esc / ? to close ").centered())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));

    let inner = block.inner(modal_area);
    frame.render_widget(block, modal_area);

    let cols = Layout::horizontal([
        Constraint::Length(key_col_width),
        Constraint::Length(gap),
        Constraint::Length(desc_col_width),
    ])
    .split(inner);

    let key_lines: Vec<Line> = controls
        .iter()
        .map(|(key, _)| Line::from(Span::styled(*key, key_style)))
        .collect();

    let desc_lines: Vec<Line> = controls
        .iter()
        .map(|(_, desc)| Line::from(Span::styled(*desc, desc_style)))
        .collect();

    let keys = Paragraph::new(key_lines).alignment(Alignment::Right);
    let descs = Paragraph::new(desc_lines).alignment(Alignment::Left);

    frame.render_widget(keys, cols[0]);
    frame.render_widget(descs, cols[2]);
}
