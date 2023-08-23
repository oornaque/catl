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

    let result = input.read_string().replace('\n', "");

    if !(std::io::stdout().is_terminal()) || matches.get_flag("ommit_newline") {
        print!("{}", result);
    } else {
        println!("{}", result);
    }
}
