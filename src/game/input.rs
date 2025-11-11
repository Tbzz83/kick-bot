use crossterm::{event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}};
use crate::config::Spell;

pub fn get_quit_key_event() -> KeyEvent {
    KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, kind: KeyEventKind::Press, state: KeyEventState::NONE }
}

pub fn get_target_key_event(arena_target: u16, spell: &Spell) -> Option<KeyEvent> {
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

pub fn read_input_event() -> Option<Event> {
    match read() {
        Ok(event) => Some(event),
        Err(_) => None,
    }
}
