# webcam-toggle v0.2.0

<<<<<<< HEAD
This program acts as a software device killswitch for devices on Windows. It will toggle the enabled/disabled status of whichever device's Device Path you supply at compile-time. Two clicks, and your webcam (or other device) is separated from the OS, instead of having to open Device Manager and navigate through all the menus. Useful for turning off your webcam while it's not in use. Some processes can veto the disablement of the device, so depending on the process that's using it, you might not be able to disable it.
=======
This program acts as a software killswitch for devices on Windows. It will toggle the enabled/disabled status of whichever device's Device Path you supply at compile-time, as long as the device is possible to Disable/Enable in Device Manager. Two clicks, and your webcam (or other device) is separated from the OS, instead of having to open Device Manager and navigate through all the menus. Useful for when your webcam LED turns on out of the blue and you don't know what the hell is using it, but you want it to... *not* be doing that.
>>>>>>> 84b488dc092dc5abcb9195e0d29bcb89790598c8

## How to Use

1. Open Device Manager.
2. Copy the Device Instance Path of the webcam/device you want to control. e.g., `USB\VID_328F&PID_2013&MI_00\6&39FBEEC7&0&0000`. This device must be able to be Disabled/Enabled through Device Manager.
3. Paste that into `.cargo/config.toml`, add double quotes on either side, and add extra backslashes to escape the formatting.
4. Compile in release mode using `cargo build --release` and place the executable on your desktop. (YOU CANNOT USE `cargo run` ON THIS PROJECT. It will fail, since the EXE requires elevated permissions.)
5. Double-click the EXE, and click "Yes". Your device's state is now toggled!
6. Double-click the EXE again, and click "Yes" again. The device is now in its original state.

## Errata

<<<<<<< HEAD
- I don't believe the API calls I'm using properly "notify" Device Manager that the device's state has been changed. If you use this program to disable a device, and then attempt to re-enable it in Device Manager, it does not work. This can be resolved either by running the program again, or by *disabling* the device in Device Manager before attempting to re-enable it.
- This program will be unable to disable a device if the disable call gets vetoed by a running process, so some processes will prevent the device from being disabled, while others will not. Future iterations might include the ability to identify and possibly even kill whichever process is using the device.
=======
I don't believe the API calls I'm using properly "notify" Device Manager that the device's state has been changed. If you use this program to disable a device, and then attempt to re-enable it in Device Manager, it does not work. This can be resolved either by running the program again, or by *disabling* the device in Device Manager before attempting to re-enable it.
>>>>>>> 84b488dc092dc5abcb9195e0d29bcb89790598c8
