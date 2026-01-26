# Usage Guide

This guide covers all the ways you can use NoEntropy to organize your files.

## Quick Start

NoEntropy offers two interface modes:

### TUI Mode (Default)
The interactive Terminal User Interface provides a visual file browser:

```bash
./noentropy
```

### CLI Mode (Commands)
Traditional command-line interface for automation and scripting:

```bash
./noentropy organize
```

Both approaches will:
1. Scan your configured downloads folder
2. Ask Gemini AI to categorize files (or use extensions in offline mode)
3. Show you a preview of the organization plan
4. Ask for confirmation before moving files
5. Execute the organization if you approve

## Commands

NoEntropy uses a command-based interface with both TUI and CLI modes:

| Command | Description |
|---------|-------------|
| (default) | Launch interactive TUI |
| `organize` | Organize files using AI categorization (CLI mode) |
| `undo` | Undo the last file organization |
| `key` | Change the Gemini API key |
| `duplicates` | Detect and delete duplicate files |

## TUI Mode (Default)

Running `noentropy` without a subcommand launches the interactive TUI:

```bash
# Launch TUI
./noentropy

# TUI with dry-run mode (preview only)
./noentropy --dry-run

# TUI for specific folder
./noentropy /path/to/folder

# TUI with offline mode
./noentropy --offline

# TUI with recursive mode
./noentropy --recursive
```

### TUI Features

**File Browser Tab**
- Scrollable list of files in the target directory
- File details panel showing name, size, extension, and path
- Keyboard navigation with `j/k` or `↑/↓` arrows
- Press `o` to start organization
- Press `t` to toggle offline mode (AI vs extension-based categorization)

**Plan Review Tab**
- View the AI-generated organization plan
- See where each file will be moved
- Navigate with `j/k` or `↑/↓` arrows
- Press `c` to confirm and execute the plan

**Progress Tab**
- Real-time progress bar during file moves
- Statistics showing moved files, errors, and remaining files
- Status updates throughout the process
- Press `r` to restart when complete or `q` to quit

**Keyboard Shortcuts**
- `j` or `↓`: Move selection down
- `k` or `↑`: Move selection up
- `Tab`: Switch to next tab
- `Shift+Tab`: Switch to previous tab
- `o`: Start organization (Files tab)
- `c`: Confirm plan (Plan tab)
- `r`: Restart after completion/error
- `t`: Toggle offline mode (switch between AI and extension-based categorization)
- `q`: Quit

**Offline Mode Toggle**
The TUI includes a live offline mode toggle:
- Press `t` to switch between online (AI-powered) and offline (extension-based) categorization
- Status is displayed in the TUI interface
- Allows you to test categorization without API calls or when internet is unavailable

## Organize Command (CLI Mode)

The `organize` command provides the traditional command-line interface for file organization.

### Basic Usage

```bash
# Organize downloads folder (CLI mode)
./noentropy organize
```

### Path Specification

Organize any directory instead of the configured download folder:

```bash
./noentropy organize /path/to/folder
```

**Usage with current directory:**
```bash
./noentropy organize .
```

**Usage with relative path:**
```bash
./noentropy organize ./subfolder
```

**When to use:**
- Organize directories other than your Downloads folder
- Quickly organize the current working directory
- Test organization on specific folders before applying to Downloads
- Organize project directories, documents, or other file collections

**Features:**
- Path validation ensures the directory exists and is accessible
- Path normalization resolves `.`, `..`, and symlinks for consistency
- Works with all other options (`--dry-run`, `--recursive`, etc.)

### Dry-Run Mode

Preview what NoEntropy would do without actually moving any files:

```bash
./noentropy organize --dry-run
```

**When to use:**
- First time using NoEntropy
- Testing new custom categories
- Checking how specific files will be categorized
- Verifying organization plan before execution

**Example output:**
```
Found 47 files. Asking Gemini to organize...
Gemini Plan received! Performing deep inspection...
Deep inspection complete!

--- EXECUTION PLAN (DRY RUN) ---
Plan: image1.png -> Images/
Plan: document.pdf -> Documents/
Plan: setup.exe -> Installers/
Plan: notes.txt -> Documents/Notes/
Plan: config.yaml -> Code/Config/

DRY RUN - No files were moved
```

### Recursive Mode

Organize files in subdirectories recursively:

```bash
./noentropy organize --recursive
```

**What it does:**
- Scans all subdirectories within your download folder
- Organizes files from the entire directory tree
- Maintains relative folder structure when creating categories

**Use case:** When you have downloads organized in subfolders that you want to consolidate into proper categories.

### Offline Mode

Use NoEntropy without making API calls (extension-based categorization only):

```bash
./noentropy organize --offline
```

**What it does:**
- Categorizes files based on their extensions only
- No AI analysis or content inspection
- Much faster execution
- No API usage/costs

**Use case:** When you don't have internet access or want to avoid API costs.

### Custom Concurrency

Adjust the number of concurrent API calls (default: 5):

```bash
./noentropy organize --max-concurrent 10
```

**When to adjust:**
- **Increase (10-15)**: If you have fast internet and many files
- **Decrease (1-3)**: If you're hitting rate limits or have slow internet
- **Keep default (5)**: Works well for most use cases

### Combined Options

You can combine multiple options:

```bash
./noentropy organize --dry-run --max-concurrent 3
```

```bash
./noentropy organize --recursive --max-concurrent 10
```

**Custom path combinations:**

```bash
# Preview organization of current directory
./noentropy organize . --dry-run
```

```bash
# Organize specific folder recursively
./noentropy organize /path/to/folder --recursive
```

```bash
# Organize current directory with custom concurrency
./noentropy organize . --max-concurrent 10
```

## Undo Command

NoEntropy tracks all file moves and allows you to undo them.

### Basic Undo

Revert the last file organization:

```bash
./noentropy undo
```

**Example output:**
```
--- UNDO PREVIEW ---
INFO: will restore 5 files:
  Documents/report.pdf -> Downloads/
  Documents/Notes/notes.txt -> Downloads/
  Code/Config/config.yaml -> Downloads/
  Code/Scripts/script.py -> Downloads/
  Images/photo.png -> Downloads/

Do you want to undo these changes? [y/N]: y

--- UNDOING MOVES ---
Restored: Documents/report.pdf -> Downloads/
Restored: Documents/Notes/notes.txt -> Downloads/
Restored: Code/Config/config.yaml -> Downloads/
Restored: Code/Scripts/script.py -> Downloads/
Restored: Images/photo.png -> Downloads/

INFO: Removed empty directory: Documents/Notes
INFO: Removed empty directory: Code/Config
INFO: Removed empty directory: Code/Scripts

UNDO COMPLETE!
Files restored: 5, Skipped: 0, Failed: 0
```

### Undo with Custom Path

Undo organization in a specific directory:

```bash
./noentropy undo /path/to/folder
```

### Undo Dry-Run

Preview what would be undone without actually reversing changes:

```bash
./noentropy undo --dry-run
```

**When to use:**
- Check what files will be restored before undoing
- Verify undo log integrity
- See if any conflicts exist

### Undo Features

The undo system provides several safety features:

- **Preview Before Action**: Always shows what will be undone before executing
- **Conflict Detection**: Checks if source path already exists before restoring
- **Missing File Handling**: Gracefully handles files that were deleted after move
- **Partial Undo Support**: Continues even if some operations fail
- **Empty Directory Cleanup**: Automatically removes empty directories after undo
- **History Retention**: Keeps undo history for 30 days with auto-cleanup

### Undo Limitations

- Only the most recent organization can be undone
- Files deleted after organization cannot be restored
- Files moved outside NoEntropy cannot be tracked
- Undo log is cleared after 30 days

## Key Command

### Change API Key

Update your Gemini API key interactively:

```bash
./noentropy key
```

This will prompt you to enter a new API key and save it to your configuration.

## Duplicates Command

### Detect Duplicate Files

Find duplicate files in your downloads folder:

```bash
./noentropy duplicates
```

### Recursive Duplicate Search

Search for duplicates in subdirectories as well:

```bash
./noentropy duplicates --recursive
```

### Dry-Run for Duplicates

Preview duplicates without deleting:

```bash
./noentropy duplicates --dry-run
```

## Interactive Confirmation

Before moving files, NoEntropy shows you the organization plan:

```
--- EXECUTION PLAN ---
Plan: image1.png -> Images/
Plan: document.pdf -> Documents/
Plan: setup.exe -> Installers/
Plan: notes.txt -> Documents/Notes/
...

Do you want to apply these changes? [y/N]:
```

**Options:**
- Type `y` or `yes` to proceed with organization
- Type `n`, `no`, or press Enter to cancel

**Tip**: Always review the plan carefully, especially when using custom categories for the first time.

## Changing Configuration

### Change API Key

Update your Gemini API key interactively:

```bash
./noentropy key
```

This will prompt you to enter a new API key and save it to your configuration.

### Change Download Folder

Edit your config file manually:

```bash
nano ~/.config/noentropy/config.toml
```

Update the `download_folder` value and save.

## Typical Workflows

### First-Time User Workflow

1. Install NoEntropy
2. Test with dry-run:
   ```bash
   ./noentropy organize --dry-run
   ```
3. Review the organization plan
4. If satisfied, run without dry-run:
   ```bash
   ./noentropy organize
   ```
5. Review and confirm the plan
6. Let NoEntropy organize your files

### Regular Usage Workflow

1. Download files throughout the week
2. Periodically run NoEntropy:
   ```bash
   ./noentropy organize
   ```
3. Review and confirm the plan
4. Files are organized automatically

### Testing Custom Categories

1. Edit config file and add custom categories
2. Test with dry-run:
   ```bash
   ./noentropy organize --dry-run
   ```
3. Review how files are categorized
4. Adjust categories if needed
5. Run without dry-run when satisfied

### Recovery Workflow

If organization didn't work as expected:

1. Undo the changes:
   ```bash
   ./noentropy undo
   ```
2. Adjust configuration or categories
3. Test with dry-run again
4. Re-run organization

## Example Terminal Output

### Successful Organization

```bash
$ ./noentropy organize

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

## Tips and Best Practices

1. **Start with dry-run**: Always test first, especially with custom categories
2. **Review the plan**: Don't blindly accept the organization plan
3. **Use undo when needed**: Don't hesitate to undo and reorganize
4. **Adjust concurrency**: Lower it if you hit rate limits
5. **Regular organization**: Run NoEntropy regularly to keep folders tidy
6. **Test categories**: Use dry-run to test custom categories before committing
7. **Backup important files**: Always maintain backups of critical files

## Performance Considerations

- **Concurrency**: Higher concurrency = faster processing but more API calls
- **Caching**: NoEntropy caches API responses for 7 days to minimize API usage
- **File count**: Large numbers of files (1000+) may take several minutes
- **Rate limits**: Gemini API has rate limits; adjust `--max-concurrent` if needed

## Command Reference

### Global Options

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

### Organize Options

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

### Undo Options

```
Usage: noentropy undo [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to undo (defaults to configured download folder)

Options:
  -d, --dry-run    Preview changes without moving files
  -h, --help       Print help
```

### Duplicates Options

```
Usage: noentropy duplicates [OPTIONS]

Options:
  -r, --recursive    Recursively search files in subdirectory
  -h, --help         Print help
```

## Next Steps

- [How It Works](HOW_IT_WORKS.md) - Understand the organization process
- [Configuration Guide](CONFIGURATION.md) - Learn about configuration options
- [Troubleshooting](TROUBLESHOOTING.md) - Solve common issues

---

[Back to Main README](../README.md)
