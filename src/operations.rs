use crate::formatter::ssh_host_to_config_string;
use crate::models::SshHost;
use crate::parser::split_by_host;
use std::fs;
use std::io::Write;
use std::process::Command;

pub fn add_ssh_config_entry(path: &str, entry: &str) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    file.write_all(entry.as_bytes()).unwrap();
}

pub fn remove_ssh_host(path: &str, hosts: &[SshHost], index_to_remove: usize) -> std::io::Result<()> {
    let mut new_config = String::new();

    for (i, host) in hosts.iter().enumerate() {
        if i != index_to_remove {
            new_config.push_str(&ssh_host_to_config_string(host));
        }
    }

    fs::write(path, new_config)?;
    Ok(())
}

pub fn edit_ssh_host(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ConfigLine;
    use std::fs;

    #[test]
    fn test_add_ssh_config_entry() {
        let path = format!("/tmp/test_add_{}", rand::random::<u32>());
        let entry = "Host test\n    HostName example.com\n\n";

        add_ssh_config_entry(&path, entry);

        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, entry);

        // Cleanup
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_remove_ssh_host() {
        let hosts = vec![
            SshHost {
                host: "server1".to_string(),
                hostname: Some("example.com".to_string()),
                user: None,
                port: None,
                identity_file: None,
                lines: vec![ConfigLine::Option {
                    key: "HostName".to_string(),
                    value: "example.com".to_string(),
                }],
            },
            SshHost {
                host: "server2".to_string(),
                hostname: Some("test.com".to_string()),
                user: None,
                port: None,
                identity_file: None,
                lines: vec![ConfigLine::Option {
                    key: "HostName".to_string(),
                    value: "test.com".to_string(),
                }],
            },
        ];

        let path = format!("/tmp/test_remove_{}", rand::random::<u32>());
        remove_ssh_host(&path, &hosts, 0).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("server2"));
        assert!(!content.contains("server1"));

        // Cleanup
        fs::remove_file(&path).ok();
    }
}
