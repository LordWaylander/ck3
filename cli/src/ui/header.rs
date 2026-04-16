use ratatui::layout::Rect;
use ratatui::Frame;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Style, Color};
use ratatui::text::Text;

pub fn header(chunk1: Rect, frame: &mut Frame) {

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