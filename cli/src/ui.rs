use ratatui::layout::{Rect, Layout, Direction, Constraint};
use ratatui::Frame;
use ratatui::widgets::{Block, Borders, Paragraph, ListItem, List, Wrap, Clear};
use ratatui::style::{Style, Color};
use ratatui::text::{Text, Line, Span};
use crate::app::CurrentScreen;

use crate::app::App;

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn ui(frame: &mut Frame, app: &App) {
    frame.render_widget(Clear, frame.area());

    let chunks: std::rc::Rc<[Rect]> = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
    ])
    .split(frame.area());

    header(chunks[0], frame);
    fill_chunk_1(chunks[1], app, frame);


    let content_footer = match app.current_screen {
        CurrentScreen::Main => {
            vec![
                Line::from("CTRL + Q => quit"),
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
            ]
        }
    };
    footer(chunks[2], frame, content_footer);

    if let CurrentScreen::Exit = app.current_screen {
        let popup = popup("Exit App", "Are you sure do you want Exit app ? (y / n)");

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(Clear, area);
        frame.render_widget(popup, area);
    }

    if let CurrentScreen::Save = app.current_screen {
        // let popup = popup("Save personnage", "TODO, input with filename ? or save directly with timestamp ?");

        let popup_block = Block::default()
        .title("Enter the filename : ")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 40, frame.area());
        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        let input_area = centered_rect(60, 25, area);

        let input = Paragraph::new(app.filename.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::bordered().title("filename"));
        frame.render_widget(input, input_area);

        use ratatui::layout::Position;
        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position can be controlled via the left and right arrow key
            input_area.x + app.character_index as u16 + 1,
            // Move one line down, from the border to the input line
            input_area.y + 1,
        ));
    }

}

fn popup<'a>(title: &'a str, text: &'a str) -> Paragraph<'a> {
    let popup_block = Block::bordered()
        .title(title)
        .borders(Borders::NONE)
        .style(Style::new().red().on_black().bold().italic());

    let text = Text::styled(
        text,
        Style::default().fg(Color::Red),
    );

    let paragraph = Paragraph::new(text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    paragraph
}

fn header(chunk1: Rect, frame: &mut Frame) {

    let block: Block<'_> = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title: Paragraph<'_> = Paragraph::new(Text::styled(
        "Generator CK3 Personnage",
        Style::default().fg(Color::Green),
    ))
    .block(block);

    frame.render_widget(title, chunk1);
}

fn footer(chunk2: Rect, frame: &mut Frame, content: Vec<Line<'_>>) {
    let block: Block<'_> = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

    let title: Paragraph<'_> = Paragraph::new(Text::from(
        content).patch_style(Style::default().fg(Color::Green))
    )
    .block(block);

    frame.render_widget(title, chunk2);
}

fn fill_chunk_1(chunk1: Rect, app: &App, frame: &mut Frame) {
    let chunks_2: std::rc::Rc<[Rect]> = 
        Layout::horizontal([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
        ]).split(chunk1);

/*------- */
    let block: Block<'_> = Block::default()
    .style(Style::default());

    let t = vec![
        Line::from(format!("Age : {}", app.personnage.age)),
        Line::from(format!("education : {}, niveau : {}", app.personnage.education.name, app.personnage.education.level)),
        Line::from(format!("points totaux : {}", app.personnage.points_totaux)),
    ];

    let text: Paragraph<'_> = Paragraph::new(Text::from(
        t).patch_style(Style::default().fg(Color::Yellow))
    )
    .block(block);
    frame.render_widget(text, chunks_2[0]);
/*------- */

    let chunks_2_1 = 
        Layout::vertical([
                Constraint::Ratio(1, 10),
                Constraint::Ratio(9, 10),
        ]).split(chunks_2[1]);


    let block: Block<'_> = Block::default()
    .style(Style::default());

    let text: Paragraph<'_> = Paragraph::new(Text::styled(
        "Statistiques : ",
        Style::default().fg(Color::Cyan),
    ))
    .block(block);

    frame.render_widget(text, chunks_2_1[0]);

    let mut list_items = Vec::<ListItem>::new();
    for key in app.personnage.statistiques.keys().cloned() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.personnage.statistiques.get(&key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }
    let list = List::new(list_items);
    frame.render_widget(list, chunks_2_1[1]);

/*------- */


    let chunks_2_2 = 
        Layout::vertical([
                Constraint::Ratio(1, 10),
                Constraint::Ratio(9, 10),
        ]).split(chunks_2[2]);


    let block: Block<'_> = Block::default()
    .style(Style::default());

    let text: Paragraph<'_> = Paragraph::new(Text::styled(
        "Personnalité : ",
        Style::default().fg(Color::Cyan),
    ))
    .block(block);

    frame.render_widget(text, chunks_2_2[0]);

    let mut list_items = Vec::<ListItem>::new();
    for personality in app.personnage.personnality.iter() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}", personality.name),
            Style::default().fg(Color::Yellow),
        ))));
    }
    let list = List::new(list_items);
    frame.render_widget(list, chunks_2_2[1]);


}