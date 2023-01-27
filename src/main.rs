use std::{
    env,
    io::{self, Write},
    path::Path,
    process::Command,
};

fn main() {
    println!("Environment Builder");
    // Change directories
    let mut root = String::new();
    println!("Please enter a directory path: ");
    io::stdin()
        .read_line(&mut root)
        .expect("An error ocurred reading line");
    println!("Path is: {}", root);
    let root = root.trim();
    let root_path = Path::new(&root);
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
