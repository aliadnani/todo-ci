# todo-ci

A simple CI/CD CLI tool for registering and checking todos in code with deadlines

## Usage

### Installation

Build from source 
```bash
cargo build --release
sudo mv ./target/release/todo-ci /usr/bin/
```

`cargo`
```bash
cargo install todo-ci
```

### Registering TODOs

Note down TODOs in code following the format `@todo(YYYY-MM-DD): A description of a todo...`
- The TODO must be on one line
- The description must follow the `@todo(YYYY-MM-DD):` and extend until the end of the line

Example:

```scala
import scala.util.Random

val x: Int = Random.nextInt(10)

// @todo(2022-09-19): Add a default case
x match
  case 0 => "zero" 
  case 1 => "one"
  case 2 => "two" // @todo(2022-09-19): A TODO at the end of a line 
  case 3 => "three" 

```

See `./tests/resources` for more examples

### Checking TODOs

Run `todo-ci` to check for overdue TODOs in the current directory

[![asciicast](https://asciinema.org/a/FzpmPuyWCSpLZnkAGlDQCzHjd.svg)](https://asciinema.org/a/FzpmPuyWCSpLZnkAGlDQCzHjd)

Run `todo-ci --help` for all options

```bash
❯ todo-ci --help
todo-ci 0.1.0
todo-ci: A simple ci tool to check overdue todos

USAGE:
    todo-ci [OPTIONS] [ROOT_DIRECTORY]

ARGS:
    <ROOT_DIRECTORY>    Root directory to check `todos` for [default: ./]

OPTIONS:
    -d, --display-mode <DISPLAY_MODE>
            Display mode:
             - concise: total number of valid + overdue todos
             - overdue-only: total number of valid + overdue todos + details of overdue todos
             - default: total number of valid + overdue todos + details of all todos
             - [PLANNED] detailed: total number of valid + overdue todos + details of all todos with
            inline code snippet

             [default: default] [possible values: concise, overdue-only, default]

    -e, --no-error
            Flag to disable returning system error code (1) if there are overdue todos

    -h, --help
            Print help information

    -n, --no-ignore
            Flag to disable ignored files by default (.gitignore, hidden files, etc.)

    -V, --version
            Print version information
```

## TODO

- Write tests
- Configurable timezone for deadline checking - currently just uses naive UTC
- Detailed display mode that shows inline code snippets along with their todos (maybe using `bat`)
- Configurable files to search/ignore TODOs for
- General code cleanup