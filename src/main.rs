use dotenv::dotenv;
use inquire::{Confirm, Select, Text};
use std::{
    env::{args, var},
    fs::{create_dir_all, remove_dir_all, File},
    io::Write,
    path::{absolute, Path},
    process::exit,
};

// Constants
const VERSION: &str = "0.1.1";
const ARGS: [&str; 4] = ["help", "quick", "simple", "full"];

// Utility functions

// Function for check if tool is installed
fn check_tool(tool: &str) {
    let env_paths = var("PATH").unwrap();
    let paths: Vec<&str> = env_paths.split(':').collect();
    for path in paths {
        let tool_path = format!("{path}/{tool}");
        if Path::new(&tool_path).exists() {
            return;
        }
    }
    eprintln!("error: {} is not installed", tool);
    exit(1);
}

// Function for creating folders and parents
fn make_dirs(dir: &str) -> std::io::Result<()> {
    let result = create_dir_all(dir);
    if let Err(e) = result {
        eprintln!("error: {}", e);
        exit(2);
    } else {
        Ok(())
    }
}

// Function for creating file with contents
fn make_file(file: &str, content: String) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    let result = file.write_all(&content.as_bytes());
    if let Err(e) = result {
        eprintln!("error: {}", e);
        exit(3);
    } else {
        Ok(())
    }
}

// Function for prompt text
fn prompt_text(question: &str, default: &str, help: &str) -> String {
    let answer = if help != "None" && default != "None" {
        Text::new(question)
            .with_help_message(help)
            .with_default(default)
            .prompt()
    } else if help != "None" {
        Text::new(question).with_help_message(help).prompt()
    } else if default != "None" {
        Text::new(question).with_default(default).prompt()
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

// Function for prompt selections
fn prompt_select(question: &str, options: Vec<&str>, help: &str) -> String {
    let answer = if help != "None" {
        Select::new(question, options)
            .with_help_message(help)
            .prompt()
    } else {
        Select::new(question, options).prompt()
    };
    answer.unwrap().to_string()
}

// Function to print help
fn print_help(exit_code: i32) {
    println!("usage: psp [shortcut]");
    println!("ie: psp [help|quick|simple|full]");
    exit(exit_code)
}

// Function that capture keyword argument
fn get_shortcut() -> String {
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        let shorcut = &args[1];
        if !ARGS.contains(&shorcut.as_str()) {
            eprintln!("error: unknown shortcut command `{}`", shorcut);
            print_help(1);
        }
        shorcut.clone()
    } else {
        "None".to_string()
    }
}

// Function that load env files
fn load_env() {
    // Load first, .env file from current working directory
    dotenv().ok();
    // Load second, .psp.envmfile from home
    let home_var = var("HOME").unwrap();
    let home_env = format!("{home_var}/.psp.env");
    dotenv::from_filename(home_env).ok();
}

// Core functions

// Project name
fn prj_name() -> (String, String) {
    // Check enviroment variable
    let env_name = var("PSP_NAME").ok();
    let name = if let Some(env_name) = env_name {
        env_name
    } else {
        prompt_text("Name of Python project:", "None", "Type name or path")
            .trim()
            .trim_end_matches("/")
            .to_string()
    };
    // Check if name is a path
    let package: String = if name.contains('/') {
        let parts: Vec<&str> = name.split('/').collect();
        let last = parts.last();
        if !last.is_none() {
            last.unwrap().to_lowercase().to_string()
        } else {
            String::new()
        }
    } else {
        name.to_lowercase()
    };
    // Check if package is empty
    if package.is_empty() {
        eprintln!("error: empty word is not allowed");
        exit(1)
    }
    // Check if package contains spaces, and replace it with underscores
    let package = package.replace(" ", "_");
    let root = name.clone();
    let project = format!("{root}/{package}");
    // Make directories structure
    let package_path = Path::new(&project);
    // Check if project path already exists
    if package_path.exists() {
        let project_exists = prompt_confirm(
            format!("Path `{root}` exists. Do you want continue?").as_str(),
            false,
            "Some files will be overwritten",
        );
        if !project_exists {
            println!("info: the path `{root}` unchanged");
            exit(0)
        }
    }
    let dir_ret = make_dirs(format!("{project}").as_str());
    if let Err(e) = dir_ret {
        eprintln!("error: {}", e);
    }
    // Make file structures
    let file_ret = make_file(
        format!("{project}/__init__.py").as_str(),
        format!(
            "#! /usr/bin/env python3
# -*- encoding: utf-8 -*-
# vim: se ts=4 et syn=python:
# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}

__version__ = '0.0.1'
"
        ),
    );
    if let Err(e) = file_ret {
        eprintln!("error: {}", e);
        exit(4);
    }
    let main_file = make_file(
        format!("{project}/__main__.py").as_str(),
        format!(
            "#! /usr/bin/env python3
# -*- encoding: utf-8 -*-
# vim: se ts=4 et syn=python:
# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}

from .__init__ import __version__

print('name: {} ')
print(f'version: {{__version__}}')
",
            package.to_lowercase()
        ),
    );
    if let Err(e) = main_file {
        eprintln!("error: {}", e);
        exit(4);
    }
    (root, package)
}

// Project git
fn prj_git(name: &str, shortcut: &String) -> bool {
    let confirm: bool;
    if shortcut == "quick" || shortcut == "full" {
        confirm = true;
    } else if shortcut == "simple" {
        confirm = false;
    } else {
        confirm = prompt_confirm("Do you want to start git repository?", true, "None");
    }
    if confirm {
        let output = std::process::Command::new("git")
            .arg("init")
            .current_dir(name)
            .output()
            .expect("git should be installed");
        // Check if command exit successfully
        if !output.status.success() {
            eprintln!("error: something wrong with `git init`");
            return false;
        }
        // Create .gitignore file
        let file_ret = make_file(
            format!("{name}/.gitignore").as_str(),
            format!(
                "# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}
### Python ###
__pycache__/
*.py[cod]
*$py.class
build/
develop-eggs/
dist/
downloads/
eggs/
.eggs/
lib/
lib64/
parts/
sdist/
var/
wheels/
share/python-wheels/
*.egg-info/
.installed.cfg
*.egg

# Environments
.env/
.venv/
env/
venv/
ENV/
env.bak/
venv.bak/

# Sphinx documentation
docs/_build/
# mkdocs documentation
site"
            ),
        );
        let ret = if let Err(e) = file_ret {
            eprintln!("error: {}", e);
            false
        } else {
            true
        };
        return ret;
    }
    false
}

// Project unit tests
fn prj_test(root: &str, name: &str, shortcut: &String) -> bool {
    let confirm: bool;
    if shortcut != "None" {
        confirm = true;
    } else {
        confirm = prompt_confirm("Do you want unit test files?", true, "None");
    }
    if confirm {
        let project_name = name.to_lowercase();
        // Make directories structure
        let dir_ret = make_dirs(format!("{root}/tests").as_str());
        if let Err(e) = dir_ret {
            eprintln!("error: {}", e);
            return false;
        }
        // Make file structures
        let init_file = make_file(
            format!("{root}/tests/__init__.py").as_str(),
            format!(
                "#! /usr/bin/env python3
# -*- encoding: utf-8 -*-
# vim: se ts=4 et syn=python:
# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}


"
            ),
        );
        if let Err(e) = init_file {
            eprintln!("error: {}", e);
            return false;
        }
        let all_module = make_file(
            format!("{root}/tests/test_{project_name}.py").as_str(),
            format!(
                "#! /usr/bin/env python3
# -*- encoding: utf-8 -*-
# vim: se ts=4 et syn=python:
# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}


import unittest
from {project_name} import __version__


class TestAll(unittest.TestCase):

    def test_all(self):
        self.assertEqual(__version__, '0.0.1')
        print('Test all {project_name} successfully!')


# Test functions for pytest
def test_all():
    assert __version__ == '0.0.1'


if __name__ == '__main__':
    unittest.main()"
            )
            .to_string(),
        );
        if let Err(e) = all_module {
            eprintln!("error: {}", e);
            return false;
        }
        true
    } else {
        false
    }
}

// Project venv
fn prj_venv(name: &str, shortcut: &String) -> bool {
    let confirm: bool;
    if shortcut == "quick" || shortcut == "full" {
        confirm = true;
    } else if shortcut == "simple" {
        confirm = false;
    } else {
        confirm = prompt_confirm("Do you want to create a virtual environment?", true, "None");
    }
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
fn prj_deps(name: &str, venv: bool, shortcut: &String) -> Vec<String> {
    let deps: String;
    if shortcut == "simple" || shortcut == "quick" {
        deps = "No".to_string();
    } else {
        deps = prompt_text(
            "Install dependencies:",
            "No",
            "Write package(s) separates with spaces or empty",
        );
    }
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
        // Build a requirements.txt file
        let content = format!(
            "# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}\n\n{}",
            dependencies.join("\n")
        );
        let requirements = make_file(format!("{name}/requirements.txt").as_str(), content);
        if let Err(e) = requirements {
            eprintln!("error: {}", e);
        }
    }
    dependencies
}

// Project pyproject.toml
fn prj_toml(root: &str, name: &str, deps: &Vec<String>, mut license: String) {
    let mut classifiers = vec!["Programming Language :: Python :: 3"];
    // Check dependencies
    let requirements = if deps.contains(&"No".to_string()) {
        "[]".to_string()
    } else {
        format!("{deps:?}")
    };
    // Check license
    if license == "None" {
        license = String::new();
    } else if license == "MIT" {
        license = "MIT License".to_string();
        classifiers.push("License :: OSI Approved :: MIT License")
    } else if license == "Apache" {
        license = "Apache Software License".to_string();
        classifiers.push("License :: OSI Approved :: Apache Software License")
    } else if license == "Creative Commons" {
        license = "Common Public License".to_string();
        classifiers.push("License :: OSI Approved :: Common Public License")
    } else if license == "Mozilla" {
        license = "Mozilla Public License 2.0 (MPL 2.0)".to_string();
        classifiers.push("License :: OSI Approved :: Mozilla Public License 2.0 (MPL 2.0)")
    } else if license == "Gnu Public License" {
        license = "GNU General Public License v3 (GPLv3)".to_string();
        classifiers.push("License :: OSI Approved :: GNU General Public License v3 (GPLv3)")
    }
    let content = format!(
        "# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}

[build-system]
requires = ['setuptools', 'wheel']
build-backend = 'setuptools.build_meta'

[project]
name = '{}'
version = '0.0.1'
readme = 'README.md'
license = {{text = '{}'}}
authors = [{{name = 'psp', email = 'psp@example.com'}}]
maintainers = [{{name = 'psp', email = 'psp@example.com'}}]
description = 'A simple but structured Python project'
requires-python = '>=3.13'
classifiers = {:?}
dependencies = {}

[project.urls]
homepage = 'https://www.python.org/'
documentation = 'https://docs.python.org/3/'
repository = 'https://github.com/python'
changelog = 'https://docs.python.org/3/whatsnew/changelog.html'
",
        name.to_lowercase(),
        license,
        classifiers,
        requirements
    );
    // Write pyproject.toml
    let pyproject = make_file(format!("{root}/pyproject.toml").as_str(), content);
    if let Err(e) = pyproject {
        eprintln!("error: {}", e);
        return;
    }
}

// Project CI
fn prj_ci(name: &str, deps: &Vec<String>, shortcut: &String) {
    let options = vec!["None", "TravisCI", "CircleCI"];
    let ci: String;
    if shortcut == "simple" || shortcut == "quick" {
        ci = "None".to_string();
    } else {
        ci = prompt_select("Select remote CI provider:", options, "None");
    }
    let requirements = if deps.contains(&"No".to_string()) {
        "".to_string()
    } else {
        deps.join(" ")
    };
    // Travis or CircleCI
    if ci.as_str() == "TravisCI" {
        let travis = make_file(
            format!("{name}/.travis.yml").as_str(),
            format!(
                "# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}

language: python
cache: pip
python:
  - 3.11
  - 3.12
  - 3.13
before_install:
  - sudo apt-get update
  - sudo apt-get install python3-pip
  - sudo apt-get install python3-pytest
install:
  - pip install {requirements} pipenv
  - pipenv install --dev
script:
  - python -m unittest discover tests
  - pytest tests"
            ),
        );
        if let Err(e) = travis {
            eprintln!("error: {}", e);
        }
    } else if ci.as_str() == "CircleCI" {
        let dir_ret = make_dirs(format!("{name}/.circleci").as_str());
        if let Err(e) = dir_ret {
            eprintln!("error: {}", e);
        }
        let circle = make_file(
            format!("{name}/.circleci/config.yml").as_str(),
            format!(
                "# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}

version: 2.1
jobs:
  build-and-test:
    docker:
      - image: circleci/python
    steps:
      - checkout
      - run:
          name: Install pytest
          command: pip install pytest
      - run:
          name: Install dependencies
          command: pip install {requirements}
      - run:
          name: Install package
          command: pip install .
      - run:
          name: Run tests
          command: python -m pytest tests
  workflows:
    main:
      jobs:
        - build-and-test
"
            ),
        );
        if let Err(e) = circle {
            eprintln!("error: {}", e);
        }
    }
}

// Project Gitlab/GitHub
fn prj_remote(root: &str, name: &str, shortcut: &String) {
    let options = vec!["None", "Gitlab", "Github"];
    let remote: String;
    if shortcut == "quick" {
        remote = "None".to_string();
    } else {
        remote = prompt_select("Select git remote provider:", options, "None");
    }
    if remote.as_str() != "None" {
        // Username of remote git service
        let username = prompt_text(format!("Username of {remote}:").as_str(), "None", "None");
        if username.is_empty() {
            eprintln!("error: the username must be not empty");
            return;
        }
        // Add git remote path
        let remote_path = format!(
            "git@{}.com:{}/{}.git",
            remote.to_lowercase(),
            username,
            name.to_lowercase()
        );
        // Test if remote has already been setted
        let origin = std::process::Command::new("git")
            .args(["remote", "-v"])
            .current_dir(root)
            .output()
            .expect("git should be installed");
        let git_verb;
        if origin.stdout.len() > 0 {
            git_verb = "set-url"
        } else {
            git_verb = "add"
        }
        // Set origin remote repository
        let output = std::process::Command::new("git")
            .args(["remote", git_verb, "origin", &remote_path])
            .current_dir(root)
            .output()
            .expect("git should be installed");
        // Check if command exit successfully
        if !output.status.success() {
            eprintln!(
                "error: username of remote repository {} setting failed",
                remote.to_lowercase()
            );
        }
        // Make remote files and folders
        // Gitlab
        if remote.as_str() == "Gitlab" {
            let issue_folder = format!("{}/.{}/issue_templates", root, remote.to_lowercase());
            let merge_folder = format!(
                "{}/.{}/merge_request_templates",
                root,
                remote.to_lowercase()
            );
            let dir_ret = make_dirs(issue_folder.as_str());
            if let Err(e) = dir_ret {
                eprintln!("error: {}", e);
            }
            let dir_ret = make_dirs(merge_folder.as_str());
            if let Err(e) = dir_ret {
                eprintln!("error: {}", e);
            }
            let feature_content = format!(
                "<!-- Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION} -->

## Description

Description of the proposal

## Proposed names of the new function, class or variables of {} package

* function or class name
* possible argument(s)

Additional context

/label ~CR",
                name.to_lowercase()
            );
            let feature_issue = make_file(
                format!("{issue_folder}/feature.md").as_str(),
                feature_content,
            );
            if let Err(e) = feature_issue {
                eprintln!("error: {}", e);
            }
            let bug_content = format!(
                "<!-- Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION} -->

## Description of problem

Provide a concise description of the bug

## Steps to Reproduce

Lines of code

## Expected Behaviour

Description of what is expected

## Your Environment

* {} version used:
* Operating System and version:

Additional context

/label ~Bug",
                name.to_lowercase()
            );
            let bug_issue = make_file(format!("{issue_folder}/bug.md").as_str(), bug_content);
            if let Err(e) = bug_issue {
                eprintln!("error: {}", e);
            }
            let merge_content = format!(
                "<!-- Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION} -->

## What does this MR do and why?

%{{first_multiline_commit}}

## MR acceptance checklist of {}

**Please evaluate this MR against the [MR acceptance checklist](https://docs.gitlab.com/ee/development/code_review.html#acceptance-checklist).**
It helps you analyze changes to reduce risks in quality, performance, reliability, security, and maintainability.

## How to set up and validate locally

Numbered steps to set up and validate the change are strongly suggested.

/assign me",
                name.to_lowercase()
            );
            let merge_issue = make_file(format!("{merge_folder}/merge.md").as_str(), merge_content);
            if let Err(e) = merge_issue {
                eprintln!("error: {}", e);
            }
        // Github
        } else if remote.as_str() == "Github" {
            let issue_folder = format!("{}/.{}/ISSUE_TEMPLATE", root, remote.to_lowercase());
            let merge_folder = format!("{}/.{}/PULL_REQUEST_TEMPLATE", root, remote.to_lowercase());
            let dir_ret = make_dirs(issue_folder.as_str());
            if let Err(e) = dir_ret {
                eprintln!("error: {}", e);
            }
            let dir_ret = make_dirs(merge_folder.as_str());
            if let Err(e) = dir_ret {
                eprintln!("error: {}", e);
            }
            let config_content = "blank_issues_enabled: false\n".to_string();
            let config_file = make_file(
                format!("{issue_folder}/config.yml").as_str(),
                config_content,
            );
            if let Err(e) = config_file {
                eprintln!("error: {}", e);
            }
            let feature_content = format!(
                "# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}

name: Feature Request
description: File a feature request.
title: '[Feature]: '
labels: ['enhancement']
assignees:
  - {}
body:
  - type: markdown
    attributes:
      value: '## Feature Request for {}!'
  - type: textarea
    attributes:
      label: Description
      description: A concise description of what you're experiencing.
      placeholder: Description of the proposal
    validations:
      required: true
  - type: textarea
    attributes:
      label: New proposed
      description: Proposed names of the new function, class or variables.
      placeholder: |
        * function or class name
        * possible argument(s)
    validations:
      required: true
  - type: textarea
    attributes:
      label: Additional context
      description: Other considerizations
    validations:
      required: false
",
                username,
                name.to_lowercase()
            );
            let feature_issue = make_file(
                format!("{issue_folder}/feature.yml").as_str(),
                feature_content,
            );
            if let Err(e) = feature_issue {
                eprintln!("error: {}", e);
            }
            let bug_content = format!(
                "# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}

name: Bug Report
description: File a bug report.
title: '[Bug]: '
labels: ['bug']
assignees:
  - {}
body:
  - type: markdown
    attributes:
      value: '## Bug Report for {}!'
  - type: textarea
    attributes:
      label: Description of problem
      description: Provide a concise description of the bug.
      placeholder: Describe here the problem
    validations:
      required: true
  - type: textarea
    attributes:
      label: Steps to Reproduce
      description: Lines of code.
      placeholder: Paste here backtrace or lines of code
    validations:
      required: false
  - type: textarea
    attributes:
      label: Expected Behaviour
      description: Description of what is expected.
    validations:
      required: true
  - type: textarea
    attributes:
      label: Your Environment
      description: Description of your environment.
      placeholder: |
        * {} version used:
        * Operating System and version:
    validations:
      required: false
  - type: textarea
    attributes:
      label: Additional context
      description: Other considerizations
    validations:
      required: false
",
                username,
                name.to_lowercase(),
                name.to_lowercase()
            );
            let bug_issue = make_file(format!("{issue_folder}/bug.yml").as_str(), bug_content);
            if let Err(e) = bug_issue {
                eprintln!("error: {}", e);
            }
            let merge_content = format!(
                "<!-- Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION} -->

# Title of Pull Request

---
name: Tracking issue
about: Use this template for tracking new features.
title: '[DATE]: [FEATURE NAME]'
labels: enhancement
assignees: {}
---

## Describe your changes

## Issue ticket number and link

## Checklist before requesting a review
- [ ] I have performed a self-review of my code
- [ ] If it is a core feature, I have added thorough tests.
- [ ] Do we need to implement analytics?
- [ ] Will this be part of a product update? If yes, please write one phrase about this update.
",
                username
            );
            let merge_issue = make_file(
                format!("{merge_folder}/pull_request_template.md").as_str(),
                merge_content,
            );
            if let Err(e) = merge_issue {
                eprintln!("error: {}", e);
            }
        }
    }
}

// Project tox
fn prj_tox(name: &str, venv: bool, deps: &Vec<String>, shortcut: &String) {
    // Create tox ini
    let confirm: bool;
    if shortcut == "quick" || shortcut == "simple" {
        confirm = false;
    } else if shortcut == "full" {
        confirm = true;
    } else {
        confirm = prompt_confirm("Do you want to configure tox?", false, "None");
    }
    if confirm {
        let mut pip = std::process::Command::new("pip3");
        // Activate venv
        if venv {
            pip.env("PATH", "venv/bin");
        }
        let output = pip
            .arg("install")
            .arg("--timeout=10")
            .arg("--retries=1")
            .arg("tox")
            .current_dir(&name)
            .output()
            .expect("pip should be installed");
        // Check if command exit successfully
        if !output.status.success() {
            eprintln!("error: tox installation failed");
            return;
        }
        // Write tox.ini
        let tox_ini_content = format!(
            "# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}

[tox]
envlist = py311, py312, py313
isolated_build = True

[testenv]
labels = test, core
deps=
    pytest
{}
commands = pytest tests",
            deps.iter()
                .map(|s| if s != "No" {
                    format!("\t{s}")
                } else {
                    String::new()
                })
                .collect::<Vec<String>>()
                .join("\n")
        );
        let tox_ini = make_file(format!("{name}/tox.ini").as_str(), tox_ini_content);
        if let Err(e) = tox_ini {
            eprintln!("error: {}", e);
        }
    }
}

// Project documentation site generator
fn prj_docs(root: &str, name: &str, venv: bool, shortcut: &String) {
    let options = vec!["None", "Sphinx", "MKDocs"];
    let docs: String;
    if shortcut == "simple" {
        docs = "None".to_string();
    } else {
        docs = prompt_select("Select documentation generator:", options, "None");
    }
    if docs != "None" {
        let docs_home = format!("{root}/docs");
        let docs_folder = Path::new(&docs_home);
        // Check if folder docs exists
        if docs_folder.exists() {
            let folder_result = remove_dir_all(docs_folder);
            if let Err(e) = folder_result {
                eprintln!("error: {}", e);
            }
        }
        // Create docs folder
        let docs_folder = make_dirs(format!("{docs_home}").as_str());
        if let Err(e) = docs_folder {
            eprintln!("error: {}", e);
        }
        if docs.as_str() == "Sphinx" {
            // Install sphinx
            let mut pip = std::process::Command::new("pip3");
            // Activate venv
            if venv {
                pip.env("PATH", "venv/bin");
            }
            let output = pip
                .arg("install")
                .arg("--timeout=10")
                .arg("--retries=1")
                .arg("sphinx")
                .current_dir(&root)
                .output()
                .expect("pip should be installed");
            // Check if command exit successfully
            if !output.status.success() {
                eprintln!("error: sphinx installation failed");
                return;
            }
            // Start documentation
            let sphinx_bin = "../venv/bin/sphinx-quickstart".to_string();
            let mut sphinx_quickstart = std::process::Command::new(sphinx_bin);
            // Activate venv
            if venv {
                sphinx_quickstart.env("PATH", "venv/bin");
            }
            let output = sphinx_quickstart
                .arg("--quiet")
                .arg("--sep")
                .arg(format!("--project={}", name.to_lowercase()))
                .arg("--author=''")
                .arg("-v='0.0.1'")
                .arg("--ext-autodoc")
                .arg("--ext-doctest")
                .arg("--ext-viewcode")
                .arg("--makefile")
                .current_dir(docs_home)
                .output()
                .expect("sphinx-quickstart should be installed");
            // Check if command exit successfully
            if !output.status.success() {
                eprintln!("error: sphinx documentation creation failed");
            }
        } else if docs.as_str() == "MKDocs" {
            // Install mkdocs
            let mut pip = std::process::Command::new("pip3");
            // Activate venv
            if venv {
                pip.env("PATH", "venv/bin");
            }
            let output = pip
                .arg("install")
                .arg("--timeout=10")
                .arg("--retries=1")
                .arg("mkdocs")
                .current_dir(&root)
                .output()
                .expect("pip should be installed");
            // Check if command exit successfully
            if !output.status.success() {
                eprintln!("error: mkdocs installation failed");
                return;
            }
            // Start documentation;
            let mkdocs_bin = "venv/bin/mkdocs".to_string();
            let mut mkdocs_new = std::process::Command::new(mkdocs_bin);
            // Activate venv
            if venv {
                mkdocs_new.env("PATH", "venv/bin");
            }
            let output = mkdocs_new
                .arg("new")
                .arg("--quiet")
                .arg(".")
                .current_dir(&root)
                .output()
                .expect("mkdocs should be installed");
            // Check if command exit successfully
            if !output.status.success() {
                eprintln!("error: mkdocs documentation creation failed");
            }
        }
    }
}

// Project common files
fn prj_files(root: &str, name: &str, shortcut: &String) {
    let confirm: bool;
    if shortcut == "quick" || shortcut == "full" {
        confirm = true;
    } else if shortcut == "simple" {
        confirm = false;
    } else {
        confirm = prompt_confirm(
            "Do you want create common files?",
            true,
            "Create README, CONTRIBUTING, CODE_OF_CONDUCT and CHANGES",
        );
    }
    if confirm {
        // Create README
        let readme_content = format!(
            "<!-- Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION} -->

# Welcome to {name}

## Install {name}
```console
pip install .
```

## Acknowledgments
Thanks Python Community!"
        );
        let readme = make_file(format!("{root}/README.md").as_str(), readme_content);
        if let Err(e) = readme {
            eprintln!("error: {}", e);
        }
        // Create CHANGES
        let changes_content = format!(
            "<!-- Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION} -->

# Release notes of {name}

## 0.0.1
- Start **{name}** project"
        );
        let changes = make_file(format!("{root}/CHANGES.md").as_str(), changes_content);
        if let Err(e) = changes {
            eprintln!("error: {}", e);
        }
        // Create CONTRIBUTING
        let contributing_content = format!(
            "<!-- Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION} -->

# Contributing to {name}

Bug fixes, feature additions, tests, documentation and more can be contributed via issues and/or pull requests.
**All contributions are welcome**.

## Bug fixes, feature additions, etc.

Please send a pull request to the `main` branch.
Please include documentation and tests for new features.
Tests or documentation without bug fixes or feature additions are welcome too.
Feel free to ask questions via issues, discussions, or mail.

- Fork the {name} repository.
- Create a branch from `main`.
- Develop bug fixes, features, tests, etc.
- Run the test suite.
- Create a pull request to pull the changes from your branch to the {name} `main`.

### Guidelines

- Separate code commits from reformatting commits.
- Provide tests for any newly added code.
- Follow PEP 8.
- Include release notes as needed or appropriate with your bug fixes, feature additions and tests.
- Do not add to the changelog for proposed changes, as that is updated after changes are merged.
"
        );
        let contributing = make_file(
            format!("{root}/CONTRIBUTING.md").as_str(),
            contributing_content,
        );
        if let Err(e) = contributing {
            eprintln!("error: {}", e);
        }
        // Create CODE_OF_CONDUCT
        let output = std::process::Command::new("curl")
            .arg("-oCODE_OF_CONDUCT.md")
            .arg("-k")
            .arg("--connect-timeout")
            .arg("10")
            .arg("https://www.contributor-covenant.org/version/2/1/code_of_conduct/code_of_conduct.md")
            .current_dir(&root)
            .output()
            .expect("curl should be installed");
        // Check if command exit successfully
        if !output.status.success() {
            eprintln!("error: CODE_OF_CONDUCT download failed");
        }
    }
}

// Project license
fn prj_license(name: &str, shortcut: &String) -> String {
    // Select license
    let options = vec![
        "None",
        "MIT",
        "Apache",
        "Mozilla",
        "Creative Commons",
        "Gnu Public License",
    ];
    let license: String;
    if shortcut == "simple" {
        license = "None".to_string();
    } else {
        license = prompt_select("Select license:", options, "None");
    }
    let mut license_url = String::new();
    if license == "MIT" {
        license_url.push_str("https://www.mit.edu/~amini/LICENSE.md")
    } else if license == "Apache" {
        license_url.push_str("https://www.apache.org/licenses/LICENSE-2.0.txt")
    } else if license == "Creative Commons" {
        license_url.push_str("https://creativecommons.org/licenses/by/4.0/legalcode.txt")
    } else if license == "Mozilla" {
        license_url.push_str("https://www.mozilla.org/media/MPL/2.0/index.f75d2927d3c1.txt")
    } else if license == "Gnu Public License" {
        license_url.push_str("https://www.gnu.org/licenses/gpl-3.0.md")
    }
    if !license_url.is_empty() {
        // Create LICENSE
        let output = std::process::Command::new("curl")
            .arg("-oLICENSE.md")
            .arg("-k")
            .arg("--connect-timeout")
            .arg("10")
            .arg(license_url)
            .current_dir(&name)
            .output()
            .expect("curl should be installed");
        // Check if command exit successfully
        if !output.status.success() {
            eprintln!("error: LICENSE download failed");
        }
    }
    license
}

// Project pypi dependencies
fn prj_pypi(root: &str, venv: bool, shortcut: &String) -> bool {
    let confirm: bool;
    if shortcut == "quick" || shortcut == "full" {
        confirm = true;
    } else if shortcut == "simple" {
        confirm = false;
    } else {
        confirm = prompt_confirm(
            "Do you want to install dependencies to publish on pypi?",
            true,
            "None",
        );
    }
    if confirm {
        // Install twine and build
        let mut pip = std::process::Command::new("pip3");
        // Activate venv
        if venv {
            pip.env("PATH", "venv/bin");
        }
        let output = pip
            .arg("install")
            .arg("--timeout=10")
            .arg("--retries=1")
            .arg("twine")
            .arg("build")
            .current_dir(&root)
            .output()
            .expect("pip should be installed");
        // Check if command exit successfully
        if !output.status.success() {
            eprintln!("error: build and twine installation failed");
            return false;
        }
        return true;
    }
    false
}

// Project Docker/Podman
fn prj_docker(root: &str, name: &str, shortcut: &String) -> bool {
    let confirm: bool;
    if shortcut == "quick" || shortcut == "full" {
        confirm = true;
    } else if shortcut == "simple" {
        confirm = false;
    } else {
        confirm = prompt_confirm(
            "Do you want to create a Dockerfile and Containerfile?",
            true,
            "None",
        );
    }
    if confirm {
        // Create README
        let dockerfile_content = format!(
            "# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}

FROM python:alpine

# Copy the Python package
COPY {name} /{name}/{name}
COPY pyproject.toml /{name}
WORKDIR /{name}

# Install dependencies
RUN pip install .

# Safe user
RUN adduser --disabled-password {name}_user
USER {name}_user

# Run package
CMD python -m {name}
"
        );
        let dockerfile = make_file(
            format!("{root}/Dockerfile").as_str(),
            dockerfile_content.clone(),
        );
        if let Err(e) = dockerfile {
            eprintln!("error: {}", e);
        }
        let containerfile = make_file(format!("{root}/Containerfile").as_str(), dockerfile_content);
        if let Err(e) = containerfile {
            eprintln!("error: {}", e);
        }
        true
    } else {
        false
    }
}

// Project Makefile
fn prj_makefile(root: &str, name: &str, tests: bool, build: bool) {
    // Set options for make
    let mut make_options = vec!["help", "run", "clean"];
    if tests {
        make_options.push("test");
    }
    if build {
        make_options.push("build");
        make_options.push("deploy");
    }
    let mut makefile_content = format!(
        "# Generated by psp (https://github.com/MatteoGuadrini/psp), version {VERSION}

PYTHON = python3
VENV = venv

.PHONY: {}

help:
\t@echo 'help: make ({})'
",
        make_options.join(" "),
        make_options.join("|")
    );
    // Check if tests variable has been specified
    if tests {
        makefile_content += format!(
            "
test:
ifneq ('$(wildcard ${{VENV}}/bin/${{PYTHON}})','')
\t${{VENV}}/bin/${{PYTHON}} -m unittest
else
\t${{PYTHON}} -m unittest
endif
"
        )
        .as_str();
    }
    // Check if build variable has been specified
    if build {
        makefile_content += format!(
            "
build:
ifneq ('$(wildcard ${{VENV}}/bin/${{PYTHON}})','')
\t${{VENV}}/bin/${{PYTHON}} -m build
else
\t${{PYTHON}} -m build
endif

deploy:
ifneq ('$(wildcard ${{VENV}}/bin/${{PYTHON}})','')
\t${{VENV}}/bin/${{PYTHON}} -m twine upload dist/*
else
\t${{PYTHON}} -m twine upload dist/*
endif
"
        )
        .as_str();
    }
    makefile_content += format!(
        "
run:
ifneq ('$(wildcard ${{VENV}}/bin/${{PYTHON}})','')
\t${{VENV}}/bin/${{PYTHON}} -m {name}
else
\t${{PYTHON}} -m {name}
endif

clean:
\trm -fr build/
\trm -fr dist/
\trm -fr .eggs/
\tfind . -name '*.egg-info' -exec rm -fr {{}} +
\tfind . -name '*.egg' -exec rm -f {{}} +
\tfind . -name '*.pyc' -exec rm -f {{}} +
\tfind . -name '*.pyo' -exec rm -f {{}} +
\tfind . -name '*~' -exec rm -f {{}} +
\tfind . -name '__pycache__' -exec rm -fr {{}} +
"
    )
    .as_str();
    let makefile = make_file(format!("{root}/Makefile").as_str(), makefile_content);
    if let Err(e) = makefile {
        eprintln!("error: {}", e);
    }
}

// Main program
fn main() {
    // Check if argument is specified
    let shortcut = get_shortcut();
    // Print help message
    if shortcut == "help" {
        print_help(0)
    }
    // Print welcome screen and version
    println!("info: welcome to psp, version {VERSION}");
    // Load env files
    load_env();
    // Check dependencies tools
    let tools = ["python3", "git", "pip3", "curl"];
    for tool in tools {
        check_tool(tool);
    }
    // Create project structure by name or path
    let (root, name) = prj_name();
    // Virtual Environment
    let venv = prj_venv(&root, &shortcut);
    // Start git
    let git = prj_git(&root, &shortcut);
    // Git remote
    if git {
        prj_remote(&root, &name, &shortcut);
    }
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
    // Common files
    prj_files(&root, &name, &shortcut);
    // License
    let license = prj_license(&root, &shortcut);
    // Build dependencies
    let build = prj_pypi(&root, venv, &shortcut);
    // Write pyproject.toml
    prj_toml(&root, &name, &deps, license);
    // Dockerfile
    prj_docker(&root, &name, &shortcut);
    // Makefile
    prj_makefile(&root, &name, tests, build);
    // Finish scaffolding process
    println!(
        "info: python project `{name}` created at {}",
        absolute(root).unwrap().display()
    )
}
