use std::fs;
use std::process::{Command, Stdio, exit};
use std::io::Write;
use std::env::var;



const HELP: &str = "\
Usage: sh-aliases [OPTION]... [ALIAS] [COMMAND]
Add ALIAS of COMMAND.

Mandatory arguments to long options are mandatory for short options too.
  -e, --edit                 edit aliases using a text editor
  -h, --help                 display this help and exit
  -r, --remove=ALIAS         remove ALIAS
  -v, --version              display version information and exit

Exit status:
 0  if OK,
 1  if problems

Full documentation <https://github.com/j-morano/sh-aliases>\
";




/* Both config and aliases' files are key-value text files.
 * Keys and values are separated by " --> ".
 * Different key-value pairs are separated by newlines.
 */

const DEFAULT_CONFIG: &str = "\
aliases_fn --> $HOME/.local/share/sh-aliases/aliases.txt
";

const CONFIG_FN: &str = "$HOME/.config/sh-aliases.conf";



fn parse(path_or_contents: &str, are_contents: bool) -> std::collections::HashMap<String, String> {
    let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let contents: String;
    if are_contents {
        contents = path_or_contents.to_string();
    } else {
        contents = fs::read_to_string(path_or_contents)
            .expect("Should have been able to read the file");
    }
    for line in contents.lines() {
        let mut kv = line.split(" --> ");
        let key = kv.next().unwrap();
        let value = kv.next().unwrap();
        map.insert(key.to_string(), value.to_string());
    }
    map
}

fn write_aliases(aliases: &std::collections::HashMap<String, String>, aliases_fn: String) {
    let mut file = std::fs::File::create(aliases_fn).unwrap();
    for (key, value) in aliases {
        writeln!(file, "{} --> {}", key, value).unwrap();
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut aliases: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    let config_options: std::collections::HashMap<String, String>;
    if std::path::Path::new(CONFIG_FN).exists() {
        config_options = parse(CONFIG_FN, false);
    } else {
        config_options = parse(DEFAULT_CONFIG, true);
    }
    let mut aliases_fn = config_options.get("aliases_fn").unwrap().to_string();
    let home = var("HOME").unwrap();
    aliases_fn = aliases_fn.replace("$HOME", home.as_str());


    if !std::path::Path::new(aliases_fn.as_str()).exists() {
        match std::fs::File::create(aliases_fn.as_str()) {
            Ok(_) => {
                eprintln!("Created file '{}'", aliases_fn);
                exit(0);
            },
            Err(_) => {
                // Get parent directory of aliases_fn.
                let mut parent = aliases_fn.clone();
                parent.pop();
                // Try to create the parent directory.
                match std::fs::create_dir_all(parent.clone()) {
                    Ok(_) => {
                        match std::fs::File::create(aliases_fn.as_str()) {
                            Ok(_) => { },
                            Err(_) => {
                                eprintln!("Failed to create file '{}'", aliases_fn);
                                exit(1);
                            }
                        }
                    },
                    Err(_) => {
                        eprintln!("Failed to create directory '{}'", parent);
                        exit(1);
                    }
                }
            }
        }
    } else {
        let contents = fs::read_to_string(aliases_fn.as_str())
            .expect("Should have been able to read the file");
        for line in contents.lines() {
            let mut kv = line.split(" --> ");
            let key = kv.next().unwrap();
            let value = kv.next().unwrap();
            aliases.insert(key.to_string(), value.to_string());
        }
    }

    if args.len() < 2 {
        for (key, value) in &aliases {
            println!("{}", "-".repeat(80));
            println!("{} --> {}", key, value);
        }
    } else {
        let option = &args[1];
        match option.as_str() {
            "-h" | "--help" => {
                println!("{}", HELP);
                exit(0);
            },
            "-v" | "--version" => {
                println!("sh-aliases {}", env!("CARGO_PKG_VERSION"));
                exit(0);
            },
            "-e" | "--edit" => {
                let editor = std::env::var("VISUAL")
                    .unwrap_or_else(|_| std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string()));
                let mut child = Command::new(editor)
                    .arg(aliases_fn)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("Failed to execute editor");
                let ecode = child.wait().expect("Failed to wait on child");
                exit(ecode.code().unwrap_or(1));
            },
            "-r" | "--remove" => {
                if args.len() < 3 {
                    eprintln!("Too few arguments");
                    exit(1);
                }
                let alias = &args[2];
                aliases.remove(alias);
                write_aliases(&aliases, aliases_fn);
                exit(0);
            },
            _ => {
                if args.len() < 3 {
                    if !aliases.contains_key(option) {
                        eprintln!(
                            "Unknow alias '{}', and too few arguments for creating a new one.",
                            option
                        );
                        exit(1);
                    } else {
                        // Print line of ---
                        println!("{}", "-".repeat(80));
                        println!("{}", aliases.get(option).unwrap());
                        println!("{}", "-".repeat(80));
                        let command_parts: Vec<&str> = aliases.get(option).unwrap().split(" ").collect();
                        let mut child = Command::new(command_parts[0])
                            .args(&command_parts[1..])
                            .stdin(Stdio::inherit())
                            .stdout(Stdio::inherit())
                            .stderr(Stdio::inherit())
                            .spawn()
                            .expect("Failed to execute command");
                        let ecode = child.wait().expect("Failed to wait on child");
                        exit(ecode.code().unwrap_or(1)); 
                    }
                }
                let alias = option;
                let command = &args[2..].join(" ");
                aliases.insert(alias.to_string(), command.to_string());
                write_aliases(&aliases, aliases_fn);
                exit(0);
            }
        }
    }
}
