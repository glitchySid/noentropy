#!/usr/bin/env pwsh
<#
.SYNOPSIS
    NoEntropy Windows Uninstaller Script
.DESCRIPTION
    Completely removes NoEntropy from Windows including PATH configuration
.PARAMETER InstallPath
    Path where NoEntropy is installed (auto-detected if not specified)
.PARAMETER RemoveConfig
    Also remove configuration files
.EXAMPLE
    .\uninstall.ps1
    Uninstall from default location
.EXAMPLE
    .\uninstall.ps1 -InstallPath "C:\NoEntropy" -RemoveConfig
    Uninstall from custom path and remove config
#>

param(
    [string]$InstallPath = "",
    [switch]$RemoveConfig = $false
)

$ErrorActionPreference = "Stop"

# Color scheme
$Colors = @{
    Success = "Green"
    Info = "Cyan"
    Warning = "Yellow"
    Error = "Red"
    Reset = "White"
}

function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = $Colors.Reset
    )
    Write-Host $Message -ForegroundColor $Color
}

function Find-Installation {
    # Try to find noentropy.exe in PATH
    try {
        $inPath = Get-Command noentropy -ErrorAction SilentlyContinue
        if ($inPath) {
            return Split-Path $inPath.Source -Parent
        }
    }
    catch {
        # Not in PATH
    }
    
    # Check common installation locations
    $commonPaths = @(
        "C:\Program Files\NoEntropy",
        "C:\Program Files (x86)\NoEntropy",
        "$env:USERPROFILE\AppData\Local\NoEntropy",
        "$env:USERPROFILE\NoEntropy",
        "C:\NoEntropy"
    )
    
    foreach ($path in $commonPaths) {
        if (Test-Path (Join-Path $path "noentropy.exe")) {
            return $path
        }
    }
    
    return $null
}

function Test-AdminPrivileges {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Remove-FromPath {
    param([string]$InstallPath)
    
    Write-ColorOutput "Removing from PATH..." $Colors.Info
    
    # Remove from user PATH
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    $pathEntries = $userPath -split ';'
    $newUserPath = $pathEntries | Where-Object { $_ -ne $InstallPath -and $_.Trim() -ne "" }
    $newUserPath = $newUserPath -join ';'
    
    if ($userPath -ne $newUserPath) {
        [Environment]::SetEnvironmentVariable("PATH", $newUserPath, "User")
        Write-ColorOutput "Removed from user PATH." $Colors.Success
    }
    
    # Remove from system PATH (requires admin)
    if (Test-AdminPrivileges) {
        try {
            $systemPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
            $pathEntries = $systemPath -split ';'
            $newSystemPath = $pathEntries | Where-Object { $_ -ne $InstallPath -and $_.Trim() -ne "" }
            $newSystemPath = $newSystemPath -join ';'
            
            if ($systemPath -ne $newSystemPath) {
                [Environment]::SetEnvironmentVariable("PATH", $newSystemPath, "Machine")
                Write-ColorOutput "Removed from system PATH." $Colors.Success
            }
        }
        catch {
            Write-ColorOutput "Could not modify system PATH (may require admin privileges)." $Colors.Warning
        }
    }
    
    # Update current session
    $env:PATH = ($env:PATH -split ';' | Where-Object { $_ -ne $InstallPath }) -join ';'
}

function Remove-InstallationFiles {
    param([string]$InstallPath)
    
    if (Test-Path $InstallPath) {
        Write-ColorOutput "Removing installation directory: $InstallPath" $Colors.Info
        
        try {
            Remove-Item $InstallPath -Recurse -Force
            Write-ColorOutput "Installation files removed." $Colors.Success
        }
        catch {
            Write-ColorOutput "Failed to remove some files: $_" $Colors.Warning
            Write-ColorOutput "You may need to manually remove: $InstallPath" $Colors.Warning
        }
    }
    else {
        Write-ColorOutput "Installation directory not found: $InstallPath" $Colors.Warning
    }
}

function Remove-Configuration {
    Write-ColorOutput "Removing configuration files..." $Colors.Info
    
    $configPaths = @(
        "$env:USERPROFILE\.config\noentropy",
        "$env:APPDATA\NoEntropy",
        "$env:LOCALAPPDATA\NoEntropy"
    )
    
    foreach ($configPath in $configPaths) {
        if (Test-Path $configPath) {
            try {
                Remove-Item $configPath -Recurse -Force
                Write-ColorOutput "Removed: $configPath" $Colors.Success
            }
            catch {
                Write-ColorOutput "Could not remove: $configPath" $Colors.Warning
            }
        }
    }
}

function Remove-Shortcuts {
    Write-ColorOutput "Removing shortcuts..." $Colors.Info
    
    $shortcutPaths = @(
        "$env:USERPROFILE\Desktop\NoEntropy.lnk",
        "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\NoEntropy.lnk"
    )
    
    foreach ($shortcutPath in $shortcutPaths) {
        if (Test-Path $shortcutPath) {
            try {
                Remove-Item $shortcutPath -Force
                Write-ColorOutput "Removed shortcut: $shortcutPath" $Colors.Success
            }
            catch {
                Write-ColorOutput "Could not remove shortcut: $shortcutPath" $Colors.Warning
            }
        }
    }
}

function Test-Uninstallation {
    # Check if noentropy is still available
    try {
        $stillExists = Get-Command noentropy -ErrorAction SilentlyContinue
        if ($stillExists) {
            Write-ColorOutput "Warning: 'noentropy' command is still available in PATH" $Colors.Warning
            Write-ColorOutput "You may need to restart your terminal for changes to take effect." $Colors.Info
            return $false
        }
        else {
            Write-ColorOutput "NoEntropy successfully removed from PATH." $Colors.Success
            return $true
        }
    }
    catch {
        Write-ColorOutput "NoEntropy successfully removed from PATH." $Colors.Success
        return $true
    }
}

function Main {
    Write-ColorOutput "`n╔══════════════════════════════════════════════════════════════╗" $Colors.Cyan
    Write-ColorOutput "║                  NoEntropy Windows Uninstaller                ║" $Colors.Cyan
    Write-ColorOutput "╚══════════════════════════════════════════════════════════════╝" $Colors.Cyan
    Write-ColorOutput "`n" $Colors.Reset
    
    # Find installation
    if ([string]::IsNullOrEmpty($InstallPath)) {
        $InstallPath = Find-Installation
        if ([string]::IsNullOrEmpty($InstallPath)) {
            Write-ColorOutput "NoEntropy installation not found." $Colors.Warning
            Write-ColorOutput "Please specify the installation path with -InstallPath" $Colors.Info
            exit 1
        }
    }
    
    Write-ColorOutput "Found installation at: $InstallPath" $Colors.Info
    
    # Confirm uninstallation
    $choice = Read-Host "Are you sure you want to uninstall NoEntropy? (y/N)"
    if ($choice -notmatch '^[Yy]') {
        Write-ColorOutput "Uninstallation cancelled." $Colors.Warning
        exit 0
    }
    
    # Remove from PATH
    Remove-FromPath -InstallPath $InstallPath
    
    # Remove installation files
    Remove-InstallationFiles -InstallPath $InstallPath
    
    # Remove configuration if requested
    if ($RemoveConfig) {
        Remove-Configuration
    }
    
    # Remove shortcuts
    Remove-Shortcuts
    
    # Test uninstallation
    Write-ColorOutput "`nTesting uninstallation..." $Colors.Info
    if (Test-Uninstallation) {
        Write-ColorOutput "`n╔══════════════════════════════════════════════════════════════╗" $Colors.Success
        Write-ColorOutput "║              NoEntropy uninstalled successfully!              ║" $Colors.Success
        Write-ColorOutput "╚══════════════════════════════════════════════════════════════╝" $Colors.Success
        Write-ColorOutput "`nPlease restart your terminal to complete the removal." $Colors.Info
    }
    else {
        Write-ColorOutput "`nUninstallation completed with warnings." $Colors.Warning
        Write-ColorOutput "Please restart your terminal and check if 'noentropy' command is still available." $Colors.Info
    }
}

# Run uninstallation
try {
    Main
    exit 0
}
catch {
    Write-ColorOutput "`nUninstallation failed with error:" $Colors.Error
    Write-ColorOutput $_ $Colors.Error
    exit 1
}