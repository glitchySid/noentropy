# How NoEntropy Works

This guide explains the internal architecture and processes that power NoEntropy's intelligent file organization.

## Overview

NoEntropy uses a multi-stage pipeline that combines AI-powered categorization with intelligent caching and concurrent processing to efficiently organize your files.

## User Interface Modes

NoEntropy offers two interface modes:

### TUI Mode (Default)

When you run `noentropy` without a subcommand, it launches an interactive TUI:

```
┌─────────────────────────────────────────────────────────────┐
│ NoEntropy - AI File Organizer                              │
├─────────────────────────────────────────────────────────────┤
│ [Files] [Plan] [Progress]                                 │
├─────────────────────────────────────────────────────────────┤
│ File: document.pdf (1.2MB)                                │
│ Content preview: ...                                      │
│ Proposed category: Documents/Invoices                     │
│                                                             │
│ [↑/↓] Navigate  [Enter] Select                            │
│ [c] Confirm all  [q] Quit                                 │
└─────────────────────────────────────────────────────────────┘
```

**TUI Workflow:**
1. **File Browser**: Browse files with keyboard navigation in Files tab
2. **Organization**: Press `o` to generate AI categorization (or `t` to toggle offline mode first)
3. **Plan Review**: Review proposed organization in Plan tab
4. **Confirmation**: Press `c` to confirm and execute
5. **Progress**: Monitor real-time progress in Progress tab
6. **Completion**: Press `r` to restart or `q` to quit when done

**TUI Features:**
- **Tab Navigation**: Switch between Files, Plan, and Progress tabs using `Tab`/`Shift+Tab`
- **Live Mode Toggle**: Press `t` to toggle between online (AI) and offline (extension-based) modes
- **Status Display**: Shows current mode, file count, and organization status
- **Interactive Controls**: Context-sensitive key bindings based on current tab and state

### CLI Mode

When you run `noentropy organize`, it uses the traditional command-line interface with step-by-step output.

## Organization Process

Both TUI and CLI modes follow the same five-step process:

```
┌─────────────────┐
│  1. Scan Files  │ → Read all files in DOWNLOAD_FOLDER 
└────────┬────────┘   (and subdirs if --recursive flag is used)
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

### Step 1: File Scanning

**What happens:**
- Scans the configured download folder
- Optionally scans subdirectories with `--recursive` flag
- Collects file paths and metadata (size, modification time)
- Filters out directories and focuses on files only

**Output:** List of file paths ready for categorization

### Step 2: Initial Categorization

**What happens:**
- Sends list of filenames to Gemini API
- AI analyzes filenames and determines appropriate categories
- Returns a categorization plan for all files
- Uses custom categories if configured, otherwise uses defaults

**AI Prompt includes:**
- List of all filenames
- Available categories (default or custom)
- Instructions to categorize based on file type and content
- Request for main category assignment

**Output:** Initial organization plan with main categories

### Step 3: Deep Inspection

**What happens:**
- Identifies text-based files that can be read
- Concurrently reads file contents (up to `--max-concurrent` files at once)
- Sends content to Gemini AI for sub-folder suggestions
- AI analyzes content and suggests relevant sub-categories
- Applies intelligent retry logic with exponential backoff

**Supported text file formats:**
```
Source Code: rs, py, js, ts, jsx, tsx, java, go, c, cpp, h, hpp, rb, php, swift, kt, scala, lua, r, m
Web/Config: html, css, json, xml, yaml, yml, toml, ini, cfg, conf
Documentation: txt, md, sql, sh, bat, ps1, log
```

**Why concurrent?**
- Processes multiple files simultaneously
- Significantly reduces total processing time
- Configurable concurrency limit prevents API rate limiting

**Output:** Enhanced organization plan with sub-folders

### Step 4: Preview & Confirmation

**What happens:**
- Displays complete organization plan to user
- Shows source file and destination path for each file
- Waits for user confirmation (y/n)
- Allows user to review before any changes are made

**User options:**
- Accept: Proceed with organization
- Decline: Cancel and exit without changes

**Output:** User decision (proceed or abort)

### Step 5: Execute Moves

**What happens:**
- Creates destination directories as needed
- Moves files to their designated locations
- Records each move in the undo log
- Reports success/failure for each operation
- Displays final summary statistics

**Safety features:**
- Only moves files after user confirmation
- Tracks all operations for undo capability
- Handles errors gracefully without stopping entire process
- Creates parent directories automatically

**Output:** Organized files and execution summary

## Caching System

NoEntropy includes an intelligent caching system to minimize API calls and improve performance.

### Cache Design

- **Location**: `.noentropy_cache.json` in project root
- **Format**: JSON with file path as key
- **Expiry**: 7 days (automatically cleaned up)
- **Max Entries**: 1000 entries (LRU eviction)
- **Change Detection**: File size + modification time (not content hash)

### How Caching Works

1. **First Run**: 
   - Files are analyzed via Gemini API
   - Categorization results are cached with metadata
   
2. **Cache Check** (subsequent runs):
   ```
   File found in cache?
   ├─ No → Analyze via API, cache result
   └─ Yes → File changed (size/time)?
       ├─ Yes → Re-analyze via API, update cache
       └─ No → Use cached categorization
   ```

3. **Cache Maintenance**:
   - Removes entries older than 7 days on every run
   - Evicts oldest entries when limit (1000) is reached
   - Validates file still exists before using cache

### Cache Benefits

- **Reduced API Costs**: Avoids re-analyzing unchanged files
- **Faster Processing**: No API call needed for cached files
- **Efficient**: Metadata-based change detection (no content hashing)
- **Automatic Cleanup**: Self-maintaining with age and size limits

### When Cache is Invalidated

Cache entries are invalidated when:
- File size changes
- File modification time changes
- Cache entry is older than 7 days
- File no longer exists
- Cache is manually deleted

## Undo Log System

NoEntropy tracks all file moves to enable undo functionality.

### Undo Log Design

- **Location**: `~/.config/noentropy/data/undo_log.json`
- **Format**: JSON array of move records
- **Retention**: 30 days (automatically cleaned up)
- **Max Entries**: 1000 entries (oldest evicted)
- **Status Tracking**: Completed, Undone, Failed states

### Move Record Structure

Each file move is recorded with:
- Source path (original location)
- Destination path (new location)
- Timestamp of move
- Status (completed/undone/failed)

### How Undo Works

1. **During Organization**:
   ```
   For each file moved:
   ├─ Record source path
   ├─ Record destination path
   ├─ Record timestamp
   └─ Mark as "completed"
   ```

2. **Undo Execution**:
   ```
   Load undo log
   ├─ Filter "completed" moves (not already undone)
   ├─ Show preview to user
   ├─ Request confirmation
   └─ If confirmed:
       ├─ Check destination exists
       ├─ Check source doesn't exist (avoid conflicts)
       ├─ Move file back to source
       ├─ Mark as "undone"
       └─ Clean up empty directories
   ```

3. **Conflict Handling**:
   - **Source exists**: Skip restore (prevent overwrite)
   - **Destination missing**: Skip restore (file was deleted)
   - **Permission error**: Skip restore, report error

### Undo Safety Features

- **Preview Before Action**: Always shows what will be undone
- **Conflict Detection**: Prevents data loss from overwrites
- **Missing File Handling**: Gracefully skips deleted files
- **Partial Undo Support**: Continues processing despite individual failures
- **Empty Directory Cleanup**: Removes empty folders after undo
- **Dry-Run Mode**: Preview undo without executing

### Undo Limitations

- Only tracks moves made by NoEntropy
- Cannot track manual file operations
- Limited to 30-day history
- Cannot restore deleted files (only moves)

## Supported File Categories

NoEntropy can organize files into these default categories:

| Category | File Types |
|----------|------------|
| **Images** | PNG, JPG, JPEG, GIF, SVG, BMP, WEBP, ICO, TIFF |
| **Documents** | PDF, DOC, DOCX, TXT, MD, RTF, ODT, PAGES |
| **Installers** | EXE, DMG, APP, PKG, DEB, RPM, MSI, APK |
| **Music** | MP3, WAV, FLAC, M4A, AAC, OGG, WMA |
| **Videos** | MP4, AVI, MKV, MOV, WMV, FLV, WEBM |
| **Archives** | ZIP, TAR, GZ, RAR, 7Z, BZ2, XZ |
| **Code** | Source code and configuration files |
| **Misc** | Everything else |

## AI Integration

NoEntropy uses Google's Gemini API for intelligent categorization.

### API Usage

- **Model**: Gemini 1.5 Flash (configurable)
- **Concurrent Requests**: 5 by default (configurable via `--max-concurrent`)
- **Retry Logic**: Exponential backoff for failed requests
- **Rate Limiting**: Respects API rate limits with configurable concurrency

### Prompt Engineering

NoEntropy uses carefully crafted prompts to get accurate categorization:

1. **Initial Categorization Prompt**:
   - Lists all filenames
   - Specifies available categories
   - Requests JSON response with categorization plan

2. **Deep Inspection Prompt**:
   - Provides file content
   - Requests sub-folder suggestion based on content
   - Asks for semantic analysis, not just extension

### Error Handling

- **Network Errors**: Retry with exponential backoff
- **Rate Limiting**: Respects limits, retries after delay
- **Invalid Responses**: Logs error, continues with other files
- **Timeout**: Configurable timeout with fallback behavior

## Performance Characteristics

### Factors Affecting Performance

1. **Number of Files**:
   - 10-50 files: ~10-30 seconds
   - 100-500 files: 1-3 minutes
   - 1000+ files: 5-10 minutes

2. **Concurrency Level**:
   - Higher = faster but more API load
   - Lower = slower but safer for rate limits
   - Default (5) balances speed and safety

3. **Cache Hit Rate**:
   - High hit rate (>80%): Significantly faster
   - Low hit rate (<20%): More API calls needed
   - Regular usage improves hit rate over time

4. **Text File Count**:
   - More text files = more deep inspection
   - Deep inspection adds processing time
   - Concurrent processing mitigates this

### Optimization Strategies

1. **Use caching**: Regular runs benefit from cached results
2. **Adjust concurrency**: Increase for faster processing
3. **Dry-run first**: Test configuration without full processing
4. **Organize regularly**: Smaller batches process faster

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                     NoEntropy                           │
│                  (Main Entry Point)                     │
└────────────┬──────────────────────┬────────────────────┘
             │                      │
    ┌────────▼─────────┐   ┌────────▼────────┐
    │   TUI Mode       │   │   CLI Mode       │
    │ (Interactive)    │   │ (Commands)       │
    └────────┬─────────┘   └────────┬────────┘
             │                      │
             └──────────┬───────────┘
                        │
    ┌───────────────────▼───────────────────┐
    │           Shared Components            │
    │                                      │
    │  ┌─────────────────┐  ┌─────────────▼──────┐
    │  │  File Scanner   │  │   Orchestrator     │
    │  │  & Detector     │  │                    │
    │  └────────┬────────┘  └────────────────────┘
    │           │                                 │
    │  ┌────────▼──────────────────────────────┐ │
    │  │           Gemini AI Client            │ │
    │  │  (with retry logic & concurrent)     │ │
    │  └────────┬──────────────────────────────┘ │
    │           │                                 │
    │  ┌────────▼─────────┐  ┌──────────────────┐ │
    │  │  Cache System    │  │   Undo Log       │ │
    │  └──────────────────┘  └──────────────────┘ │
    │           │                                 │
    │  ┌────────▼─────────┐                         │
    │  │   File Mover    │                         │
    │  └──────────────────┘                         │
    └────────────────────────────────────────────────┘
```

## TUI Architecture

The TUI is built using the `ratatui` framework and follows a modular architecture:

```
TUI Module Structure:
├── app.rs          # Application state management
├── events.rs       # Event handling and keyboard input
├── ui.rs           # UI rendering and layout
└── mod.rs          # Module exports

State Management:
├── App State       # Overall application state
├── Tab State       # Current active tab (Files/Plan/Progress)
├── File List       # Scanned files and selection
├── Organization Plan # Generated categorization plan
└── Progress        # Real-time operation progress
```

**Key TUI Components:**

1. **App State Machine**: Manages transitions between different states
   - `FileList`: Initial file browsing
   - `Fetching`: Getting AI categorization
   - `PlanReview`: Reviewing organization plan
   - `Moving`: Executing file moves
   - `Done`: Completion state
   - `Error`: Error handling

2. **Event Loop**: Handles keyboard input and terminal events
   - Navigation keys (j/k, ↑/↓, Tab)
   - Action keys (o, c, r, q, t)
   - Terminal resize handling

3. **UI Rendering**: Draws the interface using ratatui widgets
   - File list with details
   - Plan review table
   - Progress bar and statistics
   - Status messages and mode indicators

## Next Steps

- [Usage Guide](USAGE.md) - Learn how to use NoEntropy
- [Configuration Guide](CONFIGURATION.md) - Configure NoEntropy
- [Development Guide](DEVELOPMENT.md) - Contribute to NoEntropy

---

[Back to Main README](../README.md)
