use inquire::Text;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    process::exit,
};

// Constants
const VERSION: &str = "0.0.1";

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
fn prompt_text(question: &str, default: &str, help: &str) -> String {
    let answer = if default != "None" {
        Text::new(question).with_default(default).prompt()
    } else if help != "None" {
        Text::new(question).with_help_message(help).prompt()
    } else {
        Text::new(question).prompt()
    };
    answer.unwrap().to_string()
}

// Core functions

// Project name
fn prj_name() -> String {
    let name = prompt_text("Name of Python project:", "prj", "None");
    let root = format!("{name}");
    let package = format!("{}", name.to_lowercase());
    let project = format!("{root}/{package}");
    // Make directories structure
    let dir_ret = make_dirs(format!("{project}").as_str());
    match dir_ret {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(4);
        }
        Ok(_) => (),
    }
    // Make file structures
    let file_ret = make_file(
        format!("{project}/__init__.py").as_str(),
        "#! /usr/env python3\n\n".to_string(),
    );
    match file_ret {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(4);
        }
        Ok(_) => (),
    }
    name
}

fn main() {
    // Print welcome screen and version
    println!("Welcome to PSP (Python Scaffolding Projects): {VERSION}");
    // Check if Python 3 is installed
    check_tool("/usr/bin/python3");
    // Create project structure by name
    let name = prj_name();
    // Finish scaffolding process
    println!("Project `{name}` created")
}
