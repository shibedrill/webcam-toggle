extern crate winapi;

use std::io::stdin;

use winapi::{shared::cfg::DN_STARTED, um::cfgmgr32::*};

fn main() {

    let device_instance_path = env!("DEVPATH");

    println!();
    println!("{} v{} by {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    println!("Licensed under the {} license.", env!("CARGO_PKG_LICENSE"));
    println!("Compiled for devpath: {}", device_instance_path);
    println!();

    if device_instance_path = "USB\\VID_FFFF&PID_FFFF&MI_FF\\F&FFFFFFFF&F&FFFF" {
        panic!("You used the placeholder device path. Please try again.");
    }

    // Convert the path to a wide character string
    let mut device_instance_path_wide: Vec<u16> = device_instance_path
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    // Get the device node handle
    let mut dev_node_handle: u32 = 0;

    match unsafe {
        CM_Locate_DevNodeW(
            &mut dev_node_handle,
            device_instance_path_wide.as_mut_ptr(),
            0,
        )
    } {
        0 => println!("Device node located successfully."),
        _ => panic!("Unable to locate device node."),
    }

    let mut dev_status: u32 = 0;
    let mut problem_num: u32 = 0;

    unsafe { CM_Get_DevNode_Status(&mut dev_status, &mut problem_num, dev_node_handle, 0) };

    let device_original_status = dev_status & DN_STARTED;
    let toggle_status: u32 = match device_original_status {
        0 => {
            println!("Device is disabled. Enabling...");
            unsafe { CM_Enable_DevNode(dev_node_handle, 0) }
        }
        8 => {
            println!("Device is enabled. Disabling...");
            unsafe { CM_Disable_DevNode(dev_node_handle, 0) }
        }
        _ => panic!("Unknown device status. Bailing out, sorry."),
    };
    match toggle_status {
        CR_SUCCESS => println!("Device state toggle returned success."),
        CR_REMOVE_VETOED => println!("Known error: Device removal vetoed (device in use)."),
        CR_OUT_OF_MEMORY => println!("Known error: Out of memory, cannot perform operation."),
        CR_INVALID_DEVNODE => println!("Known error: Invalid device node."),
        CR_NOT_DISABLEABLE => println!("Known error: Device is not disable-able."),
        _ => println!("Unknown error: Enable/Disable returned {:x}.", toggle_status),
    }

    unsafe { CM_Get_DevNode_Status(&mut dev_status, &mut problem_num, dev_node_handle, 0) };
    match dev_status & DN_STARTED {
        0 => println!("Device is now disabled."),
        8 => println!("Device is now enabled."),
        _ => panic!("Unknown device status. Bailing out, sorry."),
    };
    match dev_status & DN_STARTED == device_original_status {
        true => println!("Oh no. Device status was not changed."),
        false => println!("Device status has changed. Have a nice day!"),
    }

    println!("Press ENTER to exit...");
    let mut hold_for_input: String = "".to_string();
    stdin().read_line(&mut hold_for_input).unwrap();

}


