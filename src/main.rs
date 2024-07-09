use std::env;
use std::path::PathBuf;

fn main() {
    let path_to_file: Option<String> = env::args().nth(1).or(None);

    match path_to_file {
        Some(path) => 
            run_file(PathBuf::from(path)),
        None => 
            run_prompt()
    }

}

fn run_file(file: PathBuf) {
    dbg!(file);
}

fn run_prompt() {
    dbg!(">  ");
}