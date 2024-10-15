# **psp** (Python Scaffolding Projects)

`psp` is a blazing fast command line utility to scaffold your _Python_ project, written in Rust.

- ⚡️ 10-100x faster
- 🛠️ `pyproject.toml` support
- 🤝 Python 3.13 compatibility
- 📦 Unit-test and [pytest](https://docs.pytest.org/) supports
- 🔧 Automatically dependencies installation
- 📏 [tox](https://tox.wiki/en/stable/) configuration supports and other remotes CI
- ⌨️ [MkDocs](https://www.mkdocs.org/) and [Sphinx](https://www.sphinx-doc.org/) documentation supports
- 🌎 Github and Gitlab remote repository supports

<img src="https://i.ibb.co/SvptyfB/psp-009.png" alt="psp" width="600"/>

> [!NOTE]
> This project is WIP: beta

The result is:

```console
$> tree test/ --filelimit=10 -a
test                    # Project folder
├── LICENSE.md          # License file
├── pyproject.toml      # Python package configuration file
├── README.md           # Readme file
├── CHANGES.md          # List of changes
├── .circleci           # CI folder
│   └── config.yml      # CI configuration file
├── CODE_OF_CONDUCT.md  # Code of Conduct
├── CONTRIBUTING.md     # Contributing guide lines
├── docs                # Documentation folder: Sphinx/MKDocs
│   ├── build
│   ├── make.bat
│   ├── Makefile
│   └── source
│       ├── conf.py
│       ├── index.rst
│       ├── _static
│       └── _templates
├── .git                # Git folder
│   ├── branches
│   ├── config
│   ├── description
│   ├── HEAD
│   ├── hooks  [14 entries exceeds filelimit, not opening dir]
│   ├── info
│   │   └── exclude
│   ├── objects
│   │   ├── info
│   │   └── pack
│   └── refs
│       ├── heads
│       └── tags
├── .github             # Github issue and merge templates
│   ├── ISSUE_TEMPLATE
│   │   ├── bug.yml
│   │   ├── config.yml
│   │   └── feature.yml
│   └── PULL_REQUEST_TEMPLATE
│       └── pull_request_template.md
├── .gitignore          # Git ignore file
├── test                # Python package
│   └── __init__.py
├── tests               # Tests package for modules
│   ├── __init__.py
│   └── test_test.py    # Test module "test_<name_python_package>"
├── tox.ini             # Tox configuration files
└── venv                # Virtual environment
    ├── bin  [33 entries exceeds filelimit, not opening dir]
    ├── include
    │   └── python3.12
    ├── lib
    │   └── python3.12
    │       └── site-packages  [68 entries exceeds filelimit, not opening dir]
    ├── lib64 -> lib
    └── pyvenv.cfg

30 directories, 39 files
```

And `git` status is:

```console
$> git status
On branch main

No commits yet
...
$> git remote get-url origin
git@github.com:MatteoGuadrini/test.git
```

## Prerequisites

`psp` has four prerequisetes installed on own machine:
- `git`
- `python3`
- `pip`
- `curl`

### Ubuntu prerequisites installation

```console
sudo apt install -y python3 python3-pip git curl
```

### Red Hat prerequisites installation

```console
sudo dnf install -y python3 python3-pip git curl
```

### Arch prerequisites installation

```console
sudo pacman -Qi python3 python3-pip git curl
```

## Installation

To install compiled file into your machine, download it:

### Linux

For all users:
```console
sudo -i
curl -L https://github.com/MatteoGuadrini/psp/releases/download/v0.0.8/psp_linux > /usr/bin/psp
chmod +x /usr/bin/psp
```

For current user:
```console
curl -L https://github.com/MatteoGuadrini/psp/releases/download/v0.0.8/psp_linux > $HOME/.local/bin/psp
chmod +x $HOME/.local/bin/psp
```

### MacOS

```console
sudo su -
curl -L https://github.com/MatteoGuadrini/psp/releases/download/v0.0.8/psp_macos > /usr/bin/psp
chmod +x /usr/bin/psp
```

### Packages

For **Debian/Ubuntu**:

```console
curl -L https://github.com/MatteoGuadrini/psp/releases/download/v0.0.8/psp.deb
sudo dpkg -i psp.deb
```

For **Fedora/Mageia/OpenSuse**:

```console
sudo rpm -i https://github.com/MatteoGuadrini/psp/releases/download/v0.0.8/psp.rpm
```

### Compile as your own

Instead, if you compile this project as own, follow this steps:

```console
git clone https://github.com/MatteoGuadrini/psp.git
cd psp && cargo build --release && sudo cp -var target/release/psp /usr/bin/psp
```

## Features

- [x] Scaffolding file and folder structures for your Python project
- [x] Prepare git and gitignore
- [x] Prepare unit test files (also with pytest)
- [x] Prepare virtual environment
- [x] Install dependencies
- [x] Prepare pyproject.toml
- [x] Prepare CI configuration files
- [x] Prepare Github/Gitlab files
- [x] Prepare tox environment
- [x] Prepare docs folder for sphinx/mkdocs documentation
- [x] Prepare README, LICENSE, CONTRIBUTING, CODE_OF_CONDUCT and CHANGES files
- [ ] Add build and deploy dependencies to distribute package
- [ ] Add Dockerfile for your project
- [ ] Add _quick_, _simple_ and _full_ argument for rapid configuration

## Open source
_psp_ is an open source project. Any contribute, It's welcome.

**A great thanks**.

For donations, press this

For me

[![paypal](https://www.paypalobjects.com/en_US/i/btn/btn_donateCC_LG.gif)](https://www.paypal.me/guos)

For [Telethon](http://www.telethon.it/)

The Telethon Foundation is a non-profit organization recognized by the Ministry of University and Scientific and Technological Research.
They were born in 1990 to respond to the appeal of patients suffering from rare diseases.
Come today, we are organized to dare to listen to them and answers, every day of the year.

[Adopt the future](https://www.ioadottoilfuturo.it/)


## Treeware

This package is [Treeware](https://treeware.earth). If you use it in production,
then we ask that you [**buy the world a tree**](https://plant.treeware.earth/matteoguadrini/mkpl) to thank us for our work.
By contributing to the Treeware forest you’ll be creating employment for local families and restoring wildlife habitats.

[![Treeware](https://img.shields.io/badge/dynamic/json?color=brightgreen&label=Treeware&query=%24.total&url=https%3A%2F%2Fpublic.offset.earth%2Fusers%2Ftreeware%2Ftrees)](https://treeware.earth)


## Acknowledgments

Thanks to Jim Blandy, Jason Orendorff and Nora Tindall for writing the  _Programming Rust_ book that make up my Rust foundation.

Thanks to Tim McNamara for writing the _Rust in Action_ book.

Thanks to [Zed IDE](https://zed.dev/) and for license of [RustRover](https://www.jetbrains.com/rust/) offered by Jetbrains.

Special thanks go to my wife, who understood the hours of absence for this development.
Thanks to my children, for the daily inspiration they give me and to make me realize, that life must be simple.

Thanks, Rust Community!
