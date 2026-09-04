use projects::*;
use std::{path::absolute, process::exit};
use utils::*;

mod projects;
mod utils;

// Main program
fn main() {
    // Exit status
    let mut exit_status: ExitStatus;
    let start_exit_status = 0;
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
    let ret_prj_name = prj_name();
    let (root, name) = (ret_prj_name.0, ret_prj_name.1);
    exit_status = set_exit_status(start_exit_status, ret_prj_name.2);
    // Virtual Environment
    let ret_prj_venv = prj_venv(&root, &shortcut);
    let venv = ret_prj_venv.0;
    exit_status = set_exit_status(exit_status, ret_prj_venv.1);
    // Start git
    let ret_prj_git = prj_git(&root, &shortcut);
    let git = ret_prj_git.0;
    exit_status = set_exit_status(exit_status, ret_prj_git.1);
    // Git remote
    let git_info = if git {
        prj_remote(&root, &name, &shortcut)
    } else {
        ("None".to_string(), "None".to_string(), 0)
    };
    exit_status = set_exit_status(exit_status, git_info.2);
    // Unit tests
    let ret_prj_test = prj_test(&root, &name, &shortcut);
    let tests = ret_prj_test.0;
    exit_status = set_exit_status(exit_status, ret_prj_test.1);
    // Install dependencies
    let ret_prj_deps = prj_deps(&root, venv, &shortcut);
    let deps = ret_prj_deps.0;
    exit_status = set_exit_status(exit_status, ret_prj_deps.1);
    // Documentation
    let ret_prj_docs = prj_docs(&root, &name, venv, &shortcut);
    exit_status = set_exit_status(exit_status, ret_prj_docs.0);
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
    // Exit with status
    exit(exit_status);
}
