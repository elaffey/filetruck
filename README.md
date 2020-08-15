# Filetruck

Simple command line application to copy files around. Specifically handy to manage dot files.

# Usage

Filetruck uses config files called plans to describe what files should be moved around.

Say we had a plan called `example.yml`

```yaml
name: example # Plans are named. This is used as the directory name when storing these files
files: # List of files. Must be files and not directories.
  - .zshrc
  - .config/nvim/init.vim
```

Now we can use filetruck to pick up those files.

`filetruct --plan example.plan pickup --from ~`

This will copy the files described in the plan file into a directory called "example" in the location that filetruck is called from.

```
example.yml
example/
  .zshrc
  .config
    nvim/
      init.vim
```

When you want to drop off those files again you can call

`filetruct --plan example.plan dropoff --to ~`

And the dot files will be copied back to the home directory.
