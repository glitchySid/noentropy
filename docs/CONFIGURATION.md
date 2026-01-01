# Configuration Guide

NoEntropy uses a simple TOML configuration file to store your API key, download folder path, and custom categories.

## Configuration File Location

NoEntropy stores configuration in `~/.config/noentropy/config.toml` following XDG Base Directory specifications.

**Platform-specific paths:**
- **Linux/macOS**: `~/.config/noentropy/config.toml`
- **Windows**: `C:\Users\<username>\AppData\Roaming\noentropy\config.toml`

## Basic Configuration

### Minimal Configuration

The minimum required configuration includes just your API key and download folder:

```toml
api_key = "AIzaSyDTEhAq414SHY094A5oy5lxNA0vhbY1O3k"
download_folder = "/home/user/Downloads"
```

### Full Configuration with Custom Categories

```toml
api_key = "AIzaSyDTEhAq414SHY094A5oy5lxNA0vhbY1O3k"
download_folder = "/home/user/Downloads"

# Optional: Custom categories for file organization
categories = ["Work", "Personal", "School", "Projects", "Bills", "Media", "Misc"]
```

## Configuration Options

| Setting | Description | Example | Required |
|---------|-------------|---------|----------|
| `api_key` | Your Google Gemini API key | `AIzaSy...` | Yes |
| `download_folder` | Path to folder to organize | `/home/user/Downloads` | Yes |
| `categories` | Custom categories for organization | `["Work", "Personal", "School"]` | No |

## Getting a Gemini API Key

1. Visit [Google AI Studio](https://ai.google.dev/)
2. Sign in with your Google account
3. Create a new API key
4. Copy the key to your configuration file

**Important**: Keep your API key secure and never commit it to version control.

## Custom Categories

NoEntropy allows you to define your own custom categories instead of using the default ones. This is perfect for organizing files based on your specific workflow or needs.

### Default Categories

If you don't specify custom categories, NoEntropy uses these defaults:
- **Images** - PNG, JPG, GIF, SVG, etc.
- **Documents** - PDF, DOC, DOCX, TXT, MD, etc.
- **Installers** - EXE, DMG, APP, PKG, etc.
- **Music** - MP3, WAV, FLAC, M4A, etc.
- **Archives** - ZIP, TAR, RAR, 7Z, etc.
- **Code** - Source code and configuration files
- **Misc** - Everything else

### Using Custom Categories

To use custom categories, add a `categories` array to your `config.toml`:

```toml
api_key = "your_api_key_here"
download_folder = "/home/user/Downloads"
categories = ["Work", "Personal", "School", "Projects", "Bills", "Media", "Misc"]
```

### Category Examples by Use Case

#### For Students
```toml
categories = ["Courses", "Assignments", "Research", "Personal", "Textbooks", "Media", "Misc"]
```

#### For Professionals
```toml
categories = ["Client Work", "Internal", "Invoices", "Contracts", "Marketing", "Resources", "Misc"]
```

#### For Creatives
```toml
categories = ["Projects", "Assets", "References", "Client Files", "Portfolio", "Tools", "Misc"]
```

#### For Personal Use
```toml
categories = ["Family", "Finance", "Health", "Home", "Travel", "Hobbies", "Misc"]
```

### Tips for Effective Custom Categories

1. **Keep it simple** - Use 5-10 categories for best results
2. **Be specific** - More descriptive names help the AI understand better
3. **Include "Misc"** - Always have a catch-all category for unclear files
4. **Think workflow** - Organize based on how you actually use files
5. **Test first** - Use `--dry-run` to preview categorization before committing

### How Custom Categories Work

When you define custom categories:
1. NoEntropy sends your file list to the Gemini AI
2. The AI is instructed to categorize files into your custom categories
3. Files are organized into folders matching your category names
4. Sub-folders are still created automatically for better organization

**Example Output with Custom Categories:**
```
Downloads/
├── Work/
│   ├── Reports/
│   │   └── Q4-Report.pdf
│   └── Presentations/
│       └── Client-Deck.pptx
├── Personal/
│   ├── Photos/
│   │   └── vacation.jpg
│   └── Documents/
│       └── resume.pdf
└── School/
    ├── Assignments/
    │   └── homework.docx
    └── Notes/
        └── lecture-notes.pdf
```

## Interactive Setup

If configuration is missing, NoEntropy will prompt you interactively:

```bash
$ ./noentropy

NoEntropy Configuration Setup
=============================

API key not found. Please enter your Google Gemini API key:
> AIzaSy...

Download folder not found. Please enter the path to organize:
(Default: /home/user/Downloads)
> /home/user/Downloads

Configuration saved to ~/.config/noentropy/config.toml
```

Configuration is automatically saved after interactive setup.

## Changing Configuration

### Changing API Key

To change your Gemini API key, use the `--change-key` flag:

```bash
./noentropy --change-key
```

Or manually edit `~/.config/noentropy/config.toml` and update the `api_key` value.

### Changing Download Folder

Edit your configuration file and update the `download_folder` value:

```toml
download_folder = "/path/to/new/folder"
```

### Adding or Modifying Categories

Edit your configuration file and add or modify the `categories` array:

```toml
categories = ["Category1", "Category2", "Category3"]
```

To remove custom categories and use defaults, simply delete the `categories` line from your config file.

## Configuration Best Practices

1. **Backup your config** - Keep a copy of your configuration, especially if you have custom categories
2. **Use absolute paths** - Always specify absolute paths for `download_folder`
3. **Test changes** - Use `--dry-run` after changing categories to preview results
4. **Keep API key secure** - Don't share or commit your API key
5. **Regular reviews** - Periodically review if your custom categories still match your needs

## Next Steps

- [Usage Guide](USAGE.md) - Learn how to use NoEntropy
- [How It Works](HOW_IT_WORKS.md) - Understand the organization process
- [Troubleshooting](TROUBLESHOOTING.md) - Solve common configuration issues

---

[Back to Main README](../README.md)
