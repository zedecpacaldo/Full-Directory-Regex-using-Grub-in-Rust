use glob::glob;
use regex::Regex;
use std::{fs, env};

fn main() {
    let args: Vec<_> = env::args().collect();                                                              // arguments array
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
            let file_names: Vec<_> = glob(&glob_pattern)
            .expect("Invalid glob pattern")
            .filter_map(Result::ok)
            .filter_map(|path| {
                let contents = fs::read_to_string(&path);
                if re.is_match(&contents.unwrap()) {
                    Some(path)
                } else {
                    None
                }
            })
            .map(|path| {
                if path.is_absolute(){
                    path.parent().unwrap().to_string_lossy().into_owned() + "\\" + &path.file_name().unwrap().to_string_lossy().into_owned()
                } else {
                    path.to_str().unwrap().to_string()
                }
            })
            .collect();
        
            println!("{}", file_names.join("\n"));
        }
        Err(_err) => {
            eprintln!("Invalid glob pattern");
            return
        }
    }
}
