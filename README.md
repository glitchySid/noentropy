# NoEntropy 🗂️

> AI-powered file organizer that intelligently sorts your messy Downloads folder using Google Gemini API

![Rust](https://img.shields.io/badge/rust-2024-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)

## About

NoEntropy is a smart command-line tool that organizes your cluttered Downloads folder automatically. It uses Google's Gemini AI to analyze files, understand their content, and categorize them into organized folder structures. Say goodbye to manually sorting through hundreds of downloads!

### Use Cases

- 📂 Organize a messy Downloads folder
- 🤖 Auto-categorize downloaded files by type and content
- 🔍 Smart sub-folder creation based on file content
- 🚀 Batch file organization without manual effort
- 💾 Reduce clutter and improve file system organization

## Features

- **🧠 AI-Powered Categorization** - Uses Google Gemini API for intelligent file sorting
- **📁 Automatic Sub-Folders** - Creates relevant sub-folders based on file content analysis
- **💨 Smart Caching** - Minimizes API calls with metadata-based caching (7-day expiry)
- **⚡ Concurrent Processing** - Parallel file inspection with configurable limits
- **👀 Dry-Run Mode** - Preview changes without moving any files
- **🔄 Retry Logic** - Exponential backoff for resilient API handling
- **📝 Text File Support** - Inspects 30+ text formats for better categorization
- **✅ Interactive Confirmation** - Review organization plan before execution
- **🎯 Configurable** - Adjust concurrency limits and model settings

## Prerequisites

- **Rust 2024 Edition** or later
- **Google Gemini API Key** - Get one at [https://ai.google.dev/](https://ai.google.dev/)
- A folder full of unorganized files to clean up!

## Installation

1. **Clone repository**
   ```bash
   git clone https://github.com/yourusername/noentropy.git
   cd noentropy
   ```

2. **Build the application**
   ```bash
   cargo build --release
   ```

3. **Run the application**
   On first run, NoEntropy will guide you through interactive setup:
   ```bash
   ./target/release/noentropy
   ```

   Or manually create config file at `~/.config/noentropy/config.toml`:
   ```bash
   cp config.example.toml ~/.config/noentropy/config.toml
   nano ~/.config/noentropy/config.toml
   ```

## Configuration

NoEntropy stores configuration in `~/.config/noentropy/config.toml` following XDG Base Directory specifications.

### Configuration File Format

```toml
api_key = "AIzaSyDTEhAq414SHY094A5oy5lxNA0vhbY1O3k"
download_folder = "/home/user/Downloads"
```

| Setting | Description | Example |
|---------|-------------|---------|
| `api_key` | Your Google Gemini API key | `AIzaSy...` |
| `download_folder` | Path to folder to organize | `/home/user/Downloads` |

### Getting a Gemini API Key

1. Visit [Google AI Studio](https://ai.google.dev/)
2. Sign in with your Google account
3. Create a new API key
4. Copy the key to your configuration file

### Interactive Setup

NoEntropy provides an interactive setup on first run:

- **Missing API key?** → You'll be prompted to enter it
- **Missing download folder?** → You'll be prompted to specify it (with default suggestion)
- **Both missing?** → You'll be guided through complete setup

Configuration is automatically saved to `~/.config/noentropy/config.toml` after interactive setup.

## Usage

### Basic Usage

Organize your Downloads folder with default settings:

```bash
cargo run --release
```

### Dry-Run Mode

Preview what would happen without moving any files:

```bash
cargo run --release -- --dry-run
```

### Custom Concurrency

Adjust the number of concurrent API calls (default: 5):

```bash
cargo run --release -- --max-concurrent 10
```

### Combined Options

Use multiple options together:

```bash
cargo run --release -- --dry-run --max-concurrent 3
```

### Command-Line Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--dry-run` | None | `false` | Preview changes without moving files |
| `--max-concurrent` | None | `5` | Maximum concurrent API requests |
| `--help` | `-h` | - | Show help message |

## How It Works

NoEntropy follows a five-step process to organize your files:

```
┌─────────────────┐
│  1. Scan Files  │ → Read all files in DOWNLOAD_FOLDER
└────────┬────────┘
         ▼
┌─────────────────────────┐
│ 2. Initial Categorization │ → Ask Gemini to categorize by filename
└────────┬────────────────┘
         ▼
┌──────────────────────┐
│  3. Deep Inspection   │ → Read text files for sub-categories
│     (Concurrent)      │   • Reads file content
│                       │   • Asks AI for sub-folder
└────────┬──────────────┘
         ▼
┌──────────────────────┐
│  4. Preview & Confirm│ → Show organization plan
│                       │   • Ask user approval
└────────┬──────────────┘
         ▼
┌──────────────────────┐
│   5. Execute Moves    │ → Move files to organized folders
└──────────────────────┘
```

### Example Terminal Output

```bash
$ cargo run --release

Found 47 files. Asking Gemini to organize...
Gemini Plan received! Performing deep inspection...
Reading content of report.pdf...
Reading content of config.yaml...
Reading content of script.py...
Deep inspection complete! Moving Files.....

--- EXECUTION PLAN ---
Plan: image1.png -> Images/
Plan: document.pdf -> Documents/
Plan: setup.exe -> Installers/
Plan: notes.txt -> Documents/Notes/
Plan: config.yaml -> Code/Config/
Plan: script.py -> Code/Scripts/

Do you want to apply these changes? [y/N]: y

--- MOVING FILES ---
Moved: image1.png -> Images/
Moved: document.pdf -> Documents/
Moved: setup.exe -> Installers/
Moved: notes.txt -> Documents/Notes/
Moved: config.yaml -> Code/Config/
Moved: script.py -> Code/Scripts/

Organization Complete!
Files moved: 47, Errors: 0
Done!
```

## Supported Categories

NoEntropy organizes files into these categories:

| Category | Description |
|----------|-------------|
| **Images** | PNG, JPG, GIF, SVG, etc. |
| **Documents** | PDF, DOC, DOCX, TXT, MD, etc. |
| **Installers** | EXE, DMG, APP, PKG, etc. |
| **Music** | MP3, WAV, FLAC, M4A, etc. |
| **Archives** | ZIP, TAR, RAR, 7Z, etc. |
| **Code** | Source code and configuration files |
| **Misc** | Everything else |

## Supported Text Formats

NoEntropy can read and analyze the content of 30+ text file formats:

```
Source Code: rs, py, js, ts, jsx, tsx, java, go, c, cpp, h, hpp, rb, php, swift, kt, scala, lua, r, m
Web/Config: html, css, json, xml, yaml, yml, toml, ini, cfg, conf
Documentation: txt, md, sql, sh, bat, ps1, log
```

## Caching

NoEntropy includes an intelligent caching system to minimize API calls:

- **Location**: `.noentropy_cache.json` in project root
- **Expiry**: 7 days (old entries auto-removed)
- **Change Detection**: Uses file metadata (size + modification time) instead of full content hashing
- **Max Entries**: 1000 entries (oldest evicted when limit reached)

### How Caching Works

1. **First Run**: Files are analyzed and categorized via Gemini API
2. **Response Cached**: Organization plan saved with file metadata
3. **Subsequent Runs**: 
   - Checks if files changed (size/modification time)
   - If unchanged, uses cached categorization
   - If changed, re-analyzes via API
4. **Auto-Cleanup**: Removes cache entries older than 7 days

## Troubleshooting

### "API key not configured"

**Solution**: NoEntropy will prompt you for your API key on first run. Alternatively, manually create `~/.config/noentropy/config.toml`:
```toml
api_key = "your_actual_api_key"
download_folder = "/home/user/Downloads"
```

### "Download folder not configured"

**Solution**: NoEntropy will prompt you for the folder path on first run. Alternatively, manually add it to your config:
```toml
download_folder = "/path/to/your/Downloads"
```

### "API rate limit exceeded"

**Solution**: 
- Wait a few minutes before trying again
- Reduce `--max-concurrent` to limit API calls
- Use caching to reduce redundant requests

### "Network error"

**Solution**:
- Check your internet connection
- Verify Gemini API service is operational
- Ensure firewall allows outbound HTTPS requests

### "Failed to move file"

**Solution**:
- Check file permissions
- Ensure destination folder is writable
- Verify source files still exist

### "Cache corrupted"

**Solution**: Delete `.noentropy_cache.json` and run again. A new cache will be created.

## Development

### Build in Debug Mode
```bash
cargo build
```

### Build in Release Mode
```bash
cargo build --release
```

### Run Tests
```bash
cargo test
```

### Run Clippy (Linting)
```bash
cargo clippy
```

### Check Code
```bash
cargo check
```

## Project Structure

```
noentropy/
├── src/
│   ├── main.rs           # Entry point and CLI handling
│   ├── lib.rs            # Library exports
│   ├── config.rs         # Configuration management
│   ├── gemini.rs         # Gemini API client
│   ├── gemini_errors.rs  # Error handling
│   ├── cache.rs          # Caching system
│   └── files.rs          # File operations
├── Cargo.toml            # Dependencies
├── config.example.toml    # Configuration template
└── README.md             # This file
```

## Future Enhancements

Based on community feedback, we're planning:

- [ ] **Custom Categories** - Define custom categories in `config.toml`
- [ ] **Recursive Mode** - Organize files in subdirectories with `--recursive` flag
- [ ] **Undo Functionality** - Revert file organization changes
- [ ] **Custom Models** - Support for other AI providers
- [ ] **GUI Version** - Desktop application for non-CLI users

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [Google Gemini API](https://ai.google.dev/)
- Inspired by the endless struggle to keep Downloads folders organized

## Show Your Support

⭐ Star this repository if you find it useful!

---

Made with ❤️ by the NoEntropy team
