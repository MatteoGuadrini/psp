# Template

**psp** uses [Handlebars](https://handlebarsjs.com/) template engine to render a template files to build the structure of your Python project.

The default repository of templates is [psp_templates](https://github.com/MatteoGuadrini/psp_templates).

## Files

| **FILENAME**          | **REFERENCE**                                              | **VARIABLES**                                                                                    |
|-----------------------|------------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| `apache.hbs`          | [License](simple.md#license)                               | `SIGNATURE`,`VERSION`, `PACKAGE`, `AUTHOR`                                                       |
| `cc.hbs`              | [License](simple.md#license)                               | `SIGNATURE`,`VERSION`, `PACKAGE`, `AUTHOR`                                                       |
| `gplv3.hbs`           | [License](simple.md#license)                               | `SIGNATURE`,`VERSION`, `PACKAGE`, `AUTHOR`                                                       |
| `mit.hbs`             | [License](simple.md#license)                               | `SIGNATURE`,`VERSION`, `PACKAGE`, `AUTHOR`                                                       |
| `mozilla.hbs`         | [License](simple.md#license)                               | `SIGNATURE`,`VERSION`, `PACKAGE`, `AUTHOR`                                                       |
| `changes.hbs`         | [Common files](simple.md#common-files)                     | `SIGNATURE`,`VERSION`, `PACKAGE`, `PRJVERSION`                                                   |
| `circleci.hbs`        | [Remote CI](simple.md#remote-ci-continuous-integration)    | `SIGNATURE`,`VERSION`, `REQUIREMENTS`                                                            |
| `container_ignore.hbs`| [Containers](simple.md#dockerpodman)                       | `SIGNATURE`,`VERSION`                                                                            |
| `containerfile.hbs`   | [Containers](simple.md#dockerpodman)                       | `SIGNATURE`,`VERSION`, `PACKAGE`                                                                 |
| `contributing.hbs`    | [Common files](simple.md#common-files)                     | `SIGNATURE`,`VERSION`, `PACKAGE`                                                                 |
| `github_bug.hbs`      | [Git remote provider](simple.md#git-remote-provider)       | `SIGNATURE`,`VERSION`, `PACKAGE`, `USERNAME`                                                     |
| `github_feature.hbs`  | [Git remote provider](simple.md#git-remote-provider)       | `SIGNATURE`,`VERSION`, `PACKAGE`, `USERNAME`                                                     |
| `github_merge.hbs`    | [Git remote provider](simple.md#git-remote-provider)       | `SIGNATURE`,`VERSION`, `PACKAGE`, `USERNAME`                                                     |
| `githubactions.hbs`   | [Remote CI](simple.md#remote-ci-continuous-integration)    | `SIGNATURE`,`VERSION`, `PACKAGE`, `PYTHON`                                                       |
| `gitignore.hbs`       | [Git](simple.md#git)                                       | `SIGNATURE`,`VERSION`                                                                            |
| `gitlab_bug.hbs`      | [Git remote provider](simple.md#git-remote-provider)       | `SIGNATURE`,`VERSION`, `PACKAGE`                                                                 |
| `gitlab_feature.hbs`  | [Git remote provider](simple.md#git-remote-provider)       | `SIGNATURE`,`VERSION`, `PACKAGE`                                                                 |
| `gitlab_merge.hbs`    | [Git remote provider](simple.md#git-remote-provider)       | `SIGNATURE`,`VERSION`, `PACKAGE`                                                                 |
| `gitlabcicd.hbs`      | [Remote CI](simple.md#remote-ci-continuous-integration)    | `SIGNATURE`,`VERSION`, `PACKAGE`, `PYTHON`                                                       |
| `makefile.hbs`        | [All](simple.md)                                           | `SIGNATURE`,`VERSION`, `ACTIONS`, `OPTIONS`, `TEST`, `BUILD`, `CONTAINER`, `PACKAGE`, `PYTHON`   |
| `pyproject.hbs`       | [All](simple.md)                                           | `SIGNATURE`,`VERSION`, `BUILDER`, `PRJ_NAME`, `PRJ_VER`, `LICENSE`, `USERNAME`, `EMAIL`, `DESCRIPTION`, `PYTHON`, `CLASSIFIERS`, `DEPS`, `HOMEPAGE`, `DOCUMENTATION`, `REPOSITORY`, `CHANGELOG`   |
| `readme.hbs`          | [Common files](simple.md#common-files)                     | `SIGNATURE`,`VERSION`, `PACKAGE`, `CONTAINER`, `PRJVERSION`                                      |
| `sample.hbs`          | [Common files](simple.md#common-files)                     | `SIGNATURE`,`VERSION`, `PACKAGE`                                                                 |
| `test_module.hbs`     | [Test files](simple.md#test-files)                         | `SIGNATURE`,`VERSION`, `PRJ_NAME`, `PRJ_VER`                                                     |
| `tox.hbs`             | [Tox tool](simple.md#tox-tool)                             | `SIGNATURE`,`VERSION`, `PYTHON`, `DEPS`                                                          |
| `travis.hbs`          | [Remote CI](simple.md#remote-ci-continuous-integration)    | `SIGNATURE`,`VERSION`, `PYTHON`, `REQUIREMENTS`                                                  |
