use crate::config::Config;
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{layout::{Constraint, Direction, Layout, Margin, Rect}, style::{Color, Style, Stylize}, text::Line, widgets::{Block, Borders, Gauge, LineGauge, List, ListItem, Padding, Widget}, DefaultTerminal, Frame};

const ARENA_TARGETS: usize = 3;
const CASTBAR_WIDTH: u16 = 3;


#[derive(Default, Debug)]
pub struct App {
    state: AppState,
}

#[derive(Default, Debug)]
enum AppState {
    #[default]
    Stopped,
    Running, 
    Started, 
}

impl App {
    fn run(&mut self) {
        self.state = AppState::Running;
    }
}


pub fn tui_init(config: &Config) -> Result<()> {
    color_eyre::install()?;
    let app = App::default();
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
        .split(area);

    Block::bordered()
        .fg(Color::White)
        .title("Arena Targets")
        .render(outer_layout[0], frame.buffer_mut());

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Max(CASTBAR_WIDTH),Constraint::Max(CASTBAR_WIDTH),Constraint::Max(CASTBAR_WIDTH)])
        .split(outer_layout[0]);

    for i in 0..ARENA_TARGETS {
        let casters_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(inner_layout[i]);

        frame.render_widget(format!("Target {i}"), Rect{
            x : casters_layout[0].x + 1,
            y : casters_layout[0].y + 1,
            ..casters_layout[0]
        });

        let cast_bar = Gauge::default()
            .block(Block::new().borders(Borders::NONE).padding(Padding::vertical(1)))
            .gauge_style(Style::default().fg(Color::Yellow))
            .ratio(0.5);

        frame.render_widget(cast_bar, casters_layout[1]);
    }
}
