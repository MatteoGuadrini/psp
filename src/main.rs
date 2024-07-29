use inquire::Text;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    process::exit,
};

// Utility functions

// Function for check if tool is installed
fn check_tool(tool: &str) {
    if !Path::new(tool).exists() {
        eprintln!("error: the tool {} not installed", tool);
        exit(1);
    }
}

// Function for creating folders and parents
fn make_dirs(dir: &str) -> std::io::Result<()> {
    let result = create_dir_all(dir);
    match result {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(2);
        }
        Ok(_) => Ok(()),
    }
}

// Function for creating file with contents
fn make_file(file: &str, content: String) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    let result = file.write_all(&content.as_bytes());
    match result {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(3);
        }
        Ok(_) => Ok(()),
    }
}

// Function for prompt text
fn prompt_text(question: &str, default: &str) -> String {
    let answer = Text::new(question).with_default(default).prompt();
    answer.unwrap().to_string()
}

// Core functions

// Project name
fn prj_name() {
    let name = prompt_text("Name of Python project: ", "None");
    // Make directories structure
    let dir_ret = make_dirs(name.as_str());
    match dir_ret {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(4);
        }
        Ok(_) => (),
    }
    // Make file structures
    let file_ret = make_file(
        format!("{name}/__init__.py").as_str(),
        "#! /usr/env python3".to_string(),
    );
    match file_ret {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(4);
        }
        Ok(_) => (),
    }
}

fn main() {
    // Check if Python 3 is installed
    check_tool("/usr/bin/python3");
    // Create project structure by name
    prj_name();
}
