# Rusk

A modern YAML-based task runner written in Rust.

## Installation

Cargo:

    cargo install rusk

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
$ rusk greet --name Friend
[INFO] Running task: greet
[RUN] echo "Hello, Friend!"
Hello, Friend!
```

### Available Commands

```bash
# Show help
$ rusk --help

# Show task-specific help
$ rusk greet --help

# Run a task
$ rusk hello

# Run with options
$ rusk greet --name "Rust" --greeting "Hi"
$ rusk greet -n "Rust" -g "Hi"  # Short flags

# Use different config file
$ rusk --file other.yml hello

# Control verbosity
$ rusk --quiet hello      # Minimal output
$ rusk --verbose hello    # Detailed output
$ rusk --silent hello     # No output

# Check version
$ rusk --version
```

## Credits

Inspired by Go [Tusk](https://github.com/rliebz/tusk).
