# Shortcut

**psp** has a three shortcut built-in: _quick_, _simple_ and _full_.

## Help

**psp** has a help to remember the usage and shortcut.

```console
[gu]# psp help
psp (Python Scaffolding Projects), version 0.5.0
usage: psp [shortcut]
ie: psp [help|quick|simple|full]

shortcut:
    help:   print this help message
    quick:  enables a rapid setup (few options included)
    simple: enables a basic setup (only Python package)
    full:   enables a full setup (all options included)

links:
    repository:     https://github.com/MatteoGuadrini/psp
    documentation:  https://psp.readthedocs.io/

variables:
    []

environments:
    Python version: 3.14
[gu]# echo $?
0
[gu]# psp whatever
error: unknown shortcut command `whatever`
usage: psp [shortcut]
ie: psp [help|quick|simple|full]
[gu]# echo $?
1
```

## Quick

The _quick_ shortcut enables a rapid configuration of your Python project:

```console
[gu]# psp quick
info: welcome to psp, version 0.5.0
> Name of Python project: mypyprj
> Select documentation generator: MKDocs
> Select license: MIT
Python project `mypyprj` created at mypyprj
```

The options included are:

* [Virtual Environment](simple.md#virtual-environment)
* [Git](simple.md#git)
* [Test files](simple.md#test-files)
* [Documentation](simple.md#documentation)
* [Common files](simple.md#common-files)
* [License](simple.md#license)
* [PyPi dependencies](simple.md#pypi-dependencies)
* [Docker/Podman](simple.md#dockerpodman)

## Simple

The _simple_ shortcut enables a basic configuration of your Python project:

```console
[gu]# psp simple
info: welcome to psp, version 0.5.0
> Name of Python project: mypyprj
Python project `mypyprj` created at mypyprj
```

The options included are:

* [Test files](simple.md#test-files)

## Full

The _full_ shortcut enables a full configuration of your Python project:

```console
[gu]# psp full
info: welcome to psp, version 0.5.0
> Name of Python project: mypyprj
> Select git remote provider: Github
> Username of Github: MatteoGuadrini
> Install dependencies: nosqlapi
> Select documention generator: MKDocs
> Select remote CI provider: TravisCI
> Select license: MIT
Python project `mypyprj` created at mypyprj
```

The options included are:

* [Virtual Environment](simple.md#virtual-environment)
* [Git](simple.md#git)
* [Git remote provider](simple.md#git-remote-provider)
* [Test files](simple.md#test-files)
* [Dependencies](simple.md#dependencies)
* [Documentation](simple.md#documentation)
* [Tox tool](simple.md#tox-tool)
* [Remote CI (Continuous Integration)](simple.md#remote-ci-continuous-integration)
* [Common files](simple.md#common-files)
* [License](simple.md#license)
* [PyPi dependencies](simple.md#pypi-dependencies)
* [Docker/Podman](simple.md#dockerpodman)
