use crate::models::{SshConfigToAdd, SshHost};
use dialoguer::{Input, Select};

pub fn input_ssh_config_to_add(existing_hosts: &[SshHost]) -> SshConfigToAdd {
    // Loop until we get a non-duplicate name
    let name = loop {
        let input_name = Input::<String>::new()
            .with_prompt("Host alias (used as `ssh <alias>`)")
            .interact_text()
            .unwrap();

        // Check for duplicates
        let is_duplicate = existing_hosts.iter().any(|h| h.host == input_name);

        if is_duplicate {
            eprintln!(
                "Error: Host alias '{}' already exists. Please enter a different name.",
                input_name
            );
        } else {
            break input_name;
        }
    };

    let address = Input::<String>::new()
        .with_prompt("HostName (IP or domain)")
        .interact_text()
        .unwrap();
    let port = Input::<u16>::new()
        .with_prompt("Port (default 22)")
        .default(22)
        .interact_text()
        .unwrap();
    let user = Input::<String>::new()
        .with_prompt("User (leave empty to use current user)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    let identity_file = Input::<String>::new()
        .with_prompt("IdentityFile path (e.g. ~/.ssh/id_rsa, optional)")
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

pub fn select_from_list(items: Vec<&str>) -> Option<usize> {
    Select::new()
        .with_prompt("Select SSH Host")
        .items(&items)
        .interact_opt()
        .unwrap()
}
