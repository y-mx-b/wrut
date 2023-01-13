# Setting up `wrut`

Before using `wrut`, you must first set up `wrut`'s storage and configuration
directories.

```sh
wrut --setup a
```

This command will set up `wrut`'s storage directories at `~/.wrut` and the
configuration directory at `~/.config/wrut`. For more information about `wrut`'s
internals, check the
[relevant documentation](../implementation/data_storage.md).

You can also use the `--setup` option to overwrite data/configuration
directories:

```sh
wrut --setup t,p,c
```

This command would overwrite the template, project, and configuration
directories to their defaults. Check `wrut --help` for more options.
