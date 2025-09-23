# Crane Config File

The crane config file is located by default at `~/.config/crane/config.toml`, but the config directory can be changed by setting the `CRANE_CONFIG_DIR` env variable.

## Brick Directories

You can define where crane should look for bricks. If no paths are set, crane will look for a `bricks` folder in the same directory as the config is placed.

```toml
brick_dirs = [
    "./bricks"
]
```

## Aliases

You can define aliases for multiple bricks.

```toml
[[alias]]
name = "rust"
bricks = [ "mit", "rustfmt", "serde" ]
```
