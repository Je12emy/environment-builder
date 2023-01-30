use colored::Colorize;
use config::Config;
use std::{
    env,
    io::{self, Write},
    path::Path,
    process::Command,
};

fn main() {
    println!("Environment Builder");
    // Read settings
    let settings = read_settings();
    // Get path
    let path = read_property_list("paths", &settings);
    println!("Selected path: {}", path.green());
    // Change directories
    let root_path = Path::new(&path);
    env::set_current_dir(&root_path).expect("An error ocurred while changing directories");

    // Get ticket key
    let key = read_property_list("keys", &settings); // Print selected key
    println!("Selected key: {}", key.green());
    // Get a ticket number
    let mut ticket = String::new();
    print!("Please enter a ticket number: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut ticket)
        .expect("Error reading ticket number");
    let ticket = ticket.trim();

    // Run worktree
    let worktree_command = Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg(format!("{}-{}", &key, ticket))
        .arg("-b")
        .arg(format!("feature/{}-{}", key, ticket))
        .status()
        .expect("An error ocurred while running worktree add");
    if worktree_command.success() {
        println!("{}", "Created new worktree and branch".green());
    } else {
        println!("{}", "Failed to create new worktree".red());
    }
}

fn read_property_list(property: &str, config: &Config) -> String {
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

fn read_settings() -> Config {
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
