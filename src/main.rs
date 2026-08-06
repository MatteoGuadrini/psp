use std::{
    path::absolute,
    process::exit,
};
mod utils;
mod projects;

use utils::*;
use projects::*;

// Constants
const VERSION: &str = "0.7.0";
const LOGFILE: &str = ".psp.log";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const TOOLS: [&str; 4] = ["python3", "git", "pip3", "curl"];
#[cfg(target_os = "windows")]
const TOOLS: [&str; 4] = ["python.exe", "git.exe", "pip.exe", "powershell.exe"];

// Main program
fn main() {
    // Load env files
    load_env();
    // Check if an argument is specified
    let shortcut = get_shortcut();
    // Print help message
    if shortcut == "help" {
        print_help(0)
    }
    // Print welcome screen and version
    print_logo();
    info(format!("welcome to psp, version {VERSION}"));
    for tool in TOOLS {
        if !check_tool(tool) {
            error(format!("`{tool}` is required"));
            exit(1);
        }
    }
    // Create a project structure by name or path
    let (root, name) = prj_name();
    // Virtual Environment
    let venv = prj_venv(&root, &shortcut);
    // Start git
    let git = prj_git(&root, &shortcut);
    // Git remote
    let git_info = if git {
        prj_remote(&root, &name, &shortcut)
    } else {
        ("None".to_string(), "None".to_string())
    };
    // Unit tests
    let tests = prj_test(&root, &name, &shortcut);
    // Install dependencies
    let deps = prj_deps(&root, venv, &shortcut);
    // Documentation
    prj_docs(&root, &name, venv, &shortcut);
    if tests {
        // Tox
        prj_tox(&root, venv, &deps, &shortcut);
        // CI configuration
        prj_ci(&root, &deps, &shortcut);
    }
    // License
    let license = prj_license(&root, &shortcut, &git_info.1);
    // Build dependencies
    let build = prj_pypi(&root, venv, &shortcut);
    // Write pyproject.toml
    prj_toml(&root, &name, &deps, git_info, license, venv);
    // Dockerfile
    let container = prj_container(&root, &name, &shortcut);
    // Common files
    prj_files(&root, &name, container, &shortcut);
    // Makefile
    prj_makefile(&root, &name, tests, build, container);
    // Delete log if enabled
    delete_log(LOGFILE);
    // Finish a scaffolding process
    info(format!(
        "python project `{name}` created at `{}`",
        absolute(root).unwrap().display()
    ));
}
