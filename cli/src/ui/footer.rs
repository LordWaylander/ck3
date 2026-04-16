use ratatui::layout::Rect;
use ratatui::Frame;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Style, Color};
use ratatui::text::{Text, Line};

pub fn footer(chunk2: Rect, frame: &mut Frame, content: Vec<Line<'_>>) {
    let block: Block<'_> = Block::default()
    .borders(Borders::ALL)
    .style(Style::default());

    let title: Paragraph<'_> = Paragraph::new(Text::from(
        content).patch_style(Style::default().fg(Color::Green))
    )
    .block(block);

    frame.render_widget(title, chunk2);
}