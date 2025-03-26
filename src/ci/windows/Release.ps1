Param(
    [Parameter()]
    [string]$Cargo = "cargo",

    [Parameter()]
    [string]$BuildFlags = ""
)

$ErrorActionPreference = "Stop"
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force

. "$PSScriptRoot\..\_shared.ps1"

function Main {
    Param(
        [Parameter()]
        [string]$Binary
    )

    if (![System.Environment]::Is64BitOperatingSystem) {
        Write-Error ">>> \`charted\` is not supported on 32-bit systems"
        Exit 1
    }

    StartGroup "Build / Windows (x64)"

    $CargoFlags = [System.Environment]::GetEnvironmentVariable("CARGOFLAGS") || ""

    # Create the `.result` directory in the root project
    # so we can put our files in there.
    New-Item -Path . -Name ".result" -ItemType Directory

    Write-Host "$ $Cargo $CargoFlags build --release --locked --bin $Binary $BuildFlags"
    Invoke-Expression "$Cargo $CargoFlags build --release --locked --bin $Binary $BuildFlags"

    Write-Host "$ mv ./target/release/$Binary.exe -> .result/$Binary-windows-x86_64.exe"
    Move-Item -Path "./target/release/$Binary.exe" ".result/$Binary-windows-x86_64.exe"

    Push-Location ".result"

    Write-Host "$ Compute checksum of binary"

    $Hash = (Get-FileHash -Path "$Binary-windows-x86_64.exe").Hash.ToLower()
    Write-Output "$Hash  $Binary-windows-x86_64.exe" | Out-File "$Binary-windows-x86_64.exe.sha256"

    Pop-Location

    Write-Host "$ Completed every task. All resources are in .result!"

    EndGroup
}

Main "charted-helm-plugin"
