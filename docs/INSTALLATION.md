# Installation Guide

This guide covers different ways to install and set up NoEntropy on your system.

## Prerequisites

Before installing NoEntropy, ensure you have:

- **Google Gemini API Key** - Get one at [https://ai.google.dev/](https://ai.google.dev/)
- A folder full of unorganized files to clean up!

## Option 1: Download Pre-built Binary

The easiest way to get started is to download a pre-built binary for your operating system.

### Step 1: Download the Binary

Visit the [releases page](https://github.com/glitchySid/noentropy/releases) and download the appropriate archive for your system:

| Operating System | Architecture | File to Download |
|------------------|--------------|------------------|
| Linux | x86_64 | `noentropy-x86_64-unknown-linux-gnu.tar.gz` |
| macOS | x86_64 (Intel) | `noentropy-x86_64-apple-darwin.tar.gz` |
| macOS | arm64 (Apple Silicon) | `noentropy-aarch64-apple-darwin.tar.gz` |
| Windows | x86_64 | `noentropy-x86_64-pc-windows-msvc.zip` |

Or download directly from the command line:

**Linux:**
```bash
curl -LO https://github.com/glitchySid/noentropy/releases/latest/download/noentropy-x86_64-unknown-linux-gnu.tar.gz
```

**macOS (Intel):**
```bash
curl -LO https://github.com/glitchySid/noentropy/releases/latest/download/noentropy-x86_64-apple-darwin.tar.gz
```

**macOS (Apple Silicon):**
```bash
curl -LO https://github.com/glitchySid/noentropy/releases/latest/download/noentropy-aarch64-apple-darwin.tar.gz
```

**Windows (PowerShell):**
```powershell
Invoke-WebRequest -Uri "https://github.com/glitchySid/noentropy/releases/latest/download/noentropy-x86_64-pc-windows-msvc.zip" -OutFile "noentropy.zip"
```

### Step 2: Extract the Archive

**Linux/macOS:**
```bash
tar -xzf noentropy-x86_64-unknown-linux-gnu.tar.gz
```

**Windows:**
Right-click the downloaded zip file and select "Extract All..." or use PowerShell:
```powershell
Expand-Archive -Path "noentropy.zip" -DestinationPath "noentropy"
```

### Step 3: Add to PATH

You need to add the folder containing `noentropy` to your system's PATH so you can run it from anywhere.

#### Linux/macOS

**Option A: User-level (recommended, no sudo required)**

```bash
# Create local bin directory if it doesn't exist
mkdir -p ~/.local/bin

# Move the binary to a location in your PATH
mv noentropy ~/.local/bin/

# Add to PATH temporarily for this session
export PATH="$HOME/.local/bin:$PATH"

# Verify it works
noentropy --help
```

To make this permanent, add this line to your shell configuration file:

**For bash (`~/.bashrc` or `~/.bash_profile`):**
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**For zsh (`~/.zshrc`):**
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**Option B: System-wide (requires sudo)**

```bash
# Move to system bin (requires sudo on most systems)
sudo mv noentropy /usr/local/bin/

# Verify it works
noentropy --help
```

#### Windows

**🚀 Option A: Automatic Installation (Recommended)**

The easiest way to install NoEntropy on Windows is using our automated installer scripts:

##### PowerShell Installer (Modern Windows)
```powershell
# Download and run the installer
Invoke-WebRequest -Uri "https://github.com/glitchySid/noentropy/releases/latest/download/install.ps1" -OutFile "install.ps1"
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser -Force
.\install.ps1

# Or run directly (if execution policy allows)
powershell -Command "iwr -Uri 'https://github.com/glitchySid/noentropy/releases/latest/download/install.ps1' | iex"
```

##### CMD Batch Installer (Legacy Windows)
```cmd
# Download and run the installer
curl -L -o install.bat https://github.com/glitchySid/noentropy/releases/latest/download/install.bat
install.bat

# Or using PowerShell in CMD
powershell -Command "Invoke-WebRequest -Uri 'https://github.com/glitchySid/noentropy/releases/latest/download/install.bat' -OutFile 'install.bat'; install.bat"
```

**Installer Features:**
- ✅ Automatically downloads the latest version
- ✅ Installs to `C:\Program Files\NoEntropy` (or custom path)
- ✅ Adds to PATH automatically
- ✅ Creates uninstaller for easy removal
- ✅ Detects existing installations
- ✅ Works with user and system-level installations

**Installer Options:**
```powershell
# Install specific version
.\install.ps1 -Version "1.0.4"

# Install to custom path
.\install.ps1 -InstallPath "C:\NoEntropy"

# Force overwrite existing installation
.\install.ps1 -Force

# CMD installer options
install.bat -version 1.0.4 -path "C:\NoEntropy" -force
```

---

**Option B: Manual Installation**

If you prefer manual installation or the automated installer doesn't work:

1. **Download the Windows binary** from [releases](https://github.com/glitchySid/noentropy/releases)
2. **Extract the archive** to get `noentropy.exe`
3. **Choose installation location:**

   **User-level (recommended):**
   ```powershell
   # Create installation directory
   New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\AppData\Local\NoEntropy"

   # Move the binary
   Move-Item -Path ".\noentropy.exe" -Destination "$env:USERPROFILE\AppData\Local\NoEntropy\"

   # Add to PATH (User level)
   $path = [Environment]::GetEnvironmentVariable("PATH", "User")
   $newPath = "$path;$env:USERPROFILE\AppData\Local\NoEntropy"
   [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
   ```

   **System-wide (requires Administrator):**
   ```powershell
   # Run PowerShell as Administrator
   Move-Item -Path ".\noentropy.exe" -Destination "C:\Program Files\NoEntropy\noentropy.exe"

   # Add to system PATH
   $path = [Environment]::GetEnvironmentVariable("PATH", "Machine")
   $newPath = "$path;C:\Program Files\NoEntropy"
   [Environment]::SetEnvironmentVariable("PATH", $newPath, "Machine")
   ```

4. **Restart your terminal** for PATH changes to take effect
5. **Verify installation:** `noentropy --help`

---

**🗑️ Uninstallation**

**Automatic Uninstallation:**
```powershell
# Download and run uninstaller
Invoke-WebRequest -Uri "https://github.com/glitchySid/noentropy/releases/latest/download/uninstall.ps1" -OutFile "uninstall.ps1"
.\uninstall.ps1

# Or run the uninstaller created during installation
C:\Program Files\NoEntropy\uninstall.bat
```

**Manual Uninstallation:**
1. Remove installation directory: `C:\Program Files\NoEntropy` (or your custom path)
2. Remove from PATH (via System Properties → Environment Variables)
3. Remove configuration files: `%APPDATA%\NoEntropy` and `%USERPROFILE%\.config\noentropy`

### Step 4: Verify Installation

```bash
noentropy --help
```

You should see the help message with available options.

---

## Option 2: Build from Source

If you prefer to build from source or want the latest development version:

### Prerequisites

- **Rust 2024 Edition** or later - Install from [rustup.rs](https://rustup.rs/)
- **Git** - For cloning the repository

### Step 1: Clone the Repository

```bash
git clone https://github.com/glitchySid/noentropy.git
cd noentropy
```

### Step 2: Build the Application

```bash
cargo build --release
```

The binary will be located at `target/release/noentropy`.

### Step 3: Install Globally (Optional)

**Linux/macOS:**
```bash
# User-level installation
mkdir -p ~/.local/bin
cp target/release/noentropy ~/.local/bin/
noentropy --help
```

**Windows:**
```powershell
# Create installation directory
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\AppData\Local\NoEntropy"

# Copy the binary
Copy-Item -Path ".\target\release\noentropy.exe" -Destination "$env:USERPROFILE\AppData\Local\NoEntropy\"

# Add to PATH (see Windows instructions above)
```

---

## First-Run Setup

On first run, NoEntropy will guide you through an interactive setup process:

### Interactive Setup

NoEntropy provides an interactive setup if configuration is missing:

- **Missing API key?** → You'll be prompted to enter it
- **Missing download folder?** → You'll be prompted to specify it (with default suggestion)
- **Both missing?** → You'll be guided through complete setup

Configuration is automatically saved to:

| OS | Path |
|----|------|
| Linux/macOS | `~/.config/noentropy/config.toml` |
| Windows | `%APPDATA%\NoEntropy\config.toml` |

### Manual Configuration

Alternatively, you can manually create the configuration file:

**Linux/macOS:**
```bash
mkdir -p ~/.config/noentropy
cp config.example.toml ~/.config/noentropy/config.toml
nano ~/.config/noentropy/config.toml
```

**Windows:**
```powershell
# Create config directory
New-Item -ItemType Directory -Force -Path "$env:APPDATA\NoEntropy"

# Copy example config
Copy-Item -Path ".\config.example.toml" -Destination "$env:APPDATA\NoEntropy\config.toml"

# Edit with Notepad
notepad "$env:APPDATA\NoEntropy\config.toml"
```

See the [Configuration Guide](CONFIGURATION.md) for detailed configuration options.

---

## Getting Your Gemini API Key

1. Visit [Google AI Studio](https://ai.google.dev/)
2. Sign in with your Google account
3. Create a new API key
4. Copy the key to your configuration file or enter it during interactive setup

---

## Verification

To verify your installation works correctly:

```bash
noentropy --help
```

If you see the help output, installation was successful!

To test file organization:

```bash
# Organize your downloads folder (or configured folder)
noentropy --dry-run
```

You should see NoEntropy scan your folder and display an organization plan without moving any files.

---

## Next Steps

- [Configure NoEntropy](CONFIGURATION.md) - Learn about configuration options
- [Usage Guide](USAGE.md) - Learn how to use NoEntropy effectively
- [How It Works](HOW_IT_WORKS.md) - Understand the organization process

---

## Troubleshooting

If you encounter issues during installation, check the [Troubleshooting Guide](TROUBLESHOOTING.md).

Common installation issues:

- **"noentropy: command not found"**: The folder is not in your PATH. Restart your terminal or run `source ~/.bashrc` (or `source ~/.zshrc`).
- **Permission denied (Linux/macOS)**: Make sure the binary has execute permissions: `chmod +x noentropy`
- **Windows PATH not updating**: Restart your terminal or computer after adding to PATH.
- **Rust not installed**: Install Rust from [rustup.rs](https://rustup.rs/)
- **Build errors**: Ensure you have the latest Rust toolchain: `rustup update`

---

[Back to Main README](../README.md)
