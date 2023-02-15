use colored::Colorize;
use environment_builder::{
    commands,
    jira::JiraTicket,
    settings::{self, RepositoryManagementMethod},
};
use std::{
    env,
    io::{self, Write},
    path::Path,
};

fn main() {
    println!("Environment Builder");
    // Read settings
    let settings = match settings::read_settings() {
        Ok(s) => s,
        Err(_) => panic!("Settings where not found"),
    };

    // Get repositories
    let repositories = settings.repositories;
    if repositories.len() == 0 {
        println!("{}", "No repositories found in settings.toml".red());
        return;
    }

    // Pick a repository
    let selected_repository = settings::pick_repository(&repositories);

    // Change directories
    let root_path = Path::new(&selected_repository.path);
    env::set_current_dir(&root_path).expect("An error ocurred while changing directories");

    // Get ticket key
    let keys = selected_repository.keys.clone();
    if keys.len() == 0 {
        println!("{}", "No repositories found in settings.toml".red());
        return;
    }
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
    let ticket = ticket.parse::<u32>();
    let ticket = match ticket {
        Ok(ticket) => ticket,
        Err(_) => {
            println!("{}", "Please enter a number".red());
            return;
        }
    };
    let jira_ticket = JiraTicket::new(key, ticket);

    match selected_repository.auto_pull {
        Some(should_update) => {
            if should_update {
                commands::update_repository(&selected_repository.base_branch)
            }
        }
        None => (),
    }

    let management_method = selected_repository.method.clone();
    match management_method {
        Some(method) => match method {
            RepositoryManagementMethod::Worktree => {
                commands::add_worktree(&jira_ticket, &selected_repository.base_branch);
            }
            RepositoryManagementMethod::Branch => {
                commands::create_branch(&jira_ticket, &selected_repository.base_branch);
            }
        },
        None => {
            commands::create_branch(&jira_ticket, &selected_repository.base_branch);
        }
    }

    match selected_repository.set_remote {
        Some(should_set_remote) => {
            if should_set_remote {
                commands::set_upstream(&jira_ticket);
            }
        }
        None => (),
    }

    // Ask to open VSCode
    loop {
        let mut open_vscode = String::new();
        print!("Open VSCode? (y/n): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut open_vscode)
            .expect("Error reading answer");
        let open_vscode = open_vscode.trim();
        if open_vscode == "y" {
            commands::open_vscode(Some(&jira_ticket));
            break;
        } else if open_vscode == "n" {
            break;
        } else {
            println!("Invalid option");
            continue;
        }
    }
    println!("{}", "All done!".green());
}
