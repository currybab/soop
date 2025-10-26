use clap::{Parser, Subcommand};
use dialoguer::{Confirm, Input, Select};
use regex::Regex;
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
    #[command(visible_alias = "c")]
    #[command(visible_alias = "ssh")]
    Connect {
        #[arg(value_name = "HOST")]
        host: Option<String>,
    },
    /// Edit SSH host
    Edit {
        #[arg(value_name = "HOST")]
        host: Option<String>,
    },
    /// Remove SSH host
    #[command(visible_alias = "rm")]
    Remove {
        #[arg(value_name = "HOST")]
        host: Option<String>,
    },
}

#[derive(Debug, Clone)]
enum ConfigLine {
    Option { key: String, value: String },
    Comment(String),
}

#[derive(Debug, Clone)]
struct SshHost {
    host: String,
    hostname: Option<String>,
    user: Option<String>,
    port: Option<u16>,
    identity_file: Option<String>,
    lines: Vec<ConfigLine>,
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
                lines: Vec::new(),
            });
        } else if !trimmed.is_empty() {
            if let Some(host) = hosts.last_mut() {
                // Check if it's a comment
                if trimmed.starts_with('#') {
                    host.lines.push(ConfigLine::Comment(trimmed.to_string()));
                } else if let Some(captured) = key_regex.captures(trimmed) {
                    let key = captured[1].to_string();
                    let value = captured[2].to_string();

                    // Store in lines
                    host.lines.push(ConfigLine::Option {
                        key: key.clone(),
                        value: value.clone(),
                    });

                    // Also parse for quick access
                    match key.as_str() {
                        "HostName" => host.hostname = Some(value),
                        "User" => host.user = Some(value),
                        "Port" => {
                            if let Ok(port) = value.parse() {
                                host.port = Some(port);
                            }
                        }
                        "IdentityFile" => host.identity_file = Some(value),
                        _ => {}
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

fn input_ssh_config_to_add(existing_hosts: &[SshHost]) -> SshConfigToAdd {
    // Loop until we get a non-duplicate name
    let name = loop {
        let input_name = Input::<String>::new()
            .with_prompt("Enter host name")
            .interact_text()
            .unwrap();

        // Check for duplicates
        let is_duplicate = existing_hosts.iter().any(|h| h.host == input_name);

        if is_duplicate {
            eprintln!(
                "Error: Host '{}' already exists. Please enter a different name.",
                input_name
            );
        } else {
            break input_name;
        }
    };

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

fn ssh_host_to_config_string(host: &SshHost) -> String {
    let mut config = format!("Host {}\n", host.host);

    // Output lines in order (preserves comments and order)
    for line in &host.lines {
        match line {
            ConfigLine::Option { key, value } => {
                config.push_str(&format!("    {} {}\n", key, value));
            }
            ConfigLine::Comment(comment) => {
                config.push_str(&format!("    {}\n", comment));
            }
        }
    }

    config.push('\n');
    config
}

fn create_ssh_config_string_to_add(
    name: &str,
    address: &str,
    port: u16,
    user: &str,
    identity_file: &str,
) -> String {
    let mut lines = Vec::new();

    // Add config lines in standard order
    lines.push(ConfigLine::Option {
        key: "HostName".to_string(),
        value: address.to_string(),
    });
    lines.push(ConfigLine::Option {
        key: "Port".to_string(),
        value: port.to_string(),
    });
    if !user.is_empty() {
        lines.push(ConfigLine::Option {
            key: "User".to_string(),
            value: user.to_string(),
        });
    }
    if !identity_file.is_empty() {
        lines.push(ConfigLine::Option {
            key: "IdentityFile".to_string(),
            value: identity_file.to_string(),
        });
    }

    let host = SshHost {
        host: name.to_string(),
        hostname: Some(address.to_string()),
        port: Some(port),
        user: if !user.is_empty() {
            Some(user.to_string())
        } else {
            None
        },
        identity_file: if !identity_file.is_empty() {
            Some(identity_file.to_string())
        } else {
            None
        },
        lines,
    };

    ssh_host_to_config_string(&host)
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

fn remove_ssh_host(path: &str, hosts: &[SshHost], index_to_remove: usize) -> std::io::Result<()> {
    let mut new_config = String::new();

    for (i, host) in hosts.iter().enumerate() {
        if i != index_to_remove {
            new_config.push_str(&ssh_host_to_config_string(host));
        }
    }

    fs::write(path, new_config)?;
    Ok(())
}

fn edit_ssh_host(
    config_path: &str,
    hosts: &[SshHost],
    index_to_edit: usize,
) -> std::io::Result<()> {
    let host_to_edit = &hosts[index_to_edit];
    let original_host_name = &host_to_edit.host;

    // Create temporary file
    let home_path = std::env::var("HOME").unwrap();
    let temp_file_path = format!("{}/.ssh/tmp_host_edit", home_path);

    // Write current host config to temp file
    let host_config = ssh_host_to_config_string(host_to_edit);
    fs::write(&temp_file_path, &host_config)?;

    // Get editor from environment or use default
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    // Open editor
    let status = Command::new(&editor)
        .arg(&temp_file_path)
        .status()
        .expect("Failed to open editor");

    if !status.success() {
        eprintln!("Editor exited with error");
        fs::remove_file(&temp_file_path)?;
        return Ok(());
    }

    // Read edited content
    let edited_content = fs::read_to_string(&temp_file_path)?;

    // Check if there are any changes
    if edited_content.trim() == host_config.trim() {
        println!("No changes made");
        fs::remove_file(&temp_file_path)?;
        return Ok(());
    }

    // Parse edited host
    let edited_hosts = split_by_host(&edited_content);

    if edited_hosts.is_empty() {
        eprintln!("Error: No valid host configuration found in edited file");
        fs::remove_file(&temp_file_path)?;
        return Ok(());
    }

    if edited_hosts.len() > 1 {
        eprintln!("Error: Multiple hosts found in edited file. Please edit only one host.");
        fs::remove_file(&temp_file_path)?;
        return Ok(());
    }

    let edited_host = &edited_hosts[0];
    let new_host_name = &edited_host.host;

    // Validate: check for duplicate host names (unless name didn't change)
    if new_host_name != original_host_name {
        for (i, host) in hosts.iter().enumerate() {
            if i != index_to_edit && &host.host == new_host_name {
                eprintln!(
                    "Error: Host name '{}' already exists. Edit cancelled.",
                    new_host_name
                );
                fs::remove_file(&temp_file_path)?;
                return Ok(());
            }
        }
    }

    // Update config: replace the edited host
    let mut new_config = String::new();
    for (i, host) in hosts.iter().enumerate() {
        if i == index_to_edit {
            new_config.push_str(&ssh_host_to_config_string(edited_host));
        } else {
            new_config.push_str(&ssh_host_to_config_string(host));
        }
    }

    fs::write(config_path, new_config)?;
    fs::remove_file(&temp_file_path)?;

    println!("Host '{}' updated successfully", new_host_name);
    Ok(())
}

fn main() {
    let home_path = std::env::var("HOME").unwrap();
    let ssh_config_path = format!("{}/.ssh/config", home_path);

    let cli = Cli::parse();

    // Parse existing hosts once (for all commands)
    let hosts = parse_ssh_config(&ssh_config_path).unwrap_or_else(|_| Vec::new());

    match &cli.command {
        Commands::Add => {
            let ssh_config_to_add = input_ssh_config_to_add(&hosts);

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
        }
        Commands::Ls => {
            if hosts.is_empty() {
                eprintln!("No SSH hosts found");
                return;
            }

            for host in &hosts {
                let mut info = String::new();

                // Add user if present
                if let Some(user) = &host.user {
                    info.push_str(user);
                    info.push('@');
                }

                // Add hostname if present
                if let Some(hostname) = &host.hostname {
                    info.push_str(hostname);
                }

                // Add port if present and not default (22)
                if let Some(port) = host.port {
                    if port != 22 {
                        info.push(':');
                        info.push_str(&port.to_string());
                    }
                }

                // Print with or without info
                if info.is_empty() {
                    println!("{}", host.host);
                } else {
                    println!("{:<20} {}", host.host, info);
                }
            }
        }
        Commands::Connect { host } | Commands::Edit { host } | Commands::Remove { host } => {
            if hosts.is_empty() {
                println!("No SSH hosts found");
                return;
            }

            let host_names: Vec<&str> = hosts.iter().map(|h| h.host.as_str()).collect();

            let selection: Option<usize> = if let Some(host) = host {
                host_names.iter().position(|&h| h == host)
            } else {
                select_from_list(host_names)
            };

            if let Some(index) = selection {
                let selected_host = &hosts[index].host;
                match &cli.command {
                    Commands::Connect { .. } => {
                        println!("Connecting to {}", selected_host);
                        let status = Command::new("ssh")
                            .arg(selected_host)
                            .status()
                            .expect("Failed to execute ssh");

                        if !status.success() {
                            eprintln!("SSH connection failed");
                        }
                    }
                    Commands::Edit { .. } => {
                        if let Err(e) = edit_ssh_host(&ssh_config_path, &hosts, index) {
                            eprintln!("Failed to edit host: {}", e);
                        }
                    }
                    Commands::Remove { .. } => {
                        let confirmed = Confirm::new()
                            .with_prompt(format!(
                                "Are you sure you want to delete '{}'?",
                                selected_host
                            ))
                            .default(false)
                            .interact()
                            .unwrap();

                        if confirmed {
                            if let Err(e) = remove_ssh_host(&ssh_config_path, &hosts, index) {
                                eprintln!("Failed to remove host: {}", e);
                            } else {
                                println!(
                                    "Removed host '{}' from {}",
                                    selected_host, ssh_config_path
                                );
                            }
                        } else {
                            println!("Cancelled");
                        }
                    }
                    _ => {
                        eprintln!("Invalid command");
                    }
                }
            }
        }
    }
}
