use std::{env::current_dir, fs::File, io::Write};

pub fn get_name_current_dir() -> String {
    let pathbuf = current_dir().expect("Failed on get current_dir");
    pathbuf.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn write_file(path: String, content: &str) {
    let mut file = File::create(path).expect("Failed on create file");
    writeln!(&mut file, "{}", content).expect("Failed on write file");
}
