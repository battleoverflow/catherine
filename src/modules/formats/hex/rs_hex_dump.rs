/*
    Project: Catherine Framework (https://github.com/azazelm3dj3d/catherine)
    Author: azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
*/

use std::{
    path::Path,
    os::raw::c_char,
    ffi::CString,
    str, env
};

use mercy::hex;
use libloading::{ Library, Symbol };
use serde_json::Value;

fn access_c_lib(convert_file: &str) {
    if Path::new(convert_file).exists() {
        let json_file: &str = "catherine.json";

        let json_parse = {
            // Load the JSON file and convert to an easier to read format
            let json_convert = std::fs::read_to_string(&json_file).expect("Unable to locate file");
            serde_json::from_str::<Value>(&json_convert).unwrap()
        };

        let filepath = &json_parse["config"]["c_hex_dump_filepath"];

        // Being precautious - don't want to even initialize an unsafe ability if the file doesn't exist
        unsafe {
            // Handles the pointer assignment
            let c_to_print = CString::new(convert_file).expect("Unable to access CString value");
    
            if env::consts::OS == "linux" {
                // Sets the shared object
                let lib = { Library::new(filepath.to_string()).unwrap() };
                
                // Grabs the C function we need to call
                let call_c_lib: Symbol<unsafe extern fn(filename: *const c_char) -> *const c_char> = lib.get("collect_hex\0".as_bytes()).unwrap();

                // Converts the function parameter call to a pointer reference
                call_c_lib(c_to_print.as_ptr());
            } else {
                println!("Your operating system is not supported yet");
            }
        }
    } else {
        println!("[access_c_lib] Unable to locate file");
    }
}

pub fn collect_hex(option: &str, convert_file: &str) {
    if option == "get_data_dump" {
        hex("hex_dump", convert_file);
    } else if option == "access_c_lib" {
        access_c_lib(convert_file);
    } else {
        println!("Unrecognized function call");
    }
}
