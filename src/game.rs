use rand::Rng;
use crossterm::{cursor, event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}, execute, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};
use std::{io::stdout, thread::sleep, time::{Duration, Instant}};

use crate::config::{Config, Spell};

pub const WAIT_TIME_MILLIS: u128 = 1000;
pub const SLEEP_SECONDS: u64 = 1;

fn get_quit_key_event() -> KeyEvent {
    KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, kind: KeyEventKind::Press, state: KeyEventState::NONE }
}

pub fn game_loop(config: &Config) {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    'game_loop: loop {
        let wait_time: u128 = get_rnd_wait_time();

        #[cfg(debug_assertions)]
        {
        let wait_time_msg = format!("wait time is {wait_time} (ms)");
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(wait_time_msg)).unwrap();
        }

        let mut now = Instant::now();
        let arena_target = get_rnd_arena_target();
        let target_spell = get_rnd_spell(config);
        let target_key_event = get_target_key_event(arena_target, target_spell).unwrap();
        let option_target_msg = format!("{} target {}", target_spell.spell_name, arena_target + 1);
        let quit_key_event = get_quit_key_event();

        while now.elapsed().as_millis() < wait_time {} // wait
        
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(option_target_msg)).unwrap();

        // For measuring rxn time
        now = Instant::now();
        'input_loop: loop {
            execute!(stdout, cursor::MoveTo(0,1)).unwrap();

            match read().unwrap() {
                Event::Key(key_event) => {
                    #[cfg(debug_assertions)]
                    {
                        println!("{:#?}", key_event);
                    }
                    if key_event == target_key_event {
                        break 'input_loop;
                    } else if key_event == quit_key_event {
                        break 'game_loop;
                    }
                },
                _ => (),
            }
            
        }

        let rxn_time = now.elapsed().as_millis();
        let rxn_time_msg = format!("Reaction time:\n{} (ms)", rxn_time);
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(rxn_time_msg)).unwrap();
        sleep(Duration::from_secs(SLEEP_SECONDS));
    }
    disable_raw_mode().unwrap();
}

fn get_rnd_spell(config: &Config) -> &Spell {
    let rng_int = rand::rng().random_range(0..config.spells.len());
    &config.spells[rng_int]
}

fn get_rnd_wait_time() -> u128 {
    // Return rnd u128 in range 0 -> 1000
    rand::rng().random_range(0..WAIT_TIME_MILLIS)
}

fn get_rnd_arena_target() -> u16 {
    rand::rng().random_range(0..3)
}

fn get_target_key_event(arena_target: u16, spell: &Spell) -> Option<KeyEvent> {
    let mut i = 0;
    let mut modifier = KeyModifiers::NONE;

    let target_keybind: &String = &spell.keybinds[arena_target as usize];

    if target_keybind.len() == 1 {
        return Some(
            KeyEvent { code: KeyCode::Char(
                target_keybind.chars().nth(0).unwrap()
            ), 
                modifiers: KeyModifiers::NONE, 
                kind: KeyEventKind::Press, 
                state: KeyEventState::NONE 
            }
        )
    }

    for char in target_keybind.as_bytes().iter() {
        if *char == b'+' {
            break;
        }
        i += 1;
    }

    let desired_char: char = target_keybind.chars().nth(i+1).unwrap();
    match &target_keybind[..i] {
        "shift" => modifier = KeyModifiers::SHIFT,
        "alt" => modifier = KeyModifiers::ALT,
        "ctrl" => modifier = KeyModifiers::CONTROL,
        _ => (),
    }

    Some(
        KeyEvent { 
            code: KeyCode::Char(
                desired_char,
            ), 
            modifiers: modifier, 
            kind: KeyEventKind::Press, 
            state: KeyEventState::NONE,
        }
    )
}

