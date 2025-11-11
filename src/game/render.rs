use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Padding},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Widget},
    Frame,
};

use crate::game::state::GameState;

const ARENA_TARGETS: usize = 3;
const CASTBAR_WIDTH: u16 = 3;

pub fn render(frame: &mut Frame, _state: &GameState) {
    let area = frame.area();

    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![Constraint::Max(60)])
        .split(area);

    Block::bordered()
        .title("Arena Targets")
        .render(outer_layout[0], frame.buffer_mut());

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints((0..ARENA_TARGETS).map(|_| Constraint::Max(CASTBAR_WIDTH)).collect::<Vec<_>>())
        .split(outer_layout[0]);

    for i in 0..ARENA_TARGETS {
        let casters_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(inner_layout[i]);

        frame.render_widget(
            format!("Target {i}"),
            Rect {
                x: casters_layout[0].x + 1,
                y: casters_layout[0].y + 1,
                ..casters_layout[0]
            },
        );

        let cast_bar = Gauge::default()
            .block(Block::new().borders(Borders::NONE).padding(Padding::vertical(1)))
            .gauge_style(Style::default().fg(Color::Yellow))
            .ratio(0.5);

        frame.render_widget(cast_bar, casters_layout[1]);
    }
}

