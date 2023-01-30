use std::{
    env,
    io::{self},
    path::Path,
    process::Command,
};

use config::Config;

fn main() {
    println!("Environment Builder");
    // Read settings
    let settings = Config::builder()
        .add_source(config::File::with_name("settings.yaml"))
        .build()
        .unwrap();
    // Print paths
    let paths: Vec<String> = settings.get("paths").unwrap();
    // Print items inside paths vec
    for (i, path) in paths.iter().enumerate() {
        println!("{}: {}", i, path);
    }    
    // Get user input
    let mut input = String::new();
    println!("Please enter a number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading number");
    let input = input.trim();
    // Convert input to usize
    let input: usize = input.parse().unwrap();
    // Print selected path
    println!("Selected path: {}", paths[input]);

    // Change directories
    let root_path = Path::new(&paths[input]);
    env::set_current_dir(&root_path).expect("An error ocurred while changing directories");

    // Get ticket key
    let keys: Vec<String> = settings.get("keys").unwrap();
    // Print items inside keys vec
    for (i, key) in keys.iter().enumerate() {
        println!("{}: {}", i, key);
    }
    // Get user input
    let mut input = String::new();
    println!("Please enter a number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading number");
    let input = input.trim();
    // Convert input to usize
    let input: usize = input.parse().unwrap();
    // Print selected key
    println!("Selected key: {}", keys[input]);

    // Get a ticket number
    let mut ticket = String::new();
    println!("Please enter a ticket number:");
    io::stdin()
        .read_line(&mut ticket)
        .expect("Error reading ticket number");
    let ticket = ticket.trim();

    // Run worktree
    let worktree_command = Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg(format!("{}-{}", &keys[input], ticket))
        .arg("-b")
        .arg(format!("feature/{}", keys[input]))
        .status()
        .expect("An error ocurred while running worktree add");
    if worktree_command.success() {
        println!("Created new worktree and branch");
    } else {
        println!("Failed to create new worktree");
    }
}
