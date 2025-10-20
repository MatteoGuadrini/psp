# Simple usage

Sometimes to start a new Python project is very hard; Write a script? Write a module? Write a package?

The best choice is [micropiecies pattern](https://py-pkgs.org/01-introduction.html#why-you-should-create-packages): write a script, so rewrite that in a module and so rewrite in a package.

At this point you have a consinstent model. But every Python project has same pattern to package. `psp` help us in this way. Let's start a simple example.

## Name of Python project

To start with **psp**, type `psp`:

```console
[gu]# psp
info: welcome to psp, version 0.3.0
? Name of Python project: mypyprj
[Type name or path]
```

!!! note
    If the _name_ contains spaces, **psp** substitutes with underscore chars (`_`); if the name contains uppercase letters, will convert into lowercase the only python package.

Now, if you type only a name, **psp** creates in the _current folder_ this structure:

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
info: welcome to psp, version 0.3.0
? Name of Python project: /tmp/mypyprj
[Type name or path]
```

!!! warning
    If the folder exists, **psp** prompt a choice of overwritten.

```console
info: welcome to psp, version 0.3.0
> Name of Python project: /tmp/mypyprj
? Path /tmp/mypyprj exists. Do you want continue? (y/N)
[Some files will be overwritten]
```

## Virtual Environment

This option creates a new Python virtual environment.

A [virtual environment](https://docs.python.org/3/library/venv.html) is created on top of an existing Python installation, known as the virtual environment’s _base_ Python,
and may optionally be isolated from the packages in the base environment, so only those explicitly installed in the virtual environment are available.

The default value is _Yes_.

```console
info: welcome to psp, version 0.3.0
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

This option creates a new git repository.

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

This option creates a git remote repository configuration files.

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

This option creates a git remote repository configuration files.

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

This option creates the Python Unit test files.

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

This option install [tox](https://tox.wiki/) and creates configuration files.

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

## License

This option download the license file and configure license into _pyproject.toml_.

The default value is _None_.

!!! warning
    License file will downloaded from internet. Check your internet settings, proxy or firewall if an error occured.

```console
...
> Select remote CI provider: CircleCI
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
└── LICENSE.md          │ License file
```
## PyPi dependencies

This option install [PyPi](https://packaging.python.org/en/latest/tutorials/packaging-projects/) tools for publish your package.

The default value is _Yes_.

```console
...
> Select license: Apache
? Do you want to install dependencies to publish on pypi? (Y/n)
```

The two packages are installed:

```console
[gu]# cd mypyprj && . venv/bin/activate && twine --version && python -m build --version
twine version 6.0.1 (keyring: 25.5.0,
pkginfo: 1.12.0, requests: 2.32.3,
requests-toolbelt: 1.0.0, urllib3: 2.2.3)
build 1.2.2.post1 (/tmp/mypyprj/venv/lib/python3.13/site-packages/build)
```

## Docker/Podman

This option [creates a containerization](https://docs.docker.com/build/) files to build an imagae of your package.

The default value is _Yes_.

```console
...
> Do you want to install dependencies to publish on pypi? Yes
? Do you want to create a Dockerfile and Containerfile? (Y/n)
```

After this option, the `psp` has finish its process:

```console
> Do you want to create a Dockerfile and Containerfile? Yes
Python project `mypyprj` created at mypyprj
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
├── Dockerfile        ┐
├── .dockerignore     │ Files for
├── .containerignore  │ containerization
└── Containerfile     ┘
```

Try to build an image:

```console
[gu]# docker build . -t mypyprj:0.0.1
STEP 1/6: FROM python:3
STEP 2/6: COPY mypyprj /mypyprj/mypyprj
--> d6d1cb90f013
STEP 3/6: COPY pyproject.toml /mypyprj
--> a0b3ead767b0
STEP 4/6: WORKDIR /mypyprj
--> e557642b415f
STEP 5/6: RUN pip install .
Processing /mypyprj
  Installing build dependencies: started
  Installing build dependencies: finished with status 'done'
  Getting requirements to build wheel: started
  Getting requirements to build wheel: finished with status 'done'
  Preparing metadata (pyproject.toml): started
  Preparing metadata (pyproject.toml): finished with status 'done'
Collecting tablib==3.4.0 (from mypyprj==0.0.1)
  Downloading tablib-3.4.0-py3-none-any.whl.metadata (3.8 kB)
Collecting pyreports<1.7.0 (from mypyprj==0.0.1)
  Downloading pyreports-1.6.0-py3-none-any.whl.metadata (9.8 kB)
Collecting scipy (from mypyprj==0.0.1)
  Downloading scipy-1.14.1-cp313-cp313-manylinux_2_17_x86_64.manylinux2014_x86_64.whl.me
Collecting numpy (from mypyprj==0.0.1)
  Downloading numpy-2.2.0-cp313-cp313-manylinux_2_17_x86_64.manylinux2014_x86_64.whl.met
  ...
Successfully built mypyprj markuppy odfpy
Installing collected packages: xlwt, pytz, pymssql, markuppy, xlrd, tzdata, tabulate, tablib, six, pyyaml, pyasn1, psycopg2-binary, numpy, nosqlapi, mysql-connector-python, et-xmlfile, defusedxml, scipy, python-dateutil, openpyxl, odfpy, ldap3, pandas, pyreports, mypyprj
Successfully installed defusedxml-0.7.1 et-xmlfile-2.0.0 ldap3-2.9.1 markuppy-1.14 mypyprj-0.0.1 mysql-connector-python-9.1.0 nosqlapi-1.0.2 numpy-2.2.0 odfpy-1.4.1 openpyxl-3.1.5 pandas-2.2.3 psycopg2-binary-2.9.10 pyasn1-0.6.1 pymssql-2.3.2 pyreports-1.6.0 python-dateutil-2.9.0.post0 pytz-2024.2 pyyaml-6.0.2 scipy-1.14.1 six-1.17.0 tablib-3.4.0 tabulate-0.9.0 tzdata-2024.2 xlrd-2.0.1 xlwt-1.3.0
WARNING: Running pip as the 'root' user can result in broken permissions and conflicting behaviour with the system package manager, possibly rendering your system unusable.It is recommended to use a virtual environment instead: https://pip.pypa.io/warnings/venv. Use the --root-user-action option if you know what you are doing and want to suppress this warning.

[notice] A new release of pip is available: 24.2 -> 24.3.1
[notice] To update, run: pip install --upgrade pip
--> 5854ebada71a
STEP 6/6: CMD [ 'python', '-m', 'mypyprj' ]
COMMIT mypyprj:0.0.1
--> a9028cd8e7e8
Successfully tagged localhost/mypyprj:0.0.1
a9028cd8e7e89b9f9aafde3db0d5e1cee419573a647c72c752bb6f202b8131e1

[gu]# docker images
podman images
REPOSITORY                         TAG         IMAGE ID      CREATED        SIZE
localhost/mypyprj                  0.0.1       a9028cd8e7e8  2 minutes ago  1.61 GB

[gu]# docker run -it --rm localhost/mypyprj:0.0.1 python
Python 3.13.0 (main, Oct 17 2024, 03:03:33) [GCC 12.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import mypyprj
>>> print(mypyprj)
<module 'mypyprj' from '/mypyprj/mypyprj/__init__.py'>
>>> print(mypyprj.__version__)
0.0.1
>>>
```

## Common files

This option configure a common files for Python projects, as a _README, CONTRIBUTING, CODE_OF_CONDUCT and CHANGES_.

The default value is _Yes_.

!!! warning
    In this option some files will downloaded from internet. Check your internet settings, proxy or firewall if an error occured.

```console
...
> Do you want to create a Dockerfile and Containerfile? Yes
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
├── Makefile
├── .circleci
│   └── ...
├── README.md           ┐
├── CONTRIBUTING.md     │ Common
├── CODE_OF_CONDUCT.md  │ files
└── CHANGES.md          ┘
```

!!! note
    As you notice, **psp** creates a Makefile to automate all process in your package.
    If you want a help, try `make help`.
