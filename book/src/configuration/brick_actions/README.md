# Brick Config

Brick actions defines what the brick should do.

Replace / Append is file action

- Add files
- Replace Files
- Inject into file at specific position
- Replace at specific position
- Run commands
- ~~Combine Bricks~~ => not brick

## Shared options

```toml
# Name must always be set if a brick.toml exists
name = ""

# Define where this brick should be executed, eg. in a subfolder (prefix regex with re:)
target_location = ""
```

## Actions

### Insert File

```toml
[[actions]]
action = "insert_file"

insert.if_file_exists = "append" # or "replace" or "fail"
```

### Modify File

```toml
[[actions]]
action = "modify_file"

modify = {
    content = "text" # or file (prefix path with file:)
    
    # Where the modification should happen
    selector = "" # either unique text or regex (prefix with re:)

    # Modify can be either type "append" or "replace":
    # append text inside file
    type = "append"
    location = "before" # or after
    
    # replace text inside file
    type = "replace"
}
```

### Run Script

```toml
[[actions]]
action = "run_command"

script = {
    command = "echo 'hi'" # command or a script file (prefix with file:)
}
```
