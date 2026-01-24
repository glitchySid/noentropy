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
- **🖥️ Interactive TUI** - Visual file browser with real-time categorization preview
- **🎮 Keyboard Navigation** - Intuitive keyboard controls for efficient workflow
- **📊 Progress Tracking** - Real-time statistics and progress visualization
- **🖥️ Interactive TUI** - Visual file browser with real-time categorization preview

## Quick Start

### Installation

### Download Pre-built Binary

Download the latest release for your operating system from [releases](https://github.com/glitchySid/noentropy/releases):

| OS | Download |
|----|----------|
| Linux x86_64 | `noentropy-x86_64-unknown-linux-gnu.tar.gz` |
| macOS x86_64 | `noentropy-x86_64-apple-darwin.tar.gz` |
| macOS arm64 | `noentropy-aarch64-apple-darwin.tar.gz` |
| Windows x86_64 | `noentropy-x86_64-pc-windows-msvc.zip` |

**Linux/macOS:**
```bash
# Download and extract
curl -LO https://github.com/glitchySid/noentropy/releases/latest/download/noentropy-x86_64-unknown-linux-gnu.tar.gz
tar -xzf noentropy-x86_64-unknown-linux-gnu.tar.gz

# Add to PATH (user-level)
mkdir -p ~/.local/bin
mv noentropy ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc

# Verify
noentropy --help
```

**Windows:**
```powershell
# Download and extract
Invoke-WebRequest -Uri "https://github.com/glitchySid/noentropy/releases/latest/download/noentropy-x86_64-pc-windows-msvc.zip" -OutFile "noentropy.zip"
Expand-Archive -Path "noentropy.zip" -DestinationPath "noentropy"

# Add to PATH (User-level)
$env:PATH += ";$env:USERPROFILE\AppData\Local\NoEntropy"

# Or add via System Properties:
# Win + R → sysdm.cpl → Environment Variables → Edit PATH
```

See the [Installation Guide](docs/INSTALLATION.md) for detailed instructions.

### Build from Source

```bash
git clone https://github.com/glitchySid/noentropy.git
cd noentropy
cargo build --release
./target/release/noentropy
```

### First Run

On first run, NoEntropy will guide you through an interactive setup to configure your API key and download folder. That's it!

### Basic Usage

```bash
# Launch interactive TUI (default)
./noentropy

# Organize your downloads folder (CLI mode)
./noentropy organize

# Organize a specific directory
./noentropy organize /path/to/folder

# Preview changes without moving files
./noentropy organize --dry-run

# Organize current directory with preview
./noentropy organize . --dry-run

# Use TUI with dry-run mode
./noentropy --dry-run

# Use TUI for specific folder
./noentropy /path/to/folder

# Use TUI with offline mode
./noentropy --offline

# Use TUI with recursive mode
./noentropy --recursive
```

# Undo the last organization
./noentropy undo

# Change API key
./noentropy key

# Find and delete duplicate files
./noentropy duplicates --recursive
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

### TUI Mode

```
+---------------------------------------------------+
| NoEntropy - AI File Organizer                    |
+---------------------------------------------------+
| [Files] [Plan] [Progress]                         |
+---------------------------------------------------+
| File: document.pdf (1.2MB)                        |
| Content preview: ...                              |
| Proposed category: Documents/Invoices             |
|                                                   |
| [↑/↓] Navigate  [Enter] Select                    |
| [c] Confirm all  [q] Quit                         |
+---------------------------------------------------+
```

### CLI Mode

```bash
$ ./noentropy organize

Found 47 files. Asking Gemini to organize...
Gemini Plan received. Performing deep inspection...
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

## Commands

NoEntropy uses a command-based interface with both CLI and TUI modes:

| Command | Description |
|---------|-------------|
| `noentropy` | Launch interactive TUI (default) |
| `noentropy organize` | Organize files using AI categorization (CLI mode) |
| `noentropy undo` | Undo the last file organization |
| `noentropy key` | Change the Gemini API key |
| `noentropy duplicates` | Detect and delete duplicate files |

### TUI Mode (Default)

Running `noentropy` without a subcommand launches the interactive TUI:

```bash
# Launch TUI
oentropy

# TUI with dry-run mode (preview only)
oentropy --dry-run

# TUI for specific folder
oentropy /path/to/folder

# TUI with offline mode
oentropy --offline

# TUI with recursive mode
oentropy --recursive
```

**TUI Features:**
- **File Browser**: Scrollable list of files with details (name, size, extension, path)
- **Organization Plan**: Preview AI categorization before confirming
- **Progress Tracking**: Real-time progress bar and statistics
- **Keyboard Navigation**:
  - `j/k` or `↑/↓`: Navigate file list
  - `Tab`: Switch between Files/Plan/Progress tabs
  - `o`: Start organization (from Files tab)
  - `c`: Confirm and execute plan (from Plan tab)
  - `r`: Restart after completion/error
  - `q`: Quit

### Organize Command (CLI Mode)

Organize files in your downloads folder or any custom directory using the traditional CLI interface:

```bash
# Organize downloads folder (CLI mode)
noentropy organize

# Organize specific directory
noentropy organize /path/to/folder

# Organize current directory
noentropy organize .

# Preview without making changes
noentropy organize --dry-run

# Recursively scan subdirectories
noentropy organize --recursive

# Use offline mode (no API calls)
noentropy organize --offline

# Customize concurrent API requests
noentropy organize --max-concurrent 10

# Combine options
noentropy organize /path/to/folder --dry-run --recursive
```

### Undo Command

Revert the last file organization:

```bash
# Undo last organization in downloads folder
noentropy undo

# Preview undo without making changes
noentropy undo --dry-run

# Undo in specific directory
noentropy undo /path/to/folder
```

### Key Command

Update your Gemini API key:

```bash
noentropy key
```

### Duplicates Command

Find and delete duplicate files:

```bash
# Find duplicates in downloads folder
noentropy duplicates

# Recursively search subdirectories
noentropy duplicates --recursive

# Preview duplicates without deleting
noentropy duplicates --dry-run
```

## Custom Path Support

NoEntropy supports organizing any directory, not just your configured Downloads folder!

### Organize Any Directory

```bash
# Organize current directory
noentropy organize .

# Organize specific folder
noentropy organize /path/to/folder

# Organize with relative path
noentropy organize ./subfolder
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
noentropy organize . --dry-run

# Organize project folder recursively
noentropy organize ./my-project --recursive

# Undo organization in specific directory
noentropy undo /path/to/folder
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

Perfect for organizing files based on your specific workflow. See the [Configuration Guide](docs/CONTRIBUTION.md) for examples.

### Smart Caching

NoEntropy caches API responses for 7 days to minimize costs and improve performance. Files are only re-analyzed if they change (based on size and modification time).

### Undo Functionality

Made a mistake? Easily undo the last organization:

```bash
noentropy undo
```

All file moves are tracked for 30 days with full conflict detection and safety features.

## Requirements

- **Rust 2024 Edition** or later (if building from source)
- **Google Gemini API Key** - Get one at [https://ai.google.dev/](https://ai.google.dev/)

## Command-Line Reference

```
Usage: noentropy <COMMAND>

Commands:
  organize    Organize downloads using AI categorization
  undo        Undo the last file organization
  key         Change the API key
  duplicates  Detect and delete duplicate files
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Organize Command Options

```
Usage: noentropy organize [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to organize (defaults to configured download folder)

Options:
  -d, --dry-run          Preview changes without moving files
  -m, --max-concurrent <MAX_CONCURRENT>
                         Maximum concurrent API requests (default: 5)
  -o, --offline          Use offline mode (extension-based categorization)
  -r, --recursive        Recursively search files in subdirectories
  -h, --help             Print help
```

### Undo Command Options

```
Usage: noentropy undo [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to undo (defaults to configured download folder)

Options:
  -d, --dry-run    Preview changes without moving files
  -h, --help       Print help
```

### Duplicates Command Options

```
Usage: noentropy duplicates [OPTIONS]

Options:
  -r, --recursive    Recursively search files in subdirectory
  -h, --help         Print help
```

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
│   ├── storage/        # Caching and undo log
│   └── tui/            # Terminal User Interface
├── Cargo.toml
└── README.md
```

See the [Development Guide](docs/DEVELOPMENT.md) for detailed architecture information.

## Command-Line Reference

```
Usage: noentropy [OPTIONS] [PATH] [COMMAND]

Commands:
  organize    Organize downloads using AI categorization (CLI mode)
  undo        Undo the last file organization
  key         Change the API key
  duplicates  Detect and delete duplicate files
  help        Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]  Path to organize (defaults to configured download folder)

Options:
  -d, --dry-run    Preview changes without moving files
  -r, --recursive  Recursively search files in subdirectory
  -o, --offline    Use offline mode (extension-based categorization)
  -h, --help       Print help
  -V, --version    Print version
```

### Organize Command Options

```
Usage: noentropy organize [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to organize (defaults to configured download folder)

Options:
  -d, --dry-run          Preview changes without moving files
  -m, --max-concurrent <MAX_CONCURRENT>
                          Maximum concurrent API requests (default: 5)
  -o, --offline          Use offline mode (extension-based categorization)
  -r, --recursive        Recursively search files in subdirectories
  -h, --help             Print help
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
