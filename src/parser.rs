use std::path::Path;
use crate::types::ExportFile;
use anyhow::Result;
use std::fs;
use crate::Config;

#[derive(Clone)]
pub struct SiteResult {
    pub name:       String,
    pub password:   Option<String>,
    pub username:   Option<String>
}

pub fn parse_file(path: &Path, config: &Config) -> Result<Vec<SiteResult>> {
    let json = read_to_json(path)?;

    if json.encrypted {
        eprintln!("It appears the file is encrypted. You need to export unencrypted from BitWarden!");
        std::process::exit(1);
    }

    let (usernames, passwords): (Vec<_>, Vec<_>) = json.items.clone().into_iter()
        .map(|f| (f.login.username, f.login.password))
        .unzip();

    let sites: Vec<_> = json.items.into_iter()
        .map(|f| (f.name, f.login.username, f.login.password))
        .map(|(name, username, password)| {

            let username = if config.username && usernames.iter().filter(|f| f.as_deref().eq(&username.as_deref())).count() > 1 {
                username
            } else {
                None
            };

            let password = if config.password && passwords.iter().filter(|f| f.as_deref().eq(&password.as_deref())).count() > 1 {
                password
            } else {
                None
            };

            SiteResult { name, username, password }
        })
        .collect();

    Ok(sites)
}

fn read_to_json(p: &Path) -> Result<ExportFile> {
    let content = fs::read_to_string(p)?;
    let json = serde_json::from_str(&content)?;
    Ok(json)
}