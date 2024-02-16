# webcam-toggle v0.1.0

This program acts as a software device killswitch for devices on Windows. It will toggle the enabled/disabled status of whichever device's Device Path you supply at compile-time. Two clicks, and your webcam (or other device) is separated from the OS, instead of having to open Device Manager and navigate through all the menus. Useful for when your webcam LED turns on out of the blue and you don't know what the hell is using it, but you want it to... *not* be doing that.

## How to Use

1. Open Device Manager.
2. Copy the Device Instance Path of the webcam/device you want to control. e.g., `USB\VID_328F&PID_2013&MI_00\6&39FBEEC7&0&0000`.
3. Paste that into `.cargo/config.toml`, add double quotes on either side, and add extra backslashes to escape the formatting.
4. Compile in release mode using `cargo build --release` and place the executable on your desktop. (YOU CANNOT USE `cargo run` ON THIS PROJECT. It will fail, since the EXE requires elevated permissions.)
5. Double-click the EXE, and click "allow". Your device's state is now toggled!
6. Double-click the EXE again, and click "allow" again. The device is now in its original state.
