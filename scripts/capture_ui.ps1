<#
.SYNOPSIS
    Screenshot harness for Occupational Hazard's title and sandbox.

.DESCRIPTION
    Thin wrapper around the shared macroquad-toolkit capture script. Builds the
    debug exe and drives it through the env-var capture hook
    (OCCUPATIONAL_HAZARD_CAPTURE_*) provided by macroquad_toolkit::capture in
    src/main.rs. Each scene resets its state and hides local save metadata so
    captures do not depend on the developer's existing saves.

.EXAMPLE
    ./scripts/capture_ui.ps1
    ./scripts/capture_ui.ps1 -Frames 60 -SkipBuild
#>
param(
    [ValidateSet("title", "gameplay")]
    [string[]]$Scenes = @("title", "gameplay"),
    [int]$Frames = 150,
    [string]$OutputDir = "docs\verification",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$shared = Join-Path (Split-Path -Parent $gameDir) "macroquad-toolkit\scripts\capture_ui.ps1"

& $shared -GameDir $gameDir -Prefix "OCCUPATIONAL_HAZARD" -Scenes $Scenes -Frames $Frames -OutputDir $OutputDir -SkipBuild:$SkipBuild
