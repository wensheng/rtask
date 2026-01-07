# RTask

A modern YAML-based task runner written in Rust.

## Installation

Cargo:

    cargo install rtask

## Usage

```yaml
# tusk.yml
tasks:
  greet:
    usage: Say hello to someone
    options:
      name:
        usage: Person to greet
        default: World
    run: echo "Hello, ${name}!"
```

```bash
$ rtask greet --name Friend
[INFO] Running task: greet
[RUN] echo "Hello, Friend!"
Hello, Friend!
```

### Available Commands

```bash
# Show help
$ rtask --help

# Show task-specific help
$ rtask greet --help

# Run a task
$ rtask hello

# Run with options
$ rtask greet --name "Rust" --greeting "Hi"
$ rtask greet -n "Rust" -g "Hi"  # Short flags

# Use different config file
$ rtask --file other.yml hello

# Control verbosity
$ rtask --quiet hello      # Minimal output
$ rtask --verbose hello    # Detailed output
$ rtask --silent hello     # No output

# Check version
$ rtask --version
```

## Credits

Inspired by Go [Tusk](https://github.com/rliebz/tusk).
