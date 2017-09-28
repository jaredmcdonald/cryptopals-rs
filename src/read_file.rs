use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::env;

// `filename` is relative to cwd, todo: get it as a command-line argument
pub fn strings_from_filename(filename: &str) -> Vec<String> {
    let mut strings = Vec::new();
    let full_path = Path::new(&env::current_dir().unwrap()).join(filename);
    match File::open(&full_path) {
        Ok(f) => {
            let file = BufReader::new(&f);
            for line in file.lines() {
                strings.push(line.unwrap());
            }
        },
        Err(e) => println!("Problem reading file at {:?}: {:?}", full_path, e),
    }
    strings
}
