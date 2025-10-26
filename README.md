# soop

A simple SSH config manager for the command line.

## Overview

`soop` (SSH cOnfig OPerator) is a command-line tool that makes managing your SSH configurations easy and safe. Add, list, edit, and remove SSH hosts while preserving comments and formatting.

The name "soop" comes from the Korean word "숲" (forest). Just as trees grow and spread throughout a forest, your SSH hosts naturally accumulate and scatter across your config file over time. `soop` helps you navigate and tend to this forest of connections, keeping everything organized and accessible.

## Features

- **Add hosts interactively** with duplicate name validation
- **List hosts** with connection details (user@hostname:port)
- **Connect quickly** with convenient aliases
- **Edit hosts** in your preferred editor with automatic change detection
- **Remove hosts** safely with confirmation prompts
- **Preserve comments** and configuration order
- **Prevent duplicates** when adding or editing hosts

## Installation

### From crates.io

```bash
cargo install soop
```

### From Source

```bash
git clone https://github.com/currybab/soop.git
cd soop
cargo build --release
```

The binary will be available at `target/release/soop`.

### Homebrew

Coming soon.

## Usage

### Add a new SSH host

```bash
soop add
```

Interactively prompts for host details and validates against existing hosts.

### List all SSH hosts

```bash
soop ls
```

Displays all configured hosts with their connection information:

```
myserver            user@192.168.1.1
production          admin@example.com:2222
testserver          10.0.0.1
```

### Connect to a host

```bash
# Interactive selection
soop connect
soop c      # alias
soop ssh    # alias

# Direct connection
soop connect myserver
```

### Edit a host

```bash
# Interactive selection
soop edit

# Edit specific host
soop edit myserver
```

Opens your preferred editor (set via `$EDITOR` environment variable, defaults to `vi`). Validates changes and prevents duplicate host names.

### Remove a host

```bash
# Interactive selection
soop remove
soop rm     # alias

# Remove specific host
soop rm myserver
```

Prompts for confirmation before deletion.

## Requirements

- Rust 1.85 or later
- Tested on macOS (should work on Linux and other Unix-like systems)

## Configuration

`soop` operates on your SSH config file located at `~/.ssh/config`.

Set your preferred editor:

```bash
export EDITOR=vim  # or nano, emacs, etc.
```

## Development Status

This project is in active development. Currently, the codebase lacks comprehensive test coverage. Contributions to add unit tests and integration tests are highly encouraged and would be greatly appreciated.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Areas where we especially need help:
- Test coverage for core functionality
- Testing on Linux and other Unix-like platforms
- Additional features and improvements

## Author

Jun Park ([@currybab](https://github.com/currybab))
