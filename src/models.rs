#[derive(Debug, Clone, PartialEq)]
pub enum ConfigLine {
    Option { key: String, value: String },
    Comment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SshHost {
    pub host: String,
    pub hostname: Option<String>,
    pub user: Option<String>,
    pub port: Option<u16>,
    pub identity_file: Option<String>,
    pub lines: Vec<ConfigLine>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SshConfigToAdd {
    pub name: String,
    pub address: String,
    pub user: String,
    pub port: u16,
    pub identity_file: String,
}
