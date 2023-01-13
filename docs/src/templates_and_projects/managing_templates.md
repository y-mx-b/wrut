# Managing Templates

You can view your templates with the `template list` command.

```sh
wrut template list
```

You can also use `t ls` for short.

```sh
wrut t ls
```

If you wish to delete a template, you can use the `template remove command`.

```sh
wrut template remove Novel
```

Now, if we run `template list`, we won't have any available templates.

```sh
$ wrut template list

```

However, the `Novel Template` subdirectory still exists. If we want to delete
the directory as well, we can use the `--delete` flag.

```sh
wrut template remove Novel --delete
```

This will delete the template directory itself alongside the reference to it.
