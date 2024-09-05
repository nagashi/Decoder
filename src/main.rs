use std::collections::HashMap;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/*
Search for the absolute path of a file on your operating system.
The `walkdir` crate is used to traverse directories recursively.
*/
fn find_file(name: &str, start_dir: &Path) -> Option<PathBuf> {
    for entry in WalkDir::new(start_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.file_name().map_or(false, |f| f == name) {
            return path.canonicalize().ok();
        }
    }
    None
}

fn decode(message_file: &str) -> String {
    let mut key_value_map: HashMap<u32, String> = HashMap::new();

    if let Ok(lines) = read_lines(&message_file) {
        for line in lines {
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split_whitespace().collect();
                if parts.len() == 2 {
                    if let Ok(key) = parts[0].parse::<u32>() {
                        key_value_map.insert(key, parts[1].to_string());
                    }
                }
            }
        }
    }

    let mut decoded_message = String::new();
    /*
    move the message away from the prompt.
    */
    decoded_message.push_str("\n");
    let mut current_key: u32 = 1;
    /*
    this will be the iterative counter.
    */
    let mut i: u32 = 1;

    loop {
        i += 1;
        match key_value_map.get(&current_key) {
            Some(value) => {
                decoded_message.push_str(value);
                decoded_message.push_str(" ");
                current_key = current_key + i;
            }
            None => break,
        }
    }
    decoded_message
}

fn read_lines(
    file_path: &str,
) -> Result<impl Iterator<Item = Result<String, std::io::Error>>, std::io::Error> {
    let file = fs::File::open(file_path)?;
    Ok(std::io::BufReader::new(file).lines())
}

fn main() {
    /*
    Determine OS and set root directory
    */
    let start_directory;
    match std::env::consts::OS {
        "windows" => {
            start_directory = Path::new("\\");
        }
        "linux" | "macos" | "ios" | "solaris" | "freebsd" | "dragonfly" | "netbsd" | "openbsd"
        | "android" => {
            start_directory = Path::new("/");
        }
        _ => {
            eprint!("Unable to detect OS!");
            /*
            Gently end the process
            with some failure.
            */
            std::process::exit(1);
        }
    };
    /*
    Input files are in the assets/decode directory.
    */
    let file_name = "coding_tree.txt";
    /*
    Starting directory for search.
    The runtime for this process
    will depend on the preciseness
    of the start directory.
    */

    match find_file(file_name, start_directory) {
        Some(file_path) => {
            let message_file = file_path.display().to_string();
            let decoded_message = decode(&message_file);
            /*
            proof of concept.
            */
            println!("{decoded_message}");
        }
        None => eprintln!("File not found"),
    }
}
