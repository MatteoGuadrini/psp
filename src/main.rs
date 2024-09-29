use inquire::{Confirm, Select, Text};
use std::{
    env::var,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    process::exit,
};

// Constants
const VERSION: &str = "0.0.8";

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

// Core functions

// Project name
fn prj_name() -> String {
    let name = prompt_text("Name of Python project:", "prj", "None");
    let root = format!("{name}");
    let package = format!("{}", name.to_lowercase());
    let project = format!("{root}/{package}");
    // Make directories structure
    let dir_ret = make_dirs(format!("{project}").as_str());
    if let Err(e) = dir_ret {
        eprintln!("error: {}", e);
        exit(4);
    }
    // Make file structures
    let file_ret = make_file(
        format!("{project}/__init__.py").as_str(),
        "#! /usr/bin/env python3
# -*- encoding: utf-8 -*-
# vim: se ts=4 et syn=python:



"
        .to_string(),
    );
    if let Err(e) = file_ret {
        eprintln!("error: {}", e);
        exit(4);
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
            "### Python ###
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
/site"
                .to_string(),
        );
        let ret = if let Err(e) = file_ret {
            eprintln!("error: {}", e);
            exit(5);
        } else {
            true
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
        if let Err(e) = dir_ret {
            eprintln!("error: {}", e);
            exit(6);
        }
        // Make file structures
        let init_file = make_file(
            format!("{name}/tests/__init__.py").as_str(),
            "#! /usr/bin/env python3
# -*- encoding: utf-8 -*-
# vim: se ts=4 et syn=python:


"
            .to_string(),
        );
        if let Err(e) = init_file {
            eprintln!("error: {}", e);
            exit(6);
        }
        let project_name = name.to_lowercase();
        let all_module = make_file(
            format!("{name}/tests/test_{name}.py").as_str(),
            format!(
                "#! /usr/bin/env python3
# -*- encoding: utf-8 -*-
# vim: se ts=4 et syn=python:


import unittest


class TestAll(unittest.TestCase):

    def test_all(self):
        print('Test all {project_name} successfully!')


# Test functions for pytest
def test_all():
    assert '{project_name}' == '{project_name}'


if __name__ == '__main__':
    unittest.main()"
            )
            .to_string(),
        );
        if let Err(e) = all_module {
            eprintln!("error: {}", e);
            exit(6);
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

// Project pyproject.toml
fn prj_toml(name: &str, deps: &Vec<String>) {
    let requirements = if deps.contains(&"No".to_string()) {
        "[]".to_string()
    } else {
        format!("{deps:?}")
    };
    let content = format!(
        "[build-system]
requires = ['setuptools', 'wheel']
build-backend = 'setuptools.build_meta'

[project]
name = '{}'
version = '0.0.1'
readme = 'README.md'
license = ''
authors = [{{name = 'psp', email = 'psp@example.com'}}]
maintainers = [{{name = 'psp', email = 'psp@example.com'}}]
description = 'A simple but structured Python project'
requires-python = '>=3.12'
classifiers = ['Programming Language :: Python :: 3']
dependencies = {}

[project.urls]
homepage = ''
documentation = ''
repository = ''
changelog = ''
",
        name.to_lowercase(),
        requirements
    );
    // Write pyproject.toml
    let pyproject = make_file(format!("{name}/pyproject.toml").as_str(), content);
    if let Err(e) = pyproject {
        eprintln!("error: {}", e);
        exit(7);
    }
}

// Project CI
fn prj_ci(name: &str, deps: &Vec<String>) {
    let options = vec!["None", "TravisCI", "CircleCI"];
    let ci = prompt_select("Select CI provider:", options, "None");
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
                "language: python
cache: pip
python:
  - 3.10
  - 3.11
  - 3.12
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
                "version: 2.1
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

// Project Gitlab/Github
fn prj_remote(name: &str) {
    let options = vec!["None", "Gitlab", "Github"];
    let remote = prompt_select("Select git remote provider:", options, "None");
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
        let output = std::process::Command::new("git")
            .args(["remote", "add", "origin", &remote_path])
            .current_dir(name)
            .output()
            .expect("git should be installed");
        // Check if command exit successfully
        if !output.status.success() {
            eprintln!(
                "error: remote repository {} setting failed",
                remote.to_lowercase()
            );
        }
        // Make remote files and folders
        // Gitlab
        if remote.as_str() == "Gitlab" {
            let issue_folder = format!("{}/.{}/issue_templates", name, remote.to_lowercase());
            let merge_folder = format!(
                "{}/.{}/merge_request_templates",
                name,
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
                "## Description

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
                "## Description of problem

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
                "## What does this MR do and why?

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
            let issue_folder = format!("{}/.{}/ISSUE_TEMPLATE", name, remote.to_lowercase());
            let merge_folder = format!("{}/.{}/PULL_REQUEST_TEMPLATE", name, remote.to_lowercase());
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
                "name: Feature Request
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
                "name: Bug Report
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
                "# Title of Pull Request
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
fn prj_tox(name: &str, venv: bool) {
    // Create tox ini
    let confirm = prompt_confirm("Do you want to configure tox?", false, "None");
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
        let tox_ini_content = "[tox]
envlist = py310, py311, py312
isolated_build = True

[testenv]
labels = test, core
deps = pytest
commands = pytest tests"
            .to_string();
        let tox_ini = make_file(format!("{name}/tox.ini").as_str(), tox_ini_content);
        if let Err(e) = tox_ini {
            eprintln!("error: {}", e);
        }
    }
}

fn prj_docs(name: &str, venv: bool) {
    let options = vec!["None", "Sphinx", "MKDocs"];
    let docs = prompt_select("Select documention generator:", options, "None");
    if docs != "None" {
        let docs_home = format!("{name}/docs");
        // Create docs folder
        let docs_folder = make_dirs(format!("{docs_home}").as_str());
        if let Err(e) = docs_folder {
            eprintln!("error: {}", e);
            exit(4);
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
                .current_dir(&name)
                .output()
                .expect("pip should be installed");
            // Check if command exit successfully
            if !output.status.success() {
                eprintln!("error: sphinx installation failed");
                return;
            }
            // Start documentation;
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
                .current_dir(&name)
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
                .current_dir(&name)
                .output()
                .expect("mkdocs should be installed");
            // Check if command exit successfully
            if !output.status.success() {
                eprintln!("error: mkdocs documentation creation failed");
            }
        }
    }
}

fn prj_files(name: &str) {
    let confirm = prompt_confirm(
        "Do you want create common files?",
        true,
        "Create README, CONTRIBUTING, CODE_OF_CONDUCT and CHANGES",
    );
    if confirm {
        // Create README
        let readme_content = format!(
            "# Welcome to {name}

## Install {name}
```console
pip install .
```

## Acknowledgments
Thanks Python Community!"
        );
        let readme = make_file(format!("{name}/README.md").as_str(), readme_content);
        if let Err(e) = readme {
            eprintln!("error: {}", e);
        }
        // Create CHANGES
        let changes_content = format!(
            "# Release notes of {name}

## 0.0.1
- Start **{name}** project"
        );
        let changes = make_file(format!("{name}/CHANGES.md").as_str(), changes_content);
        if let Err(e) = changes {
            eprintln!("error: {}", e);
        }
        // Create CONTRIBUTING
        let contributing_content = format!(
            "# Contributing to {name}

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
            format!("{name}/CONTRIBUTING.md").as_str(),
            contributing_content,
        );
        if let Err(e) = contributing {
            eprintln!("error: {}", e);
        }
        // Create CODE_OF_CONDUCT
        let output = std::process::Command::new("curl")
            .arg("-o CODE_OF_CONDUCT.md")
            .arg("-k")
            .arg("https://www.contributor-covenant.org/version/2/1/code_of_conduct/code_of_conduct.md")
            .current_dir(&name)
            .output()
            .expect("curl should be installed");
        // Check if command exit successfully
        if !output.status.success() {
            eprintln!("error: CODE_OF_CONDUCT download failed");
        }
    }
}

fn prj_license(name: &str) -> String {
    // Select license
    let options = vec![
        "None",
        "MIT",
        "Apache",
        "Creative Commons",
        "Mozilla",
        "Gnu Public License",
    ];
    let license = prompt_select("Select license:", options, "None");
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
            .arg("-o LICENSE.md")
            .arg("-k")
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

// Main program
fn main() {
    // Print welcome screen and version
    println!("Welcome to PSP (Python Scaffolding Projects): {VERSION}");
    // Check dependencies tools
    check_tool("python3");
    check_tool("git");
    check_tool("pip3");
    check_tool("curl");
    // Create project structure by name
    let name = prj_name();
    // Start git
    let git = prj_git(&name);
    // Unit tests
    prj_test(&name);
    // Virtual Environment
    let venv = prj_venv(&name);
    // Install dependencies
    let deps = prj_deps(&name, venv);
    // CI configuration
    prj_ci(&name, &deps);
    // Git remote
    if git {
        prj_remote(&name);
    }
    // Write pyproject.toml
    prj_toml(&name, &deps);
    // Tox
    prj_tox(&name, venv);
    // Documentation
    prj_docs(&name, venv);
    // Common files
    prj_files(&name);
    // License
    let _license = prj_license(&name);
    // Finish scaffolding process
    println!("Project `{name}` created")
}
