use serde::Deserialize;
use rdev::{Event, EventType, Key, listen};

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
        EventType::KeyPress(Key::KeyA) => println!("User wrote {:?}", event.name),
        EventType::KeyPress(Key::KeyB) => println!("User wrote {:?}", event.name),
        EventType::KeyPress(Key::KeyC) => println!("User wrote {:?}", event.name),
        EventType::KeyPress(Key::MetaLeft) => println!("User wrote {:?}", event.name),
        _ => (),
    }
}

fn main() {
    let config = load_config();

    for shortcut in config.shortcuts {
        println!("Registering shortcut: {} -> {}", shortcut.keys, shortcut.command);
    }

    loop {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    }



}
