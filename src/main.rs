use clap::{Parser, Subcommand};
use dialoguer::{Input, Select};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    /// Add SSH host
    Add,
    /// List SSH hosts
    Ls,
    /// Connect to SSH host
    Connect {
        #[arg(value_name = "HOST")]
        host: Option<String>,
    },
    /// Edit SSH host @TODO
    Edit {
        #[arg(value_name = "HOST")]
        host: Option<String>,
    },
    /// Remove SSH host @TODO
    #[command(visible_alias = "rm")]
    Remove {
        #[arg(value_name = "HOST")]
        host: Option<String>,
    },
}

#[derive(Debug, Clone)]
struct SshHost {
    host: String,
    hostname: Option<String>,
    user: Option<String>,
    port: Option<u16>,
    identity_file: Option<String>,
    other_options: HashMap<String, String>,
}

fn parse_ssh_config(path: &str) -> Result<Vec<SshHost>, std::io::Error> {
    let content = fs::read_to_string(path)?;

    let hosts = split_by_host(&content);

    Ok(hosts)
}

fn split_by_host(content: &str) -> Vec<SshHost> {
    let mut hosts = Vec::new();
    let host_regex = Regex::new(r"^Host\s+(.+)$").unwrap();
    let key_regex = Regex::new(r"^(.+)\s+(.+)$").unwrap();
    for line in content.lines() {
        let trimmed = line.trim();
        if host_regex.is_match(trimmed) {
            let host = host_regex.captures(trimmed).unwrap()[1].to_string();
            hosts.push(SshHost {
                host,
                hostname: None,
                user: None,
                port: None,
                identity_file: None,
                other_options: HashMap::new(),
            });
        } else if !trimmed.is_empty() {
            if let Some(captured) = key_regex.captures(trimmed) {
                let key = captured[1].to_string();
                let value = captured[2].to_string();

                if let Some(host) = hosts.last_mut() {
                    match key.as_str() {
                        "HostName" => host.hostname = Some(value),
                        "User" => host.user = Some(value),
                        "Port" => {
                            if let Ok(port) = value.parse() {
                                host.port = Some(port);
                            }
                        }
                        "IdentityFile" => host.identity_file = Some(value),
                        _ => {
                            host.other_options.insert(key, value);
                        }
                    }
                }
            }
        }
    }
    hosts
}

#[derive(Debug, Clone)]
struct SshConfigToAdd {
    name: String,
    address: String,
    user: String,
    port: u16,
    identity_file: String,
}

fn input_ssh_config_to_add() -> SshConfigToAdd {
    let name = Input::<String>::new()
        .with_prompt("Enter host name")
        .interact_text()
        .unwrap();
    let address = Input::<String>::new()
        .with_prompt("Enter host address")
        .interact_text()
        .unwrap();
    let port = Input::<u16>::new()
        .with_prompt("Enter port")
        .default(22)
        .interact_text()
        .unwrap();
    let user = Input::<String>::new()
        .with_prompt("Enter user")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    let identity_file = Input::<String>::new()
        .with_prompt("Enter identity file")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    SshConfigToAdd {
        name,
        address,
        user,
        port,
        identity_file,
    }
}

fn create_ssh_config_string_to_add(
    name: &str,
    address: &str,
    port: u16,
    user: &str,
    identity_file: &str,
) -> String {
    let mut entry = format!(
        "Host {}\n    HostName {}\n    Port {}\n",
        name, address, port
    );
    if !user.is_empty() {
        entry.push_str(&format!("    User {}\n", user));
    }
    if !identity_file.is_empty() {
        entry.push_str(&format!("    IdentityFile {}\n", identity_file));
    }
    entry.push('\n');
    entry
}

fn add_ssh_config_entry(path: &str, entry: &str) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    file.write_all(entry.as_bytes()).unwrap();
}

fn select_from_list(items: Vec<&str>) -> Option<usize> {
    Select::new()
        .with_prompt("Select SSH Host")
        .items(&items)
        .interact_opt()
        .unwrap()
}

fn main() {
    let home_path = std::env::var("HOME").unwrap();
    let ssh_config_path = format!("{}/.ssh/config", home_path);

    let cli = Cli::parse();

    if cli.command == Commands::Add {
        let ssh_config_to_add = input_ssh_config_to_add();

        let entry = create_ssh_config_string_to_add(
            &ssh_config_to_add.name,
            &ssh_config_to_add.address,
            ssh_config_to_add.port,
            &ssh_config_to_add.user,
            &ssh_config_to_add.identity_file,
        );

        add_ssh_config_entry(&ssh_config_path, &entry);

        println!(
            "Added host '{}' to {}",
            ssh_config_to_add.name, ssh_config_path
        );
    } else {
        let hosts = parse_ssh_config(&ssh_config_path).unwrap();
        if hosts.is_empty() {
            println!("No SSH hosts found");
            return;
        }
        let host_names: Vec<&str> = hosts.iter().map(|h| h.host.as_str()).collect();
        let selection: Option<usize> = match cli.command {
            Commands::Ls => select_from_list(host_names),
            Commands::Connect { host } | Commands::Edit { host } | Commands::Remove { host } => {
                if let Some(host) = host {
                    host_names.iter().position(|&h| h == host)
                } else {
                    select_from_list(host_names)
                }
            }
            _ => {
                eprintln!("Invalid command");
                None
            }
        };

        if let Some(index) = selection {
            let selected_host = &hosts[index].host;
            println!("Connecting to {}", selected_host);
            let status = Command::new("ssh")
                .arg(selected_host)
                .status()
                .expect("Failed to execute ssh");

            if !status.success() {
                eprintln!("SSH connection failed");
            }
        }
    }
}
