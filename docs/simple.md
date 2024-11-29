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

## Virtual Enviromenment

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
    ├── bin                    |
    ├── include                |
    │   └── python3.13         |
    ├── lib                    | Virtual Environment
    │   └── python3.13         |
    │       └── site-packages  |
    ├── lib64 -> lib           |
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
|   └──...
├── .git                 ┐
|   ├── branches         |
|   ├── config           |
|   ├── description      |
|   ├── HEAD             |
|   ├── hooks            |
|   ├── info             |
|   │   └── exclude      | Git repository
|   ├── objects          |
|   │   ├── info         |
|   │   └── pack         |
|   └── refs             |
|       ├── heads        |
|       └── tags         |
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
    This option is available only if **git repository** is initialize.

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
|   └──...
├── .git
|   └──...
├── .gitignore
└── .github                            ┐
    ├── ISSUE_TEMPLATE                 |
    |   ├── bug.yml                    |
    |   ├── config.yml                 | Github specific files
    |   └── feature.yml                |
    └── PULL_REQUEST_TEMPLATE          |
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
|   └──...
├── .git
|   └──...
├── .gitignore
├── .github
|   └──...
└── tests                ┐
    ├── __init__.py      | Python test files
    └── test_mypyprj.py  ┘
```
