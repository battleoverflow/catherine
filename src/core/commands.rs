/*
    Project: Catherine (https://github.com/CatherineFramework)
    Author: azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
*/

#[allow(unused_imports)]
use std::{
    net::TcpListener,
    process::Command,
    fs::File,
    path::Path,
    io::Write,
    env::{ self, var },
    thread, time
};

use colored::{ Colorize, ColoredString };
use serde_json::Value;

use super::{
    x::catherine_shell,
    utils::{
        connection_handler,
        find_open_ports,
        nmap_scanner,
        db_search,
        ThreadPool
    }
};

use crate::{
    modules::rust_hex_dump::collect_hex,
    core::utils::pretty_output
};

use crate::catherine::{
    NAME,
    VERSION,
    NETSCAN_PATH,
    LINK_PARSER_PATH,
    REDIS_ANALYSIS_PATH,
    WIN_EXE_DUMP_PATH,
    MERCY_EXT_PATH
};

/*
    Commands
*/

pub fn start_server(addr: &str) {
    // Change the ip address & port for web server here
    let listener = TcpListener::bind(addr).unwrap();
    
    let pool = ThreadPool::new(5);

    println!("\nCurrently listening on {}", addr);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Executes connection request for web server
        pool.execute(|| {
            connection_handler(stream);
        });
    }
    
    print!("Shutting down...");
}

pub fn view_modules() {
    
    // JSON file
    let json_file: &str = "/opt/catherine/modules/modules.json"; // Local

    let json_parse = {
        // Load the JSON file and convert to an easier to read format
        let json_convert = std::fs::read_to_string(&json_file).expect("Unable to locate file");
        serde_json::from_str::<Value>(&json_convert).unwrap()
    };

    // Number of elements in modules.json
    let modules_len = json_parse["ModulesList"].as_array().unwrap().len();

    for index in 0..modules_len {
        // Displays parsed JSON
        println!("\nID: {}", &json_parse["ModulesList"][index]["id"]);
        println!("Name: {}", &json_parse["ModulesList"][index]["name"]);
        println!("Description: {}", &json_parse["ModulesList"][index]["description"]);
        println!("Version: {}\n", &json_parse["ModulesList"][index]["version"]);
    }
}

pub fn set_module() {
    let set_module_mode = catherine_shell(NAME, VERSION, "set_module".blue());
    let set_module_str: &str = &set_module_mode;

    Command::new("chmod")
            .arg("+x")
            .args([NETSCAN_PATH, LINK_PARSER_PATH, REDIS_ANALYSIS_PATH, WIN_EXE_DUMP_PATH, MERCY_EXT_PATH])
            .output()
            .expect("Unable process module executable loop");

    match set_module_str {
        "netscan" | "NetScan" | "set_module netscan" => {
            let set_host = catherine_shell(NAME, VERSION, "set_module/netscan/set_host (ex: google.com)".blue());
            let module_activating: ColoredString = "Activating NetScan Module...\n".green();

            println!("{}", module_activating);
            thread::sleep(time::Duration::from_secs(1));

            // Go acts a little funky on WSL for some reason
            if set_host == "help" {
                if env::consts::OS == "linux" {
                    Command::new(NETSCAN_PATH)
                            .arg("help")
                            .status()
                            .expect("Failed to execute process");
                } else {
                    println!("Unable to run module on this operating system");
                }
            } else {
                if env::consts::OS == "linux" {
                    Command::new(NETSCAN_PATH)
                            .arg("all")
                            .arg("--host")
                            .arg(set_host)
                            .status()
                            .expect("Failed to execute process");
                } else {
                    println!("Unable to run module on this operating system");
                }
            }
        },

        "parser" | "Parser" | "set_module parser" => {
            let set_host = catherine_shell(NAME, VERSION, "set_module/parser/links/set_host (ex: https://google.com)".blue());
            let module_activating: ColoredString = "Activating Link Parser Module...\n".green();

            println!("{}", module_activating);
            thread::sleep(time::Duration::from_secs(1));

            if env::consts::OS == "linux" {
                Command::new(LINK_PARSER_PATH)
                        .arg(set_host)
                        .status()
                        .expect("Failed to execute process");
            } else {
                println!("Unable to run module on this operating system");
            }
        },

        "hex" | "Hex" | "set_module hex" => {
            print!("\nrust: Runs a custom hex dump using Rust (dumps pretty hex to stdout)\n");
            print!("c: Runs a custom hex dump using C (dumps hex strings to a file & stdout)\n\n");

            let choose_your_hex_method = catherine_shell(NAME, VERSION, "set_module/hex/dump_method (ex: c)".blue());

            if choose_your_hex_method == "rust" {
                let set_hex_dump_file = catherine_shell(NAME, VERSION, "set_module/hex/set_file (ex: main.exe)".blue());
                let module_activating: ColoredString = "Activating Hex Module...\n".green();
                
                println!("{}", module_activating);
                thread::sleep(time::Duration::from_secs(1));

                collect_hex("get_data_dump", &set_hex_dump_file);
            } else if choose_your_hex_method == "c" {
                let set_hex_dump_file = catherine_shell(NAME, VERSION, "set_module/hex/set_file (ex: /path/to/main.exe)".blue());
                let module_activating: ColoredString = "Activating Hex Module...\n".green();
                
                println!("{}", module_activating);
                thread::sleep(time::Duration::from_secs(1));
                
                collect_hex("access_c_lib", &set_hex_dump_file);
            }
        },

        "db_analysis" | "DB_Analysis" | "set_module db_analysis" => {
            let module_activating: ColoredString = "Activating Database Analysis Module...\n".green();

            println!("{}", module_activating);
            thread::sleep(time::Duration::from_secs(1));

            println!("\nSupported databases:");
            println!("[0] redis\n");

            let set_db = catherine_shell(NAME, VERSION, "set_module/db_analysis/set_db".blue());

            if set_db == "redis" || set_db == "Redis" || set_db == "0" {
                if env::consts::OS == "linux" {
                    Command::new(REDIS_ANALYSIS_PATH)
                            .status()
                            .expect("Failed to execute process");
                } else {
                    println!("Unable to run module on this operating system");
                }
            } else {
                println!("Database is not supported yet");
            }
        },

        "exec_dump" | "Exec_Dump" | "set_module exec_dump" => {
            let file_loc = catherine_shell(NAME, VERSION, "set_module/exec_dump/file_loc (ex: /path/to/file)".blue());
            let module_activating: ColoredString = "Activating Exec Dump module...\n".green();
            let note_for_user: ColoredString = "Only Windows exec format are accepted!\n".red();
            
            println!("{}", module_activating);
            println!("NOTE: {}", note_for_user);
            thread::sleep(time::Duration::from_secs(1));

            if env::consts::OS == "linux" {
                Command::new(WIN_EXE_DUMP_PATH)
                        .arg(file_loc)
                        .status()
                        .expect("Failed to execute process");
            } else {
                println!("Unable to run module on this operating system");
            }
        },

        "list" | "view" => {
            view_modules();
        },

        "" => { },

        "help" => {
            // JSON file
            let json_file = "/opt/catherine/modules/modules.json"; // Local

            let json_parse = {
                // Load the JSON file and convert to an easier to read format
                let json_convert = std::fs::read_to_string(&json_file).expect("Unable to locate file");
                serde_json::from_str::<Value>(&json_convert).unwrap()
            };

            // Number of elements in modules.json
            let modules_len = json_parse["ModulesList"].as_array().unwrap().len();

            for index in 0..modules_len {
                // Displays parsed JSON
                println!("\nID: {}", &json_parse["ModulesList"][index]["id"]);
                println!("Name: {}", &json_parse["ModulesList"][index]["name"]);
                println!("Description: {}", &json_parse["ModulesList"][index]["description"]);
                println!("Version: {}\n", &json_parse["ModulesList"][index]["version"]);
            }
        },

        _ => {
            println!("Unable to find module. Returning to base shell...");
        }
    }
}

pub fn scan_ports_catherine(scan_method: &str) {
    if scan_method == "all" {
        println!("WARNING: Catherine's port scanner is experiemental");
                
        // Scans every available port (65,535)
        for open_port in find_open_ports(0..65535) {
            print!("\nOpen Port: {}", open_port);
        }
    } else if scan_method == "set" {
        println!("WARNING: Catherine's port scanner is experiemental");

        let set_scan_range_start = catherine_shell(NAME, VERSION, "First port (ex: 1234)".blue());
        let set_scan_range_end = catherine_shell(NAME, VERSION, "Last port (ex: 8080)".blue());
        
        // Custom port range scanning
        for open_port in find_open_ports(set_scan_range_start.parse::<u16>().unwrap()..set_scan_range_end.parse::<u16>().unwrap()) {
            print!("\nOpen Port: {}", open_port);
        }
    }

    println!("\nPort scan complete");
}

pub fn scan_ports_nmap(scan_method: &str) {
    if scan_method == "all" {
        nmap_scanner("full");
    } else if scan_method == "quick" {
        nmap_scanner("quick");
    }
    
    println!("\nPort scan complete");
}

pub fn search_exploit() {
    let exploit_search = catherine_shell(NAME, VERSION, "search_exploit/db_search".red());
    db_search(exploit_search);
}

// Special Windows modules created internally to handle certain functions
pub fn set_windows_module() {
    print!("\ndawn: Dump Windows adapter information\n");

    let choose_module = catherine_shell(NAME, VERSION, "set_windows_module".blue());
    let choose_module_str: &str = &choose_module;
    
    match choose_module_str {
        "dawn" | "Dawn" => {
            #[cfg(target_os = "windows")]
            if env::consts::OS == "windows" {
                let mut file = File::create("adapter.log").expect("create failed");

                for adapter in ipconfig::get_adapters().expect("Unable to gather adapter information") {
                    let adapter_dump = format!(
                        "\n\nAdapter name: {:#?}\nAdapter description: {:#?}\nAdapter ip addresses: {:#?}\nAdapter interface: {:#?}\nAdapter prefixes: {:#?}\nAdapter gateways: {:#?}\nAdapter dns servers: {:#?}\nAdapter physical address: {:#?}\nAdapter link speed (receiver): {:#?}\nAdapter link speed (transmit): {:#?}", 
                        adapter.adapter_name(), adapter.description(), adapter.ip_addresses(), adapter.if_type(),
                        adapter.prefixes(), adapter.gateways(), adapter.dns_servers(), adapter.physical_address(),
                        adapter.receive_link_speed(), adapter.transmit_link_speed()
                    );

                    file.write_all(adapter_dump.as_bytes()).expect("write failed");
                }

                println!("Adapter information dumped to adapter.log");
            }
            
            if env::consts::OS != "windows" {
                println!("This command is only available on Windows");
            }
        },

        _ => println!("Unable to find module. Returning to base shell...")
    }
}

/*
    Displays help information when running the help command
*/
pub fn help_menu() {

    println!("\n=== General ===");
    pretty_output("start_server\nscan_ports\nsearch_exploit\nset_decode\nsys_info\n", "Start a Rust server\nScan for open local ports\nSearch ExploitDB for an available exploit to review\nDecode an encoded message using one of our provided methods\nPrint local system information to stdout", "Command", "Description");

    println!("\n=== Module ===");
    pretty_output("set_module\nview_modules\nset_windows_module", "Set one of Catherine's modules\nCurrently installed modules\nAllows you to use a module created for Windows or data generated from Windows", "Command", "Description");

    println!("\n=== Modules List ===");
    pretty_output("netscan\nparser\nhex\ndb_analysis\nexec_dump", "Collects public network information about a host\nParses web content, extracting external and internal links\nExports a custom hexadecimal dump for most file types (.exe, .toml, .c, etc.)\nTerminal-based database exploration and monitoring\nMulti-format parser built to extract various data points from executables, object binaries, DLLs and more (32-bit & 64-bit)", "Module", "Description");

    println!("\n=== Help ===");
    pretty_output("help\nversion\nexit", "Help menu\nVersion info for Catherine framework\nExit Catherine framework", "Command", "Description");
}