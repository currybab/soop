use crate::models::{ConfigLine, SshHost};
use regex::Regex;
use std::fs;

pub fn parse_ssh_config(path: &str) -> Result<Vec<SshHost>, std::io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(split_by_host(&content))
}

pub fn split_by_host(content: &str) -> Vec<SshHost> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_host() {
        let config = r#"Host myserver
    HostName example.com
    User ubuntu
    Port 22"#;

        let hosts = split_by_host(config);
        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].host, "myserver");
        assert_eq!(hosts[0].hostname, Some("example.com".to_string()));
        assert_eq!(hosts[0].user, Some("ubuntu".to_string()));
        assert_eq!(hosts[0].port, Some(22));
    }

    #[test]
    fn test_parse_multiple_hosts() {
        let config = r#"Host server1
    HostName example.com
    User admin

Host server2
    HostName test.com
    Port 2222"#;

        let hosts = split_by_host(config);
        assert_eq!(hosts.len(), 2);
        assert_eq!(hosts[0].host, "server1");
        assert_eq!(hosts[1].host, "server2");
        assert_eq!(hosts[1].port, Some(2222));
    }

    #[test]
    fn test_parse_with_comments() {
        let config = r#"Host myserver
    # This is a comment
    HostName example.com
    User ubuntu"#;

        let hosts = split_by_host(config);
        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].lines.len(), 3);

        match &hosts[0].lines[0] {
            ConfigLine::Comment(c) => assert_eq!(c, "# This is a comment"),
            _ => panic!("Expected comment"),
        }
    }

    #[test]
    fn test_parse_empty_config() {
        let config = "";
        let hosts = split_by_host(config);
        assert_eq!(hosts.len(), 0);
    }

    #[test]
    fn test_parse_with_identity_file() {
        let config = r#"Host myserver
    HostName example.com
    IdentityFile ~/.ssh/id_rsa"#;

        let hosts = split_by_host(config);
        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].identity_file, Some("~/.ssh/id_rsa".to_string()));
    }

    #[test]
    fn test_parse_custom_port() {
        let config = r#"Host myserver
    HostName example.com
    Port 8022"#;

        let hosts = split_by_host(config);
        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].port, Some(8022));
    }
}
