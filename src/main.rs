use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;
use clap::{App, Arg};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

mod types;
mod parser;

pub struct Config {
    password:   bool,
    username:   bool
}

fn main() {
    let matches = app().get_matches();
    let input_file = matches.value_of("input").expect("Missing required value 'input'");

    let config = Config {
        password: matches.is_present("check-password"),
        username: matches.is_present("check-username"),
    };

    let result = match parser::parse_file(Path::new(input_file), &config) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Parsing failed: {:?}", e);
            std::process::exit(1);
        }
    };

    let duplicate_usernames: Vec<_> = result.clone().into_iter()
        .filter(|f| f.username.is_some())
        .map(|f| (f.name, f.username.expect("Cannot be None")))
        .map(|(name, username)| (username, name))

        .collect();
    let usernames = mapify(duplicate_usernames);

    let duplicate_passwords: Vec<_> = result.into_iter()
        .filter(|f| f.password.is_some())
        .map(|f| (f.name, f.password.expect("Cannot be None")))
        .map(|(name, password)| (password, name))
        .collect();
    let passwords = mapify(duplicate_passwords);

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut green = ColorSpec::new();
    green.set_fg(Some(Color::Green));
    let mut red = ColorSpec::new();
    red.set_fg(Some(Color::Red));
    let mut white = ColorSpec::new();
    white.set_fg(Some(Color::White));
    let mut yellow = ColorSpec::new();
    yellow.set_fg(Some(Color::Yellow));

    if !usernames.is_empty() {
        let _ = stdout.set_color(&red);
        let _ = writeln!(&mut stdout, "You have duplicate usernames:\n");

        for (username, names) in usernames {
            let _ = stdout.set_color(&green);
            let _ = write!(&mut stdout, "- Username '");
            let _ = stdout.set_color(&yellow);
            let _ = write!(&mut stdout, "{}", username);
            let _ = stdout.set_color(&green);
            let _ = writeln!(&mut stdout, "' is used at: ");
            let _ = stdout.set_color(&white);

            for name in names {
                let _ = writeln!(&mut stdout, "\t- {}", name);
            }
        }
    }

    if !passwords.is_empty() {
        let _ = stdout.set_color(&red);
        let _ = writeln!(&mut stdout, "\nYou have duplicate passwords: \n");

        for (password, names) in passwords {
            let _ = stdout.set_color(&green);
            let _ = write!(&mut stdout, "- Password '");
            let _ = stdout.set_color(&yellow);
            let _ = write!(&mut stdout, "{}", password);
            let _ = stdout.set_color(&green);
            let _ = writeln!(&mut stdout, "' is used at: ");
            let _ = stdout.set_color(&white);

            for name in names {
                let _ = writeln!(&mut stdout, "\t- {}", name);
            }
        }
    }
}

fn mapify<K: Hash + Eq, V: Hash>(vec: Vec<(K, V)>) -> HashMap<K, Vec<V>> {
    let mut result: HashMap<K, Vec<V>> = HashMap::new();
    for (k, v) in vec {
        match result.remove(&k) {
            Some(mut ev) => {
                ev.push(v);
                result.insert(k, ev);
            },
            None => {
                result.insert(k, vec![v]);
            }
        }
    }

    result
}

fn app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(Arg::with_name("input")
            .required(true)
            .takes_value(true)
            .value_name("input path")
            .long("input")
            .short("i")
            .help("Path to your unencrypted BitWarden export"))
        .arg(Arg::with_name("check-password")
            .required(false)
            .takes_value(false)
            .long("check-password")
            .short("p")
            .help("Check for duplicate passwords"))
        .arg(Arg::with_name("check-username")
            .required(false)
            .takes_value(false)
            .long("check-username")
            .short("u")
            .help("Check for duplicate usernames/E-Mail addresses"))
}
