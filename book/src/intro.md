# Introduction

Crane is a CLI tool that allows you to create *bricks* that you can later add to any project.

A brick is an instruction. It can be a file that gets added, a command that executes or lines getting replaced in a target file.

## Quick Examples

### License brick

Instead of having to look up and copy your desired license from the web, you can create a brick out of it and then run `crane add some-license`.

### Language specific bricks

You can create multiple bricks and combine them behind an alias.

This way, you can easily bootstrap new projects! 

```shell
$ cargo new my_project && cd my_project
# ...
$ crane add rust
→ Executing 4 bricks
  • mit
  • serde
  • rustfmt
  • rustauthor
# ...
```