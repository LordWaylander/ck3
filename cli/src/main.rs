use app::CurrentScreen;
use clap::Parser;
use core;
use core::structs::*;
mod app;
mod ui;

use crossterm::event::KeyModifiers;
use ratatui::{
    backend::Backend,
    crossterm::{
        event::{self, Event, KeyCode, DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
    prelude::CrosstermBackend
};
use std::{io, error::Error};

/// Simple program to generate a ck3 player
#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Possible values : [martialite, diplomatie, intrigue, intendance, erudition]
    #[arg(short, long)]
    pub education: Option<String>,
    /// Possible values : [1, 2, 3, 4, 5]
    #[arg(short, long)]
    pub level: Option<i8>,
    /// 0 to 70 years old
    #[arg(short, long)]
    pub age: Option<i8>,
}

fn get_params() -> Parameters {
    let args = Args::parse();

    let params = Parameters {
        education: args.education,
        level: args.level,
        age: args.age,
    };

    params
}

pub fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;


    let params = get_params();
    let mut app = app::App::new(params);

    let res = run_app(&mut terminal, &mut app);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(),LeaveAlternateScreen,DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("kind : {}, {}", err.kind(), err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut app::App) -> io::Result<()>
where
    io::Error: From<B::Error>,
{
    while !app.exit {
        terminal.draw(|frame| ui::ui(frame, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('c'))  {
                        app.current_screen = CurrentScreen::Exit;
                    }
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('s') {
                        app.current_screen = CurrentScreen::Save;
                        // app.save()?;
                    }
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('r') {
                        let p =get_params();
                        //println!("{:?}", p);
                        app.personnage = core::generate_personnage(p);
                        // app.save()?;
                    }
                }
                CurrentScreen::Exit => match key.code {
                    KeyCode::Char('y') => {
                        app.exit()
                    }
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                }
                CurrentScreen::Save => {
                    match key.code {
                        KeyCode::Enter => app.save()?,
                        KeyCode::Char(to_insert) => app.enter_char(to_insert),
                        KeyCode::Backspace => app.delete_char(),
                        KeyCode::Left => app.move_cursor_left(),
                        KeyCode::Right => app.move_cursor_right(),
                        KeyCode::Esc => app.current_screen = CurrentScreen::Main,
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}
