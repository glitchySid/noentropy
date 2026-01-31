# Development Guide

This guide is for developers who want to understand NoEntropy's codebase, contribute features, or extend functionality.

## Project Structure

NoEntropy follows a clean modular architecture for better maintainability and testability:

```
noentropy/
├── .github/
│   └── workflows/
│       └── rust.yml              # CI/CD workflow
├── docs/                         # Documentation files
│   ├── INSTALLATION.md
│   ├── CONFIGURATION.md
│   ├── USAGE.md
│   ├── HOW_IT_WORKS.md
│   ├── TROUBLESHOOTING.md
│   ├── DEVELOPMENT.md           # This file
│   └── CONTRIBUTING.md
├── src/
│   ├── cli/
│   │   ├── mod.rs                # CLI module exports
│   │   ├── args.rs               # Command-line argument definitions
│   │   └── orchestrator.rs       # Organization & undo orchestration
│   ├── files/
│   │   ├── mod.rs                # File module exports
│   │   ├── batch.rs              # File batch processing
│   │   ├── detector.rs           # File type detection
│   │   ├── mover.rs              # File moving operations
│   │   └── undo.rs               # Undo file operations
│   ├── gemini/
│   │   ├── mod.rs                # Gemini API module exports
│   │   ├── client.rs             # Gemini API client
│   │   ├── errors.rs             # Gemini error handling
│   │   ├── prompt.rs             # AI prompt construction
│   │   └── types.rs              # Gemini API types
│   ├── models/
│   │   ├── mod.rs                # Data models exports
│   │   ├── metadata.rs           # File metadata structures
│   │   ├── move_record.rs        # File move tracking
│   │   └── organization.rs       # Organization plan structures
│   ├── settings/
│   │   ├── mod.rs                # Settings module exports
│   │   ├── config.rs             # Configuration management
│   │   ├── prompt.rs             # Interactive configuration prompts
│   │   └── tests.rs              # Settings tests
│   ├── storage/
│   │   ├── mod.rs                # Storage module exports
│   │   ├── cache.rs              # Caching system
│   │   └── undo_log.rs           # Undo log management
│   ├── tui/
│   │   ├── mod.rs                # TUI module exports
│   │   ├── app.rs                # TUI application state
│   │   ├── events.rs             # TUI event handling
│   │   └── ui.rs                 # TUI rendering
│   ├── main.rs                   # Application entry point
│   └── lib.rs                    # Library exports
├── Cargo.toml                    # Dependencies and project metadata
├── Cargo.lock                    # Dependency lock file
├── config.example.toml           # Configuration template
└── README.md                     # Project overview
```

## Module Overview

### cli/
**Purpose**: Command-line interface and orchestration logic

- **args.rs**: Defines CLI arguments using `clap` crate
  - Command-line flags and options
  - Argument validation
  - Help text generation

- **orchestrator.rs**: Main orchestration logic
  - Coordinates file scanning, categorization, and moving
  - Handles undo operations
  - Manages user interaction and confirmation

### files/
**Purpose**: File detection, batching, moving, and undo operations

- **batch.rs**: Batch file processing
  - Groups files for processing
  - Manages concurrent operations
  - Handles batch-level errors

- **detector.rs**: File type detection
  - Identifies text files that can be inspected
  - Determines file categories by extension
  - Supports 30+ text file formats

- **mover.rs**: File moving operations
  - Moves files to destination folders
  - Creates directories as needed
  - Handles file operation errors

- **undo.rs**: Undo functionality
  - Reverses file moves
  - Handles conflicts and edge cases
  - Cleans up empty directories

### gemini/
**Purpose**: Google Gemini API integration

- **client.rs**: API client implementation
  - Makes HTTP requests to Gemini API
  - Handles authentication
  - Manages response parsing

- **errors.rs**: Error types and handling
  - Custom error types for API failures
  - Error conversion and propagation
  - User-friendly error messages

- **prompt.rs**: Prompt engineering
  - Constructs prompts for categorization
  - Handles custom vs default categories
  - Formats file lists for API

- **types.rs**: API request/response types
  - Serialization/deserialization structures
  - Type-safe API communication
  - JSON parsing

### models/
**Purpose**: Core data structures

- **metadata.rs**: File metadata
  - File size, modification time
  - Used for cache invalidation
  - Lightweight file tracking

- **move_record.rs**: File move tracking
  - Records source and destination paths
  - Tracks move status (completed/undone/failed)
  - Used by undo system

- **organization.rs**: Organization plan
  - Represents categorization decisions
  - Maps files to destination paths
  - Used for preview and execution

### settings/
**Purpose**: Configuration management

- **config.rs**: Configuration loading/saving
  - Reads from TOML file
  - Validates configuration
  - Provides default values

- **prompt.rs**: Interactive setup
  - Prompts user for missing config
  - Validates user input
  - Saves configuration

- **tests.rs**: Configuration tests
  - Unit tests for config parsing
  - Validation tests
  - Edge case handling

### tui/
**Purpose**: Terminal User Interface using ratatui framework

- **app.rs**: TUI application state and state machine
  - Manages TUI state transitions (FileList → Fetching → PlanReview → Moving → Done)
  - Handles file scanning, organization, and progress tracking
  - Maintains UI state (selected items, tabs, status messages)
  - Provides state management methods for different phases

- **events.rs**: Event handling and keyboard input
  - Manages terminal input/output using crossterm
  - Handles keyboard events with context-sensitive actions
  - Implements the main TUI event loop
  - Coordinates between UI events and organization logic
  - Supports tab navigation, file selection, and action triggers

- **ui.rs**: User interface rendering and layout
  - Implements ratatui widgets and layouts
  - Handles responsive design for different terminal sizes
  - Renders file lists, organization plans, and progress indicators
  - Provides visual feedback for user actions and system status

### storage/
**Purpose**: Persistent data layer

- **cache.rs**: Caching system
  - Stores API responses
  - Implements expiry logic (7 days)
  - LRU eviction when limit reached

- **undo_log.rs**: Undo log management
  - Persists move records
  - Implements retention policy (30 days)
  - Handles log corruption

## Building and Testing

### Build in Debug Mode

For development with debug symbols:
```bash
cargo build
```

Binary location: `./target/debug/noentropy`

### Build in Release Mode

Optimized build for production:
```bash
cargo build --release
```

Binary location: `./target/release/noentropy`

### Run Tests

Run all unit tests:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Run specific test:
```bash
cargo test test_name
```

### Run with Debug Output

Enable verbose logging:
```bash
RUST_LOG=debug cargo run
```

### Run Clippy (Linting)

Check for common mistakes and code quality:
```bash
cargo clippy
```

Fix auto-fixable issues:
```bash
cargo clippy --fix
```

### Check Code

Fast check without building:
```bash
cargo check
```

### Format Code

Format code according to Rust standards:
```bash
cargo fmt
```

Check formatting without changing files:
```bash
cargo fmt -- --check
```

## Key Dependencies

NoEntropy uses these main dependencies (see `Cargo.toml` for full list):

- **clap**: Command-line argument parsing
- **tokio**: Async runtime for concurrent operations
- **reqwest**: HTTP client for API requests
- **serde**: Serialization/deserialization
- **serde_json**: JSON parsing
- **toml**: TOML configuration parsing
- **directories**: Cross-platform directory paths
- **ratatui**: Terminal User Interface framework for TUI mode
- **crossterm**: Cross-platform terminal handling for TUI
- **futures**: Async utilities for concurrent operations

## Development Workflow

### 1. Set Up Development Environment

```bash
# Clone repository
git clone https://github.com/glitchySid/noentropy.git
cd noentropy

# Build project
cargo build

# Run tests
cargo test

# Check code quality
cargo clippy
```

### 2. Create Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 3. Make Changes

- Write code following Rust conventions
- Add tests for new functionality
- Update documentation as needed
- Run `cargo fmt` before committing

### 4. Test Changes

```bash
# Run tests
cargo test

# Test manually with your changes
cargo run -- --dry-run

# Check for issues
cargo clippy
```

### 5. Commit and Push

```bash
git add .
git commit -m "Add feature: description"
git push origin feature/your-feature-name
```

### 6. Create Pull Request

- Go to GitHub and create a pull request
- Describe your changes clearly
- Reference any related issues
- Wait for review and feedback

## Code Style Guidelines

### Rust Conventions

- Follow standard Rust naming conventions
- Use `snake_case` for functions and variables
- Use `PascalCase` for types and traits
- Use `SCREAMING_SNAKE_CASE` for constants

### Documentation

- Add doc comments (`///`) for public APIs
- Explain complex logic with inline comments
- Update relevant documentation files

### Error Handling

- Use `Result` types for operations that can fail
- Provide meaningful error messages
- Avoid panicking in library code

### Testing

- Write unit tests for new functionality
- Test error cases and edge conditions
- Use descriptive test names

## Common Development Tasks

### Adding a New CLI Flag

1. Edit `src/cli/args.rs`
2. Add new field to `Args` struct
3. Add clap attribute for the flag
4. Update `orchestrator.rs` to use the flag

### Adding a New TUI Feature

1. **State Management**: Update `src/tui/app.rs`
   - Add new state to `AppState` enum if needed
   - Update `App` struct with new fields
   - Add transition methods for new state

2. **Event Handling**: Update `src/tui/events.rs`
   - Add key binding for new feature
   - Implement event handler logic
   - Update state transitions

3. **UI Rendering**: Update `src/tui/ui.rs`
   - Add new widget or modify existing layout
   - Implement visual representation
   - Handle responsive design

4. **Integration**: Update `src/main.rs` or `src/cli/orchestrator.rs`
   - Connect TUI events to backend logic
   - Handle data flow between TUI and core functionality

### Adding a New File Type

1. Edit `src/files/detector.rs`
2. Add extension to relevant detection function
3. Add tests for the new file type

### Modifying AI Prompts

1. Edit `src/gemini/prompt.rs`
2. Update prompt construction logic
3. Test with various file types
4. Consider token limits

### Adding New Configuration Options

1. Edit `src/settings/config.rs`
2. Add field to `Config` struct
3. Update serialization/deserialization
4. Add validation if needed
5. Update `config.example.toml`
6. Update documentation

## Debugging Tips

### Enable Detailed Logging

```bash
RUST_LOG=debug cargo run
```

### Use Rust Backtrace

```bash
RUST_BACKTRACE=1 cargo run
```

Full backtrace:
```bash
RUST_BACKTRACE=full cargo run
```

### Debug Specific Modules

```bash
# Debug Gemini API interactions
RUST_LOG=noentropy::gemini=debug cargo run

# Debug TUI events and state
RUST_LOG=noentropy::tui=debug cargo run

# Debug file operations
RUST_LOG=noentropy::files=debug cargo run

# Debug storage (cache/undo)
RUST_LOG=noentropy::storage=debug cargo run
```

### Print Debugging

Use `dbg!()` macro for quick debugging:
```rust
dbg!(&some_variable);
```

## Performance Profiling

### Basic Timing

```bash
time cargo run --release
```

### CPU Profiling (Linux)

```bash
cargo install flamegraph
sudo cargo flamegraph
```

### Memory Profiling

```bash
cargo install cargo-instruments
cargo instruments -t Allocations
```

## Continuous Integration

NoEntropy uses GitHub Actions for CI/CD (see `.github/workflows/rust.yml`):

- Runs on push and pull requests
- Tests on Linux, macOS, and Windows
- Checks formatting, linting, and tests
- Builds release binaries

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` (if exists)
3. Create git tag: `git tag v1.0.0`
4. Push tag: `git push origin v1.0.0`
5. GitHub Actions builds and publishes release

## Architecture Decisions

### Why Rust?

- Performance: Fast file operations and concurrent processing
- Safety: Memory safety without garbage collection
- Ecosystem: Great libraries for CLI, HTTP, and serialization

### Why Tokio?

- Async/await support for concurrent API calls
- Efficient handling of I/O operations
- Industry-standard async runtime

### Why Gemini API?

- Powerful language understanding
- Good at semantic file categorization
- Reasonable rate limits and pricing

### Why Caching?

- Reduces API costs
- Improves performance for repeated runs
- Minimizes redundant analysis

### Why TUI with ratatui?

- Interactive user experience without GUI dependencies
- Cross-platform terminal compatibility
- Real-time feedback and progress visualization
- Rich keyboard navigation and state management
- No external GUI framework dependencies

### Why Dual Interface (TUI + CLI)?

- **TUI**: Interactive exploration and visual feedback
- **CLI**: Automation, scripting, and integration
- **Flexibility**: Users can choose their preferred interaction style
- **Accessibility**: Works in different environments and use cases

## Future Enhancements

Ideas for future development:

- [ ] Custom AI model support (OpenAI, Claude, etc.)
- [ ] GUI version (desktop application)
- [ ] Watch mode (automatic organization on file creation)
- [ ] Plugin system for custom organization rules
- [ ] Dry-run improvements with more detailed analysis
- [ ] Better conflict resolution strategies
- [ ] Multi-folder support
- [ ] Rule-based organization (in addition to AI)

## Getting Help

- Read existing documentation
- Check GitHub issues for similar questions
- Ask in GitHub Discussions
- Contact maintainers

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Clap Documentation](https://docs.rs/clap/)
- [Serde Guide](https://serde.rs/)

---

[Back to Main README](../README.md) | [Contributing Guide](CONTRIBUTING.md)
