# Converted with chatgpt... i have no idea if this works
# Windows might block script execution. If so, run: Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
# Description
# Creates an entry in `events` folder with the date of the event

param(
    [string]$d,
    [switch]$help
)

# Get script directory (equivalent to dirname $0)
$HOME_DIR = Split-Path -Parent $MyInvocation.MyCommand.Path

if ($help) {
    Write-Output "Usage: log.ps1 [options]"
    Write-Output "options:"
    Write-Output "  -help        Show this help message and exit"
    Write-Output "  -d <date>    Set the date in YYYY-MM-DD format, defaults to today"
    exit 0
}

# Default date = today
$DATE = Get-Date -Format "yyyy-MM-dd"

if ($d) {
    # Validate format YYYY-MM-DD
    if ($d -notmatch '^\d{4}-\d{2}-\d{2}$') {
        Write-Error "Error: Date must be in YYYY-MM-DD format"
        exit 1
    }

    $DATE = $d
}

# Ensure folder exists
$eventsPath = Join-Path $HOME_DIR "events"
if (!(Test-Path $eventsPath)) {
    New-Item -ItemType Directory -Path $eventsPath | Out-Null
}

$filePath = Join-Path $eventsPath "entries.txt"

# Append date
Add-Content -Path $filePath -Value $DATE