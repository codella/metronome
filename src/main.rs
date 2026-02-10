mod app;
mod audio;
mod theme;
mod ui;

use app::App;
use audio::AudioEngine;
use clap::Parser;
use crossterm::event::{self, Event, KeyEventKind};
use std::time::Duration;

#[derive(Parser)]
#[command(name = "metronome", about = "A CLI metronome with interactive TUI")]
struct Cli {
    /// Beats per minute (20-300)
    #[arg(short, long, default_value_t = 120)]
    bpm: u16,

    /// Time signature (e.g. 4/4, 3/4, 6/8)
    #[arg(short, long, default_value = "4/4")]
    time_signature: String,
}

fn parse_time_signature(s: &str) -> color_eyre::Result<(u8, u8)> {
    let (num, den) = s
        .split_once('/')
        .ok_or_else(|| color_eyre::eyre::eyre!("Invalid time signature format. Use N/N (e.g. 4/4)"))?;
    Ok((num.parse()?, den.parse()?))
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    let time_signature = parse_time_signature(&cli.time_signature)?;

    let audio = AudioEngine::new()?;
    let mut app = App::new(cli.bpm, time_signature);

    let mut terminal = ratatui::init();

    let result = run_loop(&mut terminal, &mut app, &audio);

    ratatui::restore();

    result
}

fn run_loop(
    terminal: &mut ratatui::DefaultTerminal,
    app: &mut App,
    audio: &AudioEngine,
) -> color_eyre::Result<()> {
    loop {
        app.tick(audio);

        terminal.draw(|frame| ui::render(frame, app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key);
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
