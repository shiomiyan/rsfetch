use std::fs::metadata;

fn dist(path: &str) -> String {
    let file = std::fs::File::open(path).unwrap();
    let line: String = crate::shared_functions::line(file, 0); // Expects NAME= to be on first line
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
