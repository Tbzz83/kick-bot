use crate::config::Config;
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{layout::{Constraint, Direction, Layout, Margin}, style::{Color, Style, Stylize}, text::Line, widgets::{Block, Borders, Gauge, LineGauge, List, ListItem, Padding, Widget}, DefaultTerminal, Frame};

pub fn test(config: &Config) -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let area = frame.area();

    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![Constraint::Max(60)])
        .split(frame.area());

    Block::bordered()
        .fg(Color::White)
        .title("Arena Targets")
        .render(outer_layout[0], frame.buffer_mut());

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Length(2),Constraint::Length(2),Constraint::Length(2)])
        .split(outer_layout[0]);

    for i in 0..3 {
        let cast_bar = Gauge::default()
            .block(Block::new().borders(Borders::NONE).padding(Padding::vertical(1)))
            .gauge_style(Style::default().fg(Color::Yellow))
            .ratio(0.5);

        frame.render_widget(cast_bar, inner_layout[i]);
    }
}
