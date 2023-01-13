# Template Configuration

When you initialize or create a new template, a `.wrut.toml` is automatically
created. It will have the same format as a global configuration, but is specific
to the template. As such, it will automatically be loaded when you generate a
project from said template.

## Default Configuration

```toml
[template]
ignore_dirs = [
    '.git',
    'target',
    '.build',
]
ignore_files = ['.wrut.toml']

[project]
```
