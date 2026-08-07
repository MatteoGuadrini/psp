// Core functions

use crate::utils::*;
use crate::{LOGFILE, SIGNATURE, VERSION};
use std::{
    collections::HashMap,
    env::var,
    fs::{copy, remove_dir_all},
    path::Path,
    process::exit,
};

// Constants
#[cfg(any(target_os = "linux", target_os = "macos"))]
const PYTHON_BIN: &str = "python3";
#[cfg(any(target_os = "linux", target_os = "macos"))]
const PIP_BIN: &str = "pip3";
#[cfg(target_family = "windows")]
const PYTHON_BIN: &str = "python.exe";
#[cfg(target_family = "windows")]
const PIP_BIN: &str = "pip.exe";
#[cfg(target_os = "windows")]
const SUPPORTED_PM: [&str; 2] = ["conda.exe", "uv.exe"];
#[cfg(target_os = "windows")]
const SUPPORTED_BUILDER: [&str; 3] = ["hatch.exe", "poetry.exe", "maturin.exe"];

// Project name
pub fn prj_name() -> (String, String) {
    // Check psp log for update
    let log_step = "prj_name";
    if check_log(log_step, LOGFILE) {
        let log_content = read_log(LOGFILE);
        let value = get_log_value(log_step, log_content.unwrap().as_str());
        if let Some(v) = value {
            let values: Vec<&str> = v.split(" ").collect();
            if std::fs::exists(values[0]).unwrap() {
                return (values[0].to_string(), values[1].to_string());
            }
        }
    }
    // Check environment variable
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let folder_separator = "/";
    #[cfg(target_os = "windows")]
    let folder_separator = "\\";
    let env_name = var("PSP_NAME").ok();
    let mut name = if let Some(env_name) = env_name {
        info(format!("project name: {env_name}"));
        env_name
    } else {
        String::new()
    };
    // Check is path is empty
    while name.is_empty() {
        name = prompt_text("Name of Python project:", "None", "Type name or path")
            .trim()
            .trim_end_matches(folder_separator)
            .to_string();
        if name.is_empty() {
            warning("write a valid name or path".to_string());
        }
    }
    // Make package path parts
    let project_name = name.replace(" ", "_");
    let project = Path::new(&project_name);
    let package = project.join(
        project
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .to_lowercase(),
    );
    let root = Path::new(package.parent().unwrap());
    // Check if a project path already exists
    if package.exists() {
        let project_exists = prompt_confirm(
            format!("Path `{}` exists. Do you want continue?", root.display()).as_str(),
            false,
            "Some files will be overwritten",
        );
        if !project_exists {
            info(format!("the path `{}` unchanged", root.display()));
            exit(0)
        }
    }
    let dir_ret = make_dirs(format!("{}", package.display()).as_str());
    if let Err(e) = dir_ret {
        error(format!("{e}"));
    }
    // Check the version of a Python project
    let pyver = env_pyversion();
    let content = format!("__version__ = \"{pyver}\"");
    let package_path = package.display();
    if !create_python_package(package.as_path(), content.as_str()) {
        error(format!("package {package_path} creation failed."));
        exit(1);
    }
    let main_file = package.join("__main__.py");
    let main_file_ret = make_file(
        format!("{}", main_file.display()).as_str(),
        format!(
            "#! /usr/bin/env python3
# -*- encoding: utf-8 -*-
# vim: se ts=4 et syn=python:
# {SIGNATURE}, version {VERSION}

from .__init__ import __version__

print('name: {}')
print(f'version: {{__version__}}')
",
            package
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
                .to_lowercase()
        ),
    );
    if let Err(e) = main_file_ret {
        error(format!("{e}"));
        exit(4);
    }
    let values = (
        root.to_string_lossy().to_string(),
        package.file_name().unwrap().to_string_lossy().to_string(),
    );
    // Write psp log
    write_log(
        LOGFILE,
        format!("{}: {} {}", log_step, values.0, values.1).as_str(),
    );
    values
}

// Project git
pub fn prj_git(name: &str, shortcut: &String) -> bool {
    // Check psp log for update
    let log_step = "prj_git";
    if check_log(log_step, LOGFILE) {
        let log_content = read_log(LOGFILE);
        let value = get_log_value(log_step, log_content.unwrap().as_str());
        if let Some(v) = value {
            return v.parse::<bool>().unwrap();
        }
    }
    // Check environment variable
    let env_git = var("PSP_GIT").unwrap_or("false".to_string()).parse().ok();
    let mut ret = false;
    let confirm = if let Some(true) = env_git {
        info(format!("git enable: {}", env_git.unwrap().to_string()));
        true
    } else if shortcut == "quick" || shortcut == "full" {
        true
    } else if shortcut == "simple" {
        false
    } else {
        prompt_confirm("Do you want to start git repository?", true, "None")
    };
    if confirm {
        let mut git = make_command("git", name, name, vec!["init".to_string()], false);
        let output = git.output().expect("git should be installed");
        // Check if the command exits successfully
        if !output.status.success() {
            error("something wrong with `git init`".to_string());
            return false;
        }
        // Create a data map with variables
        let mut data = HashMap::new();
        data.insert("SIGNATURE", SIGNATURE);
        data.insert("VERSION", VERSION);
        let gitignore_template = Path::new(name).join(".gitignore").display().to_string();
        let file_ret = render_template("gitignore.hbs", &gitignore_template, data);
        ret = if file_ret {
            true
        } else {
            error("`.gitignore` creation failed".to_string());
            false
        };
    }
    // Write psp log
    write_log(LOGFILE, format!("{}: {}", log_step, ret).as_str());
    ret
}

// Project unit tests
pub fn prj_test(root: &str, name: &str, shortcut: &String) -> bool {
    // Check psp log for update
    let ret: bool;
    let log_step = "prj_test";
    if check_log(log_step, LOGFILE) {
        let log_content = read_log(LOGFILE);
        let value = get_log_value(log_step, log_content.unwrap().as_str());
        if let Some(v) = value {
            return v.parse::<bool>().unwrap();
        }
    }
    // Check environment variable
    let env_test = var("PSP_TEST").unwrap_or("false".to_string()).parse().ok();
    let confirm = if let Some(true) = env_test {
        info(format!("tests enable: {}", env_test.unwrap().to_string()));
        true
    } else if shortcut != "None" {
        true
    } else {
        prompt_confirm("Do you want unit test files?", true, "None")
    };
    if confirm {
        // Make directories structure
        let tests_dir = Path::new(root).join("tests");
        let tests_path = tests_dir.display();
        if !create_python_package(tests_dir.as_path(), "") {
            error(format!("tests package {tests_path} creation failed."));
            return false;
        }
        let project_name = name.to_lowercase();
        let project_version = env_pyversion();
        let data = HashMap::from([
            ("SIGNATURE", SIGNATURE),
            ("VERSION", VERSION),
            ("PRJ_NAME", &project_name),
            ("PRJ_VER", &project_version),
        ]);
        let test_module = tests_dir.join(format!("test_{project_name}.py"));
        let file_ret = render_template("test_module.hbs", test_module.to_str().unwrap(), data);
        if !file_ret {
            error(format!("`test_{project_name}.py` render failed"));
            return false;
        }
        ret = true;
    } else {
        ret = false;
    }
    // Write psp log
    write_log(LOGFILE, format!("{}: {}", log_step, ret).as_str());
    ret
}

// Project venv
pub fn prj_venv(name: &str, shortcut: &String) -> bool {
    // Check psp log for update
    let mut ret: bool;
    let log_step = "prj_venv";
    if check_log(log_step, LOGFILE) {
        let log_content = read_log(LOGFILE);
        let value = get_log_value(log_step, log_content.unwrap().as_str());
        if let Some(v) = value {
            return v.parse::<bool>().unwrap();
        }
    }
    // Check environment variable
    let env_venv = var("PSP_VENV").unwrap_or("false".to_string()).parse().ok();
    let confirm = if let Some(true) = env_venv {
        info(format!(
            "virtual environment enable: {}",
            env_venv.unwrap().to_string()
        ));
        true
    } else if shortcut == "quick" || shortcut == "full" {
        true
    } else if shortcut == "simple" {
        false
    } else {
        prompt_confirm("Do you want to create a virtual environment?", true, "None")
    };
    ret = false;
    if confirm {
        let mut python = make_command(
            PYTHON_BIN,
            name,
            name,
            vec!["-m".to_string(), "venv".to_string(), ".venv".to_string()],
            false,
        );
        let output = python
            .output()
            .expect(format!("{PYTHON_BIN} should be installed").as_str());
        // Check if the command exits successfully
        if !output.status.success() {
            error("`.venv` creation failed".to_string());
        } else {
            ret = true;
        }
    }
    // Write psp log
    write_log(LOGFILE, format!("{}: {}", log_step, ret).as_str());
    ret
}

// Project dependencies
pub fn prj_deps(name: &str, venv: bool, shortcut: &String) -> Vec<String> {
    // Check psp log for update
    let log_step = "prj_deps";
    if check_log(log_step, LOGFILE) {
        let log_content = read_log(LOGFILE);
        let value = get_log_value(log_step, log_content.unwrap().as_str());
        if let Some(v) = value {
            let values: Vec<&str> = v.split(" ").collect();
            if values[0] != "No" {
                return values.iter().map(|s| s.to_string()).collect();
            }
        }
    }
    // Check environment variableS
    let env_common_deps = var("PSP_COMMON_DEPS").ok();
    let env_deps = var("PSP_DEPS").ok();
    let env_pm = var("PSP_PACKAGE_MANAGER").ok();
    let deps = if let Some(env_deps) = env_deps {
        info(format!("dependencies: {env_deps}"));
        env_deps
    } else if shortcut == "simple" || shortcut == "quick" {
        "No".to_string()
    } else {
        prompt_text(
            "Install dependencies:",
            "No",
            "Write package(s) separates with spaces or empty",
        )
    };
    // Check if there are common dependencies
    let common_dependencies = if let Some(env_common_deps) = env_common_deps {
        info(format!("common dependencies: {env_common_deps}"));
        env_common_deps
            .as_str()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    } else {
        vec![]
    };
    // Split String into Vector
    let mut dependencies: Vec<String> = if deps.to_lowercase() != "no" {
        deps.as_str()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    } else {
        vec![]
    };
    // Extend dependencies
    if !common_dependencies.is_empty() {
        dependencies.extend(common_dependencies);
    }
    if !dependencies.is_empty() {
        let bin = if let Some(env_pm) = &env_pm {
            env_pm.as_str()
        } else {
            PIP_BIN
        };
        let mut pm = make_pm(bin, name, name, dependencies.clone(), venv);
        let output = pm
            .output()
            .expect(format!("{bin} should be installed").as_str());
        // Check if the command exits successfully
        if !output.status.success() {
            error(format!("dependencies ({deps}) installation failed"));
        }
        // Build a requirements.txt file
        let content = format!(
            "# {SIGNATURE}, version {VERSION}\n\n{}",
            dependencies.join("\n")
        );
        let requirements_file = Path::new(name);
        let requirements = make_file(
            requirements_file
                .join("requirements.txt")
                .display()
                .to_string()
                .as_str(),
            content,
        );
        if let Err(e) = requirements {
            error(format!("{e}"));
        }
    }
    // Write psp log
    write_log(LOGFILE, format!("{}: {}", log_step, deps).as_str());
    dependencies
}

// Project pyproject.toml
pub fn prj_toml(
    root: &str,
    name: &str,
    deps: &Vec<String>,
    git_info: (String, String),
    license: String,
    venv: bool,
) {
    // Check git information
    let mut documentation = "https://docs.python.org/3/".to_string();
    let mut repository = "https://github.com/python".to_string();
    let mut changelog = "https://docs.python.org/3/whatsnew/changelog.html".to_string();
    // Check the homepage of a Python project
    let env_pyhomepage = var("PSP_PYHOMEPAGE").ok();
    let pyhomepage = if let Some(homepage) = env_pyhomepage {
        homepage
    } else {
        "https://python.org/".to_string()
    };
    // Check description of a Python project
    let env_pydescription = var("PSP_PYDESC").ok();
    let pydescription = if let Some(description) = env_pydescription {
        description
    } else {
        format!("{} Python package", name.to_lowercase())
    };
    let description = pydescription;
    let (mut username, mut email) = env_pyauthor();
    let mut homepage = pyhomepage;
    if git_info.0 != "None" && git_info.1 != "None" {
        let git_repo = &git_info.0.to_lowercase();
        let git_user = &git_info.1.to_lowercase();
        repository = format!(
            "https://{}.com/{}/{}",
            git_repo,
            git_user,
            name.to_lowercase()
        );
        changelog = format!("{}/blob/main/CHANGES.md", &repository);
        if email == "psp@python.com" {
            username = format!("{}", git_user);
            email = format!("{}@{}.com", git_user, git_repo);
        }
        if homepage == "https://python.org/" {
            homepage = format!("https://{}.org/", name.to_lowercase());
        }
        documentation = format!("{}/docs", repository);
    }
    let classifiers = vec!["Programming Language :: Python :: 3"];
    // Check dependencies
    let requirements = if deps.contains(&"No".to_string()) {
        "[]".to_string()
    } else {
        format!("{deps:?}")
    };
    // Create a data map with variables
    let builder = make_builder(root, venv);
    let project_name = name.to_lowercase();
    let project_version = env_pyversion();
    let python_version = get_python_version();
    let stringed_classifiers = format!("{:?}", classifiers);
    let mut data = HashMap::from([
        ("SIGNATURE", SIGNATURE),
        ("VERSION", VERSION),
        ("BUILDER", &builder),
        ("PRJ_NAME", &project_name),
        ("PRJ_VER", &project_version),
        ("USERNAME", &username),
        ("EMAIL", &email),
        ("DESCRIPTION", &description),
        ("PYTHON", &python_version),
        ("CLASSIFIERS", &stringed_classifiers),
        ("DEPS", &requirements),
        ("HOMEPAGE", &homepage),
        ("DOCUMENTATION", &documentation),
        ("REPOSITORY", &repository),
        ("CHANGELOG", &changelog),
    ]);
    // Check if license is set
    if license != "None" {
        data.insert("LICENSE", "true");
    }
    let pyproject_template = Path::new(root).join("pyproject.toml").display().to_string();
    let file_ret = render_template("pyproject.hbs", &pyproject_template, data);
    if !file_ret {
        error("`pyproject.toml` render failed".to_string());
    }
}

// Project CI
pub fn prj_ci(name: &str, deps: &Vec<String>, shortcut: &String) {
    // Check psp log for update
    let log_step = "prj_ci";
    if check_log(log_step, LOGFILE) {
        return;
    }
    let options = vec![
        "None",
        "CircleCI",
        "Github Actions",
        "Gitlab CI/CD",
        "TravisCI",
    ];
    let env_ci = var("PSP_CI").ok();
    let ci = if let Some(env_ci) = env_ci {
        info(format!("remote CI provider: {env_ci}"));
        env_ci
    } else if shortcut == "simple" || shortcut == "quick" {
        "None".to_string()
    } else {
        prompt_select("Select remote CI provider:", options, "None")
    };
    let requirements = if deps.contains(&"No".to_string()) {
        "".to_string()
    } else {
        deps.join(" ")
    };
    let python_version = get_python_version();
    let package_name = Path::new(name).file_name().unwrap().to_str().unwrap();
    // Select Ci configurations
    if ci.as_str().to_lowercase() == "travisci" {
        // Create a data map with variables
        let data = HashMap::from([
            ("SIGNATURE", SIGNATURE),
            ("VERSION", VERSION),
            ("REQUIREMENTS", &requirements),
            ("PYTHON", &python_version),
        ]);
        let travis_template = Path::new(name).join(".travis.yml").display().to_string();
        let file_ret = render_template("travis.hbs", &travis_template, data);
        if !file_ret {
            error("`.travis.yml render failed".to_string());
        }
    } else if ci.as_str().to_lowercase() == "circleci" {
        let circleci_dir = Path::new(name).join(".circleci");
        let dir_ret = make_dirs(circleci_dir.display().to_string().as_str());
        if let Err(e) = dir_ret {
            error(format!("{e}"));
        }
        // Create a data map with variables
        let data = HashMap::from([
            ("SIGNATURE", SIGNATURE),
            ("VERSION", VERSION),
            ("REQUIREMENTS", &requirements),
        ]);
        let circleci_template = Path::new(circleci_dir.as_path())
            .join("config.yml")
            .display()
            .to_string();
        let file_ret = render_template("circleci.hbs", &circleci_template, data);
        if !file_ret {
            error("`.circleci/config.yml` render failed".to_string());
        }
    } else if ci.as_str().to_lowercase().replace(" ", "").replace("/", "") == "githubactions" {
        let github_dir = Path::new(name).join(".github").join("workflows");
        let dir_ret = make_dirs(github_dir.display().to_string().as_str());
        if let Err(e) = dir_ret {
            error(format!("{e}"));
        }
        // Create a data map with variables
        let data = HashMap::from([
            ("SIGNATURE", SIGNATURE),
            ("VERSION", VERSION),
            ("PYTHON", &python_version),
            ("PACKAGE", package_name),
        ]);
        let github_template = Path::new(github_dir.as_path())
            .join("python-app.yml")
            .display()
            .to_string();
        let file_ret = render_template("githubactions.hbs", &github_template, data);
        if !file_ret {
            error("`python-app.yml` render failed".to_string());
        }
    } else if ci.as_str().to_lowercase().replace(" ", "").replace("/", "") == "gitlabcicd" {
        // Create a data map with variables
        let data = HashMap::from([
            ("SIGNATURE", SIGNATURE),
            ("VERSION", VERSION),
            ("PYTHON", &python_version),
            ("PACKAGE", package_name),
        ]);
        let gitlab_template = Path::new(name).join(".gitlab-ci.yml").display().to_string();
        let file_ret = render_template("gitlabcicd.hbs", &gitlab_template, data);
        if !file_ret {
            error("`.gitlab-ci.yml` render failed".to_string());
        }
    } else if ci.as_str().to_lowercase() != "none" {
        warning(format!("`{ci}` is not recognized as remote CI"));
    }
    // Write psp log
    write_log(LOGFILE, format!("{}: {}", log_step, ci).as_str());
}

// Project Gitlab/GitHub
pub fn prj_remote(root: &str, name: &str, shortcut: &String) -> (String, String) {
    // Check psp log for update
    let log_step = "prj_remote";
    if check_log(log_step, LOGFILE) {
        let log_content = read_log(LOGFILE);
        let value = get_log_value(log_step, log_content.unwrap().as_str());
        if let Some(v) = value {
            let values: Vec<&str> = v.split(" ").collect();
            return (values[0].to_string(), values[1].to_string());
        }
    }
    let mut git_user = "None".to_string();
    let git_remote;
    let options = vec!["None", "Github", "Gitlab", "Custom"];
    // Check environment variable
    let env_remote = var("PSP_GIT_REMOTE").ok();
    let remote = if let Some(env_remote) = env_remote {
        info(format!("git remote provider: {env_remote}"));
        env_remote
    } else if shortcut == "quick" {
        "None".to_string()
    } else {
        prompt_select("Select git remote provider:", options, "None")
    };
    if remote.as_str().to_lowercase() != "none" {
        // Custom
        if remote.as_str().to_lowercase() == "custom" {
            let env_git_server = var("PSP_GIT_CUSTOM").ok();
            let mut git_custom = if let Some(env_git_server) = env_git_server {
                info(format!("git server: {env_git_server}"));
                env_git_server
            } else {
                String::new()
            };
            while git_custom.is_empty() {
                git_custom = prompt_text(
                    "FQDN of custom git server:",
                    "None",
                    "Type FQDN of custom git server",
                );
                if git_custom.is_empty() {
                    warning("The FQDN server must not be empty".to_string());
                }
            }
            git_remote = git_custom.to_lowercase();
        } else {
            git_remote = remote.to_owned().to_lowercase() + ".com";
        }
        // Check environment variable
        let env_git_user = var("PSP_GIT_USER").ok();
        // Username of remote git service
        let mut username = if let Some(env_git_user) = env_git_user {
            info(format!("git username: {env_git_user}"));
            env_git_user
        } else {
            String::new()
        };
        while username.is_empty() {
            username = prompt_text(
                format!("Username of `{git_remote}`:").as_str(),
                "None",
                "Type username without spaces",
            );
            if username.is_empty() {
                warning("The username must not be empty".to_string());
            }
        }
        git_user = username.to_owned();
        // Add a git remote path
        let remote_path = format!(
            "git@{}:{}/{}.git",
            git_remote,
            username,
            name.to_lowercase()
        );
        // Test if remote has already been set
        let git_bin = "git";
        let mut origin_exists = make_command(
            git_bin,
            root,
            root,
            vec!["remote".to_string(), "-v".to_string()],
            false,
        );
        let output = origin_exists.output().expect("git should be installed");
        // Check if the command exits successfully
        if !output.status.success() {
            error("something wrong with `git remote -v`".to_string());
        }
        let git_verb;
        if output.stdout.len() > 0 {
            git_verb = "set-url"
        } else {
            git_verb = "add"
        }
        // Set origin remote repository
        let mut origin = make_command(
            git_bin,
            root,
            root,
            vec![
                "remote".to_string(),
                git_verb.to_string(),
                "origin".to_string(),
                remote_path,
            ],
            false,
        );
        let output = origin.output().expect("git should be installed");
        // Check if the command exits successfully
        if !output.status.success() {
            error(format!(
                "username of remote repository `{}` setting failed",
                remote.to_lowercase()
            ));
        }
        // Make remote files and folders
        // Gitlab
        if remote.as_str().to_lowercase() == "gitlab" {
            let issue_folder = Path::new(root)
                .join(remote.to_lowercase())
                .join("issue_templates");
            let merge_folder = Path::new(root)
                .join(remote.to_lowercase())
                .join("merge_request_templates");
            let dir_ret = make_dirs(issue_folder.display().to_string().as_str());
            if let Err(e) = dir_ret {
                error(format!("{e}"));
            }
            let dir_ret = make_dirs(merge_folder.display().to_string().as_str());
            if let Err(e) = dir_ret {
                error(format!("{e}"));
            }
            // Create a data map with variables
            let data = HashMap::from([
                ("SIGNATURE", SIGNATURE),
                ("VERSION", VERSION),
                ("PACKAGE", name),
            ]);
            // Feature template
            let gitlab_feature_template = issue_folder.join("feature.md").display().to_string();
            let file_ret =
                render_template("gitlab_feature.hbs", &gitlab_feature_template, data.clone());
            if !file_ret {
                error("`feature.md` render failed".to_string());
            }
            // Bug template
            let gitlab_bug_template = issue_folder.join("bug.md").display().to_string();
            let file_ret = render_template("gitlab_bug.hbs", &gitlab_bug_template, data.clone());
            if !file_ret {
                error("`bug.md` render failed".to_string());
            }
            // Merge template
            let gitlab_merge_template = merge_folder.join("merge.md").display().to_string();
            let file_ret =
                render_template("gitlab_merge.hbs", &gitlab_merge_template, data.clone());
            if !file_ret {
                error("`merge.md` render failed".to_string());
            }
        // Github
        } else if remote.as_str().to_lowercase() == "github" {
            let issue_folder = Path::new(root).join(".github").join("ISSUE_TEMPLATE");
            let merge_folder = Path::new(root)
                .join(".github")
                .join("PULL_REQUEST_TEMPLATE");
            let dir_ret = make_dirs(issue_folder.display().to_string().as_str());
            if let Err(e) = dir_ret {
                error(format!("{e}"));
            }
            let dir_ret = make_dirs(merge_folder.display().to_string().as_str());
            if let Err(e) = dir_ret {
                error(format!("{e}"));
            }
            // Create a data map with variables
            let data = HashMap::from([
                ("SIGNATURE", SIGNATURE),
                ("VERSION", VERSION),
                ("PACKAGE", name),
                ("USERNAME", username.as_str()),
            ]);
            // Feature template
            let github_feature_template = issue_folder.join("feature.yml").display().to_string();
            let file_ret =
                render_template("github_feature.hbs", &github_feature_template, data.clone());
            if !file_ret {
                error("`feature.yml` render failed".to_string());
            }
            // Bug template
            let github_bug_template = issue_folder.join("bug.yml").display().to_string();
            let file_ret = render_template("github_bug.hbs", &github_bug_template, data.clone());
            if !file_ret {
                error("`bug.yml` render failed".to_string());
            }
            // Merge template
            let github_merge_template = merge_folder
                .join("pull_request_template.md")
                .display()
                .to_string();
            let file_ret =
                render_template("github_merge.hbs", &github_merge_template, data.clone());
            if !file_ret {
                error("`pull_request_template.yml` render failed".to_string());
            }
        } else {
            if remote.to_lowercase() != "custom" {
                warning(format!(
                    "`{remote}` is not recognized as remote git provider"
                ));
            }
        }
    }
    // Write psp log
    write_log(
        LOGFILE,
        format!("{}: {} {}", log_step, remote, git_user).as_str(),
    );
    (remote, git_user)
}

// Project tox
pub fn prj_tox(name: &str, venv: bool, deps: &Vec<String>, shortcut: &String) {
    // Check psp log for update
    let log_step = "prj_tox";
    if check_log(log_step, LOGFILE) {
        return;
    }
    // Check environment variable
    let env_tox = var("PSP_TOX").unwrap_or("false".to_string()).parse().ok();
    let env_pm = var("PSP_PACKAGE_MANAGER").ok();
    let confirm = if let Some(true) = env_tox {
        info(format!("configure tox: {}", env_tox.unwrap().to_string()));
        true
    } else if shortcut == "quick" || shortcut == "simple" {
        false
    } else if shortcut == "full" {
        true
    } else {
        prompt_confirm("Do you want to configure tox?", false, "None")
    };
    // Create tox ini
    if confirm {
        let bin = if let Some(env_pm) = &env_pm {
            env_pm.as_str()
        } else {
            PIP_BIN
        };
        // Install tox
        let mut pm = make_pm(bin, name, name, vec!["tox".to_string()], venv);
        let output = pm
            .output()
            .expect(format!("{bin} should be installed").as_str());
        // Check if the command exits successfully
        if !output.status.success() {
            error("`tox` installation failed".to_string());
            return;
        }
        // Create a data map with variables
        let python_version = format!("py{}", get_python_version().replace(".", ""));
        let dependencies = deps
            .iter()
            .map(|s| {
                if s != "No" {
                    format!("\t{s}")
                } else {
                    String::new()
                }
            })
            .collect::<Vec<String>>()
            .join("\n");
        let data = HashMap::from([
            ("SIGNATURE", SIGNATURE),
            ("VERSION", VERSION),
            ("PYTHON", python_version.as_str()),
            ("DEPS", dependencies.as_str()),
        ]);
        // Tox template
        let tox_template = Path::new(name).join("tox.ini").display().to_string();
        let file_ret = render_template("tox.hbs", &tox_template, data.clone());
        if !file_ret {
            error("`tox.ini` render failed".to_string());
        }
    }
    // Write psp log
    write_log(LOGFILE, format!("{}: {}", log_step, confirm).as_str());
}

// Project documentation site generator
pub fn prj_docs(root: &str, name: &str, venv: bool, shortcut: &String) {
    // Check psp log for update
    let log_step = "prj_docs";
    if check_log(log_step, LOGFILE) {
        return;
    }
    let options = vec!["None", "Sphinx", "MKDocs"];
    // Check environment variable
    let env_docs = var("PSP_DOCS").ok();
    let env_pm = var("PSP_PACKAGE_MANAGER").ok();
    let docs = if let Some(env_docs) = env_docs {
        info(format!("documentation generator: {env_docs}"));
        env_docs
    } else if shortcut == "simple" {
        "None".to_string()
    } else {
        prompt_select("Select documentation generator:", options, "None")
    };
    if docs != "None" {
        let docs_home = Path::new(root).join("docs");
        let docs_folder = docs_home.as_path();
        // Check if folder docs exist
        if docs_folder.exists() {
            let folder_result = remove_dir_all(docs_folder);
            if let Err(e) = folder_result {
                error(format!("{e}"));
            }
        }
        // Create a docs folder
        let docs_folder = make_dirs(docs_folder.display().to_string().as_str());
        if let Err(e) = docs_folder {
            error(format!("{e}"));
        }
        let bin = if let Some(env_pm) = &env_pm {
            env_pm.as_str()
        } else {
            PIP_BIN
        };
        if docs.as_str().to_lowercase() == "sphinx" {
            // Install sphinx
            let mut pm = make_pm(bin, root, root, vec!["sphinx".to_string()], venv);
            let output = pm
                .output()
                .expect(format!("{bin} should be installed").as_str());
            // Check if the command exits successfully
            if !output.status.success() {
                error("`sphinx` installation failed".to_string());
            }
            // Check the version of a Python project
            let pyver = env_pyversion();
            // Start documentation
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            let sphinx_bin = "sphinx-quickstart";
            #[cfg(target_os = "windows")]
            let sphinx_bin = "sphinx-quickstart.exe";
            let args = vec![
                "--quiet".to_string(),
                "--sep".to_string(),
                format!("--project={}", name.to_lowercase()),
                "--author=''".to_string(),
                format!("-v='{pyver}'"),
                "--ext-autodoc".to_string(),
                "--ext-doctest".to_string(),
                "--ext-viewcode".to_string(),
                "--makefile".to_string(),
                "--quiet".to_string(),
                "--sep".to_string(),
            ];
            let mut sphinx_quickstart = make_command(
                sphinx_bin,
                root,
                docs_home.display().to_string().as_str(),
                args,
                venv,
            );
            let output = sphinx_quickstart
                .output()
                .expect(format!("{sphinx_bin} should be installed").as_str());
            if !output.status.success() {
                error("`sphinx` documentation creation failed".to_string());
            }
        } else if docs.as_str().to_lowercase() == "mkdocs" {
            // Install mkdocs
            let mut pm = make_pm(bin, root, root, vec!["mkdocs".to_string()], venv);
            let output = pm
                .output()
                .expect(format!("{bin} should be installed").as_str());
            // Check if the command exits successfully
            if !output.status.success() {
                error("`mkdocs` installation failed".to_string());
            }
            // Start documentation
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            let mkdocs_bin = "mkdocs";
            #[cfg(target_os = "windows")]
            let mkdocs_bin = "mkdocs.exe";
            let args = vec!["new".to_string(), "--quiet".to_string(), ".".to_string()];
            let mut mkdocs_new = make_command(mkdocs_bin, root, root, args, venv);
            let output = mkdocs_new
                .output()
                .expect(format!("{mkdocs_bin} should be installed").as_str());
            // Check if the command exits successfully
            if !output.status.success() {
                error("`mkdocs` documentation creation failed".to_string());
            }
        } else if docs.as_str().to_lowercase() != "none" {
            warning(format!(
                "`{docs}` is not recognized as documentation generator"
            ));
        }
        // Link requirements
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            use std::{env, os::unix::fs::symlink};
            let cwd = env::current_dir().ok();
            env::set_current_dir(&docs_home).ok();
            let requirements_file = Path::new("..").join("requirements.txt");
            if requirements_file.exists() {
                symlink(requirements_file, docs_home.join("requirements.txt")).ok();
            }
            env::set_current_dir(&cwd.unwrap()).ok();
        }
    }
    // Write psp log
    write_log(LOGFILE, format!("{}: {}", log_step, docs).as_str());
}

// Project common files
pub fn prj_files(root: &str, name: &str, container: bool, shortcut: &String) {
    // Check psp log for update
    let log_step = "prj_files";
    if check_log(log_step, LOGFILE) {
        return;
    }
    // Check environment variable
    let env_files = var("PSP_FILES").unwrap_or("false".to_string()).parse().ok();
    let confirm = if let Some(true) = env_files {
        info(format!(
            "common files creation: {}",
            env_files.unwrap().to_string()
        ));
        true
    } else if shortcut == "quick" || shortcut == "full" {
        true
    } else if shortcut == "simple" {
        false
    } else {
        prompt_confirm(
            "Do you want create common files?",
            true,
            "Create README, CONTRIBUTING, CODE_OF_CONDUCT and CHANGES",
        )
    };
    if confirm {
        // Check the version of a Python project
        let pyver = env_pyversion();
        let container_enable = if container { "true" } else { "" };
        // Create a data map with variables
        let data = HashMap::from([
            ("SIGNATURE", SIGNATURE),
            ("VERSION", VERSION),
            ("PACKAGE", name),
            ("PRJVERSION", pyver.as_str()),
            ("CONTAINER", container_enable),
        ]);
        // README template
        let readme_template = Path::new(root).join("README.md").display().to_string();
        let file_ret = render_template("readme.hbs", &readme_template, data.clone());
        if !file_ret {
            error("`README.md` render failed".to_string());
        }
        // CHANGES template
        let changes_template = Path::new(root).join("CHANGES.md").display().to_string();
        let file_ret = render_template("changes.hbs", &changes_template, data.clone());
        if !file_ret {
            error("`CHANGES.md` render failed".to_string());
        }
        // CONTRIBUTING template
        let contributing_template = Path::new(root)
            .join("CONTRIBUTING.md")
            .display()
            .to_string();
        let file_ret = render_template("contributing.hbs", &contributing_template, data.clone());
        if !file_ret {
            error("`CONTRIBUTING.md` render failed".to_string());
        }
        // Create CODE_OF_CONDUCT
        get_file_from_url(
            "https://www.contributor-covenant.org/version/2/1/code_of_conduct/code_of_conduct.md",
            ".",
            "CODE_OF_CONDUCT.md",
        );
        // SAMPLE template
        let sample_dir = Path::new(root).join("samples");
        // Check if sample folder exist
        let dir_ret = make_dirs(sample_dir.display().to_string().as_str());
        if let Err(e) = dir_ret {
            error(format!("{e}"));
        }
        let contributing_template = sample_dir
            .join(format!("{name}_sample.py"))
            .display()
            .to_string();
        let file_ret = render_template("sample.hbs", &contributing_template, data.clone());
        if !file_ret {
            error(format!("`{name}_sample.py` render failed"));
        }
    }
    // Write psp log
    write_log(LOGFILE, format!("{}: {}", log_step, confirm).as_str());
}

// Project license
pub fn prj_license(name: &str, shortcut: &String, author: &String) -> String {
    // Check psp log for update
    let log_step = "prj_license";
    if check_log(log_step, LOGFILE) {
        let log_content = read_log(LOGFILE);
        let value = get_log_value(log_step, log_content.unwrap().as_str());
        if let Some(v) = value {
            return v;
        }
    }
    // Check author
    let author = if author == "None" {
        "<maintainers>"
    } else {
        author
    };

    // Select license
    let options = vec![
        "None",
        "MIT",
        "Apache",
        "Mozilla",
        "Creative Commons",
        "Gnu Public License",
    ];
    // Check environment variable
    let env_license = var("PSP_LICENSE").ok();
    let license = if let Some(env_license) = env_license {
        info(format!("license: {env_license}"));
        env_license
    } else if shortcut == "simple" {
        "None".to_string()
    } else {
        prompt_select("Select license:", options, "None")
    };
    if license != "None" {
        let mut license_file = String::new();
        if license.to_lowercase() == "mit" {
            license_file.push_str("mit.hbs");
        } else if license.to_lowercase() == "apache" {
            license_file.push_str("apache.hbs");
        } else if license.to_lowercase() == "creative commons" || license.to_lowercase() == "cc" {
            license_file.push_str("cc.hbs");
        } else if license.to_lowercase() == "mozilla" {
            license_file.push_str("mozilla.hbs");
        } else if license.to_lowercase() == "gnu public license" || license.to_lowercase() == "gpl"
        {
            license_file.push_str("gplv3.hbs");
        } else if license.as_str().to_lowercase() != "none" {
            warning(format!("`{license}` is not recognized as a valid license"));
        }
        let license_template = Path::new(name).join(&license_file).display().to_string();
        // Create a data map with variables
        let package_name = Path::new(name).file_name().unwrap().to_str().unwrap();
        let data = HashMap::from([
            ("SIGNATURE", SIGNATURE),
            ("VERSION", VERSION),
            ("AUTHOR", author),
            ("PACKAGE", package_name),
        ]);
        let file_ret = render_template(
            &license_file,
            &license_template.replace(&license_file, "LICENSE.md"),
            data,
        );
        if !file_ret {
            error("`LICENSE.md` render failed".to_string());
        }
    }
    // Write psp log
    write_log(LOGFILE, format!("{}: {}", log_step, license).as_str());
    license
}

// Project pypi dependencies
pub fn prj_pypi(root: &str, venv: bool, shortcut: &String) -> bool {
    // Check psp log for update
    let log_step = "prj_pypi";
    if check_log(log_step, LOGFILE) {
        let log_content = read_log(LOGFILE);
        let value = get_log_value(log_step, log_content.unwrap().as_str());
        if let Some(v) = value {
            return v.parse::<bool>().unwrap();
        }
    }
    // Check environment variable
    let env_pypi = var("PSP_PYPI").unwrap_or("false".to_string()).parse().ok();
    let env_pm = var("PSP_PACKAGE_MANAGER").ok();
    let mut ret = false;
    let confirm = if let Some(true) = env_pypi {
        info(format!(
            "install publish packages: {}",
            env_pypi.unwrap().to_string()
        ));
        true
    } else if shortcut == "quick" || shortcut == "full" {
        true
    } else if shortcut == "simple" {
        false
    } else {
        prompt_confirm(
            "Do you want to install dependencies to publish on pypi?",
            true,
            "None",
        )
    };
    if confirm {
        let bin = if let Some(env_pm) = &env_pm {
            env_pm.as_str()
        } else {
            PIP_BIN
        };
        // Install twine and build
        let mut pm = make_pm(
            bin,
            root,
            root,
            vec!["twine".to_string(), "build".to_string()],
            venv,
        );
        let output = pm
            .output()
            .expect(format!("{bin} should be installed").as_str());
        // Check if the command exits successfully
        if !output.status.success() {
            error("`twine` and/or `build` installation failed".to_string());
        } else {
            ret = true;
        }
    }
    // Write psp log
    write_log(LOGFILE, format!("{}: {}", log_step, ret).as_str());
    ret
}

// Project Docker/Podman
pub fn prj_container(root: &str, name: &str, shortcut: &String) -> bool {
    // Check psp log for update
    let log_step = "prj_container";
    if check_log(log_step, LOGFILE) {
        let log_content = read_log(LOGFILE);
        let value = get_log_value(log_step, log_content.unwrap().as_str());
        if let Some(v) = value {
            return v.parse::<bool>().unwrap();
        }
    }
    // Check environment variable
    let ret;
    let env_container = var("PSP_CONTAINER")
        .unwrap_or("false".to_string())
        .parse()
        .ok();
    let confirm = if let Some(true) = env_container {
        info(format!(
            "create container files: {}",
            env_container.unwrap().to_string()
        ));
        true
    } else if shortcut == "quick" || shortcut == "full" {
        true
    } else if shortcut == "simple" {
        false
    } else {
        prompt_confirm(
            "Do you want to create a Dockerfile and Containerfile?",
            true,
            "None",
        )
    };
    if confirm {
        // Create a data map with variables
        let data = HashMap::from([
            ("SIGNATURE", SIGNATURE),
            ("VERSION", VERSION),
            ("PACKAGE", name),
        ]);
        let container_template = Path::new(root).join("Dockerfile").display().to_string();
        let file_ret = render_template("containerfile.hbs", &container_template, data.clone());
        // Copy Dockerfile to Containerfile
        copy(
            &container_template,
            &container_template.replace("Dockerfile", "Containerfile"),
        )
        .ok();
        if !file_ret {
            error("`Dockerfile` render failed".to_string());
        }
        // Create .dockerignore/.containerignore
        let container_ignore_template = Path::new(root).join(".dockerignore").display().to_string();
        let file_ret = render_template(
            "container_ignore.hbs",
            &container_ignore_template,
            data.clone(),
        );
        // Copy .dockerignore to .containerignore
        copy(
            &container_ignore_template,
            &container_ignore_template.replace(".dockerignore", ".containerignore"),
        )
        .ok();
        if !file_ret {
            error("`.dockerignore` render failed".to_string());
        }
        ret = true;
    } else {
        ret = false;
    }
    // Write psp log
    write_log(LOGFILE, format!("{}: {}", log_step, ret).as_str());
    ret
}

// Project Makefile
pub fn prj_makefile(root: &str, name: &str, tests: bool, build: bool, container: bool) {
    // Set options for make
    let mut make_options = vec!["help", "all", "run", "clean"];
    // Create a data map with variables
    let mut data = HashMap::from([
        ("SIGNATURE", SIGNATURE),
        ("VERSION", VERSION),
        ("PACKAGE", name),
    ]);
    let container_value = format!(
        "
container:
\tpodman build . -t {name}:latest || docker build . -t {name}:latest"
    );
    if tests {
        make_options.push("test");
        let test_value = "
test:
ifneq ('$(wildcard ${VENV}/bin/${PYTHON})','')
\t${VENV}/bin/${PYTHON} -m unittest
else
\t${PYTHON} -m unittest
endif";
        data.insert("TEST", test_value);
    }
    if build {
        make_options.push("build");
        make_options.push("deploy");
        let build_value = "
build:
ifneq ('$(wildcard ${VENV}/bin/${PYTHON})','')
\t${VENV}/bin/${PYTHON} -m build
else
\t${PYTHON} -m build
endif

deploy:
ifneq ('$(wildcard ${VENV}/bin/${PYTHON})','')
\t${VENV}/bin/${PYTHON} -m twine upload dist/*
else
\t${PYTHON} -m twine upload dist/*
endif";
        data.insert("BUILD", build_value);
    }
    if container {
        make_options.push("container");
        data.insert("CONTAINER", container_value.as_str());
    }
    let options = make_options.join(" ");
    let actions = make_options.join("|");
    data.insert("OPTIONS", options.as_str());
    data.insert("ACTIONS", actions.as_str());
    let makefile_template = Path::new(root).join("Makefile").display().to_string();
    let file_ret = render_template("makefile.hbs", &makefile_template, data.clone());
    if !file_ret {
        error("`Makefile` render failed".to_string());
    }
}
