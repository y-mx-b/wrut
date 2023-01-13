# Managing Templates

To view your existing templates, use the `list` subcommand.

```sh
wrut template list
```

You can remove templates with the `remove` subcommand.

```sh
wrut template remove <TEMPLATE>
```

Removing a template will only delete the symlink to the template directory. You
can use the `--delete` flag in order to delete the project directory as well.
