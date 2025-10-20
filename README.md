<img src="img/psp_logo.svg" alt="Logo" align="right" width="150"/> **psp** (Python Scaffolding Projects)
======

`psp` is a blazing fast command line utility to scaffold your _Python_ project, written in Rust.

- ⚡️ 1-100x faster compared to other scaffolding tools
- 🛠️ `pyproject.toml` support
- 🤝 Python 3.14 compatibility
- 🗃 Scaffolding file and folder structures for your Python project
- 📦 Unit-test and [pytest](https://docs.pytest.org/) support
- 🧪 Create a virtual environment
- 🔧 Automagically dependencies installation
- 🪛 Add build and deploy dependencies to distribute the package
- 📏 [tox](https://tox.wiki/en/stable/) configuration supports and remotes CI like [CircleCI](https://circleci.com/) and [TravisCI](https://www.travis-ci.com/)
- ⌨️ [MkDocs](https://www.mkdocs.org/) and [Sphinx](https://www.sphinx-doc.org/) documentation support
- 🧰 Initialize git repository and `gitignore` file
- 🌎 GitHub and Gitlab remote repository support
- 📑 Create `README`, `LICENSE`, `CONTRIBUTING`, `CODE_OF_CONDUCT` and `CHANGES` files
- 🐳 Create `Dockerfile` and `Containerfile` for your project
- 💡 Can use _quick_, _simple_ and _full_ argument for rapid configuration
- 💾 Create `$HOME/.psp.env` and `$PWD/.env` files with your customizations
- 🎛️ Can use some `PSP_` variables to control your defaults

## 🚀 Get Started in 30 Seconds

[![asciicast](https://asciinema.org/a/750186.svg)](https://asciinema.org/a/750186)

<img src="https://i.ibb.co/4RDPZWtC/psp030.png" alt="psp" width="790"/>

The result is:

```console
$> tree test/ --filelimit=19 -a
test                    # Project folder
├── LICENSE.md          # License file
├── pyproject.toml      # Python package configuration file
├── README.md           # Readme file
├── CHANGES.md          # List of changes
├── .circleci           # CI folder
│   └── config.yml      # CI configuration file
├── CODE_OF_CONDUCT.md  # Code of Conduct
├── CONTRIBUTING.md     # Contributing guide lines
├── Containerfile       # Standard container file for build image
├── Dockerfile          # Docker container file for build image
├── Makefile            # Makefile for command make; make help
├── requirements.txt    # Dependencies list used by third programs
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
├── .dockerignore       # Docker ignore file
├── .containerignore    # Container ignore file
├── test                # Python package
│   └── __init__.py
├── tests               # Tests package for modules
│   ├── __init__.py
│   └── test_test.py    # Test module "test_<name_python_package>"
├── tox.ini             # Tox configuration files
└── venv                # Virtual environment
    ├── bin  [33 entries exceeds filelimit, not opening dir]
    ├── include
    │   └── python3.14
    ├── lib
    │   └── python3.14
    │       └── site-packages  [68 entries exceeds filelimit, not opening dir]
    ├── lib64 -> lib
    └── pyvenv.cfg

29 directories, 44 files
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

## Help

For help message, type:

```console
$> psp help
psp (Python Scaffolding Projects), version 0.3.0
usage: psp [shortcut]
ie: psp [help|quick|simple|full]

shortcut:
    help:   print this help message
    quick:  enables a rapid setup (few options included)
    simple: enables a basic setup (only Python package)
    full:   enables a full setup (all options)

links:
    repository:     https://github.com/MatteoGuadrini/psp
    documentation:  https://psp.readthedocs.io/

variables:
    ["PSP_GIT","PSP_GIT_REMOTE","PSP_GIT_USER"]
```

> [!NOTE]
> More details for shortcuts, variables and other things, available in official documentation: [psp docs](https://psp.readthedocs.io/)

## 🔌 Prerequisites

`psp` has four mandatory prerequisetes installed on own machine:
- `git`
- `python3`
- `pip`
- `curl` (Linux/MacOS only)

### MacOS prerequisites installation

```console
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install python git
```

### Windows prerequisites installation

```console
winget install -e --id Git.Git
winget install -e --id Python.Python.3.14
```

### Ubuntu based prerequisites installation

```console
sudo apt install -y python3 python3-pip git curl
```

### Red Hat based prerequisites installation

```console
sudo dnf install -y python3 python3-pip git curl
```

### Arch based prerequisites installation

```console
sudo pacman -Qi python3 python3-pip git curl
```

## 💿 Installation

To install compiled file into your machine, download it:

### Linux

For **all users** (required root access):
```console
sudo -i
curl -L https://github.com/MatteoGuadrini/psp/releases/download/v0.3.0/psp_linux -o /usr/bin/psp
chmod +x /usr/bin/psp
```

For **current user**:
```console
curl -L https://github.com/MatteoGuadrini/psp/releases/download/v0.3.0/psp_linux -o $HOME/.local/bin/psp
chmod +x $HOME/.local/bin/psp
```

### MacOS

```console
sudo su -
curl -L https://github.com/MatteoGuadrini/psp/releases/download/v0.3.0/psp_macos -o /usr/bin/psp
chmod +x /usr/bin/psp
```

### Windows

For **all users** (required Administrator):
```powershell
iwr -OutFile "C:\Windows\system32\psp.exe" "https://github.com/MatteoGuadrini/psp/releases/download/v0.3.0/psp_windows"
```

For **current user**:
```powershell
mkdir "$($Env:USERPROFILE)\bin"
[System.Environment]::SetEnvironmentVariable("PATH", $Env:PATH + ";$($Env:USERPROFILE)\bin","USER")
iwr -OutFile "$($Env:USERPROFILE)\bin\psp.exe" "https://github.com/MatteoGuadrini/psp/releases/download/v0.3.0/psp_windows"
```

### Packages

If you want to install OS package, follow instructions for your Operating System:

For **Debian/Ubuntu**:

```console
curl -L https://github.com/MatteoGuadrini/psp/releases/download/v0.3.0/psp.deb -o psp.deb
sudo dpkg -i psp.deb
```

For **Fedora/Mageia/OpenSuse**:

```console
sudo rpm -i https://github.com/MatteoGuadrini/psp/releases/download/v0.3.0/psp.rpm
```

### Compile as your own

Instead, if you compile this project as own, follow this steps:

```console
git clone https://github.com/MatteoGuadrini/psp.git
cd psp && cargo build --release && sudo cp -v target/release/psp /usr/bin/psp && chmod +x /usr/bin/psp
```

## 🧰 Next features
- [x] `windows` operating system support
- [ ] Container support for psp program
- [ ] `conda`, `uv` and `poetry` support
- [ ] `hatch` support
- [ ] `docker-compose` and `kubernetes` support
- [ ] command line flags support
- [ ] updating/merging project
- [ ] templating folder support
- [ ] YAML configuration file

## Open source
_psp_ is an open source project. Any contribution, It's welcome.

**A great thanks**.

For donations, press this

For me

[![paypal](https://www.paypalobjects.com/en_US/i/btn/btn_donateCC_LG.gif)](https://www.paypal.me/guos)

For [Telethon](http://www.telethon.it/)

The Telethon Foundation is a non-profit organization recognized by the Ministry of University and Scientific and Technological Research.
They were born in 1990 to respond to the appeal of patients suffering from rare diseases.
Come today, we are organized to dare to listen to them and answers, every day of the year.

[Adopt the future](https://www.ioadottoilfuturo.it/)


## Licence
This package is [Treeware](https://treeware.earth).
If you use it in production, then we ask that you [**buy the world a tree**](https://plant.treeware.earth/MatteoGuadrini/psp) to thank us for our work.
By contributing to the Treeware forest you’ll be creating employment for local families and restoring wildlife habitats.

[![Buy us a tree](https://img.shields.io/badge/Treeware-%F0%9F%8C%B3-lightgreen?style=for-the-badge)](https://plant.treeware.earth/MatteoGuadrini/psp)


## Acknowledgments

Thanks to Jim Blandy, Jason Orendorff and Nora Tindall for writing the  _Programming Rust_ book that make up my Rust foundation.

Thanks to Tim McNamara for writing the _Rust in Action_ book.

Thanks to [Zed IDE](https://zed.dev/) and for license of [RustRover](https://www.jetbrains.com/rust/) offered by Jetbrains.

Special thanks go to my wife, who understood the hours of absence for this development.
Thanks to my children, for the daily inspiration they give me and to make me realize, that life must be simple.

Thanks, Rust Community!
