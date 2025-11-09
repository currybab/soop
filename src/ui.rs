use crate::models::{SshConfigToAdd, SshHost};
use dialoguer::{Input, Select};

pub fn input_ssh_config_to_add(existing_hosts: &[SshHost]) -> SshConfigToAdd {
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

pub fn select_from_list(items: Vec<&str>) -> Option<usize> {
    Select::new()
        .with_prompt("Select SSH Host")
        .items(&items)
        .interact_opt()
        .unwrap()
}
