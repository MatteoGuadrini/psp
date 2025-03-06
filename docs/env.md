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
