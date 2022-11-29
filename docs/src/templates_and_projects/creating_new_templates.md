# Creating New Templates

Let's say your creative urges have struck and you have a sudden desire to write
a novel. Being the scatterbrained yet technically savvy individual you are, you
know very well that you're going to create, delete, and create ever so many
writing projects, but you can't be bothered to set up your projects manually.
Fear not, for `wrut` is here to help!

Let's create a new template to serve as the backbone for our writing projects.
First, we'll create a new directory and add a couple files to it.

```sh
mkdir "Novel Template"
cd "Novel Template"
echo "# Title\nThis is my new novel!" >novel.md
```

Now, let's initialize this directory as a `wrut` project template.

```sh
wrut template init Novel
```

Alternatively, you could use the shorter aliases well.

```sh
wrut t i Novel
```

Now, if we run the `list` command, we can see that we've added a new template!

```sh
$ wrut t ls
Novel
```

Perfect!
