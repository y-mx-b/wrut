# Data Storage

| ⚠️  | `wrut` is not in a complete state, so this is mostly a project goal and may change. |
| --- | :---------------------------------------------------------------------------------- |

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
