use crate::models::{ConfigLine, SshHost};

pub fn ssh_host_to_config_string(host: &SshHost) -> String {
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

pub fn create_ssh_config_string_to_add(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_host_to_config_string_basic() {
        let host = SshHost {
            host: "myserver".to_string(),
            hostname: Some("example.com".to_string()),
            user: Some("ubuntu".to_string()),
            port: Some(22),
            identity_file: None,
            lines: vec![
                ConfigLine::Option {
                    key: "HostName".to_string(),
                    value: "example.com".to_string(),
                },
                ConfigLine::Option {
                    key: "User".to_string(),
                    value: "ubuntu".to_string(),
                },
                ConfigLine::Option {
                    key: "Port".to_string(),
                    value: "22".to_string(),
                },
            ],
        };

        let result = ssh_host_to_config_string(&host);
        assert!(result.contains("Host myserver"));
        assert!(result.contains("    HostName example.com"));
        assert!(result.contains("    User ubuntu"));
        assert!(result.contains("    Port 22"));
    }

    #[test]
    fn test_ssh_host_to_config_string_with_comment() {
        let host = SshHost {
            host: "myserver".to_string(),
            hostname: Some("example.com".to_string()),
            user: None,
            port: None,
            identity_file: None,
            lines: vec![
                ConfigLine::Comment("# Production server".to_string()),
                ConfigLine::Option {
                    key: "HostName".to_string(),
                    value: "example.com".to_string(),
                },
            ],
        };

        let result = ssh_host_to_config_string(&host);
        assert!(result.contains("    # Production server"));
        assert!(result.contains("    HostName example.com"));
    }

    #[test]
    fn test_create_ssh_config_string_minimal() {
        let result = create_ssh_config_string_to_add("test", "example.com", 22, "", "");

        assert!(result.contains("Host test"));
        assert!(result.contains("    HostName example.com"));
        assert!(result.contains("    Port 22"));
        assert!(!result.contains("User"));
        assert!(!result.contains("IdentityFile"));
    }

    #[test]
    fn test_create_ssh_config_string_full() {
        let result = create_ssh_config_string_to_add(
            "myserver",
            "example.com",
            2222,
            "admin",
            "~/.ssh/id_rsa",
        );

        assert!(result.contains("Host myserver"));
        assert!(result.contains("    HostName example.com"));
        assert!(result.contains("    Port 2222"));
        assert!(result.contains("    User admin"));
        assert!(result.contains("    IdentityFile ~/.ssh/id_rsa"));
    }

    #[test]
    fn test_create_ssh_config_string_custom_port() {
        let result = create_ssh_config_string_to_add("test", "example.com", 8022, "user", "");

        assert!(result.contains("    Port 8022"));
        assert!(result.contains("    User user"));
        assert!(!result.contains("IdentityFile"));
    }
}
