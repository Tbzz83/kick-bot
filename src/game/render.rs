use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Padding},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Widget},
    Frame,
};

use crate::game::state::GameState;
use crate::config::SpellType;

const ARENA_TARGETS: usize = 3;
const CASTBAR_WIDTH: u16 = 3;

pub fn render(frame: &mut Frame, state: &GameState) {
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

    let mut casters_areas: Vec<Rect> = vec![];
    for i in 0..ARENA_TARGETS {
        let casters_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(inner_layout[i]);

        let area = Rect {
            x: casters_layout[0].x + 1,
            y: casters_layout[0].y + 1,
            ..casters_layout[0]
        };

        // Print caster name
        frame.render_widget(
            format!("Target {i}"),
            area
        );

        casters_areas.push(casters_layout[1]);
    }

    let current_spell = &state.current_spell.expect("Spell does not exist");
    let current_target = &state.current_target.expect("Target does not exist");
    let caster_area = casters_areas.get(*current_target as usize).expect("Caster area could not be retrieved");

    match current_spell.spell_type {

        SpellType::Interrupt => {
            let mut spell_cast_time_elapsed_millis = state.spell_cast_time_elapsed_millis.expect("Error with spell_cast_time_elapsed. Does not exist");
            let cast_time_secs = current_spell.cast_time_secs.expect("Current spell does not have cast time");
            let cast_time_millis = (cast_time_secs * 1000.0) as u128;

            // Keeps value between 0-cast_time
            if spell_cast_time_elapsed_millis > cast_time_millis {
                spell_cast_time_elapsed_millis = cast_time_millis;
            }
            let ratio = (spell_cast_time_elapsed_millis as f64) / (cast_time_millis as f64);

            let cast_bar = Gauge::default()
                .block(Block::new().borders(Borders::NONE).padding(Padding::vertical(1)))
                .gauge_style(Style::default().fg(Color::Yellow))
                .ratio(ratio);
            frame.render_widget(cast_bar, *caster_area);
        }

        SpellType::CC => {
            frame.render_widget(
                format!("<< {}!", current_spell.spell_name), 
                Rect {
                    y: caster_area.y+1,
                    ..*caster_area
                },
            );
        }
    }
}

