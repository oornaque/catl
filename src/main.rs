use std::{
    path::PathBuf,
    io::{
        BufReader,
        BufRead,
        self,
        IsTerminal,
    },
    fs
};

use regex::Regex;

use clap:: {
    crate_name,
    Command,
    Arg,
    ArgGroup,
    value_parser, ArgAction,
};

enum Input {
    StdIn,
    File(PathBuf),
    String(String),
}

impl Input {
    pub fn read_string(&self) -> String {
        let mut contents = String::new();
        match self {
            Input::StdIn => {
                let mut reader: Box<dyn BufRead> = Box::new(BufReader::new(io::stdin()));
                let _ = reader.read_to_string(&mut contents);
                contents.trim_end().to_string()
            },
            Input::File(path) => {
                let mut reader: Box<dyn BufRead> = Box::new(BufReader::new(fs::File::open(path).unwrap()));
                let _ = reader.read_to_string(&mut contents);
                contents
            },
            Input::String(string) => {
                string.to_string()
            }
        }
    }
}


fn main() {
    let matches = Command::new(crate_name!())
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("File to read the input from")
                .num_args(1)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("string")
                .help("Input from the cli arguments. Leave it blank and do not set a file to read from stdin")
                .num_args(1)
        )
        .group(
            ArgGroup::new("inputs")
                .args(["file", "string"])
        )
        .arg(
            Arg::new("ommit_newline")
                .short('n')
                .long("ommit_newline")
                .help("Ommit the newline when printing the output. Useful when redirecting the output to a file")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("until")
                .short('u')
                .long("until")
                .help("Regex until the program will concatenate lines. After the regex the program will output a new line")
                .num_args(1)
        )
        .get_matches();

    let input = if matches.contains_id("file") {
        let file = matches.get_one::<PathBuf>("file");
        match file {
            None => Input::StdIn,
            Some(path) => Input::File(path.to_path_buf()),
        }
    } else {
        let string = matches.get_one::<String>("string");
        match string {
            None => Input::StdIn,
            Some(string) => Input::String(string.to_string()),
        }
    };

    #[cfg(target_os = "windows")]
    let newline = "\r\n";
    #[cfg(not(target_os = "windows"))]
    let newline = '\n';
    
    let input_string = input.read_string();

    let input_strings: Vec<&str> = match matches.get_one::<String>("until") {
        Some(until) => {
            let re = Regex::new(until).expect("Could not compile regex");
            re.split(&input_string).collect()
        },
        None => vec![&input_string],
    };
    let results: Vec<String> = input_strings.iter().map(|l| l.replace(newline, "")).collect();

    let should_output_without_newline = !(std::io::stdout().is_terminal()) || matches.get_flag("ommit_newline");
    for result in &results{
        if should_output_without_newline && result == results.last().unwrap() {
            print!("{}", result);
        } else {
            println!("{}", result);
        }
    }
}
