# Contributing to NoEntropy

Thank you for considering contributing to NoEntropy! This guide will help you get started with contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Testing Guidelines](#testing-guidelines)
- [Documentation Guidelines](#documentation-guidelines)

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for everyone. Please be respectful and constructive in your interactions.

### Expected Behavior

- Be respectful and inclusive
- Accept constructive criticism gracefully
- Focus on what's best for the community
- Show empathy towards other community members

### Unacceptable Behavior

- Harassment or discriminatory language
- Trolling or insulting comments
- Public or private harassment
- Publishing others' private information

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates.

**How to submit a good bug report:**

1. **Use a clear and descriptive title**
2. **Describe the exact steps to reproduce the problem**
3. **Provide specific examples**
4. **Describe the behavior you observed and what you expected**
5. **Include system information**:
   - OS and version
   - Rust version
   - NoEntropy version

**Bug report template:**

```markdown
**Description**
A clear description of the bug.

**Steps to Reproduce**
1. Run command '...'
2. See error '...'

**Expected Behavior**
What you expected to happen.

**Actual Behavior**
What actually happened.

**System Information**
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- NoEntropy version: [e.g., 1.0.0]

**Additional Context**
Any other relevant information.
```

### Suggesting Enhancements

Enhancement suggestions are welcome! Please provide:

1. **Clear use case**: Explain why this enhancement would be useful
2. **Detailed description**: Describe how it should work
3. **Examples**: Provide examples of how it would be used
4. **Alternatives considered**: Mention alternative solutions you've considered

**Enhancement template:**

```markdown
**Feature Description**
A clear description of the feature.

**Use Case**
Why this feature would be useful.

**Proposed Solution**
How you think it should work.

**Alternatives Considered**
Other solutions you've thought about.

**Additional Context**
Any other relevant information.
```

### Contributing Code

We love code contributions! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch**
3. **Make your changes**
4. **Test thoroughly**
5. **Submit a pull request**

See [Development Setup](#development-setup) below for details.

### Improving Documentation

Documentation improvements are highly valued:

- Fix typos or unclear explanations
- Add examples or clarifications
- Improve code comments
- Write tutorials or guides

### Helping Others

- Answer questions in GitHub Issues
- Participate in discussions
- Help review pull requests
- Share your use cases and experiences

## Development Setup

### Prerequisites

- **Rust 2024 Edition** or later
- **Git** for version control
- **Google Gemini API Key** for testing (get one at [https://ai.google.dev/](https://ai.google.dev/))

### Setup Steps

1. **Fork and clone the repository**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/noentropy.git
   cd noentropy
   ```

2. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/glitchySid/noentropy.git
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

5. **Set up configuration** (for testing):
   ```bash
   cp config.example.toml ~/.config/noentropy/config.toml
   # Edit config.toml with your API key and test folder
   ```

6. **Verify installation**:
   ```bash
   cargo run -- --dry-run
   ```

### Keeping Your Fork Updated

```bash
git fetch upstream
git checkout main
git merge upstream/main
git push origin main
```

## Pull Request Process

### Before Submitting

1. **Ensure tests pass**:
   ```bash
   cargo test
   ```

2. **Run code formatter**:
   ```bash
   cargo fmt
   ```

3. **Run linter**:
   ```bash
   cargo clippy
   ```

4. **Test manually** with various scenarios

5. **Update documentation** if needed

### Creating a Pull Request

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and commit:
   ```bash
   git add .
   git commit -m "Add feature: description"
   ```

3. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Open a pull request** on GitHub

5. **Fill out the PR template** with:
   - Description of changes
   - Related issue number (if applicable)
   - Testing performed
   - Screenshots (if UI changes)

### PR Review Process

1. **Automated checks** will run (tests, linting, formatting)
2. **Maintainers will review** your code
3. **Address feedback** by making additional commits
4. **Once approved**, your PR will be merged

### PR Guidelines

- **Keep PRs focused**: One feature or fix per PR
- **Write clear descriptions**: Explain what and why
- **Reference issues**: Use "Fixes #123" to link issues
- **Be responsive**: Respond to review feedback promptly
- **Be patient**: Reviews may take time

## Coding Standards

### Rust Style Guide

Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/):

- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common mistakes
- Follow Rust naming conventions:
  - `snake_case` for functions and variables
  - `PascalCase` for types and traits
  - `SCREAMING_SNAKE_CASE` for constants

### Code Organization

- Keep functions focused and small
- Group related functionality in modules
- Use meaningful variable and function names
- Avoid deep nesting (prefer early returns)

### Error Handling

- Use `Result` types for fallible operations
- Provide meaningful error messages
- Don't panic in library code
- Use `?` operator for error propagation

### Comments and Documentation

- Add doc comments (`///`) for public APIs
- Explain "why" not "what" in comments
- Keep comments up-to-date with code changes
- Use examples in doc comments when helpful

### Example

```rust
/// Calculates the destination path for a file based on its category.
///
/// # Arguments
///
/// * `file_path` - The original file path
/// * `category` - The category to organize into
/// * `subfolder` - Optional subfolder within category
///
/// # Examples
///
/// ```
/// let dest = calculate_destination("/downloads/file.txt", "Documents", Some("Work"));
/// assert_eq!(dest, "/downloads/Documents/Work/file.txt");
/// ```
///
/// # Errors
///
/// Returns an error if the path cannot be constructed.
pub fn calculate_destination(
    file_path: &Path,
    category: &str,
    subfolder: Option<&str>
) -> Result<PathBuf, Error> {
    // Implementation
}
```

## Commit Message Guidelines

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type

- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, etc.)
- **refactor**: Code refactoring
- **test**: Adding or updating tests
- **chore**: Maintenance tasks

### Examples

**Good commit messages:**

```
feat(cli): add recursive flag for subdirectory scanning

Adds --recursive flag to scan and organize files in subdirectories.
This is useful when downloads are already partially organized.

Fixes #42
```

```
fix(cache): prevent cache corruption on interrupted writes

Use atomic writes to ensure cache file is not corrupted if
process is interrupted during save operation.
```

```
docs: update configuration guide with new examples

Add examples for different user types (students, professionals).
Clarify custom category usage.
```

**Bad commit messages:**

```
fix stuff
```

```
WIP
```

```
more changes
```

### Tips

- Use present tense ("add feature" not "added feature")
- Be specific and descriptive
- Reference issues and PRs when relevant
- Keep subject line under 72 characters
- Separate subject from body with blank line

## Testing Guidelines

### Writing Tests

- Write tests for new functionality
- Test both success and error cases
- Use descriptive test names
- Keep tests focused and independent

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_config() {
        // Arrange
        let config_str = r#"
            api_key = "test_key"
            download_folder = "/tmp/test"
        "#;

        // Act
        let result = parse_config(config_str);

        // Assert
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.api_key, "test_key");
        assert_eq!(config.download_folder, "/tmp/test");
    }

    #[test]
    fn test_parse_invalid_config() {
        let config_str = "invalid toml";
        let result = parse_config(config_str);
        assert!(result.is_err());
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run tests in specific module
cargo test module_name::
```

## Documentation Guidelines

### Code Documentation

- Document all public APIs
- Include examples in doc comments
- Explain parameters and return values
- Document errors that can occur

### User Documentation

When updating user-facing docs:

- Keep language clear and simple
- Provide examples
- Include screenshots for UI changes
- Update all relevant docs

### Documentation Files

- **README.md**: Project overview and quick start
- **docs/INSTALLATION.md**: Installation instructions
- **docs/CONFIGURATION.md**: Configuration guide
- **docs/USAGE.md**: Usage examples and options
- **docs/HOW_IT_WORKS.md**: Architecture and internals
- **docs/TROUBLESHOOTING.md**: Common issues and solutions
- **docs/DEVELOPMENT.md**: Development guide
- **docs/CONTRIBUTING.md**: This file

## Questions?

- Check [existing documentation](../README.md)
- Search [GitHub Issues](https://github.com/glitchySid/noentropy/issues)
- Ask in [GitHub Discussions](https://github.com/glitchySid/noentropy/discussions)
- Contact maintainers

## Recognition

Contributors will be recognized in:
- GitHub contributors list
- Release notes (for significant contributions)
- Project documentation (for major features)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to NoEntropy!

[Back to Main README](../README.md) | [Development Guide](DEVELOPMENT.md)
