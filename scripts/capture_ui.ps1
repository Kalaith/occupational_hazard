<#
.SYNOPSIS
    Screenshot harness for Occupational Hazard's title and game screen.

.DESCRIPTION
    Thin wrapper around the shared macroquad-toolkit capture script. Builds the
    debug exe and drives it through the env-var capture hook
    (OCCUPATIONAL_HAZARD_CAPTURE_*) provided by macroquad_toolkit::capture in
    src/main.rs. Each scene resets its state and hides transient UI state so
    captures do not depend on the developer's existing saves.

.EXAMPLE
    ./scripts/capture_ui.ps1
    ./scripts/capture_ui.ps1 -Frames 60 -SkipBuild
#>
param(
    [ValidateSet("title", "gameplay", "settings", "promotion", "report", "victory")]
    [string[]]$Scenes = @("title", "gameplay", "settings", "promotion", "report", "victory"),
    [int]$Frames = 150,
    [string]$OutputDir = "docs\verification",
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$gameDir = Split-Path -Parent $PSScriptRoot
$shared = Join-Path (Split-Path -Parent $gameDir) "macroquad-toolkit\scripts\capture_ui.ps1"

# Keep all deterministic verification screens directly in docs/verification.
& $shared -GameDir $gameDir -Prefix "OCCUPATIONAL_HAZARD" -Scenes $Scenes -Frames $Frames -OutputDir $OutputDir -MinBytes 10000 -SkipBuild:$SkipBuild

