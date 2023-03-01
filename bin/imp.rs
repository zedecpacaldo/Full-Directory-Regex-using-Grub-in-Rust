use glob::glob;
use regex::Regex;
use std::{fs, env};


fn main() {
    let args: Vec<_> = env::args().collect();                                                              // arguments array

    let mut array: Vec<String> = vec![String::new()];

    let source_files_glob = &args[1]; 

    let re = Regex::new(&args[2]).unwrap();
    array.pop();

    for entry in glob(source_files_glob).expect("Invalid glob path") {
        match entry {
            Ok(path) => {
                let contents = fs::read_to_string(&path);
                if re.is_match(&contents.unwrap()) {
                    array.push(path.parent().unwrap().to_str().unwrap().to_string() + "\\" + path.file_name().unwrap().to_str().unwrap());
                }
            }
            Err(_e) => {
                println!("Error");
            }
        }
    }

    println!("{}", array.join("\n"));

}
