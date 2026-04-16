use ratatui::layout::{Rect, Layout, Direction, Constraint};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::style::{Style, Color};
use ratatui::text::Text;

pub fn popup<'a>(title: &'a str, text: &'a str) -> Paragraph<'a> {
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

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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