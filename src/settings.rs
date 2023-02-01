use config::Config;
use dirs;
use std::{
    env,
    io::{self, Write},
};

pub struct Settings {
    configuration: Config,
}

impl Settings {
    pub fn new(&self) -> Settings {
        let config = self.read();
        Settings {
            configuration: config,
        }
    }

    fn read(&self) -> Config {
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
        let config = Config::builder()
            .add_source(config::File::with_name(&settings_path))
            .build()
            .unwrap();
        return config;
    }
    // TODO figure out if configuration.get must always return a string
    pub fn get(&self, name: &str) -> String {
        // This is deprecated! https://docs.rs/config/latest/config/struct.Config.html#method.get
        let options: Vec<String> = self.configuration.get(name).unwrap();
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
}