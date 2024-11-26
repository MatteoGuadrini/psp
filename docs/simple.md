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
├── pyproject.toml         ┐
├── mypyprj                │ Package source code, metadata,
│   ├── __init__.py        │ and build instructions
│   └── __main__.py        ┘
...
```

You can also specify a relative/absolute path; in this case the last name of path is your _project name_:

```console
[gu]# psp
Welcome to PSP (Python Scaffolding Projects): 0.1.0
? Name of Python project: /tmp/mypyprj
[Type name or path]
```
