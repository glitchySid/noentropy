# Troubleshooting Guide

This guide helps you solve common issues you might encounter while using NoEntropy.

## Configuration Issues

### "API key not configured"

**Problem**: NoEntropy cannot find your Gemini API key.

**Solutions**:

1. **Interactive Setup** (Recommended):
   - Simply run NoEntropy and it will prompt you for your API key
   - The configuration will be saved automatically

2. **Manual Configuration**:
   ```bash
   mkdir -p ~/.config/noentropy
   nano ~/.config/noentropy/config.toml
   ```
   
   Add:
   ```toml
   api_key = "your_actual_api_key_here"
   download_folder = "/home/user/Downloads"
   ```

3. **Use Change Key Flag**:
   ```bash
   ./noentropy --change-key
   ```

**Verify**: Run `./noentropy --dry-run` to test your configuration.

---

### "Download folder not configured"

**Problem**: NoEntropy doesn't know which folder to organize.

**Solutions**:

1. **Interactive Setup**:
   - Run NoEntropy and it will prompt you for the folder path
   - It will suggest a default location based on your system

2. **Manual Configuration**:
   ```bash
   nano ~/.config/noentropy/config.toml
   ```
   
   Add or update:
   ```toml
   download_folder = "/path/to/your/Downloads"
   ```

**Important**: Use absolute paths, not relative paths (e.g., `/home/user/Downloads`, not `~/Downloads`).

---

### "Configuration file not found"

**Problem**: The configuration file doesn't exist or is in the wrong location.

**Expected Locations**:
- **Linux/macOS**: `~/.config/noentropy/config.toml`
- **Windows**: `C:\Users\<username>\AppData\Roaming\noentropy\config.toml`

**Solutions**:

1. **Create configuration directory**:
   ```bash
   mkdir -p ~/.config/noentropy
   ```

2. **Copy example configuration**:
   ```bash
   cp config.example.toml ~/.config/noentropy/config.toml
   ```

3. **Edit configuration**:
   ```bash
   nano ~/.config/noentropy/config.toml
   ```

---

### "Invalid API key format"

**Problem**: Your API key is incorrectly formatted or invalid.

**Solutions**:

1. **Get a new API key**:
   - Visit [Google AI Studio](https://ai.google.dev/)
   - Create a new API key
   - Copy it exactly (including all characters)

2. **Check for common issues**:
   - No extra spaces before or after the key
   - Key is enclosed in quotes in TOML file
   - No line breaks within the key

**Example of correct format**:
```toml
api_key = "AIzaSyDTEhAq414SHY094A5oy5lxNA0vhbY1O3k"
```

---

## API and Network Issues

### "API rate limit exceeded"

**Problem**: You're making too many requests to the Gemini API.

**Solutions**:

1. **Wait and retry**:
   - Wait 5-10 minutes before trying again
   - Rate limits reset over time

2. **Reduce concurrency**:
   ```bash
   ./noentropy --max-concurrent 2
   ```

3. **Use caching**:
   - NoEntropy caches results for 7 days
   - Subsequent runs of same files won't hit API
   - Cache is automatic, no configuration needed

4. **Check API quota**:
   - Visit [Google AI Studio](https://ai.google.dev/)
   - Check your API usage and quotas
   - Consider upgrading if you hit limits frequently

---

### "Network error" or "Connection timeout"

**Problem**: Cannot connect to Gemini API.

**Solutions**:

1. **Check internet connection**:
   ```bash
   ping google.com
   ```

2. **Verify API service status**:
   - Check [Google Cloud Status](https://status.cloud.google.com/)
   - Verify Gemini API is operational

3. **Check firewall settings**:
   - Ensure outbound HTTPS (port 443) is allowed
   - Whitelist generativelanguage.googleapis.com if needed

4. **Try with fewer files**:
   - API timeout might be due to large batch
   - Reduce `--max-concurrent` value
   - Organize files in smaller batches

5. **Check proxy settings**:
   - If behind corporate proxy, configure proxy settings
   - Some proxies block API calls

---

### "Invalid API response" or "Failed to parse response"

**Problem**: API returned unexpected or malformed data.

**Solutions**:

1. **Retry the operation**:
   - Temporary API glitch might resolve itself
   - Run NoEntropy again

2. **Check API key validity**:
   - API key might have been revoked
   - Create a new API key

3. **Clear cache and retry**:
   ```bash
   rm .noentropy_cache.json
   ./noentropy
   ```

4. **Report issue**:
   - If problem persists, it might be a bug
   - Check [GitHub Issues](https://github.com/glitchySid/noentropy/issues)

---

## File Operation Issues

### "Failed to move file" or "Permission denied"

**Problem**: Cannot move files to destination folder.

**Solutions**:

1. **Check file permissions**:
   ```bash
   ls -la /path/to/file
   ```

2. **Check destination permissions**:
   ```bash
   ls -la /path/to/Downloads
   ```

3. **Fix permissions**:
   ```bash
   chmod 644 /path/to/file  # For files
   chmod 755 /path/to/directory  # For directories
   ```

4. **Run as appropriate user**:
   - Don't run as root unless necessary
   - Ensure you own the files and folder

5. **Check disk space**:
   ```bash
   df -h /path/to/Downloads
   ```

---

### "File not found" or "Source file doesn't exist"

**Problem**: Files were deleted or moved before organization completed.

**Solutions**:

1. **Verify files exist**:
   ```bash
   ls -la /path/to/Downloads
   ```

2. **Check if another process is moving files**:
   - Browser might be cleaning up
   - Another organization tool might be running
   - Antivirus might have quarantined files

3. **Use dry-run first**:
   ```bash
   ./noentropy --dry-run
   ```
   - This ensures files exist before actual move

---

### "Destination already exists" or "File conflict"

**Problem**: A file with the same name already exists at destination.

**Solutions**:

1. **Current behavior**: NoEntropy will skip the file to prevent overwrites

2. **Manual resolution**:
   - Rename the existing file
   - Delete the existing file if not needed
   - Run organization again

3. **Check for duplicates**:
   - You might have duplicate downloads
   - Consider cleaning duplicates manually

---

## Cache Issues

### "Cache corrupted" or "Failed to read cache"

**Problem**: Cache file is corrupted or malformed.

**Solutions**:

1. **Delete cache file**:
   ```bash
   rm .noentropy_cache.json
   ```

2. **Run NoEntropy again**:
   - A new cache will be created automatically
   - Files will be analyzed fresh via API

3. **Prevent corruption**:
   - Don't manually edit cache file
   - Let NoEntropy manage cache automatically

---

### "Cache not working" or "Too many API calls"

**Problem**: Cache doesn't seem to reduce API calls.

**Diagnosis**:

Check if files are being modified:
```bash
ls -lt /path/to/Downloads | head
```

**Reasons cache might not work**:

1. **Files are constantly changing**:
   - If file modification time or size changes, cache is invalidated
   - This is expected behavior

2. **Cache expired**:
   - Cache entries older than 7 days are removed
   - Run organization more frequently

3. **Cache deleted**:
   - Check if `.noentropy_cache.json` exists in project root
   - If not, cache will rebuild on next run

**Solution**: This is often expected behavior, not a bug.

---

## Undo Issues

### "No completed moves to undo"

**Problem**: Cannot undo because no undo history exists.

**Reasons**:

1. **No files have been organized yet**
   - Run organization first

2. **All moves already undone**
   - You've already undone the previous organization

3. **Undo log was deleted**
   - Check if `~/.config/noentropy/data/undo_log.json` exists

4. **Undo log expired**
   - Entries older than 30 days are automatically removed

**Solution**: Run organization to create new undo history.

---

### "Undo log not found"

**Problem**: The undo log file doesn't exist.

**Solutions**:

1. **Create data directory**:
   ```bash
   mkdir -p ~/.config/noentropy/data
   ```

2. **Run organization**:
   - This will create the undo log automatically

---

### "Skipping [file] - source already exists"

**Problem**: During undo, a file already exists at the original location.

**This is a safety feature**:
- NoEntropy won't overwrite existing files
- Prevents data loss

**Solutions**:

1. **Manual inspection**:
   - Check both source and destination files
   - Determine which one to keep
   - Manually resolve the conflict

2. **Rename existing file**:
   - Move or rename the file at source location
   - Run undo again

---

### "Failed to restore [file]"

**Problem**: Unable to move file back to original location during undo.

**Common causes**:

1. **Permission issues**: See "Permission denied" section above
2. **File deleted**: Destination file was deleted after organization
3. **Disk space**: Not enough space at source location

**Solution**: Check logs for specific error, then address the underlying cause.

---

## Build and Development Issues

### "Rust not installed" or "cargo command not found"

**Problem**: Rust toolchain is not installed.

**Solution**:

Install Rust from official source:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then restart your terminal and try again.

---

### Build errors or compilation failures

**Problem**: Cannot build from source.

**Solutions**:

1. **Update Rust toolchain**:
   ```bash
   rustup update
   ```

2. **Clean build directory**:
   ```bash
   cargo clean
   cargo build --release
   ```

3. **Check Rust version**:
   ```bash
   rustc --version
   ```
   - Ensure you have Rust 2024 edition or later

4. **Check dependencies**:
   ```bash
   cargo update
   ```

---

## Platform-Specific Issues

### Linux/macOS: "Permission denied" when running binary

**Solution**:
```bash
chmod +x noentropy
```

---

### Windows: "Windows protected your PC" warning

**This is expected** for unsigned binaries.

**Solution**:
1. Click "More info"
2. Click "Run anyway"

Alternatively, build from source to avoid this warning.

---

### macOS: "noentropy cannot be opened because the developer cannot be verified"

**Solution**:

1. **First method** (Recommended):
   ```bash
   xattr -d com.apple.quarantine noentropy
   ```

2. **Alternative method**:
   - Right-click the binary
   - Select "Open"
   - Click "Open" in the dialog

---

## Getting More Help

If you've tried the solutions above and still have issues:

1. **Check GitHub Issues**: [https://github.com/glitchySid/noentropy/issues](https://github.com/glitchySid/noentropy/issues)
   - Someone might have reported the same issue
   - Solutions might already be available

2. **Create a new issue**:
   - Describe the problem clearly
   - Include error messages
   - Mention your OS and NoEntropy version
   - Describe steps to reproduce

3. **Join discussions**:
   - Check GitHub Discussions for community help
   - Share your use case and ask questions

## Debug Mode

For detailed error information, run with Rust backtrace:

```bash
RUST_BACKTRACE=1 ./noentropy
```

This provides detailed stack traces for debugging issues.

---

[Back to Main README](../README.md)
