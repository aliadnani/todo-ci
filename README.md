# todo-ci

Check for TODOs in code with deadlines 

## Quick Start

Write your todos in the format: `@todo(YYYY-MM-DD): A description of a todo...`

```rust
fn main() {
    // @todo(2022-08-10): Print something besides "Hello World!"
    println!("Hello World!");
}
```

Run `todo-ci <directory>` to check for overdue TODOs in the specified directory

```bash
# Local installation
todo-ci ./

# Docker
docker run -v $(pwd):/volume -it ghcr.io/aliadnani/todo-ci:latest /volume
```

![todo-ci](docs/todo-ci.gif)

## Use Case

Run `todo-ci` as part of your `ci/cd` runs to check for any outstanding TODOs in code. If any are found, a `1` exit code is emitted, hence failing the run.

`todo-ci` is similar to [todo-or-die](https://github.com/davidpdrsn/todo-or-die) & [todo-macro](https://github.com/rgwood/todo-macro) but without the use of Rust macros and instead run as a seperate CLI tool. This allows `todo-ci` to be language agnostic and lets you run it in any any project using a langauge that supports comments.

## Installation

Using `cargo`:

```bash
cargo install todo-ci
```

Using `docker`:

```bash
# Pull the latest image
# See https://github.com/aliadnani/todo-ci/pkgs/container/todo-ci
docker pull ghcr.io/aliadnani/todo-ci:latest
```

## Features

```bash
Options:
  -n, --no-ignore
          For disabling ignored files by default (.gitignore, hidden files, etc.)
  -e, --no-error
          For disabling returning system error code (1) if there are overdue todos
  -d, --display-mode <DISPLAY_MODE>
          Display mode:
           - concise: total number of valid + overdue todos
           - overdue-only: total number of valid + overdue todos + details of overdue todos
           - default: total number of valid + overdue todos + details of all todos
           [default: default] [possible values: concise, overdue-only, default]
  -p, --pattern <IGNORE_PATTERN>
          Pattern to check `todos` for (i.e. `*.rs` , `main.*`, etc.) [default: *]
  -t, --timezone-offset <TIMEZONE_OFFSET>
          Timezone to use for date checking [default: +00:00]
  -h, --help
          Print help information
  -V, --version
          Print version information
```


