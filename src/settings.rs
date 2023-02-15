use config::{Config, ConfigError};
use serde::Deserialize;
use std::{
    env,
    io::{self, Write},
};

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum RepositoryManagementMethod {
    #[serde(rename = "worktree")]
    Worktree,
    #[serde(rename = "branch")]
    Branch,
}

impl RepositoryManagementMethod {
    pub fn from_string(method: &str) -> RepositoryManagementMethod {
        match method {
            "worktree" => RepositoryManagementMethod::Worktree,
            "branch" => RepositoryManagementMethod::Branch,
            _ => RepositoryManagementMethod::Branch,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RepositorySettings {
    pub path: String,
    pub method: Option<RepositoryManagementMethod>,
    pub keys: Vec<String>,
    #[serde(rename = "auto-pull")]
    pub auto_pull: Option<bool>,
    #[serde(rename = "set-remote")]
    pub set_remote: Option<bool>,
    #[serde(rename = "base-branch")]
    pub base_branch: String,
}

#[derive(Debug, Deserialize)]
pub struct UserSettings {
    pub repositories: Vec<RepositorySettings>,
}

pub fn read_settings() -> Result<UserSettings, ConfigError> {
    // Check OS and set path
    let mut _settings_path = String::new();
    let home_dir = dirs::home_dir().unwrap();
    let os = env::consts::OS;
    if os == "windows" {
        _settings_path = format!(
            "{}\\.environment-builder\\settings-new.json",
            home_dir.to_str().unwrap()
        );
    } else {
        _settings_path = format!(
            "{}/.config/environment-builder/settings.json",
            home_dir.to_string_lossy()
        );
    }
    let config = Config::builder()
        .add_source(config::File::with_name(&_settings_path))
        .build()
        .unwrap();
    let config = config.try_deserialize();
    return config;
}

pub fn pick_repository(repositories: &Vec<RepositorySettings>) -> &RepositorySettings {
    // Get repositories
    let paths: Vec<String> = repositories
        .into_iter()
        .map(|repo| repo.path.clone())
        .collect();
    let selected_repository = pick_option(&paths);
    println!("Selected repository: {}", selected_repository);
    let selected_repository = repositories
        .into_iter()
        .find(|repo| repo.path == selected_repository)
        .unwrap()
        .clone();
    return selected_repository;
}

pub fn pick_option(options: &Vec<String>) -> String {
    loop {
        // Print options
        options.into_iter().enumerate().for_each(|(i, x)| {
            println!("{}: {}", i + 1, x);
        });

        let mut input = String::new();
        print!("Please pick an option: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading number");
        let input = input.trim();
        // Convert input to usize
        let input: usize = input.parse().unwrap();

        match options.get(input - 1) {
            Some(selected) => {
                break selected.to_string();
            }
            None => {
                println!("Invalid option");
                continue;
            }
        };
    }
}
