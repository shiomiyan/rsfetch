use regex::Regex;
use std::fs::{metadata, File};
use std::io::{BufReader, Read};

fn line(file: File) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let file_vector: Vec<&str> = contents.split('\n').collect();
    let r = Regex::new("^NAME")?;
    let line: String = file_vector.into_iter().filter(|s| r.is_match(s)).collect();
    Ok(line)
}

fn dist(path: &str) -> String {
    let file = File::open(path).unwrap();
    let line: String = line(file).unwrap();
    let distro_vec: Vec<&str> = line.split('=').collect();
    String::from(distro_vec[1])
}

pub fn distro() -> String {
    if metadata("/bedrock/etc/os-release").is_ok() {
        dist("/bedrock/etc/os-release")
    } else if metadata("/etc/os-release").is_ok() {
        dist("/etc/os-release")
    } else if metadata("/usr/lib/os-release").is_ok() {
        dist("/usr/lib/os-release")
    } else {
        "N/A (could not obtain distro name, please file a bug as your os-release file may just be in a weird place)".to_string()
    }
}
