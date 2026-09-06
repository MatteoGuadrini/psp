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
    exit_status = set_exit_status(exit_status, ret_prj_docs.1);
    // Test factory
    if tests {
        // Tox
        let ret_prj_tox = prj_tox(&root, venv, &deps, &shortcut);
        exit_status = set_exit_status(exit_status, ret_prj_tox.1);
        // CI configuration
        let ret_prj_ci = prj_ci(&root, &deps, &shortcut);
        exit_status = set_exit_status(exit_status, ret_prj_ci.1);
    }
    // License
    let ret_prj_license = prj_license(&root, &shortcut, &git_info.1);
    let license = ret_prj_license.0;
    exit_status = set_exit_status(exit_status, ret_prj_license.1);
    // Build dependencies
    let ret_prj_pypi = prj_pypi(&root, venv, &shortcut);
    let build = ret_prj_pypi.0;
    exit_status = set_exit_status(exit_status, ret_prj_pypi.1);
    // Write pyproject.toml
    let ret_prj_toml = prj_toml(&root, &name, &deps, git_info, license, venv);
    exit_status = set_exit_status(exit_status, ret_prj_toml.1);
    // Dockerfile
    let ret_prj_container = prj_container(&root, &name, &shortcut);
    let container = ret_prj_container.0;
    exit_status = set_exit_status(exit_status, ret_prj_container.1);
    // Common files
    let ret_prj_files = prj_files(&root, &name, container, &shortcut);
    exit_status = set_exit_status(exit_status, ret_prj_files.1);
    // Makefile
    let ret_prj_makefile = prj_makefile(&root, &name, tests, build, container);
    exit_status = set_exit_status(exit_status, ret_prj_makefile.1);
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
