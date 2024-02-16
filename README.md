# webcam-toggle v0.1.0

This program will toggle the enabled/disabled status of whichever device's Device Path you supply at compile-time. Two clicks, and your webcam is separated from the OS, instead of having to open Device Manager and navigate through all the menus.

## How to Use

1. Open Device Manager.
2. Copy the Device Instance Path of the webcam/device you want to control. e.g., `USB\VID_328F&PID_2013&MI_00\6&39FBEEC7&0&0000`.
3. Paste that into `.cargo/config.toml`, and add extra backslashes to escape the formatting.
4. Compile in release mode and place the executable on your desktop. (YOU CANNOT USE `cargo run` ON THIS PROJECT. IT REQUIRES ELEVATED PERMISSIONS TO RUN.)
5. Double-click the EXE, and click "allow". Your device's state is now toggled!