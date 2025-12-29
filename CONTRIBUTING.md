# Contributing to Iris

Thank you for your interest in contributing to Iris! 🎉

We welcome contributions of all kinds: bug reports, feature requests, documentation improvements, and code contributions. This guide will help you get started.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Ways to Contribute](#ways-to-contribute)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing](#testing)
- [Getting Help](#getting-help)

## Code of Conduct

By participating in this project, you agree to:
- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on what's best for the community
- Show empathy towards other community members

## Ways to Contribute

### 🐛 Report Bugs
Found a bug? [Open an issue](https://github.com/lordaimer/iris/issues/new) with:
- A clear, descriptive title
- Steps to reproduce the problem
- Expected vs actual behavior
- Your environment (OS, Iris version (run `iris --version`))
- Relevant logs or screenshots

### 💡 Suggest Features
Have an idea? Check the [roadmap](.project/roadmap.md) first, then [open an issue](https://github.com/lordaimer/iris/issues/new) describing:
- The problem you're trying to solve
- Your proposed solution
- Any alternatives you've considered

### 📝 Improve Documentation
Documentation improvements are always welcome! This includes:
- README updates
- Code comments
- Usage examples
- Tutorial content

### 💻 Contribute Code
Ready to code? Check out something from the [roadmap](.project/roadmap.md) and start coding!

## Getting Started

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For version control
- **A code editor**: RustRover is recommended

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/iris.git
   cd iris
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/lordaimer/iris.git
   ```

## Development Setup

### Install Hooks

Before starting development, please run the installation script for git hooks. This ensures you don't cause build errors or push a release with a version bump but without a changelog entry.

```bash
./scripts/hooks/install-hooks.sh
```

### Build the Project

```bash
cargo build
```

### Run Iris Locally

```bash
cargo run -- sort /path/to/folder
```

Or install it locally:
```bash
cargo install --path .
```

### Platform-Specific Notes

#### Windows
- Context menu features require administrator privileges
- Icon resources are built via `build.rs`

#### Linux/macOS
- Shell completion scripts are installed to standard locations
- Ensure you have the necessary permissions for system directories

## Coding Standards

### Rust Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Run `cargo fmt` before committing to format your code
- Run `cargo clippy` to catch common mistakes

### Code Organization

- Keep functions focused and single-purpose
- Use meaningful variable and function names
- Add module-level documentation for public APIs

### Comments

- **All comments should be lowercase** (project convention)
- Use `//` for inline comments
- Use `///` for documentation comments
- Explain *why*, not *what* (the code shows what)

Example:

#### Good Example
```rust
// check if the path is protected before sorting
if is_protected_path(&path) {
    return Err(Error::ProtectedPath);
}
```

#### Bad Example
```rust
// Check if the path is protected before sorting.
if is_protected_path(&path) {
    return Err(Error::ProtectedPath);
}
```

### Error Handling

- Use `anyhow::Result` for application errors
- Provide helpful error messages
- Don't panic unless it's truly unrecoverable

## Commit Message Guidelines

We use a custom commit message format with square brackets for the scope:

```
<type>[<scope>]: <subject>

<body>

<footer>
```

### Types

- `feature`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```
feature[config]: add presets_path option

Allows users to specify a custom directory for preset files,
reducing the size of the main iris.toml configuration.

Closes #123
```

```
fix[windows]: update icon cache on context menu install

The context menu icon wasn't updating properly. Now we
explicitly refresh the icon cache after installation.
```

### Scope

Common scopes in this project:
- `config`: Configuration system
- `sort`: Sorting logic
- `context`: Windows context menu
- `completion`: Shell completion

## Pull Request Process

### Before Submitting

1. **Create a feature branch**:
   ```bash
   git checkout -b feat/your-feature-name
   ```

2. **Keep your branch updated**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

4. **Run linters**:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

5. **Update documentation** if needed

6. **Update CHANGELOG.md** following the existing format

### Submitting

1. Push your branch to your fork:
   ```bash
   git push origin feat/your-feature-name
   ```

2. Open a Pull Request on GitHub with:
   - A clear title following commit message guidelines
   - Description of what changed and why
   - Reference to related issues (e.g., "Closes #123")
   - Screenshots/demos for UI changes

### Review Process

- Maintainers will review your PR
- CI tests must pass (builds and tests on all platforms)
- Address any requested changes
- Once approved, a maintainer will merge your PR

### After Merging

- Delete your feature branch
- Pull the latest changes:
  ```bash
  git checkout main
  git pull upstream main
  ```

## Testing

### Running Tests

```bash
# run all tests
cargo test

# run tests with output
cargo test -- --nocapture

# run specific test
cargo test test_name
```

### Writing Tests

- Add unit tests in the same file as the code
- Add integration tests in `tests/` directory
- Test edge cases and error conditions
- Mock filesystem operations when possible

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_extension() {
        let result = parse_extension("file.txt");
        assert_eq!(result, Some("txt"));
    }
}
```

### Platform-Specific Testing

If your change is platform-specific:
- Mark tests with `#[cfg(target_os = "...")]`
- Note platform requirements in PR description
- CI will test on Windows, Linux, and macOS

## Getting Help

- **Questions?** Open a [discussion](https://github.com/lordaimer/iris/issues/new)
- **Stuck?** Comment on the issue you're working on
- **Found a bug?** [Open an issue](https://github.com/lordaimer/iris/issues/new)

## Recognition

Contributors will be:
- Listed in release notes
- Credited in the project
- Part of building something awesome! 🚀

---

Thank you for contributing to Iris! Every contribution, no matter how small, makes a difference. ❤️
