use clap::{Parser, Subcommand};
use regex::Regex;
use std::collections::HashMap;
use std::fs;

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
    Add {
        /// Host name
        name: String,
        /// Host address
        address: String,
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

fn main() {
    let home_path = std::env::var("HOME").unwrap();
    let ssh_config_path = format!("{}/.ssh/config", home_path);
    let cli = Cli::parse();

    match cli.command {
        Commands::Ls => {
            let hosts = parse_ssh_config(&ssh_config_path).unwrap();
            for host in hosts {
                println!("{}", host.host);
            }
        }
        Commands::Add { name, address } => {
            let mut hosts = parse_ssh_config(&ssh_config_path).unwrap();
            hosts.push(SshHost {
                host: name,
                hostname: Some(address),
                user: None,
                port: None,
                identity_file: None,
                other_options: HashMap::new(),
            });
            // fs::write(&ssh_config_path, hosts)
        }
    }
}
//
