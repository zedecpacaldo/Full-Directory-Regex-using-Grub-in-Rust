use glob::glob;
use regex::Regex;
use std::{fs, env};


fn main() {
    let args: Vec<_> = env::args().collect();                                                              // arguments array

    let source_files_glob = &args[1]; 

    let re = Regex::new(&args[2]).unwrap();


    let file_names: Vec<_> = glob(source_files_glob)
    .expect("Failed to read glob pattern")
    .filter_map(Result::ok)
    .filter_map(|path| {
        let contents = fs::read_to_string(&path);
        if re.is_match(&contents.unwrap()) {
            Some(path)
        } else {
            None
        }
    })
    .map(|path| path.parent().unwrap().to_string_lossy().into_owned() + "\\" + &path.file_name().unwrap().to_string_lossy().into_owned())
    .collect();

    println!("{}", file_names.join("\n"));

}
