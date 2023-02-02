use std::process::Command;
use colored::Colorize;

pub fn add_worktree(key: String, ticket: String) {
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
