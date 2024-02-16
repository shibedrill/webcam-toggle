extern crate winapi;

use std::io::stdin;

use winapi::{shared::cfg::DN_STARTED, um::cfgmgr32::{
    CM_Disable_DevNode, CM_Enable_DevNode, CM_Get_DevNode_Status, CM_Locate_DevNodeW, CR_SUCCESS
}};

fn main() {

    println!("");

    let device_instance_path = env!("DEVPATH");
    // Convert the path to a wide character string
    let mut device_instance_path_wide: Vec<u16> = device_instance_path
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    // Get the device node handle
    let mut dev_node_handle: u32 = 0;

    let result = unsafe {
        CM_Locate_DevNodeW(
            &mut dev_node_handle,
            device_instance_path_wide.as_mut_ptr(),
            0,
        )
    };
    if result != 0 {
        println!("Failed to locate device node.");
        return;
    } else {
        println!("Located device node.");
    }

    let mut dev_status: u32 = 0;
    let mut problem_num: u32 = 0;

    let get_status_ret = unsafe { CM_Get_DevNode_Status(&mut dev_status, &mut problem_num, dev_node_handle, 0) };
    println!("Status ret: {:x}", get_status_ret);
    println!("Dev status: {:x}", dev_status & DN_STARTED);

    let toggle_status: u32 = match dev_status & DN_STARTED {
        0 => {
            println!("Device is disabled. Enabling...");
            unsafe { CM_Enable_DevNode(dev_node_handle, 0) }
        }
        8 => {
            println!("Device is enabled. Disabling...");
            unsafe { CM_Disable_DevNode(dev_node_handle, 0) }
        }
        _ => panic!("Oh shit, unknown device status. Bailing. Sorry."),
    };
    match toggle_status {
        CR_SUCCESS => { println!("Successfully toggled device state."); },
        _ => { println!("Error: Enable/Disable returned {:x}.", toggle_status); },
    }

    unsafe { CM_Get_DevNode_Status(&mut dev_status, &mut problem_num, dev_node_handle, 0) };
    match dev_status & DN_STARTED {
        0 => {
            println!("Device is now disabled.");
        }
        8 => {
            println!("Device is now enabled.");
        }
        _ => panic!("Oh shit, unknown device status. Bailing. Sorry."),
    };

    println!("Press ENTER to exit...");
    let mut hold_for_input: String = "".to_string();
    stdin().read_line(&mut hold_for_input).unwrap();

}


