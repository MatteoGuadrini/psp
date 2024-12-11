# Shortcut

**psp** has a three shortcut built-in: _quick_, _simple_ and _full_.

## Help

**psp** has a help to remember the usage and shortcut.

```console
[gu]# psp help
usage: psp [shortcut]
ie: psp [help|quick|simple|full]

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

The _quick_ shortcut enables a rapid configuration of your project:

```console
[gu]# psp quick
Welcome to PSP (Python Scaffolding Projects): 0.1.0
> Name of Python project: mypyprj
> Select documentation generator: MKDocs
> Select license: MIT
Python project `mypyprj` created at mypyprj
```

The options included are:

* [Git](simple.md#git)
* [Test files](simple.md#test-files)
* [Documentation](simple.md#documentation)
* [Common files](simple.md#common-files)
* [License](simple.md#license)
* [PyPi dependencies](simple.md#pypi-dependencies)
* [Docker/Podman](simple.md#dockerpodman)
