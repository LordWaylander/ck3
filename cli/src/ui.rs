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
    footer(chunks[2], frame);

    if let CurrentScreen::Exit = app.current_screen {
        //

        let popup_block = Block::bordered()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::new().red().on_black().bold().italic());

        let exit_text = Text::styled(
            "Are you sure do you want Exit app ?",
            Style::default().fg(Color::Red),
        );

        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }

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

fn footer(chunk2: Rect, frame: &mut Frame) {
    let block: Block<'_> = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

    let title: Paragraph<'_> = Paragraph::new(Text::styled(
        "q => quit \ns => save",
        Style::default().fg(Color::Green),
    ))
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

    let t = format!(
r#"Age : {},
education : {}, niveau : {},
points totaux : {}"#, 
        app.personnage.age,
        app.personnage.education.name, app.personnage.education.level,
        app.personnage.points_totaux
    );

    let text: Paragraph<'_> = Paragraph::new(Text::styled(
        t,
        Style::default().fg(Color::Yellow),
    ))
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