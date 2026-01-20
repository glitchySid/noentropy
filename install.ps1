#!/usr/bin/env pwsh
<#
.SYNOPSIS
    NoEntropy Windows Installer Script
.DESCRIPTION
    Automatically downloads and installs NoEntropy on Windows with PATH configuration
.PARAMETER Version
    Specific version to install (defaults to latest)
.PARAMETER InstallPath
    Custom installation path (defaults to Program Files)
.PARAMETER Force
    Overwrite existing installation
.EXAMPLE
    .\install.ps1
    Install latest version to default location
.EXAMPLE
    .\install.ps1 -Version "1.0.4" -InstallPath "C:\NoEntropy"
    Install specific version to custom path
#>

param(
    [string]$Version = "",
    [string]$InstallPath = "",
    [switch]$Force = $false
)

# Enhanced error handling
$ErrorActionPreference = "Stop"

# Color scheme for output
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

function Test-AdminPrivileges {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Get-LatestVersion {
    try {
        Write-ColorOutput "Fetching latest version information..." $Colors.Info
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/glitchySid/noentropy/releases/latest" -Headers @{
            "User-Agent" = "NoEntropy-Installer"
        }
        return $response
    }
    catch {
        Write-ColorOutput "Failed to fetch latest version: $_" $Colors.Error
        exit 1
    }
}

function Get-SpecificVersion {
    param([string]$Version)
    try {
        Write-ColorOutput "Fetching version $Version information..." $Colors.Info
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/glitchySid/noentropy/releases/tags/v$Version" -Headers @{
            "User-Agent" = "NoEntropy-Installer"
        }
        return $response
    }
    catch {
        Write-ColorOutput "Failed to fetch version $Version: $_" $Colors.Error
        exit 1
    }
}

function Test-Installation {
    param([string]$Path)
    
    $noentropyExe = Join-Path $Path "noentropy.exe"
    
    if (Test-Path $noentropyExe) {
        try {
            $versionOutput = & $noentropyExe --version 2>$null
            Write-ColorOutput "NoEntropy is already installed at: $Path" $Colors.Warning
            Write-ColorOutput "Current version: $versionOutput" $Colors.Info
            
            if (-not $Force) {
                $choice = Read-Host "Do you want to overwrite? (y/N)"
                if ($choice -notmatch '^[Yy]') {
                    Write-ColorOutput "Installation cancelled." $Colors.Warning
                    exit 0
                }
            }
            return $true
        }
        catch {
            # Installation exists but broken, continue
            Write-ColorOutput "Existing installation appears broken, continuing..." $Colors.Warning
        }
    }
    return $false
}

function Install-NoEntropy {
    param(
        [object]$ReleaseInfo,
        [string]$InstallPath
    )
    
    # Find Windows asset
    $windowsAsset = $ReleaseInfo.assets | Where-Object { $_.name -like "*windows*" -or $_.name -like "*pc-windows-msvc*" }
    
    if (-not $windowsAsset) {
        Write-ColorOutput "No Windows binary found in release!" $Colors.Error
        exit 1
    }
    
    Write-ColorOutput "Downloading: $($windowsAsset.name)" $Colors.Info
    $tempFile = Join-Path $env:TEMP $windowsAsset.name
    
    try {
        Invoke-WebRequest -Uri $windowsAsset.browser_download_url -OutFile $tempFile
        Write-ColorOutput "Download completed successfully." $Colors.Success
    }
    catch {
        Write-ColorOutput "Download failed: $_" $Colors.Error
        exit 1
    }
    
    # Extract if it's a zip
    if ($windowsAsset.name -like "*.zip") {
        Write-ColorOutput "Extracting archive..." $Colors.Info
        $tempExtract = Join-Path $env:TEMP "noentropy_extract"
        
        if (Test-Path $tempExtract) {
            Remove-Item $tempExtract -Recurse -Force
        }
        
        New-Item -ItemType Directory -Path $tempExtract -Force | Out-Null
        Expand-Archive -Path $tempFile -DestinationPath $tempExtract -Force
        
        # Find the executable
        $exeFile = Get-ChildItem -Path $tempExtract -Name "noentropy.exe" -Recurse | Select-Object -First 1
        if (-not $exeFile) {
            Write-ColorOutput "noentropy.exe not found in archive!" $Colors.Error
            exit 1
        }
        
        $sourceExe = Join-Path $tempExtract $exeFile
    }
    else {
        $sourceExe = $tempFile
    }
    
    # Create installation directory
    if (-not (Test-Path $InstallPath)) {
        Write-ColorOutput "Creating installation directory: $InstallPath" $Colors.Info
        New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    }
    
    # Copy executable
    $targetExe = Join-Path $InstallPath "noentropy.exe"
    Write-ColorOutput "Installing to: $targetExe" $Colors.Info
    Copy-Item -Path $sourceExe -Destination $targetExe -Force
    
    # Cleanup
    if (Test-Path $tempExtract) {
        Remove-Item $tempExtract -Recurse -Force
    }
    Remove-Item $tempFile -Force
    
    Write-ColorOutput "Installation completed!" $Colors.Success
}

function Add-ToPath {
    param([string]$InstallPath)
    
    Write-ColorOutput "Adding to PATH..." $Colors.Info
    
    # Get current PATH
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    
    # Check if already in PATH
    if ($currentPath -split ';' -contains $InstallPath) {
        Write-ColorOutput "Path already exists in user PATH." $Colors.Info
        return
    }
    
    # Add to PATH
    $newPath = "$currentPath;$InstallPath"
    [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
    
    # Also update current session
    $env:PATH = "$env:PATH;$InstallPath"
    
    Write-ColorOutput "Added to user PATH successfully." $Colors.Success
    Write-ColorOutput "Note: You may need to restart your terminal for changes to take effect." $Colors.Warning
}

function Create-Uninstaller {
    param([string]$InstallPath)
    
    $uninstallerScript = @"
@echo off
echo Uninstalling NoEntropy...
echo.

REM Remove from PATH
set "CURRENT_PATH=%PATH%"
set "INSTALL_PATH=$InstallPath"
set "NEW_PATH=%CURRENT_PATH:;%INSTALL_PATH%=%"
setx PATH "%NEW_PATH%" >nul 2>&1

REM Remove installation directory
if exist "%INSTALL_PATH%" (
    rmdir /s /q "%INSTALL_PATH%"
    echo Removed installation directory.
)

echo.
echo NoEntropy uninstalled successfully!
echo Please restart your terminal to complete the removal.
pause
"@
    
    $uninstallerPath = Join-Path $InstallPath "uninstall.bat"
    $uninstallerScript | Out-File -FilePath $uninstallerPath -Encoding ASCII
    Write-ColorOutput "Created uninstaller at: $uninstallerPath" $Colors.Info
}

function Test-InstallationSuccess {
    param([string]$InstallPath)
    
    $noentropyExe = Join-Path $InstallPath "noentropy.exe"
    
    if (Test-Path $noentropyExe) {
        try {
            Write-ColorOutput "`n" $Colors.Reset
            Write-ColorOutput "Testing installation..." $Colors.Info
            $versionOutput = & $noentropyExe --version 2>$null
            Write-ColorOutput "✓ NoEntropy installed successfully!" $Colors.Success
            Write-ColorOutput "✓ Version: $versionOutput" $Colors.Info
            Write-ColorOutput "✓ Location: $InstallPath" $Colors.Info
            Write-ColorOutput "`nYou can now run 'noentropy' from any terminal." $Colors.Success
            return $true
        }
        catch {
            Write-ColorOutput "Installation test failed: $_" $Colors.Error
            return $false
        }
    }
    else {
        Write-ColorOutput "Installation failed - executable not found!" $Colors.Error
        return $false
    }
}

# Main installation logic
function Main {
    Write-ColorOutput "`n╔══════════════════════════════════════════════════════════════╗" $Colors.Cyan
    Write-ColorOutput "║                    NoEntropy Windows Installer                 ║" $Colors.Cyan
    Write-ColorOutput "╚══════════════════════════════════════════════════════════════╝" $Colors.Cyan
    Write-ColorOutput "`n" $Colors.Reset
    
    # Check if running with appropriate privileges
    $isAdmin = Test-AdminPrivileges
    $defaultInstallPath = "C:\Program Files\NoEntropy"
    
    if (-not $isAdmin -and (Test-Path $defaultInstallPath)) {
        Write-ColorOutput "Administrator privileges required for installation to Program Files." $Colors.Warning
        Write-ColorOutput "Either:" $Colors.Info
        Write-ColorOutput "1. Run this script as Administrator" $Colors.Info
        Write-ColorOutput "2. Use -InstallPath to specify a user directory (e.g., 'C:\NoEntropy')" $Colors.Info
        exit 1
    }
    
    # Set installation path
    if ([string]::IsNullOrEmpty($InstallPath)) {
        $InstallPath = $defaultInstallPath
    }
    
    # Get version information
    if ([string]::IsNullOrEmpty($Version)) {
        $releaseInfo = Get-LatestVersion
        $Version = $releaseInfo.tag_name.TrimStart('v')
        Write-ColorOutput "Installing latest version: $Version" $Colors.Info
    }
    else {
        $releaseInfo = Get-SpecificVersion -Version $Version
        Write-ColorOutput "Installing version: $Version" $Colors.Info
    }
    
    # Check existing installation
    Test-Installation -Path $InstallPath
    
    # Install NoEntropy
    Install-NoEntropy -ReleaseInfo $releaseInfo -InstallPath $InstallPath
    
    # Add to PATH
    Add-ToPath -InstallPath $InstallPath
    
    # Create uninstaller
    Create-Uninstaller -InstallPath $InstallPath
    
    # Test installation
    if (Test-InstallationSuccess -InstallPath $InstallPath) {
        Write-ColorOutput "`nInstallation completed successfully!" $Colors.Success
        Write-ColorOutput "To uninstall, run: $InstallPath\uninstall.bat" $Colors.Info
    }
    else {
        Write-ColorOutput "`nInstallation completed with warnings. Please check the output above." $Colors.Warning
        exit 1
    }
}

# Run installation
try {
    Main
    exit 0
}
catch {
    Write-ColorOutput "`nInstallation failed with error:" $Colors.Error
    Write-ColorOutput $_ $Colors.Error
    exit 1
}