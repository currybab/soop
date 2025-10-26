# Contributing to soop

Thank you for considering contributing to soop! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

- Rust 1.85 or later
- Git
- A Unix-like environment (macOS, Linux, etc.)

### Development Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/soop.git
   cd soop
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run tests:
   ```bash
   cargo test
   ```

## How to Contribute

### Reporting Bugs

Before creating a bug report, please check existing issues to avoid duplicates.

When filing a bug report, include:
- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Your environment (OS, Rust version, etc.)
- Relevant logs or error messages

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:
- A clear, descriptive title
- Detailed description of the proposed feature
- Explanation of why this enhancement would be useful
- Any relevant examples or mockups

### Pull Requests

1. Create a new branch for your feature or fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following the code style guidelines below

3. Test your changes:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. Commit your changes with a clear commit message:
   ```bash
   git commit -m "Add feature: description of your changes"
   ```

5. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

6. Open a Pull Request with:
   - Clear description of the changes
   - Reference to related issues
   - Screenshots or examples if applicable

## Code Style Guidelines

### Rust Code

- Follow the standard Rust formatting style (use `cargo fmt`)
- Run `cargo clippy` and address warnings
- Write clear, descriptive variable and function names
- Add comments for complex logic
- Keep functions focused and reasonably sized

### Commit Messages

- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests when relevant

Example:
```
Add duplicate validation for host names

- Check for existing hosts when adding new entries
- Prompt user to enter different name if duplicate found
- Add tests for validation logic

Fixes #123
```

## Testing

- Write tests for new features
- Ensure existing tests pass before submitting PR
- Test on macOS if possible (primary supported platform)

## Documentation

- Update README.md if adding new features or changing usage
- Add inline documentation for public functions
- Update examples if behavior changes

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive feedback
- Assume good intentions

## Questions?

Feel free to open an issue for questions or discussion about potential contributions.

## License

By contributing to soop, you agree that your contributions will be licensed under the MIT License.
