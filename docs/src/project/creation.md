# Creating New Projects

There are two ways to create new projects: the `init` and `new` subcommands.

Using the `init` subcommand will initialize a new project in the current
directory:

```sh
wrut project init <TEMPLATE> [NAME]
```

`<TEMPLATE>` will be the template to generate the project from and `[NAME]` is
an optional parameter to set the name of the project. If `[NAME]` is not
provided, then the name of the current directory will be used.

Using the `new` subcommand will create a new directory and then initialize a new
project in that directory:

```sh
wrut project new <TEMPLATE> <NAME>
```

`<TEMPLATE>` will be the template to generate the project from and `<NAME>` will
be the name of the directory to create and initialize.
