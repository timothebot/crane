<div align="center">
<h1>crane 🏗️</h1>

Create, use and share easily reusable bootstrap packages called bricks.
<br>
<br>
<sup>
⭐ Consider starring this repo – your support motivates me a lot! ⭐
</sup>
<div>
<a href="https://crates.io/crates/crane">
  <img alt="Crates.io Version" src="https://img.shields.io/crates/v/crane">
</a>
<a href="https://crates.io/crates/crane">
  <img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/crane">
</a>
<a href="https://github.com/timothebot/crane/">
  <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/timothebot/crane">
</a>
</div>
<br>
<a href="https://crane.tiimo.space/setup">Setup</a>
&nbsp;~&nbsp;
<a href="https://crane.tiimo.space/faq">FAQ</a>
&nbsp;~&nbsp;
<a href="https://crane.tiimo.space/">Documentation</a>
</div>



<br>

Crane is a CLI tool that allows you to create *bricks* that you can later add to any project.

A brick is an instruction. It can be a file that gets added, a command that
executes or lines getting replaced in a target file.

## Examples

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

## Getting Started

Install the CLI using cargo (more options coming soon!)

```shell
cargo install crane
```

and set up shell completions (not required):

<details>
<summary>ZSH</summary>

```shell
# ~/.zshrc
eval "$(crane completion zsh)"
```

</details>

<details>
<summary>Bash</summary>

```shell
# ~/.bashrc
eval "$(crane completion bash)"
```

</details>

<details>
<summary>Fish</summary>

```shell
# ~/.config/fish/config.fish
crane completion fish | source
```

</details>

## Issues & Contributions

Feel free to open an issue or a PR. Help is always appreciated!

---

made with <3 by [timothebot](https://github.com/timothebot)
