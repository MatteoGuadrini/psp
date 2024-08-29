# **psp** (Python Scaffolding Projects)

`psp` is a fast command line utility to scaffold your Python project, written in Rust.

```console
psp   # Press Enter
Welcome to PSP (Python Scaffolding Projects): 0.0.4
> Name of Python project: test
> Do you want to start git repository? Yes
> Do you want unit test files? Yes
> Do you want to create a virtual environment? Yes
Project `test` created
```

> This project is WIP

## Prerequisites

`psp` has three prerequisetes installed on own machine:
- `git`
- `python3`
- `pip`

## Installation

To install compiled file into your machine, download it:

```console
# For Linux (all users)
sudo -i
curl -L https://github.com/MatteoGuadrini/psp/releases/download/v0.0.4/psp_linux > /usr/bin/psp
chmod +x /usr/bin/psp

# For Linux (current user)
curl -L https://github.com/MatteoGuadrini/psp/releases/download/v0.0.4/psp_linux > $HOME/.local/bin/psp
chmod +x $HOME/.local/bin/psp

# For MacOS
sudo su -
curl -L https://github.com/MatteoGuadrini/psp/releases/download/v0.0.4/psp_macos > /usr/bin/psp
chmod +x /usr/bin/psp
```

Instead, if you compile this project as own, follow this steps:

```console
git clone https://github.com/MatteoGuadrini/psp.git
cd psp && cargo build && sudo cp -var target/debug/psp /usr/bin/psp
```

## Features

- [x] Scaffolding file and folder structures for your Python project
- [x] Prepare git and gitignore
- [x] Prepare unit test files (also with pytest)
- [x] Prepare virtual environment
- [ ] Install dependencies
- [ ] Prepare pyproject.toml
- [ ] Prepare CI configuration files
- [ ] Prepare Github/Gitlab files
- [ ] Prepare virtual environment
- [ ] Prepare tox environment
- [ ] Prepare docs folder for sphinx/mkdocs documentation
- [ ] Prepare README, LICENSE, CONTRIBUTING, CODE_OF_CONDUCT and CHANGES files
- [ ] Add _quick_ argument for rapid configuration

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
