pub mod languages;

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub language: String,
}

#[derive(Debug)]
enum LoadError {
    OpenFile,
    ReadFile,
    Format,
}

impl Settings {
    pub fn path() -> std::path::PathBuf {
        let mut path = std::env::current_dir().unwrap();

        path.push("data/settings.json");

        path
    }

    pub fn save(&self) {
        println!("Saving settings");

        let json = serde_json::to_string_pretty(&self).unwrap();

        let path = Self::path();

        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).unwrap();
        }

        let mut file = std::fs::File::create(path).unwrap();

        file.write_all(json.as_bytes()).unwrap();
    }

    pub async fn load() -> Settings {
        fn get_saved_settings() -> Result<Settings, LoadError> {
            let mut contents = String::new();

            let mut file = std::fs::File::open(Settings::path()).map_err(|_| LoadError::OpenFile)?;

            file.read_to_string(&mut contents).map_err(|_| LoadError::ReadFile)?;

            serde_json::from_str(&contents).map_err(|_| LoadError::Format)
        }

        println!("Loading settings");

        get_saved_settings().unwrap_or_else(|error| {
            println!("Failed to load settings: {:?}", error);
            Self::default()
        })
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            language: "en".to_string(),
        }
    }
}