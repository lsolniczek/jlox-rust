use std::io::Write;
use std::{env, fs, io};
use std::path::PathBuf;
use scanner::Scanner;

pub mod scanner;

fn main() {
    let path_to_file: Option<String> = env::args().nth(1).or(None);

    match path_to_file {
        Some(path) => 
            run_file(PathBuf::from(path)),
        None => 
            run_prompt()
    }

}

fn run_file(path_to_file: PathBuf) {
    if path_to_file.is_file() {
        let os_string = path_to_file.into_os_string();
        let path = os_string.into_string().unwrap();
        let source = fs::read_to_string(path).unwrap();
        run(&source);
    }
}

fn run_prompt() {
    loop {
        print!("jlox> ");
        let mut buffer = String::new();
        for _ in io::stdin().read_line(&mut buffer).iter().enumerate() {
            print!("{}", buffer);
            io::stdout().flush().unwrap();
        }
    }
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens.iter().enumerate() {
        println!("{:?}", token);
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, place: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, place, message);
}