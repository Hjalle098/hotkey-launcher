use serde::Deserialize;
use rdev::{Event, EventType, Key, listen};
use std::collections::HashSet;
use std::sync::Mutex;
use std::sync::LazyLock;

#[derive(Deserialize)]
pub struct Shortcut {
    pub label: String,
    pub keys: String,
    pub command: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub shortcuts: Vec<Shortcut>,
}

struct RegisteredShortcut {
    label: String,
    command: String,
    keys: HashSet<String>,
}

static PRESSED: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));
static SHORTCUTS: std::sync::OnceLock<Vec<RegisteredShortcut>> = std::sync::OnceLock::new();

fn load_config() -> Config {
    let config_path = dirs::home_dir()
        .expect("could not find home directory")
        .join(".config")
        .join("hotkey-launcher")
        .join("config.json");

    let contents = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", config_path.display(), e));

    serde_json::from_str(&contents).unwrap_or_else(|e| panic!("failed to parse config: {}", e))
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => {
            let key_name = key_to_string(&key);
            let mut pressed = PRESSED.lock().unwrap();
            pressed.insert(key_name);
            println!("Pressed keys: {:?}", *pressed);
        }
        EventType::KeyRelease(key) => {
            let key_name = key_to_string(&key);
            let mut pressed = PRESSED.lock().unwrap();
            pressed.remove(&key_name);
            println!("Pressed keys: {:?}", *pressed);
        }, 
        _ => (),
    }
}

fn normalize_key(name: &str) -> String {
    match name.to_lowercase().as_str() {
        "cmd" | "command" | "meta" | "super" => "meta".to_string(),
        "ctrl" | "control" => "control".to_string(),
        "shift" => "shift".to_string(),
        other => other.to_string()
    }
}

fn parse_shortcuts(config: &Config) -> Vec<RegisteredShortcut> {
    config.shortcuts.iter().map(|s| {
        let keys: HashSet<String> = s.keys
            .split('+')
            .map(|k| normalize_key(k.trim()))
            .collect();

        RegisteredShortcut {
            label: s.label.clone(),
            command: s.command.clone(),
            keys,
        }
    }).collect()
}

fn key_to_string(key: &Key) -> String {
    match key {
        Key::MetaLeft | Key::MetaRight => "meta".to_string(),
        Key::ControlLeft | Key::ControlRight => "ctrl".to_string(),
        Key::Alt | Key::AltGr => "alt".to_string(),
        Key::ShiftLeft | Key::ShiftRight => "shift".to_string(),
        Key::KeyA => "a".to_string(),
        Key::KeyB => "b".to_string(),
        Key::KeyC => "c".to_string(),
        Key::KeyD => "d".to_string(),
        Key::KeyE => "e".to_string(),
        Key::KeyF => "f".to_string(),
        Key::KeyG => "g".to_string(),
        Key::KeyH => "h".to_string(),
        Key::KeyI => "i".to_string(),
        Key::KeyJ => "j".to_string(),
        Key::KeyK => "k".to_string(),
        Key::KeyL => "l".to_string(),
        Key::KeyM => "m".to_string(),
        Key::KeyN => "n".to_string(),
        Key::KeyO => "o".to_string(),
        Key::KeyP => "p".to_string(),
        Key::KeyQ => "q".to_string(),
        Key::KeyR => "r".to_string(),
        Key::KeyS => "s".to_string(),
        Key::KeyT => "t".to_string(),
        Key::KeyU => "u".to_string(),
        Key::KeyV => "v".to_string(),
        Key::KeyW => "w".to_string(),
        Key::KeyX => "x".to_string(),
        Key::KeyY => "y".to_string(),
        Key::KeyZ => "z".to_string(),
        Key::Num0 => "0".to_string(),
        Key::Num1 => "1".to_string(),
        Key::Num2 => "2".to_string(),
        Key::Num3 => "3".to_string(),
        Key::Num4 => "4".to_string(),
        Key::Num5 => "5".to_string(),
        Key::Num6 => "6".to_string(),
        Key::Num7 => "7".to_string(),
        Key::Num8 => "8".to_string(),
        Key::Num9 => "9".to_string(),
        Key::Space => "space".to_string(),
        Key::Return => "return".to_string(),
        Key::Tab => "tab".to_string(),
        Key::Escape => "escape".to_string(),
        Key::Backspace => "backspace".to_string(),
        Key::Delete => "delete".to_string(),
        Key::UpArrow => "up".to_string(),
        Key::DownArrow => "down".to_string(),
        Key::LeftArrow => "left".to_string(),
        Key::RightArrow => "right".to_string(),
        Key::F1 => "f1".to_string(),
        Key::F2 => "f2".to_string(),
        Key::F3 => "f3".to_string(),
        Key::F4 => "f4".to_string(),
        Key::F5 => "f5".to_string(),
        Key::F6 => "f6".to_string(),
        Key::F7 => "f7".to_string(),
        Key::F8 => "f8".to_string(),
        Key::F9 => "f9".to_string(),
        Key::F10 => "f10".to_string(),
        Key::F11 => "f11".to_string(),
        Key::F12 => "f12".to_string(),
        other => format!("{:?}", other).to_lowercase(),
    }
}

fn main() {
    let config = load_config();
    let shortcuts = parse_shortcuts(&config);

    for shortcut in shortcuts {
        for key in &shortcut.keys {
            println!("Shortcut '{}' includes key '{}'", shortcut.label, key);
        }
    }

    loop {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    }



}
