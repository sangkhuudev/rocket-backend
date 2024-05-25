use clap::{Parser, Subcommand};
use rocket_backend::commands::create_user;

#[derive(Parser)]
#[command(version, about)]
pub struct BackendCli {
    #[command(subcommand)]
    command: MainCommand,
}

#[derive(Subcommand)]
enum MainCommand {
    /// User management
    #[command(subcommand)]
    Users(UserCommands),
}

#[derive(Subcommand)]
enum UserCommands {
    /// Create new user 
    Create {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
        #[arg(short, long)]
        roles: String,
    },
    /// List all users
    List,
    /// Delete a user by id
    Delete {
        id: i32,
    }
}

// Custom function to parse roles separated by commas
fn parse_roles(s: &str) -> Result<Vec<String>, String> {
    let roles: Vec<String> = s
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|r| !r.is_empty())
        .collect();

    // Check for any invalid characters in roles
    if roles.iter().any(|role| role.contains(|c: char| c.is_whitespace() || c == ',')) {
        return Err("Roles contain invalid characters".to_string());
    }
    Ok(roles)
}


#[tokio::main]
async fn main() {
    let backend_cli = BackendCli::parse();

    match backend_cli.command {
        MainCommand::Users(user_command) => {
            match user_command {
                UserCommands::Create { username, password, roles } => {
                    let role_codes = parse_roles(roles.as_str()).unwrap();
                    create_user(username, password, role_codes).await;
                },
                UserCommands::List => {
                    list_users();
                },
                UserCommands::Delete { id } => {
                    delete_user(id);
                }
            }
        },
    }
}


fn list_users() {
    // In a real application, this would fetch users from a database.
    println!("Listing users:");
    // Example: Iterate over users in a HashMap or a database
    // Placeholder example:
    println!("user1: role1");
    println!("user2: role2");
}

fn delete_user(id: i32) {
    // In a real application, this would interact with a database.
    println!("Deleting user: {}", id);
    // Example: Remove user from a HashMap or a database
}

