# Template

**psp** uses [Handlebars](https://handlebarsjs.com/) template engine to render a template files to build the structure
of your Python project.

The default repository of templates is [psp_templates](https://github.com/MatteoGuadrini/psp_templates).

## Files

The table below lists the files you can customize in your template repository. In the _variables_ column you can find
the names of the variables that can be used via the handlebars syntax; for example `{{PACKAGE}}` or `{{{DEPS}}}` (to
avoid replacing some [special characters](https://handlebarsjs.com/guide/#html-escaping))

| **FILENAME**           | **REFERENCE**                                           | **VARIABLES**                                                                                                                                                                                   |
|------------------------|---------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `apache.hbs`           | [License](simple.md#license)                            | `SIGNATURE`,`VERSION`, `PACKAGE`, `AUTHOR`                                                                                                                                                      |
| `cc.hbs`               | [License](simple.md#license)                            | `SIGNATURE`,`VERSION`, `PACKAGE`, `AUTHOR`                                                                                                                                                      |
| `gplv3.hbs`            | [License](simple.md#license)                            | `SIGNATURE`,`VERSION`, `PACKAGE`, `AUTHOR`                                                                                                                                                      |
| `mit.hbs`              | [License](simple.md#license)                            | `SIGNATURE`,`VERSION`, `PACKAGE`, `AUTHOR`                                                                                                                                                      |
| `mozilla.hbs`          | [License](simple.md#license)                            | `SIGNATURE`,`VERSION`, `PACKAGE`, `AUTHOR`                                                                                                                                                      |
| `changes.hbs`          | [Common files](simple.md#common-files)                  | `SIGNATURE`,`VERSION`, `PACKAGE`, `PRJVERSION`                                                                                                                                                  |
| `circleci.hbs`         | [Remote CI](simple.md#remote-ci-continuous-integration) | `SIGNATURE`,`VERSION`, `REQUIREMENTS`                                                                                                                                                           |
| `container_ignore.hbs` | [Containers](simple.md#dockerpodman)                    | `SIGNATURE`,`VERSION`                                                                                                                                                                           |
| `containerfile.hbs`    | [Containers](simple.md#dockerpodman)                    | `SIGNATURE`,`VERSION`, `PACKAGE`                                                                                                                                                                |
| `contributing.hbs`     | [Common files](simple.md#common-files)                  | `SIGNATURE`,`VERSION`, `PACKAGE`                                                                                                                                                                |
| `github_bug.hbs`       | [Git remote provider](simple.md#git-remote-provider)    | `SIGNATURE`,`VERSION`, `PACKAGE`, `USERNAME`                                                                                                                                                    |
| `github_feature.hbs`   | [Git remote provider](simple.md#git-remote-provider)    | `SIGNATURE`,`VERSION`, `PACKAGE`, `USERNAME`                                                                                                                                                    |
| `github_merge.hbs`     | [Git remote provider](simple.md#git-remote-provider)    | `SIGNATURE`,`VERSION`, `PACKAGE`, `USERNAME`                                                                                                                                                    |
| `githubactions.hbs`    | [Remote CI](simple.md#remote-ci-continuous-integration) | `SIGNATURE`,`VERSION`, `PACKAGE`, `PYTHON`                                                                                                                                                      |
| `gitignore.hbs`        | [Git](simple.md#git)                                    | `SIGNATURE`,`VERSION`                                                                                                                                                                           |
| `gitlab_bug.hbs`       | [Git remote provider](simple.md#git-remote-provider)    | `SIGNATURE`,`VERSION`, `PACKAGE`                                                                                                                                                                |
| `gitlab_feature.hbs`   | [Git remote provider](simple.md#git-remote-provider)    | `SIGNATURE`,`VERSION`, `PACKAGE`                                                                                                                                                                |
| `gitlab_merge.hbs`     | [Git remote provider](simple.md#git-remote-provider)    | `SIGNATURE`,`VERSION`, `PACKAGE`                                                                                                                                                                |
| `gitlabcicd.hbs`       | [Remote CI](simple.md#remote-ci-continuous-integration) | `SIGNATURE`,`VERSION`, `PACKAGE`, `PYTHON`                                                                                                                                                      |
| `makefile.hbs`         | [All](simple.md)                                        | `SIGNATURE`,`VERSION`, `ACTIONS`, `OPTIONS`, `TEST`, `BUILD`, `CONTAINER`, `PACKAGE`, `PYTHON`                                                                                                  |
| `pyproject.hbs`        | [All](simple.md)                                        | `SIGNATURE`,`VERSION`, `BUILDER`, `PRJ_NAME`, `PRJ_VER`, `LICENSE`, `USERNAME`, `EMAIL`, `DESCRIPTION`, `PYTHON`, `CLASSIFIERS`, `DEPS`, `HOMEPAGE`, `DOCUMENTATION`, `REPOSITORY`, `CHANGELOG` |
| `readme.hbs`           | [Common files](simple.md#common-files)                  | `SIGNATURE`,`VERSION`, `PACKAGE`, `CONTAINER`, `PRJVERSION`                                                                                                                                     |
| `sample.hbs`           | [Common files](simple.md#common-files)                  | `SIGNATURE`,`VERSION`, `PACKAGE`                                                                                                                                                                |
| `test_module.hbs`      | [Test files](simple.md#test-files)                      | `SIGNATURE`,`VERSION`, `PRJ_NAME`, `PRJ_VER`                                                                                                                                                    |
| `tox.hbs`              | [Tox tool](simple.md#tox-tool)                          | `SIGNATURE`,`VERSION`, `PYTHON`, `DEPS`                                                                                                                                                         |
| `travis.hbs`           | [Remote CI](simple.md#remote-ci-continuous-integration) | `SIGNATURE`,`VERSION`, `PYTHON`, `REQUIREMENTS`                                                                                                                                                 |

## Variables

Each variable in each file correspond to a specific value, used by `psp` to substitute into handlebars template. These
variables are listed below.

| **VARIABLES**   | **VALUE**                                                    |
|-----------------|--------------------------------------------------------------|
| `SIGNATURE`     | _"Generated by psp (https://github.com/MatteoGuadrini/psp)"_ |
| `VERSION`       | psp version like _"0.0.1"_                                   |
| `PACKAGE`       | package name like _"test"_                                   |
| `PRJ_NAME`      | project name like _"test"_                                   |
| `PRJ_VER`       | project version like _"0.0.1"_                               |
| `AUTHOR`        | remote git author like _"MatteoGuadrini"_                    |
| `USERNAME`      | remote git author like _"MatteoGuadrini"_                    |
| `DEPS`          | dependencies list like _"dep1\ndep2\n"_                      |
| `REQUIREMENTS`  | dependencies list like _"dep1 dep2"_                         |
| `PYTHON`        | python version like _"3.14"_                                 |
| `ACTIONS`       | Makefile actions like _"all                                  |
| `OPTIONS`       | Makefile options like _"all test build"_                     |
| `TEST`          | Makefile test option (run `unittest tests`)                  |
| `BUILD`         | Makefile build option (run `python -m build`)                |
| `CONTAINER`     | Makefile container option (run `docker/podman build`)        |
| `LICENSE`       | pyproject license option                                     |
| `EMAIL`         | pyproject email option                                       |
| `DESCRIPTION`   | pyproject description project option                         |
| `CLASSIFIERS`   | pyproject classifiers option                                 |
| `HOMEPAGE`      | pyproject homepage URL option                                |
| `DOCUMENTATION` | pyproject documentation URL option                           |
| `REPOSITORY`    | pyproject repository URL option                              |
| `CHANGELOG`     | pyproject changelog URL option                               |

## Create own templates repository

You can create your own repository, remote or local, of the templates used by `psp`.

!!! note
    If a template is missing, the default `psp` templates repository is used for that template.

### Remote repository

Remote repository may a git remote repository or website with raw file access. All templates must be in the root folder
of the repository. The default repository of `psp`, is a template GitHub repository that can you use to create your own.
Follow
this [Creating a repository from a template](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template).

```console
[gu]# git clone https://github.com/<your_user>/<your_templates>
[gu]# ls <your_templates>   # I selected only few templates
changes.hbs readme.hbs contributing.hbs
[gu]# cat <your_templates>/changes.hbs
<!-- {{SIGNATURE}}, version {{VERSION}} -->

# Changelog

All notable changes to **{{PACKAGE}}** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [{{PRJVERSION}}] - today

### Added
- Start **{{PACKAGE}}** project

### Changed
- None

### Removed
- None

### Fixed
- None
[gu]# export PSP_TEMPLATES="https://raw.githubusercontent.com/<your_user>/<your_templates>/refs/heads/main"
```

## Local repository

You can create a local repository like the procedure of remote repository.

```console
[gu]# git clone https://github.com/<your_user>/<your_templates>
[gu]# ls <your_templates>   # I selected only few templates
changes.hbs readme.hbs contributing.hbs
[gu]# cat <your_templates>/changes.hbs
<!-- {{SIGNATURE}}, version {{VERSION}} -->

# Changelog

All notable changes to **{{PACKAGE}}** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [{{PRJVERSION}}] - today

### Added
- Start **{{PACKAGE}}** project

### Changed
- None

### Removed
- None

### Fixed
- None
[gu]# export PSP_TEMPLATES="<your_templates>"
```

## Enable custom templates

To enable custom templates, set the `PSP_TEMPLATES` environment variable to the path of your templates' repository.

### Enable cache

To enable caching of templates, set the `PSP_CACHE` environment variable to the path where you want to store the cached
templates.

!!! note
    To clear cache, you can delete the cache folder (`$HOME/.psp_cache`) or set `PSP_CACHE` to `false`.
