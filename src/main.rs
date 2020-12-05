use std::{
    process::exit,
    path::Path,
    env::args,
    fs::File,
    io::{BufReader,BufRead},
    collections::VecDeque
};
use colored::Colorize;
use regex::Regex;
fn main() {
    let mut args_v: Vec<String> = args().collect();
    args_v.remove(0);
// BEGIN HELP CLOSURE
    let help_closure = || {
        let opts_help_txt = String::from("OPTIONS").green().bold();
        let usage_help_txt = String::from("USAGE").cyan().bold(); 
        let file_help_txt = String::from("FILE").blue().bold();
        let err_help_txt = String::from("ERRORS").red().bold();
        print!("\
            \n Print unique lines.\
            \n E.g. onlyone /path/to/file \
            \n\n @{}:\
            \n\tonlyone {}...\
            \n\tonlyone [{}...]...\
            \n\tonlyone [{}...]... {}...\
            \n\n @{}:\n\tFile\t\tAny file to read.\
            \n\n @{}:\n\t-h,--help\tThis HELP message.\
            \n\n @{}: Integers - Exit Codes.\
            \n\t0\t\tNo errors.\
            \n\t1\t\tNo parameters passed.\
            \n\t2\t\tNo valid file passed.\
            \n\t3\t\tPath does not exist.\
            \n\t4\t\tCould not read file.\
            \n\t5\t\tNo lines in file.\
            \n\t6\t\tError while reading line.\
            \n\n",
                usage_help_txt,file_help_txt,
                opts_help_txt,opts_help_txt,
                file_help_txt,file_help_txt,
                opts_help_txt,err_help_txt
            );
        exit(0);
    };
// END HELP CLOSURE
    let help_a_reg = Regex::new(r"^--[hH][eE][lL][pP]$").unwrap();
    let help_b_reg = Regex::new(r"^-[hH]$").unwrap();
    for arg in &args_v {
        if  help_a_reg.is_match(&arg) ||
            help_b_reg.is_match(&arg) {
            help_closure();
       } 
    }
    let args_v = match args_v.len() > 0 {
        true => args_v,
        false => {
            println!("Not enough parameters passed.");
            exit(1);
        } 
    };
    let args_v = args_v.join(" ");
    let file_path = Path::new(&args_v);
    let file_path = match file_path.exists() {
        true => {
            if ! file_path.is_file() {
                println!("'{}' is not a file",args_v);
                exit(2);
            }
            file_path
        },
        false => {
            println!("'{}' does not exist.",file_path.to_string_lossy());
            exit(3);             
        }
    };
    let file_open = match File::open(&file_path) {
        Err(_) => {
            println!("Could not open: '{}'.",file_path.to_string_lossy());
            exit(4);
        }
        Ok(file_open) => file_open
    };
    let reader = BufReader::new(file_open);
    let lines: Vec<_> = reader.lines().collect();
    let mut new_lines: VecDeque<String> = VecDeque::new();
    if lines.len() == 0 {
        println!("No lines to read.");
        exit(5);
    }
    for line_obj in &lines {
        match line_obj.as_ref() {
            Ok(line) => {
                if ! new_lines.contains(&line) {
                    new_lines.push_back((line as &str).to_string());
                }
            },
            Err(err) => {
                println!("Error: {}\nin file: {}.",err,args_v);
                exit(6);
            }
        };
    }
    for line in new_lines {
        println!("{}",line);
    }
}

