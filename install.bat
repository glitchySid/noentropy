@echo off
setlocal enabledelayedexpansion

REM NoEntropy Windows Batch Installer
REM Compatible with older Windows systems

echo.
echo ╔══════════════════════════════════════════════════════════════╗
echo ║                NoEntropy Windows Installer                  ║
echo ╚══════════════════════════════════════════════════════════════╝
echo.

REM Default settings
set "VERSION=LATEST"
set "INSTALL_PATH=C:\Program Files\NoEntropy"
set "FORCE=0"

REM Parse command line arguments
if "%~1"=="/?" goto :help
if "%~1"=="-?" goto :help
if "%~1"=="--help" goto :help

:parse_args
if "%~1"=="" goto :args_done
if /i "%~1"=="-version" (
    set "VERSION=%~2"
    shift
    shift
    goto :parse_args
)
if /i "%~1"=="-path" (
    set "INSTALL_PATH=%~2"
    shift
    shift
    goto :parse_args
)
if /i "%~1"=="-force" (
    set "FORCE=1"
    shift
    goto :parse_args
)
shift
goto :parse_args

:args_done

REM Check if running as administrator for Program Files installation
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo [WARNING] Administrator privileges not detected.
    if "%INSTALL_PATH%"=="C:\Program Files\NoEntropy" (
        echo [INFO] Either run as Administrator or use -path to specify user directory
        echo [INFO] Example: install.bat -path "C:\NoEntropy"
        pause
        exit /b 1
    )
)

echo [INFO] Installing NoEntropy...
echo [INFO] Version: %VERSION%
echo [INFO] Path: %INSTALL_PATH%
echo.

REM Check for existing installation
if exist "%INSTALL_PATH%\noentropy.exe" (
    echo [WARNING] NoEntropy already installed at: %INSTALL_PATH%
    if %FORCE%==0 (
        set /p "choice=Do you want to overwrite? (y/N): "
        if /i not "!choice!"=="y" (
            echo [INFO] Installation cancelled.
            pause
            exit /b 0
        )
    )
    echo [INFO] Removing existing installation...
    rmdir /s /q "%INSTALL_PATH%" >nul 2>&1
)

REM Determine download URL
if "%VERSION%"=="LATEST" (
    set "API_URL=https://api.github.com/repos/glitchySid/noentropy/releases/latest"
) else (
    set "API_URL=https://api.github.com/repos/glitchySid/noentropy/releases/tags/v%VERSION%"
)

echo [INFO] Fetching release information...

REM Use PowerShell to fetch JSON since CMD has no built-in HTTP client
set "PS_SCRIPT=powershell -Command "& {try { $response = Invoke-RestMethod -Uri '%API_URL%' -Headers @{'User-Agent'='NoEntropy-Installer'}; $asset = $response.assets | Where-Object { $_.name -like '*windows*' -or $_.name -like '*pc-windows-msvc*' }; if ($asset) { Write-Output $asset.browser_download_url; Write-Output $asset.name } else { Write-Error 'No Windows binary found'; exit 1 } } catch { Write-Error $_.Exception.Message; exit 1 } }""

for /f "tokens=1,2 delims=`n" %%a in ('%PS_SCRIPT%') do (
    set "DOWNLOAD_URL=%%a"
    set "FILENAME=%%b"
)

if "%DOWNLOAD_URL%"=="" (
    echo [ERROR] Failed to get download URL or no Windows binary found
    pause
    exit /b 1
)

echo [INFO] Downloading: %FILENAME%

REM Download using PowerShell
powershell -Command "& { try { Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%TEMP%\%FILENAME%' -ErrorAction Stop; Write-Host 'Download completed' } catch { Write-Error $_.Exception.Message; exit 1 } }"

if %errorlevel% neq 0 (
    echo [ERROR] Download failed
    pause
    exit /b 1
)

REM Create installation directory
if not exist "%INSTALL_PATH%" (
    echo [INFO] Creating installation directory: %INSTALL_PATH%
    mkdir "%INSTALL_PATH%" >nul 2>&1
    if %errorlevel% neq 0 (
        echo [ERROR] Failed to create installation directory
        pause
        exit /b 1
    )
)

REM Extract if ZIP file
if "%FILENAME:~-4%"==".zip" (
    echo [INFO] Extracting archive...
    set "TEMP_EXTRACT=%TEMP%\noentropy_extract"
    
    REM Clean up previous extraction
    if exist "%TEMP_EXTRACT%" rmdir /s /q "%TEMP_EXTRACT%" >nul 2>&1
    mkdir "%TEMP_EXTRACT%" >nul 2>&1
    
    REM Extract using PowerShell
    powershell -Command "& { try { Expand-Archive -Path '%TEMP%\%FILENAME%' -DestinationPath '%TEMP_EXTRACT%' -Force; Write-Host 'Extraction completed' } catch { Write-Error $_.Exception.Message; exit 1 } }"
    
    if %errorlevel% neq 0 (
        echo [ERROR] Extraction failed
        pause
        exit /b 1
    )
    
    REM Find the executable
    for /r "%TEMP_EXTRACT%" %%f in (noentropy.exe) do (
        set "SOURCE_EXE=%%f"
        goto :found_exe
    )
    
    :found_exe
    if "!SOURCE_EXE!"=="" (
        echo [ERROR] noentropy.exe not found in archive
        pause
        exit /b 1
    )
    
    REM Copy to installation directory
    echo [INFO] Installing to: %INSTALL_PATH%\noentropy.exe
    copy "!SOURCE_EXE!" "%INSTALL_PATH%\noentropy.exe" >nul 2>&1
    
    REM Cleanup
    rmdir /s /q "%TEMP_EXTRACT%" >nul 2>&1
) else (
    REM Direct executable
    echo [INFO] Installing to: %INSTALL_PATH%\noentropy.exe
    copy "%TEMP%\%FILENAME%" "%INSTALL_PATH%\noentropy.exe" >nul 2>&1
)

REM Cleanup download
del "%TEMP%\%FILENAME%" >nul 2>&1

if %errorlevel% neq 0 (
    echo [ERROR] Installation failed
    pause
    exit /b 1
)

echo [INFO] Installation completed!

REM Add to PATH
echo [INFO] Adding to PATH...
set "CURRENT_PATH=%PATH%"
echo %CURRENT_PATH% | findstr /i /c:"%INSTALL_PATH%" >nul
if %errorlevel% neq 0 (
    REM Add to user PATH using setx
    setx PATH "%CURRENT_PATH%;%INSTALL_PATH%" >nul 2>&1
    if %errorlevel% equ 0 (
        echo [SUCCESS] Added to user PATH
        echo [INFO] Please restart your terminal for changes to take effect
    ) else (
        echo [WARNING] Failed to add to PATH automatically
        echo [INFO] Please add %INSTALL_PATH% to your PATH manually
    )
) else (
    echo [INFO] Already in PATH
)

REM Create uninstaller
echo [INFO] Creating uninstaller...
set "UNINSTALLER_SCRIPT=@echo off
echo Uninstalling NoEntropy...
echo.

REM Remove from PATH
set "CURRENT_PATH=%%PATH%%"
set "INSTALL_PATH=%INSTALL_PATH%"
set "NEW_PATH=%%CURRENT_PATH:;%%INSTALL_PATH%%=%%"
setx PATH "%%NEW_PATH%%" ^>nul 2^>^&1

REM Remove installation directory
if exist "%%INSTALL_PATH%%" (
    rmdir /s /q "%%INSTALL_PATH%%"
    echo Removed installation directory.
)

echo.
echo NoEntropy uninstalled successfully!
echo Please restart your terminal to complete the removal.
pause"

echo %UNINSTALLER_SCRIPT% > "%INSTALL_PATH%\uninstall.bat"

REM Test installation
echo.
echo [INFO] Testing installation...
"%INSTALL_PATH%\noentropy.exe" --version >nul 2>&1
if %errorlevel% equ 0 (
    echo.
    echo ══════════════════════════════════════════════════════════════
    echo [SUCCESS] NoEntropy installed successfully!
    echo [INFO] You can now run 'noentropy' from any terminal.
    echo [INFO] To uninstall, run: %INSTALL_PATH%\uninstall.bat
    echo ══════════════════════════════════════════════════════════════
) else (
    echo [ERROR] Installation test failed
    pause
    exit /b 1
)

echo.
pause
exit /b 0

:help
echo.
echo NoEntropy Windows Installer
echo.
echo Usage: install.bat [options]
echo.
echo Options:
echo   -version VERSION    Install specific version (default: latest)
echo   -path PATH          Custom installation path
echo   -force              Overwrite existing installation
echo   -help, -?, /?       Show this help message
echo.
echo Examples:
echo   install.bat
echo   install.bat -version 1.0.4
echo   install.bat -path "C:\NoEntropy"
echo   install.bat -version 1.0.4 -path "C:\NoEntropy" -force
echo.
pause
exit /b 0