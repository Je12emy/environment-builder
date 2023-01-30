use std::{
    env,
    io::{self, Write},
    path::Path,
    process::Command, fs::read,
};

use config::Config;

fn main() {
    println!("Environment Builder");
    // Read settings
    let settings = Config::builder()
        .add_source(config::File::with_name("settings.yaml"))
        .build()
        .unwrap();

    // Get path
    let path = read_property_list("paths", &settings);
    println!("Selected path: {}", path);
    // Change directories
    let root_path = Path::new(&path);
    env::set_current_dir(&root_path).expect("An error ocurred while changing directories");

    // Get ticket key
    let key = read_property_list("keys", &settings);   // Print selected key
    println!("Selected key: {}", key);
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
        println!("Created new worktree and branch");
    } else {
        println!("Failed to create new worktree");
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
