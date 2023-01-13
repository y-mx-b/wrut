# `wrut`

A utility to manage project templates.

## Table of Contents

- [Feature Overview](#feature-overview)
- [Documentation](#documentation)
- [Data Storage](#data-storage)
- [Configuration](#configuration)
  - [Default Configuration](#default-configuration)
- [Command-Line Usage](#command-line-usage)
- [TODO](#todo)

## Feature Overview

- Project template creation and management.
- Project creation and management.
- Macro expansion when creating new projects from existing templates (WIP).
- Tags to better organize and filter templates and projects (WIP).

## Documentation

**Warning**: Documentation for the `wrut` utility is still very much a
work-in-progress.

Documentation for the `wrut` utility can be viewed at
<https://y-mx-b.github.io/wrut/>.

## Data Storage

**Warning**: `wrut` is not in a complete state, so this is mostly a project goal
and may change.

The data directory contains the `templates`, `projects`, `tags`, and `.obj`
subdirectories. The `.obj` directory is used by `wrut` to store internal
representations of templates for faster project generation. The other three are
used to store data regarding templates, projects, and tags respectively.

The `templates` and `projects` directories contain symbolic links to actual
template or project directories. The `tags` directory is a little more complex,
in that it stores directories named after each tag, and each of these
directories contains a `templates` and `projects` directory, which each contain
an empty file named after the appropriate template/project.

Example `~/.wrut` directory structure:

```sh
.wrut
├── projects
│   ├── Essay         # symlink; could point to ~/foo/bar, doesn't matter
│   └── wrut           # file name and real path are considered separately
├── tags
│   ├── Programming
│   │   ├── projects
│   │   │   └── wrut
│   │   └── templates
│   │       └── Rust
│   └── Writing
│       ├── projects
│       │   └── Essay
│       └── templates
│           └── LaTeX
└── templates
    ├── Rust          # symlink; same as with projects
    └── LaTeX
```

## Configuration

`wrut` takes a `--config` argument, allowing you to specify the configuration
file to use. By default, the configuration file is located at
`~/.config/wrut/config.toml`.

### Default Configuration

```toml
[template]
ignore_dirs = [
    '.git',
    'target',
    '.build',
]
ignore_files = ['.wut.toml']

[project]
```

Configuration files are stored in the `~/.config/wrut` configuration directory.

## Command-Line Usage

You can run `wrut --help` to get the full help message.

You can also run `wrut <TYPE> <COMMAND> --help` to get more detailed information
about a given command.

## TODO

- [x] Template creation and management
- [x] Project creation and management
- [x] Split backend logic into a separate library
- [ ] Documentation
- [ ] Tags
- [ ] Develop internal template format
- [ ] Macros
