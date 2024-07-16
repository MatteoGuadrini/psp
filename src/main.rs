use std::{path::Path, process::exit};

// Function for check if tool is installed
fn check_tool(tool: &str) {
    if !Path::new(tool).exists() {
        eprintln!("error: the tool {} not installed", tool);
        exit(1);
    }
}

fn main() {
    println!("Hello by PSP (Python Scaffolding Projects!)");
    println!("This project is a WIP! Be careful!");
}
