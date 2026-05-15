use clap::Parser;
use dialoguer::Confirm;
use soop::cli::{Cli, Commands};
use soop::formatter::create_ssh_config_string_to_add;
use soop::operations::{add_ssh_config_entry, edit_ssh_host, remove_ssh_host};
use soop::parser::parse_ssh_config;
use soop::ui::{input_ssh_config_to_add, select_from_list};
use std::process::Command;

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
                eprintln!("No SSH hosts found");
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
                                "Are you sure you want to remove '{}'?",
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
                    _ => unreachable!("outer match already restricts to Connect/Edit/Remove"),
                }
            }
        }
    }
}
