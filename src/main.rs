use crossterm::{cursor, event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}, execute, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};
use std::{io::stdout, process::exit, thread::sleep, time::{Duration, Instant}};

use rand::{seq::IndexedRandom, Rng};

pub const WAIT_TIME_MILLIS: u128 = 1000;
pub const SLEEP_SECONDS: u64 = 1;

// TODO get these values from a function that reads keybinds.yaml
pub const USER_SHEEPS: [&str; 3] = ["ctrl+7", "ctrl+8", "ctrl+9"];
pub const USER_KICKS: [&str; 3] = ["&", "*", "("];
pub const CC_OPTS: [&str; 2] = ["kick", "sheep"];

pub enum OptionArrays {
    UserKicks,
    UserSheeps,
    CcOpts,
}

fn get_desired_keybind(arena_target: usize) -> (String, String) {
    let cc_opt = get_rnd_str_from_const_arr(OptionArrays::CcOpts);

    match cc_opt.as_str() {
        "sheep" => return (cc_opt, USER_SHEEPS[arena_target].to_string()),
        "kick" => return (cc_opt, USER_KICKS[arena_target].to_string()),
        _ => (String::from(""), String::from("")),
    }
    //(String::from("kick"), USER_KICKS[arena_target].to_string())
}

fn get_rnd_wait_time() -> u128 {
    // Return rnd u128 in range 0 -> 1000
    rand::rng().random_range(0..WAIT_TIME_MILLIS)
}

fn get_rnd_arena_target() -> u16 {
    rand::rng().random_range(0..3)
}

fn get_rnd_str_from_const_arr(opt: OptionArrays) -> String {
    let mut rng = rand::rng();
    let rnd_str: Option<&&str>;

    match opt {
        OptionArrays::UserKicks => rnd_str = USER_KICKS.choose(&mut rng),
        OptionArrays::UserSheeps => rnd_str = USER_SHEEPS.choose(&mut rng),
        OptionArrays::CcOpts => rnd_str = CC_OPTS.choose(&mut rng),
    }

    match rnd_str {
        Some(x) => return String::from(*x),
        None => println!("get random from array failed!"),
    }

    String::from("")
}

fn get_key_modifier_and_key_from_target_keybind(target_keybind: &String) -> (KeyModifiers, char) {
    let mut i = 0;


    if target_keybind.len() == 1 {
        return (KeyModifiers::NONE, target_keybind.chars().nth(0).unwrap())
    }

    for char in target_keybind.as_bytes().iter() {
        if *char == b'+' {
            break;
        }
        i += 1;
    }

    let modifier = &target_keybind[..i];
    let char_pressed: char = target_keybind.chars().nth(i+1).unwrap();

    println!("modifier: {}, char_pressed: {}", modifier, char_pressed);

    match modifier {
        "shift" => return (KeyModifiers::SHIFT, char_pressed),
        "ctrl" => return (KeyModifiers::CONTROL, char_pressed),
        "alt" => return (KeyModifiers::ALT, char_pressed),
        _ => return (KeyModifiers::NONE, char_pressed),
    }
}

fn main() {
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
        while now.elapsed().as_millis() != wait_time {} // wait

        now = Instant::now();
        let arena_target = get_rnd_arena_target();
        
        let (option, target_keybind) = get_desired_keybind(arena_target.into());
        if target_keybind == "" {
            println!("Error with getting desired keybind function");
            exit(-1);
        }

        let (key_modifier, char_pressed) = get_key_modifier_and_key_from_target_keybind(&target_keybind);

        let option_target_msg = format!("{} target {}", option, arena_target + 1);
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(option_target_msg)).unwrap();

        'input_loop: loop {
            execute!(stdout, cursor::MoveTo(0,1)).unwrap();

            match read().unwrap() {
                // User enters correct keybind
                Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                    code: KeyCode::Char(c),
                    modifiers: m,
                }) if c == char_pressed && m == key_modifier => break 'input_loop,

                // User enters wrong keybind
                Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                    code: KeyCode::Char(c),
                    modifiers: m,
                }) if c != 'c' && m != KeyModifiers::CONTROL => println!("input '{}' is incorrect. char_pressed: {} modifier: {}, target {}", c, char_pressed, key_modifier, target_keybind),

                // User wants to quit the game
                Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                }) => {
                    disable_raw_mode().unwrap(); 
                    break 'game_loop;
                }
                _ => (),
            }
        }

        let rxn_time = now.elapsed().as_millis();
        let rxn_time_msg = format!("Your reaction time was {} (ms)", rxn_time);
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(rxn_time_msg)).unwrap();
        sleep(Duration::from_secs(SLEEP_SECONDS));
    }
    disable_raw_mode().unwrap();
}
