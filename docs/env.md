# Env

**psp** accept some `PSP_` environment variables, that you can configure also with files. These variables substitute questions and its values, to a better customization.

## Files

You can configure two files for configure these environment variables:

* `.env` file in the current working directory
* `.psp.env` file into your `$HOME`

!!! note
    The `.env` file has a precedence respect the `.psp.env` file

## Variables

| **NAME**          | **REFERENCE**                                              | **VALUE**                      |
|-------------------|------------------------------------------------------------|--------------------------------|
| `PSP_NAME`        | [Name of Python Project](simple.md#name-of-python-project) | `name`                         |
| `PSP_VENV`        | [Virtual Environment](simple.md#virtual-environment)       | `true|false`                   |
| `PSP_GIT`         | [Git](simple.md#git)                                       | `true|false`                   |
| `PSP_GIT_REMOTE`  | [Git remote provider](simple.md#git-remote-provider)       | `github|gitlab`                |
| `PSP_GIT_USER`    | [Git remote username](simple.md#git-remote-username)       | `username`                     |
| `PSP_TEST`        | [Test files](simple.md#test-files)                         | `true|false`                   |
| `PSP_COMMON_DEPS` | [Common Depedencies](simple.md#dependencies)               | `dep1 dep2==0.0.1`             |
| `PSP_DEPS`        | [Depedencies](simple.md#dependencies)                      | `dep1 dep2==0.0.1`             |
| `PSP_DOCS`        | [Documentation generator](simple.md#documentation)         | `sphinx|mkdocs`                |
| `PSP_TOX`         | [Tox tool](simple.md#tox-tool)                             | `true|false`                   |
| `PSP_CI`          | [Remote CI](simple.md#remote-ci-continuous-integration)    | `travisci|circleci`            |
| `PSP_FILES`       | [Common files](simple.md#common-files)                     | `true|false`                   |
| `PSP_LICENSE`     | [License](simple.md#license)                               | `mit|apache|cc|mozilla|gpl`    |
| `PSP_PYPI`        | [PyPi dependencies](simple.md#pypi-dependencies)           | `true|false`                   |
| `PSP_CONTAINER`   | [Containers](simple.md#dockerpodman)                       | `true|false`                   |

## Python variables

| **NAME**          | **REFERENCE**                                              | **VALUE**                      |
|-------------------|------------------------------------------------------------|--------------------------------|
| `PSP_PYVER`       | Python project's version                                   | `0.0.1`                        |

## Examples

This is an example of file `.psp.env` in the `$HOME` path.

```bash
#PSP_NAME=mypy_package          # This is treated as a default name
PSP_VENV=true
PSP_GIT=true
PSP_GIT_REMOTE=github
PSP_GIT_USER=matteoguadrini
PSP_TEST=true
PSP_COMMON_DEPS="pytest"        # These are additional dependencies
#PSP_DEPS="tablib pandas"       # These are project dependencies
PSP_DOCS=mkdocs
PSP_TOX=true
PSP_CI=circleci
PSP_FILES=true
PSP_LICENSE=gpl
PSP_PYPI=true
PSP_CONTAINER=true
```

If you create a `.env` file into _CWD_ (Current Working Directory), every `PSP_` variables will overwritten.

```console
[gu]# cat $HOME/.psp.env
PSP_VENV=true
PSP_GIT=true
PSP_GIT_REMOTE=github
PSP_GIT_USER=matteoguadrini
PSP_TEST=true
PSP_COMMON_DEPS="pytest"
PSP_DOCS=mkdocs
PSP_TOX=true
PSP_CI=circleci
PSP_FILES=true
PSP_LICENSE=gpl
PSP_PYPI=true
PSP_CONTAINER=true
[gu]# pwd
/tmp/mypyenv/
[gu]# cat .env
PSP_DEPS="numpy scipy pydata-sphinx-theme"
PSP_DOCS=sphinx     # Overwritten
PSP_LICENSE=mit     # Overwritten
[gu]# psp
info: welcome to psp, version 0.1.1
> Name of Python project: biopy
info: python project `biopy` created at /tmp/mypyenv/biopy
[gu]# ll biopy
total 36
-rw-rw-r-- 1 gu gu  142 Mar  7 12:47 CHANGES.md
-rw-rw-r-- 1 gu gu 1070 Mar  7 12:47 CONTRIBUTING.md
-rw-rw-r-- 1 gu gu  336 Mar  7 12:48 Containerfile
-rw-rw-r-- 1 gu gu  336 Mar  7 12:48 Dockerfile
-rw-rw-r-- 1 gu gu 1008 Mar  7 12:48 Makefile
-rw-rw-r-- 1 gu gu 1451 Mar  7 12:48 LICENSE.md
-rw-rw-r-- 1 gu gu 1265 Mar  7 12:48 CODE_OF_CONDUCT.md
-rw-rw-r-- 1 gu gu  192 Mar  7 12:47 README.md
drwxrwxr-x 2 gu gu   80 Mar  7 12:46 biopy
drwxrwxr-x 4 gu gu  120 Mar  7 12:47 docs
-rw-rw-r-- 1 gu gu  813 Mar  7 12:48 pyproject.toml
-rw-rw-r-- 1 gu gu  106 Mar  7 12:47 requirements.txt
drwxrwxr-x 2 gu gu   80 Mar  7 12:46 tests
-rw-rw-r-- 1 gu gu  239 Mar  7 12:47 tox.ini
drwxrwxr-x 5 gu gu  140 Mar  7 12:46 venv
[gu]# cd biopy && make test
venv/bin/python3 -m unittest
Test all biopy successfully!
.
----------------------------------------------------------------------
Ran 1 test in 0.000s

OK
```

!!! note
    All loaded _psp_ variable are visible in help subcommand: `psp help`
