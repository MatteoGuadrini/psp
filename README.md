# **psp** (Python Scaffolding Projects)

`psp` is a blazing fast command line utility to scaffold your _Python_ project, written in Rust.

> [!NOTE]
> This project is WIP: beta

![Demo](https://i.ibb.co/KcZtw58/psp008.gif)

```console
psp   # Press Enter
Welcome to PSP (Python Scaffolding Projects): 0.0.8
> Name of Python project: test
> Do you want to start git repository? Yes
> Do you want unit test files? Yes
> Do you want to create a virtual environment? Yes
> Install dependencies: scipy numpy
> Select CI provider: CircleCI
> Select git remote provider: Github
> Username of Github: MatteoGuadrini
> Do you want to configure tox? Yes
> Select document generator: Sphinx
Project `test` created
```

The result is:

```console
tree test --filelimit=10 -a
test                  # project folder
├── pyproject.toml    # python package configuration file
├── .circleci         # CI folder
│   └── config.yml    # CI configuration file
├── docs              # documentation folder: Sphinx/MKDocs
│   ├── build
│   ├── make.bat
│   ├── Makefile
│   └── source
│       ├── conf.py
│       ├── index.rst
│       ├── _static
│       └── _templates
├── .git              # git folder
│   ├── branches
│   ├── config
│   ├── description
│   ├── HEAD
│   ├── hooks  [14 entries exceeds filelimit, not opening dir]
│   ├── info
│   │   └── exclude
│   ├── objects
│   │   ├── info
│   │   └── pack
│   └── refs
│       ├── heads
│       └── tags
├── .github           # Github issue and merge templates
│   ├── ISSUE_TEMPLATE
│   │   ├── bug.yml
│   │   ├── config.yml
│   │   └── feature.yml
│   └── PULL_REQUEST_TEMPLATE
│       └── pull_request_template.md
├── .gitignore        # git ignore file
├── test              # python package
│   └── __init__.py
├── tests             # tests package for modules
│   ├── __init__.py
│   └── test_test.py  # test module "test_<name_python_package>"
├── tox.ini           # Tox configuration files
└── venv              # virtual environment
    ├── bin  [33 entries exceeds filelimit, not opening dir]
    ├── include
    │   └── python3.12
    ├── lib
    │   └── python3.12
    │       └── site-packages  [68 entries exceeds filelimit, not opening dir]
    ├── lib64 -> lib
    └── pyvenv.cfg

30 directories, 20 files
```

## Prerequisites

`psp` has three prerequisetes installed on own machine:
- `git`
- `python3`
- `pip`

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

Instead, if you compile this project as own, follow this steps:

```console
git clone https://github.com/MatteoGuadrini/psp.git
cd psp && cargo build && sudo cp -var target/release/psp /usr/bin/psp
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
- [ ] Prepare README, LICENSE, CONTRIBUTING, CODE_OF_CONDUCT and CHANGES files
- [ ] Add build and deploy dependencies to distribute package
- [ ] Add _quick_, _nodocs_ and _full_ argument for rapid configuration

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

Thanks, Rust!
