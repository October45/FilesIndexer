use std::fs;

fn main() {
    ctrlc::set_handler(move || {
        println!("\nExiting...");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    let root = match fs::read_dir("/") {
        Ok(root) => root,
        Err(e) => {
            eprintln!("12Error: {}", e);
            return;
        }
    };
    for item in root {
        match item {
            Ok(entry) => {
                let path = entry.path();
                let path_str = match path.to_str() {
                    Some(path_str) => path_str,
                    None => {
                        eprintln!("Error: path is not valid UTF-8");
                        continue;
                    }
                };
                if path.is_dir() {
                    list_dir(path_str);
                    continue;
                }
                println!("{}", path_str);
            }
            Err(e) => {
                eprintln!("34Error: {}", e);
            }
        }
    }
}

fn list_dir(dir: &str) {
    let dir = match fs::read_dir(dir) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("44Error: {}", e);
            return;
        }
    };
    for item in dir {
        match item {
            Ok(entry) => {
                let path = entry.path();
                let path_str = match path.to_str() {
                    Some(path_str) => path_str,
                    None => {
                        eprintln!("Error: path is not valid UTF-8");
                        continue;
                    }
                };
                if path.is_dir() {
                    list_dir(path_str);
                    continue;
                }
                println!("{}", path_str);
            }
            Err(e) => {
                eprintln!("66Error: {}", e);
            }
        }
    }
}
