use std::fs;
use std::process::{Command, Stdio, exit};
use std::io::Write;



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


// Key-value text file.
// Keys and values are separated by " --> ".
// Different key-value pairs are separated by newlines.
const ALIASES_FN: &str = "/run/media/morano/SW1000/etc/aliases.txt";


fn write_aliases(aliases: &std::collections::HashMap<String, String>) {
    let mut file = std::fs::File::create(ALIASES_FN).unwrap();
    for (key, value) in aliases {
        writeln!(file, "{} --> {}", key, value).unwrap();
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut aliases: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    if !std::path::Path::new(ALIASES_FN).exists() {
        std::fs::File::create(ALIASES_FN).unwrap();
    } else {
        let contents = fs::read_to_string(ALIASES_FN)
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
                    .arg(ALIASES_FN)
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
                write_aliases(&aliases);
                exit(0);
            },
            _ => {
                if args.len() < 3 {
                    eprintln!(
                        "Unknow alias '{}', and too few arguments for creating a new one.",
                        option
                    );
                    exit(1);
                }
                let alias = option;
                let command = &args[2..].join(" ");
                aliases.insert(alias.to_string(), command.to_string());
                write_aliases(&aliases);
                exit(0);
            }
        }
    }
}
