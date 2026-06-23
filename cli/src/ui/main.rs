use ratatui::layout::{Rect, Layout, Constraint};
use ratatui::Frame;
use ratatui::widgets::{Block, Paragraph, ListItem, List, Clear};
use ratatui::style::{Style, Color};
use ratatui::text::{Text, Line, Span};

use crate::app::App;

pub fn main(chunk1: Rect, app: &App, frame: &mut Frame) {
    frame.render_widget(Clear, chunk1);
    
    let chunks_2: std::rc::Rc<[Rect]> = 
        Layout::horizontal([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
        ]).split(chunk1);

/*------- */
    let block: Block<'_> = Block::default()
    .style(Style::default());

    let style = if app.personnage.points_totaux > 400 {
        Style::new().fg(Color::Red)
    } else {
        Style::default().fg(Color::Yellow)
    };
    let span1 = Span::styled(format!("{}", app.personnage.points_totaux), style);
    let span2 = Span::styled("points totaux : ", Style::default().fg(Color::Yellow));

    let pt_totaux = Line::from(vec![
        span2, span1
    ]);

    let infos = vec![
        Line::from(format!("Age : {}", app.personnage.age)).style(Style::default().fg(Color::Yellow)),
        Line::from(format!("education : {}, niveau : {}", app.personnage.education.name, app.personnage.education.level)).style(Style::default().fg(Color::Yellow)),
        pt_totaux
    ];

    let text: Paragraph<'_> = Paragraph::new(Text::from(infos))
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