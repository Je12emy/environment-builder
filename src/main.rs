use std::{
    env,
    io::{self},
    path::Path,
    process::Command,
};

use config::Config;

fn main() {
    println!("Environment Builder");
    // Read toml file's settings
    let settings = Config::builder()
        .add_source(config::File::with_name("settings"))
        .build()
        .unwrap();
    let path: String = settings.get("path").unwrap();
    // Change directories
    let root_path = Path::new(&path);
    env::set_current_dir(&root_path).expect("An error ocurred while changing directories");

    // Get ticket key
    let mut key = String::new();
    println!("Please enter a JIRA ticket key:");
    io::stdin()
        .read_line(&mut key)
        .expect("Error reading JIRA ticket key");
    let key = key.trim();

    // Run worktree
    let worktree_command = Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg(key)
        .arg("-b")
        .arg(format!("feature/{}", key))
        .status()
        .expect("An error ocurred while running worktree add");
    if worktree_command.success() {
        println!("Created new worktree and branch");
    } else {
        println!("Failed to create new worktree");
    }
}
