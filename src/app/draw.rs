use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use super::*;

const VIDEO_HEIGHT: usize = 32;
const VIDEO_WIDTH: usize = 64;

impl App {
    pub(super) fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Rusty CHIP-8")
            .title_alignment(Alignment::Center);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let video = self.chip8.get_video();
        let width = inner.width as usize;
        let height = inner.height as usize;

        let mut lines: Vec<Line> = Vec::with_capacity(height);

        for ty in 0..height {
            let py = (ty * VIDEO_HEIGHT) / height.max(1);

            let mut spans: Vec<Span> = Vec::with_capacity(width);

            for tx in 0..width {
                let px = (tx * VIDEO_WIDTH) / width.max(1);

                if video[py * VIDEO_WIDTH + px] {
                    spans.push(Span::styled("█", Style::default().fg(Color::Green)));
                } else {
                    spans.push(Span::raw(" "));
                }
            }

            lines.push(Line::from(spans));
        }

        frame.render_widget(Paragraph::new(lines), inner);
    }
}
