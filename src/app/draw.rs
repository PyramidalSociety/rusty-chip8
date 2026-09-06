use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use super::*;

const VIDEO_WIDTH: usize = 64;
const VIDEO_HEIGHT: usize = 32;

const MIN_WIDTH: u16 = VIDEO_WIDTH as u16 + 2;
const MIN_HEIGHT: u16 = (VIDEO_HEIGHT / 2) as u16 + 2;

impl App {
    pub(super) fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
            let msg = format!(
                "Terminal window too small\nNeed at least {MIN_WIDTH}x{MIN_HEIGHT}\nCurrent size: {}x{}",
                area.width, area.height
            );

            let paragraph = Paragraph::new(msg)
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Red));

            frame.render_widget(Clear, area);
            frame.render_widget(paragraph, area);

            return;
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Rusty CHIP-8")
            .title_alignment(Alignment::Center);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let scale = (inner.width / VIDEO_WIDTH as u16)
            .min(inner.height / (VIDEO_HEIGHT as u16 / 2))
            .max(1);

        let screen_width = VIDEO_WIDTH as u16 * scale;
        let screen_height = VIDEO_HEIGHT as u16 / 2 * scale;

        let screen_area = Rect {
            x: inner.x + (inner.width - screen_width) / 2,
            y: inner.y + (inner.height - screen_height) / 2,
            width: screen_width,
            height: screen_height,
        };

        let video = self.chip8.get_video();

        let mut lines: Vec<Line> = Vec::with_capacity(screen_height as usize);
        for ty in 0..screen_height as usize {
            let sy1 = 2 * ty / scale as usize;
            let sy2 = (2 * ty + 1) / scale as usize;

            let mut spans: Vec<Span> = Vec::with_capacity(screen_width as usize);
            for tx in 0..screen_width as usize {
                let sx = tx / scale as usize;

                let top_px = video[sy1 * VIDEO_WIDTH + sx];
                let bot_px = video[sy2 * VIDEO_WIDTH + sx];

                let ch = match (top_px, bot_px) {
                    (true, true) => "█",
                    (true, false) => "▀",
                    (false, true) => "▄",
                    (false, false) => " ",
                };

                if ch == " " {
                    spans.push(Span::raw(ch));
                } else {
                    spans.push(Span::styled(ch, Style::default().fg(Color::Green)));
                }
            }

            lines.push(Line::from(spans));
        }

        frame.render_widget(Paragraph::new(lines), screen_area);
    }
}
