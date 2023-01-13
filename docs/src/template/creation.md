# Creating New Templates

New templates can be created with the `init` subcommand:

```sh
wut template init [NAME]
```

Upon running this command, a template will be initialized using the current
directory. `[NAME]` is an optional argument, and if it is not provided, the name
of the current directory will be used.

When a template is initialized, a `.wrut.toml` file will be created and a
symlink to the directory will be created in the `~/.wrut/templates` directory.
See the [Template Configuration](../configuration/template.md) section for more
details.
