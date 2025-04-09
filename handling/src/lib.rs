use std::path::Path;
use std::io::ErrorKind;
use std::fs::File;
use std::io::Write;



pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    match File::open(&path) {
        Ok(mut file) => {
            writeln!(file, "{}", content).unwrap();
        }
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            let mut file = File::create(&path).expect("Failed");
            writeln!(file, "{}", content).expect("Failed");
        }
        Err(_e) => {
            panic!("faild to open");
        }
    }
}

