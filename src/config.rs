use std::path::PathBuf;

fn config_path() -> PathBuf {
    let dir = if cfg!(windows) {
        let appdata = std::env::var("APPDATA").unwrap_or_default();
        PathBuf::from(appdata).join("RustyNotes")
    } else {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join(".config").join("rustynotes")
    };

    std::fs::create_dir_all(&dir).ok();
    dir.join("config.yaml")
}

pub fn load_recent_vaults() -> Vec<PathBuf> {
    let path = config_path();
    match std::fs::read_to_string(&path) {
        Ok(content) => serde_yaml::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn save_recent_vaults(vaults: &[PathBuf]) {
    let path = config_path();
    if let Ok(content) = serde_yaml::to_string(vaults) {
        let _ = std::fs::write(&path, content);
    }
}
