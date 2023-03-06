use glob::{glob};
use regex::Regex;
use std::{fs, env};

fn main() {
    // arguments array
    let args: Vec<_> = env::args().collect();                                                                                           
    let mut array: Vec<String> = vec![String::new()];
    let glob_pattern = format!("{}", args[1]); 
    match glob(&glob_pattern) {
        Ok(_paths) => {
            let reg_pattern = format!("{}", args[2]);
            let re = match Regex::new(&reg_pattern) {
                Ok(regex) => regex,
                Err(_error) => {
                    eprintln!("Invalid regex string");
                    return
                }
            };
            array.pop();
            for entry in glob(&glob_pattern).unwrap() {
                match entry {
                    Ok(path) => {
                        let contents = fs::read_to_string(&path);
                        if re.is_match(&contents.unwrap()) {
                            if path.is_absolute(){array.push(path.parent().unwrap().to_str().unwrap().to_string() + "\\" + path.file_name().unwrap().to_str().unwrap());}
                            else{array.push(path.to_str().unwrap().to_string());}
                        }
                    }
                    Err(_e) => {
                        println!("Invalid file");
                    }
                }
            }
            array.sort();
            println!("{}", array.join("\n"));
        }
        Err(_err) => {
            eprintln!("Invalid glob pattern");
            return
        }
    }
}
