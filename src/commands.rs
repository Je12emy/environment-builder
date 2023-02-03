use colored::Colorize;
use std::{env, process::Command};

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

fn update_repository() {
    let update_command = Command::new("git")
        .arg("pull")
        .arg("origin")
        .arg("develop")
        .status()
        .expect("An error ocurred while running git pull");
    if update_command.success() {
        println!("{}", "Updated repository".green());
    } else {
        println!("{}", "Failed to update repository".red());
    }
}

pub fn set_upstream(ticket: &JiraTicket) {
    let set_upstream_command = Command::new("git")
        .arg("push")
        .arg("--set-upstream")
        .arg("origin")
        .arg(format!("feature/{}", ticket.to_string()))
        .status()
        .expect("An error ocurred while running git push");
    if set_upstream_command.success() {
        println!("{}", "Created new branch".green());
    } else {
        println!("{}", "Failed to create new branch".red());
    }
}

pub fn create_branch(ticket: &JiraTicket) {
    let branch_command = Command::new("git")
        .arg("branch")
        .arg(format!("feature/{}", ticket.to_string()))
        .status()
        .expect("An error ocurred while creating new branch");
    if branch_command.success() {
        println!("{}", "Created new branch".green());
        switch_branch(ticket)
    } else {
        println!("{}", "Failed to create new branch".red());
    }
}

fn switch_branch(ticket: &JiraTicket) {
    let switch_command = Command::new("git")
        .arg("switch")
        .arg(format!("feature/{}", ticket.to_string()))
        .status()
        .expect("An error ocurred while switching branch");
    if switch_command.success() {
        println!("{}", "Switched to new branch".green());
    } else {
        println!("{}", "Failed to switch branch".red());
    }
}

fn vs_code_command() -> String {
    let os = env::consts::OS;
    if os == "windows" {
        return "code.cmd".to_string();
    } else {
        return "code".to_string();
    }
}

pub fn open_vscode(ticket: Option<&JiraTicket>) {
    let path = match ticket {
        Some(path) => path.to_string(),
        None => ".".to_string(),
    };
    let vscode_command = Command::new(vs_code_command())
        .arg(path)
        .status()
        .expect("An error ocurred while opening vs code");
    if vscode_command.success() {
        println!("{}", "Opened VSCode".green());
    } else {
        println!("{}", "Failed to open VSCode".red());
    }
}
