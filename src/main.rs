use std::fs;
use std::path::Path;
use std::process::{Command, Stdio, exit};
use std::io::Write;
use std::env::var;
use std::collections::HashMap;

const HELP: &str = "\
Usage: sh-aliases [OPTION]... [ALIAS] [COMMAND]
Manage shell aliases.

Options:
  -e, --edit                 edit aliases using a text editor
  -h, --help                 display this help and exit
  -r, --remove ALIAS         remove ALIAS
  -v, --version              display version information and exit
  -l, --locations            show locations of config and aliases files
  -s, --search TEXT          search command (partial) and print its alias

Exit status:
  0  if OK,
  1  if problems

Full documentation <https://github.com/j-morano/sh-aliases>\
";

/* Both config and aliases' files use a block format.
 * Keys start with a '#' on a new line.
 * Subsequent lines until the next '#' are the values (commands).
 */

const DEFAULT_CONFIG: &str = "\
#aliases_fn
$HOME/.local/share/sh-aliases/aliases.txt
";

const CONFIG_FN: &str = "$HOME/.config/sh-aliases.conf";

fn parse(path_or_contents: &str, are_contents: bool) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let contents = if are_contents {
        path_or_contents.to_string()
    } else {
        fs::read_to_string(path_or_contents)
            .expect("Should have been able to read the file")
    };

    let mut current_key: Option<String> = None;
    let mut current_value: Vec<String> = Vec::new();

    for line in contents.lines() {
        if let Some(stripped) = line.strip_prefix('#') {
            // Save the previous key-value pair if it exists
            if let Some(key) = current_key {
                map.insert(key, current_value.join("\n").trim().to_string());
            }
            // Start a new key (remove the '#' and trim)
            current_key = Some(stripped.trim().to_string());
            current_value.clear();
        } else if current_key.is_some() {
            // Collect lines for the current key
            current_value.push(line.to_string());
        }
    }

    // Insert the final key-value pair after the loop finishes
    if let Some(key) = current_key {
        map.insert(key, current_value.join("\n").trim().to_string());
    }

    map
}

fn write_aliases(aliases: &HashMap<String, String>, aliases_fn: String) {
    let mut file = fs::File::create(aliases_fn).unwrap();
    for (key, value) in aliases {
        writeln!(file, "#{}", key).unwrap();
        writeln!(file, "{}\n", value).unwrap(); // Added extra newline for readability
    }
}

fn print_separator() {
    println!("{}", "-".repeat(80));
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let home = var("HOME").unwrap();

    let mut aliases: HashMap<String, String> = HashMap::new();
    let mut config_fn = CONFIG_FN.to_string();
    config_fn = config_fn.replace("$HOME", home.as_str());

    let config_options = if Path::new(&config_fn).exists() {
        parse(CONFIG_FN, false)
    } else {
        parse(DEFAULT_CONFIG, true)
    };

    let mut aliases_fn = config_options.get("aliases_fn").unwrap().to_string();
    aliases_fn = aliases_fn.replace("$HOME", home.as_str());

    if !Path::new(aliases_fn.as_str()).exists() {
        match fs::File::create(aliases_fn.as_str()) {
            Ok(_) => {
                eprintln!("Created file '{}'", aliases_fn);
                exit(0);
            },
            Err(_) => {
                // Get parent directory of aliases_fn.
                let mut parent = aliases_fn.clone();
                parent.pop(); // Remove the filename to get the dir path

                // Try to create the parent directory.
                match fs::create_dir_all(parent.clone()) {
                    Ok(_) => {
                        match fs::File::create(aliases_fn.as_str()) {
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
        // Use the newly adapted parse function instead of duplicating logic
        aliases = parse(aliases_fn.as_str(), false);
    }

    if args.len() < 2 {
        // Order aliases by key. Lowercase all keys.
        let mut sorted_aliases: Vec<_> = aliases.iter().collect();
        sorted_aliases.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
        for (key, value) in &sorted_aliases {
            print_separator();
            println!("#{}\n{}", key, value);
        }
        print_separator();
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
                println!("Removed alias '{}'", alias);
                exit(0);
            },
            "-l" | "--locations" => {
                println!("Config file: {}", config_fn);
                println!("Aliases file: {}", aliases_fn);
                exit(0);
            },
            "-s" | "--search" => {
                if args.len() < 3 {
                    eprintln!("Too few arguments");
                    exit(1);
                }
                let search_term = &args[2].to_lowercase();
                let mut found = false;
                for (key, value) in &aliases {
                    if value.to_lowercase().contains(search_term) {
                        print_separator();
                        println!("#{}\n{}", key, value);
                        found = true;
                    }
                }
                if !found {
                    eprintln!("No aliases found containing '{}'", search_term);
                    exit(1);
                }
                print_separator();
                exit(0);
            },
            _ => {
                if args.len() < 3 {
                    if !aliases.contains_key(option) {
                        eprintln!(
                            "Unknown alias '{}', and too few arguments for creating a new one.",
                            option
                        );
                        exit(1);
                    } else {
                        let command = aliases.get(option).unwrap();
                        // Print line of ---
                        print_separator();
                        println!("{}", command);
                        print_separator();
                        let mut child = Command::new("sh")
                            .arg("-c")
                            .arg(command)
                            .stdin(Stdio::inherit())
                            .stdout(Stdio::inherit())
                            .stderr(Stdio::inherit())
                            .spawn()
                            .expect("Failed to execute command");
                        let ecode = child.wait().expect("Failed to wait on child");
                        print_separator();
                        exit(ecode.code().unwrap_or(1));
                    }
                } else {
                    let alias = option;
                    let command = &args[2..].join(" ");
                    aliases.insert(alias.to_string(), command.to_string());
                    write_aliases(&aliases, aliases_fn);
                    println!("Added alias '{}'", alias);
                    exit(0);
                }
            }
        }
    }
}
