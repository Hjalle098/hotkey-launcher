use serde::Deserialize;

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

pub fn load_config() -> Config {
    let config_path = dirs::home_dir()
        .expect("could not find home directory")
        .join(".config")
        .join("hotkey-launcher")
        .join("config.json");

    let contents = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", config_path.display(), e));

    serde_json::from_str(&contents)
        .unwrap_or_else(|e| panic!("failed to parse config: {}", e))
}

fn main() {
    let config = load_config();

    for shortcut in config.shortcuts {
        println!("Registering shortcut: {} -> {}", shortcut.keys, shortcut.command);
    }
}
