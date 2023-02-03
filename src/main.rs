use colored::Colorize;
use environment_builder::{commands, jira::JiraTicket, settings};
use std::{
    env,
    io::{self, Write},
    path::Path,
};

fn main() {
    println!("Environment Builder");
    // Read settings
    let settings = settings::read_settings();
    // Get repositories
    let repositories = settings.get_array("repositories");
    let repositories = match repositories {
        Ok(repositories) => repositories,
        Err(_) => {
            println!("{}","No repositories found in settings.toml".red());
            return;
        }
    };
    let selected_repository = settings::pick_option(&repositories);
    println!("Selected repository: {}", selected_repository.green());
    // Change directories
    let root_path = Path::new(&selected_repository);
    env::set_current_dir(&root_path).expect("An error ocurred while changing directories");

    // Get ticket key
    let keys = settings.get_array("keys");
    let keys = match keys {
        Ok(keys) => keys,
        Err(_) => {
            println!("{}","No keys found in settings.toml".red());
            return;
        }
    };
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
            println!("{}","Please enter a number".red());
            return;
        }
    };
    let jira_ticket = JiraTicket::new(key, ticket);
    // Run worktree
    commands::add_worktree(&jira_ticket);

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
