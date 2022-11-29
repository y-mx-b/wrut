# Creating New Projects

Now, let's get to writing! First, let's create a new project directory.

```sh
cd ..
mkdir "My Novel"
cd "My Novel"
wrut project init Novel
```

We could provide `wrut project init` a project name as well, but if we don't,
it'll simply use the name of the directory.

```sh
$ wrut p ls
My Novel
```

Now, we should have `novel.md` in this directory. Let's edit its contents!

```markdown
<!-- novel.md -->

# My Novel 

This is my new novel!
```

Beautiful.

We can also use `project new` as an alternative to `project init`.

```sh
cd ..
wrut project new Novel "My Second Novel"
```

The `new` subcommand will create a new directory `"My Second Novel"` and then
initialize it. It's no different from creating the directory and running
`project init`, it's just quicker.
