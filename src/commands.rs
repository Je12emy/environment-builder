use colored::Colorize;
use std::process::Command;

use crate::jira::JiraTicket;

pub fn add_worktree(ticket: &JiraTicket) {
    let worktree_command = Command::new("git")
        .arg("worktree")
        .arg("add")
        .arg(ticket.to_string())
        .arg("-b")
        .arg(format!("feature/{}", ticket.to_string()))
        .status()
        .expect("An error ocurred while running worktree add");
    if worktree_command.success() {
        println!("{}", "Created new worktree and branch".green());
    } else {
        println!("{}", "Failed to create new worktree".red());
    }
}

pub fn open_vscode(ticket: Option<&JiraTicket>) {
    let path = match ticket {
        Some(path) => path.to_string(),
        None => ".".to_string(),
    };
        let vscode_command = Command::new("code.cmd")
        .arg(path)
        .status()
        .expect("An error ocurred while opening vs code");
    if vscode_command.success() {
        println!("{}", "Opened VSCode".green());
    } else {
        println!("{}", "Failed to open VSCode".red());
    }
}
