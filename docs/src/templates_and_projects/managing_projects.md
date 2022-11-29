# Managing Projects

We can also view our current projects with the `list` subcommand.

```sh
$ wrut project list
My Novel
My Second Novel
```

Naturally, the shortened version of the command will work as well.

```sh
$ wrut p ls
My Novel
My Second Novel
```

We can also use the `remove` subcommand to delete projects, and the `--delete`
flag will also work, allowing us to delete references to projects as well as the
project directory itself.

```sh
$ wrut project remove "My Novel"
$ wrut p rm "My Second Novel" --delete
$ wrut p ls

```
