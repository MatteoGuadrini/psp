# Changelog

All notable changes to **psp** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.5.0] - 2026-01-13

### Added
- Add new `PSP_PY*` variables for `pyproject.toml`
- Add support for `hatch`, `poetry` and `maturin` builders
- Add **env_pybuild** function
- Add **make_builder** function
- Add **check_builder** function

### Changed
- Change Rust minimum version to 1.82
- Change homepage into pyprojects
- Create dist folder for builder

### Fixed
- Fix conditional code for package managers and builders

## [0.4.0] - 2026-01-13

### Added
- Add new `PSP_PY*` variables for `pyproject.toml`
- Add support for `uv` and `conda` package manager
- Add **get_file_from_url** function
- Add **check_pm** function
- Add **env_pyauthor** function
- Add **make_pm** function

### Changed
- Move from _venv_ folder to _.venv_

### Fixed
- Fix error if name of project is empty

## [0.3.1] - 2025-11-24

### Added
- Add new `PSP_PY*` variables for `pyproject.toml`

### Changed
- Add current Python version to help message
- Change licenses repository to https://raw.githubusercontent.com/MatteoGuadrini/psp/refs/heads/main/licenses

### Fixed
- Fix the insertion of new path into `join_env_path` function

## [0.3.0] - 2025-10-20

### Added
- Add Windows OS support
- Add more stuff into _gitignore_ file
- Add more details in _README_ file
- Add _container_ into **prj_makefile** function
- Add **make_command** function

### Changed
- Rewrite **help** function
- Convert some variable into constant

### Fixed
- Fix absolute path on _pip_ process

## [0.2.0] - 2025-03-11

### Added
- Add enviroment variables `PSP_*`
- Add **load_env** function

### Changed
- Add _git_info_ into **prj_toml** function
- Add type into return statement on **prj_remote** function
- Change homepage and changelog into **prj_toml** function

### Fixed
- Fix `none` condition into _remote ci_ part

## [0.1.1] - 2024-01-31

### Added
- Add info message when path unchanged
- Add dependencies to _tox.ini_ into **prj_tox** function
- Add build and deploy into makefile into **prj_makefile** function

### Changed
- Change welcome screen and result string into **main** function
- Change default output of \_\_main\_\_.py Python module
- Optimize container image and add safe user

### Fixed
- Fix check if docs folder exists into **prj_docs** function

## [0.1.0] - 2024-11-11

### Added
- Add **prj_pypi** function
- Add **prj_docker** function
- Add **prj_makefile** function
- Add **print_help** core function
- Add _quick_, _simple_ and _full_ argument for rapid configuration
- Add _requirements.txt_file

### Changed
- Rewrite **check_tool** function with PATH enviroment variable

### Fixed
- Fix if _name_ of project is empty
- Fix if _name_ of project contains spaces

## [0.0.9] - 2024-10-15

### Added
- Add **prj_files** function
- Add return bool into **prj_test** function
- Add Apache v2 license

### Fixed
- Fix creation docs folder into **prj_docs** functionspaces

## [0.0.8] - 2024-09-23

### Added
- Add **prj_tox** function
- Add **prj_docs** function

## [0.0.7] - 2024-09-17

### Added
- Add **prj_remote** function

## [0.0.6] - 2024-09-06

### Added
- Add **prj_ci** function

### Fixed
- Fix _pyproject.toml_ file

## [0.0.5] - 2024-09-06

### Added
- Add **prj_deps** function

## [0.0.4] - 2024-08-29

### Added
- Add **prj_venv** function

### Fixed
- Fix grouped check tools
- Fix prompt errors

## [0.0.3] - 2024-08-08

### Added
- Add **prj_test** function
- Add user's bin path into **check_tool** function

### Fixed
- Fix return type to **prj_git** function
- Fix env path into files

## [0.0.2] - 2024-08-02

### Added
- Add **prompt_confirm** function
- Add **prj_git** function

### Fixed
- Fix lowercase name of package

## [0.0.1] - 2024-07-31

### Added
- Add **check_tool** function
- Add **make_dirs** function
- Add **make_file** function
- Add **prompt_text** function
- Add **prj_name** function
- Add _help_ message to **prompt_text** function
