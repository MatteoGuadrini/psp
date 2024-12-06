# Simple usage

Sometimes to start a new Python project is very hard; Write a script? Write a module? Write a package?

The best choice is [micropiecies pattern](https://py-pkgs.org/01-introduction.html#why-you-should-create-packages): write a script, so rewrite that in a module and so rewrite in a package.

At this point you have a consinstent model. But every Python project has same pattern to package. `psp` help us in this way. Let's start a simple example.

## Name of Python project

To start with **psp**, type `psp`:

```console
[gu]# psp
Welcome to PSP (Python Scaffolding Projects): 0.1.0
? Name of Python project: mypyprj
[Type name or path]
```

!!! note
    If the _name_ contains spaces, **psp** substitutes with underscore chars (`_`); if the name contains uppercase letters, will convert into lowercase the only python package.

Now, if you type only a name, **psp** create in the _current folder_ this structure:

```
mypyprj
├── pyproject.toml          ┐
└─── mypyprj                │ Package source code, metadata,
     ├── __init__.py        │ and build instructions
     └── __main__.py        ┘
```

You can also specify a relative/absolute path; in this case the last name of path is your _project name_:

```console
[gu]# psp
Welcome to PSP (Python Scaffolding Projects): 0.1.0
? Name of Python project: /tmp/mypyprj
[Type name or path]
```

## Virtual Environment

This option create a new Python virtual environment.

A [virtual environment](https://docs.python.org/3/library/venv.html) is created on top of an existing Python installation, known as the virtual environment’s _base_ Python,
and may optionally be isolated from the packages in the base environment, so only those explicitly installed in the virtual environment are available.

The default value is _Yes_.

```console
Welcome to PSP (Python Scaffolding Projects): 0.1.0
> Name of Python project: mypyprj
? Do you want to create a virtual environment? (Y/n)
```

The project structure after this choosen:

```
mypyprj
├── pyproject.toml
├── mypyprj
│   └──...
└── venv                       ┐
    ├── bin                    │
    ├── include                │
    │   └── python3.13         │
    ├── lib                    │ Virtual Environment
    │   └── python3.13         │
    │       └── site-packages  │
    ├── lib64 -> lib           │
    └── pyvenv.cfg             ┘
```

## Git

This option create a new git repository.

[Git](https://git-scm.com/) is a free and open source distributed version control system designed to handle everything from small to very large projects with speed and efficiency.

The default value is _Yes_.

```console
...
> Name of Python project: mypyprj
> Do you want to create a virtual environment? Yes
? Do you want to start git repository? (Y/n)
```

The project structure after this choosen:

```
mypyprj
├── pyproject.toml
├── mypyprj
│   └──...
├── venv
│   └──...
├── .git                 ┐
│   ├── branches         │
│   ├── config           │
│   ├── description      │
│   ├── HEAD             │
│   ├── hooks            │
│   ├── info             │
│   │   └── exclude      │ Git repository
│   ├── objects          │
│   │   ├── info         │
│   │   └── pack         │
│   └── refs             │
│       ├── heads        │
│       └── tags         │
└── .gitignore           ┘
```

The git repository is initialized:

```console
[gu]# cd mypyprj && git status
On branch main

No commits yet

Untracked files:
  (use "git add <file>..." to include in what will be committed)
...
```


## Git remote provider

This option create a git remote repository configuration files.

!!! note
    This option is available only if [git repository](#git) is initialize.

```console
...
> Do you want to start git repository? Yes
? Select git remote provider:
  None
  Gitlab
> Github
[↑↓ to move, enter to select, type to filter]
```

The project structure after this choosen:

```
mypyprj
├── pyproject.toml
├── mypyprj
│   └──...
├── venv
│   └──...
├── .git
│   └──...
├── .gitignore
└── .github                            ┐
    ├── ISSUE_TEMPLATE                 │
    │   ├── bug.yml                    │
    │   ├── config.yml                 │ Github specific files
    │   └── feature.yml                │
    └── PULL_REQUEST_TEMPLATE          │
        └── pull_request_template.md   ┘
```

### Git remote username

This option create a git remote repository configuration files.

!!! note
    This option is available only if **git remote option** is set.

```console
...
> Do you want to create a virtual environment? Yes
> Do you want to start git repository? Yes
> Select git remote provider: Github
? Username of Github: MatteoGuadrini
```

Now the git repository has remote endpoint:

```console
[gu]# cd mypyprj && git remote get-url origin
git@github.com:MatteoGuadrini/mypyprj.git
```

## Test files

This option create the Python Unit test files.

The default value is _Yes_.

```console
...
> Select git remote provider: Github
> Username of Github: MatteoGuadrini
? Do you want unit test files? (Y/n)
```

The project structure after this choosen:

```
mypyprj
├── pyproject.toml
├── mypyprj
│   └──...
├── venv
│   └──...
├── .git
│   └──...
├── .gitignore
├── .github
│   └──...
└── tests                ┐
    ├── __init__.py      │ Python test files
    └── test_mypyprj.py  ┘
```

## Dependencies

With this option specify dependencies of your project.
This option use `pip` to install all dependencies.

!!! note
    This option follow the format of [_requirements specifiers_](https://pip.pypa.io/en/stable/reference/requirement-specifiers) without any space betwenn name and version.

The default value is _No_.

```console
...
> Username of Github: MatteoGuadrini
> Do you want unit test files? Yes
? Install dependencies: (No) tablib==3.4.0 pyreports<1.7.0 scipy numpy
[Write package(s) separates with spaces or empty]
```

!!! warning
    If has been specified the option [Virtual Environment](#virtual-environment), the packages will be installed in the _Virtual Environment_; otherwise, for the current user.

The project structure after this choosen will be the same; will change the _venv_ folder:

```console
[gu]# ls -l mypyprj/venv/lib64/python3.12/site-packages
total 25700
...
drwxrwxr-x 24 gu gu      960 Dec  2 10:32 numpy
drwxrwxr-x  2 gu gu      180 Dec  2 10:32 numpy-2.1.3.dist-info
drwxrwxr-x  2 gu gu      100 Dec  2 10:32 numpy.libs
...
drwxrwxr-x  3 gu gu      180 Dec  2 10:32 pyreports
drwxrwxr-x  2 gu gu      200 Dec  2 10:32 pyreports-1.6.0.dist-info
...
drwxrwxr-x 22 gu gu      600 Dec  2 10:32 scipy
drwxrwxr-x  2 gu gu      160 Dec  2 10:32 scipy-1.14.1.dist-info
drwxrwxr-x  2 gu gu      140 Dec  2 10:32 scipy.libs
...
drwxrwxr-x  5 gu gu      200 Dec  2 10:32 tablib
drwxrwxr-x  2 gu gu      200 Dec  2 10:32 tablib-3.4.0.dist-info
...
```

## Documentation

With this option specify documentation generator.
If you select one option, will be installed the package relative at the chosen.

The default value is _None_.

```console
...
> Install dependencies: tablib==3.4.0 pyreports<1.7.0 scipy numpy
? Select documentation generator:
  None
> Sphinx
  MKDocs
[↑↓ to move, enter to select, type to filter]
```

!!! warning
    If has been specified the option [Virtual Environment](#virtual-environment), the package will be installed in the _Virtual Environment_; otherwise, for the current user.

The project structure after this choosen:

```
mypyprj
├── pyproject.toml
├── mypyprj
│   └──...
├── venv
│   └──...
├── .git
│   └──...
├── .gitignore
├── .github
│   └──...
├── tests
│   └──...
└── docs                  ┐
    ├── Makefile          │
    ├── build             │
    ├── make.bat          │ Documentation metadata,
    └── source            │ files and folders,
        ├── _static       │ and build instructions
        ├── _templates    │
        ├── conf.py       │
        └── index.rst     ┘
```

## Tox tool

This option install [tox](https://tox.wiki/) and create configuration files.

!!! note
    This option is available only if [test files](#test-files) has been selected.

!!! warning
    If has been specified the option [Virtual Environment](#virtual-environment), `tox` will be installed in the _Virtual Environment_; otherwise, for the current user.

The default value is _No_.

```console
...
> Install dependencies: tablib==3.4.0 pyreports<1.7.0 scipy numpy
> Select documention generator: Sphinx
? Do you want to configure tox? (y/N) y
```

The project structure after this choosen:

```
mypyprj
├── pyproject.toml
├── mypyprj
│   └──...
├── venv
│   └──...
├── .git
│   └──...
├── .gitignore
├── .github
│   └──...
├── tests
│   └──...
├── docs
│   └──...
└── tox.ini   │ tox configuration file
```

## Remote CI (Continuous Integration)

This option configure a remote _Continuous Integration_ provider.

The default value is _None_.

```console
...
> Do you want to configure tox? Yes
? Select remote CI provider:
  None
  TravisCI
> CircleCI
[↑↓ to move, enter to select, type to filter]
```

The project structure after this choosen:

```
mypyprj
├── pyproject.toml
├── mypyprj
│   └──...
├── venv
│   └──...
├── .git
│   └──...
├── .gitignore
├── .github
│   └──...
├── tests
│   └──...
├── docs
│   └──...
├── tox.ini
└── .circleci        ┐ Remote CI
    └── config.yml   ┘ provider
```

## Common files

This option configure a common files for Python projects, as a _README, CONTRIBUTING, CODE_OF_CONDUCT and CHANGES_.

The default value is _Yes_.

```console
...
> Select remote CI provider: CircleCI
? Do you want create common files? (Y/n)
[Create README, CONTRIBUTING, CODE_OF_CONDUCT and CHANGES]
```

The project structure after this choosen:

```
mypyprj
├── pyproject.toml
├── mypyprj
│   └──...
├── venv
│   └──...
├── .git
│   └──...
├── .gitignore
├── .github
│   └──...
├── tests
│   └──...
├── docs
│   └──...
├── tox.ini
├── .circleci
│   └── ...
├── README.md           ┐
├── CONTRIBUTING.md     │ Common
├── CODE_OF_CONDUCT.md  │ files
└── CHANGES.md          ┘
```

## License

This option download the license file and configure license into _pyproject.toml_.

The default value is _None_.

```console
...
> Do you want create common files? Yes
? Select license:
  None
  MIT
> Apache
  Mozilla
  Creative Commons
  Gnu Public License
[↑↓ to move, enter to select, type to filter]
```

The project structure after this choosen:

```
mypyprj
├── pyproject.toml
├── mypyprj
│   └──...
├── venv
│   └──...
├── .git
│   └──...
├── .gitignore
├── .github
│   └──...
├── tests
│   └──...
├── docs
│   └──...
├── tox.ini
├── .circleci
│   └── ...
├── README.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── CHANGES.md
└── LICENSE.md          │ License file
```
