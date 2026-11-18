# Secretarybird with battery tray icons

- The BlackShark V3 Pro and Viper V3 Pro each have a transparent, black battery ring.
- The center number and the filled arc show the latest available battery percentage.
- Hover over either icon for its device name, percentage, and charging status.
- Secretarybird reads both devices at startup and normally refreshes every 30 seconds. If the device stops reporting, its icon is hidden until the next successful read.
- The original tray menu, device editor, and profiles are still available. Razer Synapse is unnecessary for battery readings.

## Install on Windows

Download the `Secretarybird-custom-Windows` artifact from the latest successful run of **Custom Windows build** in this repository's **Actions** tab. Extract the downloaded ZIP and run `Secretarybird-Custom-Setup-0.6.0.exe`. Leave **Start Secretarybird with Windows** selected and launch Secretarybird when setup finishes. If Windows puts either icon under the taskbar's hidden-icons arrow, drag that icon onto the visible tray.

The installer uses the upstream installation ID to update an existing Secretarybird installation. The upstream updater can replace this build with the official version. To keep this build installed, turn off update notifications in Secretarybird and use the fork's installer for subsequent custom builds.

## Build

In this repository, open **Actions → Custom Windows build → Run workflow** and select `main`. The workflow runs the Windows Rust tests and builds the installer with Inno Setup.

The icon packs in `assets/` contain the multi-resolution battery-ring ICO files. `crates/secbird-tray/build.rs` expands them into Windows resources, with IDs 1000–1100 for the headset and 1200–1300 for the mouse.
