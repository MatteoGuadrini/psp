use std::{fs::create_dir_all, path::Path, process::exit};

// Function for check if tool is installed
fn check_tool(tool: &str) {
    if !Path::new(tool).exists() {
        eprintln!("error: the tool {} not installed", tool);
        exit(1);
    }
}

// Function for creating folders and parents
fn make_dirs(dir: &str) {
    let result = create_dir_all(dir);
    match result {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(2);
        }
        Ok(_) => (),
    }
}

fn main() {
    println!("Hello by PSP (Python Scaffolding Projects!)");
    println!("This project is a WIP! Be careful!");
}
