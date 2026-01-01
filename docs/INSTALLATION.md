# Installation Guide

This guide covers different ways to install and set up NoEntropy on your system.

## Prerequisites

Before installing NoEntropy, ensure you have:

- **Rust 2024 Edition** or later (if building from source)
- **Google Gemini API Key** - Get one at [https://ai.google.dev/](https://ai.google.dev/)
- A folder full of unorganized files to clean up!

## Option 1: Download Pre-built Binary

The easiest way to get started is to download a pre-built binary for your operating system.

1. **Download Binary**
   
   Visit the releases page and download the binary for your operating system (Windows, Linux, or macOS):
   ```bash
   https://github.com/glitchySid/noentropy/releases
   ```

2. **Give Permission (Linux/macOS only)**
   
   Make the binary executable:
   ```bash
   chmod +x noentropy
   ```

3. **Run NoEntropy**
   
   ```bash
   ./noentropy
   ```

## Option 2: Build from Source

If you prefer to build from source or want the latest development version:

1. **Clone the Repository**
   
   ```bash
   git clone https://github.com/glitchySid/noentropy.git
   cd noentropy
   ```

2. **Build the Application**
   
   ```bash
   cargo build --release
   ```

3. **Run the Application**
   
   ```bash
   ./target/release/noentropy
   ```

## First-Run Setup

On first run, NoEntropy will guide you through an interactive setup process:

### Interactive Setup

NoEntropy provides an interactive setup if configuration is missing:

- **Missing API key?** → You'll be prompted to enter it
- **Missing download folder?** → You'll be prompted to specify it (with default suggestion)
- **Both missing?** → You'll be guided through complete setup

Configuration is automatically saved to `~/.config/noentropy/config.toml` after interactive setup.

### Manual Configuration

Alternatively, you can manually create the configuration file:

```bash
cp config.example.toml ~/.config/noentropy/config.toml
nano ~/.config/noentropy/config.toml
```

See the [Configuration Guide](CONFIGURATION.md) for detailed configuration options.

## Getting Your Gemini API Key

1. Visit [Google AI Studio](https://ai.google.dev/)
2. Sign in with your Google account
3. Create a new API key
4. Copy the key to your configuration file or enter it during interactive setup

## Verification

To verify your installation works correctly:

1. Run NoEntropy with the `--dry-run` flag:
   ```bash
   ./noentropy --dry-run
   ```

2. You should see NoEntropy scan your downloads folder and display an organization plan without moving any files.

## Next Steps

- [Configure NoEntropy](CONFIGURATION.md) - Learn about configuration options
- [Usage Guide](USAGE.md) - Learn how to use NoEntropy effectively
- [How It Works](HOW_IT_WORKS.md) - Understand the organization process

## Troubleshooting

If you encounter issues during installation, check the [Troubleshooting Guide](TROUBLESHOOTING.md).

Common installation issues:

- **Rust not installed**: Install Rust from [rustup.rs](https://rustup.rs/)
- **Build errors**: Ensure you have the latest Rust toolchain: `rustup update`
- **Permission denied**: Make sure the binary has execute permissions (Linux/macOS)

---

[Back to Main README](../README.md)
