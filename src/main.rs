use crossterm::{cursor, event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers}, execute, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};
use std::{collections::HashMap, env, fs, io::stdout, process::exit, thread::sleep, time::{Duration, Instant}};
use serde::Deserialize;

use rand::{seq::IndexedRandom, Rng};
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

// Singleton config
fn get_config(args: Option<Vec<String>>) -> &'static Config {
    CONFIG.get_or_init(|| {
        if let Some(a) = args {
            
            let mut config_toml_path: &String = &String::from("Config.toml");
            if a.len() > 1 {
                config_toml_path = &a[1];
            } 

            let toml_string = fs::read_to_string(config_toml_path).expect("Should have been able to read the file");

            let config: Config = toml::from_str(toml_string.as_str()).unwrap();
            return config
        } else {
            return Config {
                spells: vec![]
            }
        }

    })
}

#[derive(Debug, Deserialize)]
struct Config {
    spells: Vec<Spell>,
}

#[derive(Debug, Deserialize)]
pub struct Spell {
    spell_name: String,
    keybinds: Vec<String>,
}

pub const WAIT_TIME_MILLIS: u128 = 1000;
pub const SLEEP_SECONDS: u64 = 1;


// TODO get these values from a function that reads keybinds.yaml
pub const USER_SHEEPS: [&str; 3] = ["ctrl+7", "ctrl+8", "ctrl+9"];
pub const USER_KICKS: [&str; 3] = ["shift+&", "shift+*", "shift+("];
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

fn get_target_key_event(target_keybind: &String) -> Option<KeyEvent> {
    let mut i = 0;
    let mut modifier = KeyModifiers::NONE;

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

fn game_loop() {
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

        let target_key_event = get_target_key_event(&target_keybind).unwrap();

        let option_target_msg = format!("{} target {}", option, arena_target + 1);
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(option_target_msg)).unwrap();

        let quit_key_event = KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, kind: KeyEventKind::Press, state: KeyEventState::NONE };

        'input_loop: loop {
            execute!(stdout, cursor::MoveTo(0,1)).unwrap();

            match read().unwrap() {
                Event::Key(key_event) => {
                    println!("{:#?}", key_event);
                    if key_event == target_key_event {
                        break 'input_loop;
                    } else if key_event == quit_key_event {
                        break 'game_loop;
                    }
                },
                _ => (),
            }
            

//            match read().unwrap() {
//                // User enters correct keybind
//                Event::Key(KeyEvent {
//                    kind: KeyEventKind::Press,
//                    state: KeyEventState::NONE,
//                    code: KeyCode::Char(c),
//                    modifiers: m,
//                }) => {
//                    if c == 'c' && m == KeyModifiers::CONTROL {
//                        break 'game_loop;
//                    } else if c == desired_char && m == desired_key_modifier {
//                        break 'input_loop;
//                    }
//                    print!("input '{}' with modifier '{}' is incorrect. ", c, m);
//                    println!("desired modifier: '{}', target '{}'", desired_key_modifier, target_keybind);
//                }
//                _ => (),
//            }
        }

        let rxn_time = now.elapsed().as_millis();
        let rxn_time_msg = format!("Your reaction time was {} (ms)", rxn_time);
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(rxn_time_msg)).unwrap();
        sleep(Duration::from_secs(SLEEP_SECONDS));
    }
    disable_raw_mode().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: &'static Config = get_config(Some(args));
    println!("{:?}", config);
    //game_loop();
}

