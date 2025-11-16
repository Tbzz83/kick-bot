use std::time::Duration;

use color_eyre::Result;
use rand::Rng;
use crossterm::event::{poll, read, Event};
use ratatui::DefaultTerminal;

use crate::config::{Config, Spell, SpellType};
use crate::game::{
    input::{get_quit_key_event, get_target_key_event, read_input_event},
    render::render,
    state::GameState,
    timing::{random_target, random_wait_time, sleep_short, wait_for},
};

pub struct GameController<'a> {
    state: GameState<'a>,
}

impl<'a> GameController<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            state: GameState::new(config),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        color_eyre::install()?;
        let terminal = ratatui::init();
        let result = self.game_loop(terminal);
        ratatui::restore();
        result
    }

    fn game_loop(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;

        let quit_key_event = get_quit_key_event();

        'game_loop: loop {
            let wait_time = random_wait_time();
            wait_for(wait_time);

            let arena_target = random_target();
            let target_spell = self.get_rnd_spell();
            let target_key_event = get_target_key_event(arena_target, target_spell).unwrap();

            self.state.current_target = Some(arena_target);
            self.state.current_spell = Some(target_spell);

            // Set interrupted state to false if spell is interruptable
            match target_spell.spell_type {
                SpellType::Interrupt => self.state.successful_interrupt = Some(false),
                _ => (),
            }

            let now = std::time::Instant::now();
            loop {
                self.state.interruptable_cast_time_elapsed_millis = Some(now.elapsed().as_millis().clone());
                terminal.draw(|f| render(f, &self.state))?;

                let timeout = Duration::from_secs_f32(1.0 /20.0);
                if poll(timeout)? {
                    if let Event::Key(key_event) = read()? {
                        if key_event == target_key_event {
                            self.state.successful_interrupt = Some(true);
                            terminal.draw(|f| render(f, &self.state))?;
                            break;
                        } else if key_event == quit_key_event {
                            break 'game_loop;
                        }
                    }
                }
            }

            let rxn_time = now.elapsed().as_millis();
            //println!("Reaction time: {} ms", rxn_time);
            sleep_short();
        }

        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    fn get_rnd_spell(&self) -> &'a Spell {
        let rng_int = rand::rng().random_range(0..self.state.config.spells.len());
        &self.state.config.spells[rng_int]
    }
}
