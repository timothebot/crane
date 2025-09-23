# Brick Config

The brick config defines what the brick should do. The file must be called `brick.toml`
and must be located at the root of the brick directory.

To get started, create a new directory in a [defined brick directory](./crane_config.md#brick-directories).
Inside, create a file called `brick.toml` and add a name for your brick.

```toml
name = "my_brick_name"
```

## No Config

If you have a brick directory without a `brick.toml` file, it will still work. By default, this will take the directory name as brick name and add the [insert file](#insert-file) action.

## Shared options

```toml
# Name must always be set if a brick.toml exists
name = ""
```

## Actions

You can define as many actions as you want. For all actions, you may specify a specific `working_dir`,
which is a relative path from where the action would execute to where it should actually execute.

This is useful if you want to add files from a project root that are located in subfolders.

```toml
working_dir = "./src/"
```

### Insert File

```toml
[[actions]]
action = "insert_file"

if_file_exists = "append" # or "replace" or "pass"

# Which files should be inserted
sources = [
    "file.txt"
]
```

If no `sources` are defined, it will use all files in the brick directory (except the config file).

### Modify File

Allows you to modify a specific part of a file.
You can do different operations, like *replace*, *append* or *prepend*.

```toml
[[actions]]
action = "modify_file"

# The content that will be inserted into the file.
content = "text" # or file (prefix path with file:)

# Where the modification should happen
selector = "[dependencies]" # either text or regex (prefix with re:)

# Modify can be type "append" (default), "prepend" or "replace":
# append text inside file
type = "append"


# Specify which files this action applies to
sources = [
    "file.txt"
]
```

If no `sources` are defined, it will use all files in the brick directory (except the config file).

### Run Script

Allows you to run a command or a script file.

```toml
[[actions]]
action = "run_command"

command = "echo 'hi'" # command or a script file (prefix with file:)
```

This is by far the most simple yet powerful action.
If you need more complex behaviour, you can add a custom script that does what you need.
