use crate::parser::parse_file;

mod types;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        eprintln!("You need to provide arguments. Use --help to see possible arguments.");
        std::process::exit(1);
    }

    let mut _valid_arg_given = false;

    if args.contains(&"--help".to_string()) {
        _valid_arg_given = true;

        println!("Available commands: \n\
        \t --help: \t\t\tDisplay available commands\n\
        \t --from <path>: \t\tParse a BitWarden export from the provided path");

        std::process::exit(0);
    }

    if args.contains(&"--from".to_string()) {
        _valid_arg_given = true;

        let from_path_option = get_argument_value(&args, "--from");
        if from_path_option.is_none() {
            eprintln!("'--from' requires a value to be given!");
            std::process::exit(1);
        }

        let from_path = from_path_option.unwrap();
        println!("Reading from '{}'", from_path);

        parse_file(std::path::Path::new(&from_path));
    }

    if !_valid_arg_given {
        eprintln!("Invalid argument given. Run with '--help' for a list of available commands.");
        std::process::exit(1);
    }
}

fn get_argument_value(args: &Vec<String>, arg_name: &str) -> Option<String> {
    for i in 0..args.len() {
        if args.get(i).unwrap().eq(&arg_name.to_string()) {
            if i + 1 > args.len() {
                return None;
            }
            let val = args.get(i+1);
            if val.is_none() {
                return None;
            }

            return Some(val.unwrap().clone());
        }
    }

    return None;
}
