use config::Config;
use dirs;
use std::{
    env,
    io::{self, Write},
};

pub fn get_property(property: &str, config: &Config) -> String {
    let option: String = config.get(property).unwrap();
    return option;
}

pub fn get_property_list(property: &str, config: &Config) -> String {
    let options: Vec<String> = config.get(property).unwrap();
    for (i, path) in options.iter().enumerate() {
        println!("{}: {}", i, path);
    }
    // Get user input
    let mut input = String::new();
    print!("Please pick an option: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading number");
    let input = input.trim();
    // Convert input to usize
    let input: usize = input.parse().unwrap();
    return options[input].clone();
}

pub fn get_settings() -> Config {
    // Check OS and set path
    let mut settings_path = String::new();
    let home_dir = dirs::home_dir().unwrap();
    let os = env::consts::OS;
    if os == "windows" {
        settings_path = format!(
            "{}\\.environment-builder\\settings.yaml",
            home_dir.to_str().unwrap()
        );
    } else {
        settings_path = format!(
            "{}/environment-builder/settings.yaml",
            home_dir.to_string_lossy()
        );
    }
    let settings = Config::builder()
        .add_source(config::File::with_name(&settings_path))
        .build()
        .unwrap();
    return settings;
}
