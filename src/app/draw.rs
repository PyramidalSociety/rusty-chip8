use std::ops::Deref;

use ratatui::{
    Frame,
    style::Color,
    symbols::Marker::HalfBlock,
    widgets::{
        Block,
        canvas::{Canvas, Points},
    },
};

use super::*;

const VIDEO_HEIGHT: usize = 32;
const VIDEO_WIDTH: usize = 64;

impl App {
    pub(super) fn draw(&self, frame: &mut Frame) {
        let points: Vec<(_, _)> = self
            .chip8
            .get_video()
            .iter()
            .enumerate()
            .filter(|(_, px)| **px)
            .map(|(i, _)| {
                let x = (i % VIDEO_WIDTH) as f64;
                let y = (VIDEO_HEIGHT - 1 - i / VIDEO_WIDTH) as f64;
                (x, y)
            })
            .collect();

        let canvas = Canvas::default()
            .block(Block::bordered().title("Rusty CHIP8"))
            .marker(HalfBlock)
            .x_bounds([0.0, VIDEO_WIDTH as f64])
            .y_bounds([0.0, VIDEO_HEIGHT as f64])
            .paint(|ctx| {
                ctx.draw(&Points {
                    coords: &points,
                    color: Color::Green,
                });
            });

        frame.render_widget(canvas, frame.area());
    }
}
