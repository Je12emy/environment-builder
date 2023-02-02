use config::{Config, Value};
use dirs;
use std::{
    env,
    io::{self, Write},
};

pub fn read_settings() -> Config {
    // Check OS and set path
    let mut _settings_path = String::new();
    let home_dir = dirs::home_dir().unwrap();
    let os = env::consts::OS;
    if os == "windows" {
        _settings_path = format!(
            "{}\\.environment-builder\\settings.yaml",
            home_dir.to_str().unwrap()
        );
    } else {
        _settings_path = format!(
            "{}/environment-builder/settings.yaml",
            home_dir.to_string_lossy()
        );
    }
    let config = Config::builder()
        .add_source(config::File::with_name(&_settings_path))
        .build()
        .unwrap();
    return config;
}

pub fn pick_option(options: &Vec<Value>) -> String {
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
