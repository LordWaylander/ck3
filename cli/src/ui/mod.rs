use ratatui::layout::{Rect, Layout, Direction, Constraint, Position};
use ratatui::Frame;
use ratatui::widgets::{Block, Borders, Paragraph, Clear};
use ratatui::style::{Style, Color};
use ratatui::text::Line;

use crate::app::CurrentScreen;
use crate::app::App;
mod footer;
mod header;
mod main;
mod blocks;



pub fn ui(frame: &mut Frame, app: &App) {

    let chunks: std::rc::Rc<[Rect]> = Layout::new(
        Direction::Vertical,
        [
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ]
    )
    .split(frame.area());

    header::header(chunks[0], frame);
    main::main(chunks[1], app, frame);


    let content_footer = match app.current_screen {
        CurrentScreen::Main => {
            vec![
                Line::from("CTRL + Q / C => quit"),
                Line::from("CTRL + S => save in file"),
                Line::from("CTRL + L => load a save file"),
                Line::from("CTRL + R => regenerate personnage")
            ]
        },
        CurrentScreen::Exit => {
            vec![
                Line::from("Y => quit app"),
                Line::from("N => back to main menu"),
            ]
        },
        CurrentScreen::Save => {
            vec![
                Line::from("ESC => back to main menu"),
                Line::from("ENTER => Save personnage"),
                Line::from("ESC => Cancel"),
            ]
        },
        CurrentScreen::Load => {
            vec![
                Line::from("ESC => back to main menu"),
                Line::from("ENTER => Load personnage"),
                Line::from("ESC => Cancel"),
            ]
        }
    };
    footer::footer(chunks[2], frame, content_footer);

    if let CurrentScreen::Exit = app.current_screen {
        let popup = blocks::popup("Exit App", "Are you sure do you want Exit app ? (y / n)");

        let area = blocks::centered_rect(60, 40, frame.area());
        frame.render_widget(Clear, area);
        frame.render_widget(popup, area);
    }

    if let CurrentScreen::Save = app.current_screen {
        // let popup = popup("Save personnage", "TODO, input with filename ? or save directly with timestamp ?");

        let popup_block = Block::default()
        .title("Enter the filename to save: ")
        .title("Press Enter to confirm, ESC to cancel ")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

        let area = blocks::centered_rect(60, 60, frame.area());
        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        let input_area = blocks::centered_rect(60, 25, area);

        let input = Paragraph::new(app.filename.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::bordered().title("filename"));
        frame.render_widget(input, input_area);

        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position can be controlled via the left and right arrow key
            input_area.x + app.character_index as u16 + 1,
            // Move one line down, from the border to the input line
            input_area.y + 1,
        ));
    }

    if let CurrentScreen::Load = app.current_screen {
        let popup_block = Block::default()
        .title("Enter the filename to load: ")
        .title("Press Enter to confirm, ESC to cancel ")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

        let area = blocks::centered_rect(60, 60, frame.area());
        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        let input_area = blocks::centered_rect(60, 25, area);

        let input = Paragraph::new(app.filename.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::bordered().title("filename"));
        frame.render_widget(input, input_area);

        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position can be controlled via the left and right arrow key
            input_area.x + app.character_index as u16 + 1,
            // Move one line down, from the border to the input line
            input_area.y + 1,
        ));
    }

}


