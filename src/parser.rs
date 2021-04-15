use std::path::Path;
use crate::types::ExportFile;
use std::collections::HashMap;

pub fn parse_file(path: &Path) {
    let file_as_str = std::fs::read_to_string(path);
    if file_as_str.is_err() {
        eprintln!("An error occurred while reading the file: {:?}", file_as_str.err().unwrap());
        std::process::exit(1);
    }

    let export_file: serde_json::error::Result<ExportFile> = serde_json::from_str::<ExportFile>(&file_as_str.unwrap());
    if export_file.is_err() {
        eprintln!("Failed to parse file: {:?}", export_file.err().unwrap());
        std::process::exit(1);
    }

    let parsed_data = export_file.unwrap();
    if parsed_data.encrypted {
        eprintln!("It appears the file is encrypted. You need to export unencrypted from BitWarden!");
        std::process::exit(1);
    }

    let mut passwords_with_sites: HashMap<String, Vec<String>> = HashMap::new();
    for item in parsed_data.items {
        if item.login.password.is_none() {
            continue;
        }

        let password = item.login.password.unwrap();

        if passwords_with_sites.get(&password) == None {
            passwords_with_sites.insert(password, vec![item.name]);
        } else {
            let mut val = passwords_with_sites.remove(&password).unwrap();
            val.push(item.name);

            passwords_with_sites.insert(password, val);
        }
    }

    let mut reused_passwords: Vec<String> = vec![];
    let mut _has_duplicate_passwords = false;
    for (k, v) in passwords_with_sites {
        if  v.len() > 1 {
            println!("\nDuplicate password '{}'. Found {} duplicates:", k, v.len());
            for site in v {
                println!("\t{}", site);
            }

            _has_duplicate_passwords = true;
            reused_passwords.push(format!("'{}'", k));
        }
    }

    if _has_duplicate_passwords {
        println!("You have used the following passwords on multiple different sites: {}", reused_passwords.join(", "));
    } else {
        println!("Congratulations! You do not have any duplicate passwords!");
    }
}