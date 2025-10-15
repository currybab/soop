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

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all SSH hosts
    Ls,
    // Add SSH host
    Add,
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

fn main() {
    let home_path = std::env::var("HOME").unwrap();
    let ssh_config_path = format!("{}/.ssh/config", home_path);

    let cli = Cli::parse();

    match cli.command {
        Commands::Ls => {
            let hosts = parse_ssh_config(&ssh_config_path).unwrap();
            if hosts.is_empty() {
                println!("No SSH hosts found");
                return;
            }

            let host_names: Vec<&str> = hosts.iter().map(|h| h.host.as_str()).collect();

            let selection = Select::new()
                .with_prompt("Select SSH host to connect")
                .items(&host_names)
                .interact_opt()
                .unwrap();

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
        Commands::Add => {
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

            let mut entry = format!(
                "Host {}\n    HostName {}\n    Port {}\n",
                name, address, port,
            );
            if !user.trim().is_empty() {
                entry.push_str(&format!("    User {}\n", user));
            }
            if !identity_file.trim().is_empty() {
                entry.push_str(&format!("    IdentityFile {}\n", identity_file));
            }
            entry.push('\n');

            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&ssh_config_path)
                .unwrap();
            file.write_all(entry.as_bytes()).unwrap();

            println!("Added host '{}' to {}", name, ssh_config_path);
        }
    }
}
//
