use colored::Colorize;
use environment_builder::settings;
use std::{
    env,
    io::{self, Write},
    path::Path,
    process::Command,
};

fn main() {
    println!("Environment Builder");
    // Read settings
    let settings = settings::read_settings();
    // Get repositories
    let paths = settings.get_array("paths").unwrap();
    let selected_path = settings::pick_option(&paths);
    println!("Selected path: {}", selected_path.green());
    // Change directories
    let root_path = Path::new(&selected_path);
    env::set_current_dir(&root_path).expect("An error ocurred while changing directories");

    // Get ticket key
    let keys = settings.get_array("keys").unwrap();
    let key = settings::pick_option(&keys); // Print selected key
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
