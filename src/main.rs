use inquire::{Confirm, Text};
use std::{
    env::var,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    process::exit,
};

// Constants
const VERSION: &str = "0.0.5";

// Utility functions

// Function for check if tool is installed
fn check_tool(tool: &str) {
    let home = var("HOME").unwrap();
    let root_bin = format!("/usr/bin/{tool}");
    let user_bin = format!("{home}/.local/bin/{tool}");
    if !Path::new(&root_bin).exists() && !Path::new(&user_bin).exists() {
        eprintln!("error: {} is not installed", tool);
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

// Function for prompt confirm (yes/no)
fn prompt_confirm(question: &str, default: bool, help: &str) -> bool {
    let answer = if help != "None" {
        Confirm::new(question)
            .with_default(default)
            .with_help_message(help)
            .prompt()
    } else {
        Confirm::new(question).with_default(default).prompt()
    };
    answer.unwrap()
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
        "#! /usr/bin/env python3\n\n".to_string(),
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

// Project git
fn prj_git(name: &str) -> bool {
    let confirm = prompt_confirm("Do you want to start git repository?", true, "None");
    if confirm {
        let output = std::process::Command::new("git")
            .arg("init")
            .current_dir(name)
            .output()
            .expect("git should be installed");
        // Check if command exit successfully
        if !output.status.success() {
            eprintln!("error: something wrong with `git init`");
            exit(5)
        }
        // Create .gitignore file
        let file_ret = make_file(
            format!("{name}/.gitignore").as_str(),
            "### Python ###\n\
                __pycache__/\n\
                *.py[cod]\n\
                *$py.class\n\
                build/\n\
                develop-eggs/\n\
                dist/\n\
                downloads/\n\
                eggs/\n\
                .eggs/\n\
                lib/\n\
                lib64/\n\
                parts/\n\
                sdist/\n\
                var/\n\
                wheels/\n\
                share/python-wheels/\n\
                *.egg-info/\n\
                .installed.cfg\n\
                *.egg\n\
                # Environments\n\
                .env\n\
                .venv\n\
                env/\n\
                venv/\n\
                ENV/\n\
                env.bak/\n\
                venv.bak/\n\
                # Sphinx documentation\n\
                docs/_build/\n\
                # mkdocs documentation\n\
                /site\n"
                .to_string(),
        );
        let ret = match file_ret {
            Err(e) => {
                eprintln!("error: {}", e);
                exit(5);
            }
            Ok(_) => true,
        };
        return ret;
    }
    false
}

// Project unit tests
fn prj_test(name: &str) {
    let confirm = prompt_confirm("Do you want unit test files?", true, "None");
    if confirm {
        // Make directories structure
        let dir_ret = make_dirs(format!("{name}/tests").as_str());
        match dir_ret {
            Err(e) => {
                eprintln!("error: {}", e);
                exit(6);
            }
            Ok(_) => (),
        }
        // Make file structures
        let init_file = make_file(
            format!("{name}/tests/__init__.py").as_str(),
            "#! /usr/bin/env python3\n\n".to_string(),
        );
        match init_file {
            Err(e) => {
                eprintln!("error: {}", e);
                exit(6);
            }
            Ok(_) => (),
        }
        let all_module = make_file(
            format!("{name}/tests/test_{name}.py").as_str(),
            format!(
                "#! /usr/bin/env python3\n\n\n\
            import unittest\n\n\n\
            class TestAll(unittest.TestCase):\n\n\
            \tdef test_all(self):\n\
            \t\tprint('Test all {} successfully!')\n\n\n\
            if __name__ == '__main__':\n\
            \tunittest.main()",
                name.to_lowercase()
            )
            .to_string(),
        );
        match all_module {
            Err(e) => {
                eprintln!("error: {}", e);
                exit(6);
            }
            Ok(_) => (),
        }
    }
}

// Project venv
fn prj_venv(name: &str) -> bool {
    let confirm = prompt_confirm("Do you want to create a virtual environment?", true, "None");
    if confirm {
        let output = std::process::Command::new("python3")
            .args(["-m", "venv", "venv"])
            .current_dir(name)
            .output()
            .expect("python should be installed");
        // Check if command exit successfully
        if !output.status.success() {
            eprintln!("error: `venv` creation failed");
        } else {
            return true;
        }
    }
    false
}

// Project dependencies
fn prj_deps(name: &str, venv: bool) -> Vec<String> {
    let deps = prompt_text(
        "Install dependencies:",
        "No",
        "Write package(s) separates with spaces or empty",
    );
    // Split String into Vector
    let dependencies: Vec<String> = deps
        .as_str()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    if deps != "No" {
        let mut pip = std::process::Command::new("pip3");
        // Activate venv
        if venv {
            pip.env("PATH", "venv/bin");
        }
        let output = pip
            .arg("install")
            .arg("--timeout=10")
            .arg("--retries=1")
            .args(&dependencies)
            .current_dir(&name)
            .output()
            .expect("pip should be installed");
        // Check if command exit successfully
        if !output.status.success() {
            eprintln!("error: dependencies ({deps}) installation failed");
        }
    }
    dependencies
}

// Main program
fn main() {
    // Print welcome screen and version
    println!("Welcome to PSP (Python Scaffolding Projects): {VERSION}");
    // Check dependencies tools
    check_tool("python3");
    check_tool("git");
    check_tool("pip3");
    // Create project structure by name
    let name = prj_name();
    // Start git
    let _git = prj_git(&name);
    // Unit tests
    prj_test(&name);
    // Virtual Environment
    let venv = prj_venv(&name);
    // Install dependencies
    let _deps = prj_deps(&name, venv);
    // Finish scaffolding process
    println!("Project `{name}` created")
}
