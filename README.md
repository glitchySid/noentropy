# NoEntropy 🗂️

> AI-powered file organizer that intelligently sorts your messy Downloads folder using Google Gemini API

![Rust](https://img.shields.io/badge/rust-2024-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)

## About

NoEntropy is a smart command-line tool that organizes your cluttered Downloads folder automatically. It uses Google's Gemini AI to analyze files, understand their content, and categorize them into organized folder structures. Say goodbye to manually sorting through hundreds of downloads!

## Features

- **🧠 AI-Powered Categorization** - Uses Google Gemini API for intelligent file sorting
- **🎨 Custom Categories** - Define your own categories for personalized organization
- **📁 Automatic Sub-Folders** - Creates relevant sub-folders based on file content analysis
- **💨 Smart Caching** - Minimizes API calls with metadata-based caching (7-day expiry)
- **⚡ Concurrent Processing** - Parallel file inspection with configurable limits
- **👀 Dry-Run Mode** - Preview changes without moving any files
- **📝 Text File Support** - Inspects 30+ text formats for better categorization
- **✅ Interactive Confirmation** - Review organization plan before execution
- **↩️ Undo Support** - Revert file organization changes if needed

## Quick Start

### Installation

**Option 1: Download Pre-built Binary**

Download the binary for your operating system from [releases](https://github.com/glitchySid/noentropy/releases):

```bash
# Linux/macOS: Give execute permissions
chmod +x noentropy

# Run NoEntropy
./noentropy
```

**Option 2: Build from Source**

```bash
# Clone repository
git clone https://github.com/glitchySid/noentropy.git
cd noentropy

# Build and run
cargo build --release
./target/release/noentropy
```

### First Run

On first run, NoEntropy will guide you through an interactive setup to configure your API key and download folder. That's it!

### Basic Usage

```bash
# Organize your downloads folder
./noentropy

# Organize a specific directory (current directory)
./noentropy .

# Organize a specific directory (absolute path)
./noentropy /path/to/folder

# Preview changes without moving files
./noentropy --dry-run

# Preview organization of current directory
./noentropy . --dry-run

# Undo the last organization
./noentropy --undo
```

## Documentation

Comprehensive documentation is available in the `docs/` directory:

- **[Installation Guide](docs/INSTALLATION.md)** - Detailed installation instructions and setup
- **[Configuration Guide](docs/CONFIGURATION.md)** - Configure API key, folders, and custom categories
- **[Usage Guide](docs/USAGE.md)** - Command-line options and usage examples
- **[How It Works](docs/HOW_IT_WORKS.md)** - Architecture, caching, and internal processes
- **[Troubleshooting](docs/TROUBLESHOOTING.md)** - Common issues and solutions
- **[Development Guide](docs/DEVELOPMENT.md)** - Project structure and development setup
- **[Contributing Guide](docs/CONTRIBUTING.md)** - How to contribute to NoEntropy

## Example Output

```bash
$ ./noentropy

Found 47 files. Asking Gemini to organize...
Gemini Plan received! Performing deep inspection...
Deep inspection complete!

--- EXECUTION PLAN ---
Plan: image1.png -> Images/
Plan: document.pdf -> Documents/
Plan: notes.txt -> Documents/Notes/
Plan: config.yaml -> Code/Config/
Plan: script.py -> Code/Scripts/
...

Do you want to apply these changes? [y/N]: y

--- MOVING FILES ---
Moved: image1.png -> Images/
Moved: document.pdf -> Documents/
Moved: notes.txt -> Documents/Notes/
...

Organization Complete!
Files moved: 47, Errors: 0
Done!
```

## Custom Path Support

NoEntropy now supports organizing any directory, not just your configured Downloads folder!

### Organize Any Directory

```bash
# Organize current directory
./noentropy .

# Organize specific folder
./noentropy /path/to/folder

# Organize with relative path
./noentropy ./subfolder
```

### Features

- **Path Validation**: Ensures the directory exists and is accessible
- **Path Normalization**: Resolves `.`, `..`, and symlinks for consistency
- **Full Compatibility**: Works with all existing options (`--dry-run`, `--recursive`, etc.)
- **Security**: Prevents path traversal attacks and invalid paths

### Use Cases

- Quickly organize project directories
- Clean up specific folders without changing configuration
- Test organization on different directories
- Organize documents, downloads, or any file collection

```bash
# Preview organization of current directory
./noentropy . --dry-run

# Organize project folder recursively
./noentropy ./my-project --recursive

# Undo organization in specific directory
./noentropy /path/to/folder --undo
```

## Use Cases

- 📂 Organize a messy Downloads folder
- 🤖 Auto-categorize downloaded files by type and content
- 🔍 Smart sub-folder creation based on file content
- 🚀 Batch file organization without manual effort
- 💾 Reduce clutter and improve file system organization

## Key Features Explained

### Custom Categories

Define your own categories instead of using defaults:

```toml
# config.toml
categories = ["Work", "Personal", "School", "Projects", "Bills", "Media", "Misc"]
```

Perfect for organizing files based on your specific workflow. See the [Configuration Guide](docs/CONFIGURATION.md) for examples.

### Smart Caching

NoEntropy caches API responses for 7 days to minimize costs and improve performance. Files are only re-analyzed if they change (based on size and modification time).

### Undo Functionality

Made a mistake? Easily undo the last organization:

```bash
./noentropy --undo
```

All file moves are tracked for 30 days with full conflict detection and safety features.

## Requirements

- **Rust 2024 Edition** or later (if building from source)
- **Google Gemini API Key** - Get one at [https://ai.google.dev/](https://ai.google.dev/)

## Command-Line Options

| Option | Short | Description |
|--------|-------|-------------|
| `[PATH]` | - | Path to organize (defaults to configured download folder) |
| `--dry-run` | `-d` | Preview changes without moving files |
| `--max-concurrent` | `-m` | Maximum concurrent API requests (default: 5) |
| `--recursive` | - | Recursively search files in subdirectories |
| `--undo` | - | Undo the last file organization |
| `--change-key` | - | Change Gemini API key |
| `--help` | `-h` | Show help message |

See the [Usage Guide](docs/USAGE.md) for detailed examples and workflows.

## Project Structure

```
noentropy/
├── docs/               # Comprehensive documentation
├── src/
│   ├── cli/            # Command-line interface
│   ├── files/          # File operations and detection
│   ├── gemini/         # AI integration
│   ├── models/         # Data structures
│   ├── settings/       # Configuration management
│   └── storage/        # Caching and undo log
├── Cargo.toml
└── README.md
```

See the [Development Guide](docs/DEVELOPMENT.md) for detailed architecture information.

## Contributing

Contributions are welcome! Please see our [Contributing Guide](docs/CONTRIBUTING.md) for:

- How to report bugs
- How to suggest features
- Development setup
- Code standards
- Pull request process

## Future Enhancements

- [x] Custom Categories
- [x] Recursive Mode
- [x] Undo Functionality
- [ ] Custom AI Models (OpenAI, Claude, etc.)
- [ ] GUI Version
- [ ] Watch Mode

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [Google Gemini API](https://ai.google.dev/)
- Inspired by the endless struggle to keep Downloads folders organized

## Support

- Check the [Troubleshooting Guide](docs/TROUBLESHOOTING.md) for common issues
- Browse [GitHub Issues](https://github.com/glitchySid/noentropy/issues) for known problems
- Create a new issue for bugs or feature requests
- Star this repository if you find it useful!

---

Made with ❤️ by the NoEntropy team
