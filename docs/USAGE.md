# Usage Guide

This guide covers all the ways you can use NoEntropy to organize your files.

## Quick Start

The simplest way to organize your downloads folder:

```bash
./noentropy
```

This will:
1. Scan your configured downloads folder
2. Ask Gemini AI to categorize files
3. Show you a preview of the organization plan
4. Ask for confirmation before moving files
5. Execute the organization if you approve

## Command-Line Options

NoEntropy supports several command-line flags to customize its behavior:

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `[PATH]` | - | - | Path to organize (defaults to configured download folder) |
| `--dry-run` | `-d` | `false` | Preview changes without moving files |
| `--max-concurrent` | `-m` | `5` | Maximum concurrent API requests |
| `--recursive` | - | `false` | Recursively search files in subdirectories |
| `--undo` | - | `false` | Undo the last file organization |
| `--change-key` | - | `false` | Change Gemini API key |
| `--help` | `-h` | - | Show help message |

## Usage Examples

### Custom Path Organization

Organize any directory instead of the configured download folder:

```bash
./noentropy /path/to/folder
```

**Usage with current directory:**
```bash
./noentropy .
```

**Usage with relative path:**
```bash
./noentropy ./subfolder
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
./noentropy --dry-run
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
./noentropy --recursive
```

**What it does:**
- Scans all subdirectories within your download folder
- Organizes files from the entire directory tree
- Maintains relative folder structure when creating categories

**Use case:** When you have downloads organized in subfolders that you want to consolidate into proper categories.

### Custom Concurrency

Adjust the number of concurrent API calls (default: 5):

```bash
./noentropy --max-concurrent 10
```

**When to adjust:**
- **Increase (10-15)**: If you have fast internet and many files
- **Decrease (1-3)**: If you're hitting rate limits or have slow internet
- **Keep default (5)**: Works well for most use cases

### Combined Options

You can combine multiple options:

```bash
./noentropy --dry-run --max-concurrent 3
```

```bash
./noentropy --recursive --max-concurrent 10
```

**Custom path combinations:**

```bash
# Preview organization of current directory
./noentropy . --dry-run
```

```bash
# Organize specific folder recursively
./noentropy /path/to/folder --recursive
```

```bash
# Organize current directory with custom concurrency
./noentropy . --max-concurrent 10
```

```bash
# Undo organization in specific directory
./noentropy /path/to/folder --undo
```

## Undo Operations

NoEntropy tracks all file moves and allows you to undo them.

### Basic Undo

Revert the last file organization:

```bash
./noentropy --undo
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

### Undo Dry-Run

Preview what would be undone without actually reversing changes:

```bash
./noentropy --undo --dry-run
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

## Interactive Confirmation

Before moving files, NoEntropy shows you the organization plan:

```
--- EXECUTION PLAN ---
Plan: image1.png -> Images/
Plan: document.pdf -> Documents/
Plan: setup.exe -> Installers/
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
./noentropy --change-key
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
2. Run with `--dry-run` to preview:
   ```bash
   ./noentropy --dry-run
   ```
3. Review the organization plan
4. If satisfied, run without dry-run:
   ```bash
   ./noentropy
   ```
5. Review and confirm the plan
6. Let NoEntropy organize your files

### Regular Usage Workflow

1. Download files throughout the week
2. Periodically run NoEntropy:
   ```bash
   ./noentropy
   ```
3. Review and confirm the plan
4. Files are organized automatically

### Testing Custom Categories

1. Edit config file and add custom categories
2. Test with dry-run:
   ```bash
   ./noentropy --dry-run
   ```
3. Review how files are categorized
4. Adjust categories if needed
5. Run without dry-run when satisfied

### Recovery Workflow

If organization didn't work as expected:

1. Undo the changes:
   ```bash
   ./noentropy --undo
   ```
2. Adjust configuration or categories
3. Test with dry-run again
4. Re-run organization

## Example Terminal Output

### Successful Organization

```bash
$ ./noentropy

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

## Next Steps

- [How It Works](HOW_IT_WORKS.md) - Understand the organization process
- [Configuration Guide](CONFIGURATION.md) - Learn about configuration options
- [Troubleshooting](TROUBLESHOOTING.md) - Solve common issues

---

[Back to Main README](../README.md)
