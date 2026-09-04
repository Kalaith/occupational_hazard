# Clean game shell verification

- The asset registry and source size tests pass. Tests for the removed demo systems were removed with those systems.
- Formatting and Clippy with warnings denied pass.
- Title, empty workspace, and settings screenshots were captured and visually inspected at 1280x720.
- The intentionally empty workspace compresses to about 20 KB; the project capture wrapper uses a 10 KB threshold instead of the shared 40 KB demo threshold.
- The generated browser page contains `body class="viewport-game"`; shared CSS gives its canvas the full viewport and hides external page chrome.
- Native configuration opens fullscreen outside screenshot capture. UI layout uses the current screen dimensions.
- No-argument `publish.ps1` builds and packages Windows/WebGL and deploys the local preview with exit code 0. Optional Project Roost tracking still warns that localhost:80 is unavailable.
- The GDD is unchanged. Existing on-disk demo saves are not deleted; the game no longer loads them.
