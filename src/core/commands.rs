/*
    CyberSuki (https://github.com/cybersuki)
    File: src/core/commands.rs

    Author(s): {
        Hifumi1337 (https://github.com/Hifumi1337)
    }
*/

#[allow(unused_imports)]
use std::{
    net::TcpListener,
    process::Command,
    fs::File,
    path::Path,
    io::Write,
    env::{
        self,
        set_current_dir,
        current_dir,
        var 
    }, thread, time
};

use colored::{ Colorize, ColoredString };
use serde_json::Value;

use super::{
    x::catherine_shell,
    utils::{
        connection_handler,
        find_open_ports,
        nmap_scanner,
        existence,
        db_search,
        set_home,
        ThreadPool
    }
};

use crate::{
    modules::hex::hex,
    core::utils::{git_downloader, pretty_output}
};

use crate::catherine::{
    NAME,
    VERSION,
    NETSCAN_PATH,
    PARSER_PATH,
    REDIS_ANALYSIS_PATH,
    EXE_PATH,
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
    let reset_dir_buf = current_dir().expect("Unable to get directory");
    set_home();

    // Checks if the modules directory exists
    if existence(".catherine/catherine-modules") != true {
        println!("Missing modules!\n");
        println!("Would you like to download Catherine's external modules now (y/n)? ");
        
        let dl_modules = catherine_shell(NAME, VERSION, "dl_modules".blue());

        if dl_modules == "yes" || dl_modules == "y" {
            git_downloader("https://github.com/cybersuki/catherine-modules");

            Command::new("rm")
                    .arg("-r")
                    .arg(".catherine")
                    .output()
                    .expect("Unable to process request");

            Command::new("mkdir")
                    .arg(".catherine")
                    .output()
                    .expect("Unable to process request");

            Command::new("mv")
                    .arg("catherine-modules")
                    .arg(".catherine")
                    .output()
                    .expect("Unable to process request");
        
        } else {
            println!("Modules not installed");
        }
    } else {
        // JSON file
        let json_file: &str = ".catherine/catherine-modules/modules.json"; // Local
    
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

    set_current_dir(reset_dir_buf).expect("Unable to set directory");
}

pub fn set_module() {
    let reset_dir_buf = current_dir().expect("Unable to get directory");
    set_home();

    // Checks if the modules directory exists
    if existence(".catherine/catherine-modules") != true {
        println!("Missing modules!\n");
        println!("Would you like to download Catherine's external modules now (y/n)? ");

        let dl_modules = catherine_shell(NAME, VERSION, "dl_modules".blue());

        if dl_modules == "yes" || dl_modules == "y" {
            git_downloader("https://github.com/cybersuki/catherine-modules");

            Command::new("rm")
                    .arg("-r")
                    .arg(".catherine")
                    .output()
                    .expect("Unable to process request");

            Command::new("mkdir")
                    .arg(".catherine")
                    .output()
                    .expect("Unable to process request");

            Command::new("mv")
                    .arg("catherine-modules")
                    .arg(".catherine")
                    .output()
                    .expect("Unable to process request");
        
        } else {
            println!("Modules not installed");
        }

        set_current_dir(reset_dir_buf).expect("Unable to set directory");
    } else {
        let set_module_mode = catherine_shell(NAME, VERSION, "set_module".blue());
        let set_module_str: &str = &set_module_mode;

        Command::new("chmod")
                .arg("+x")
                .args([NETSCAN_PATH, PARSER_PATH, REDIS_ANALYSIS_PATH, EXE_PATH, MERCY_EXT_PATH])
                .output()
                .expect("Unable process module executable loop");
    
        match set_module_str {
            "netscan" | "NetScan" => {
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

                set_current_dir(reset_dir_buf).expect("Unable to set directory");
            },
    
            "parser" | "Parser" => {
                let module_activating: ColoredString = "Activating Web Parser Module...\n".green();
    
                println!("{}", module_activating);
                thread::sleep(time::Duration::from_secs(1));
    
                if env::consts::OS == "linux" {
                    Command::new(PARSER_PATH)
                            .status()
                            .expect("Failed to execute process");
                } else {
                    println!("Unable to run module on this operating system");
                }

                set_current_dir(reset_dir_buf).expect("Unable to set directory");
            },
    
            "hex" | "Hex" => {
                print!("\nrust: Runs a custom hex dump using Rust (dumps pretty hex to stdout)\n");
                print!("c: Runs a custom hex dump using C (dumps hex strings to a file & stdout)\n\n");
    
                let choose_your_hex_method = catherine_shell(NAME, VERSION, "set_module/hex/dump_method (ex: c)".blue());
    
                if choose_your_hex_method == "rust" {
                    let set_hex_dump_file = catherine_shell(NAME, VERSION, "set_module/hex/set_file (ex: main.exe)".blue());
                    let module_activating: ColoredString = "Activating Hex Module...\n".green();
                    
                    println!("{}", module_activating);
                    thread::sleep(time::Duration::from_secs(1));
                    set_current_dir(reset_dir_buf).expect("Unable to set directory");

                    hex("get_data_dump", &set_hex_dump_file);
                } else if choose_your_hex_method == "c" {
                    let set_hex_dump_file = catherine_shell(NAME, VERSION, "set_module/hex/set_file (ex: /path/to/main.exe)".blue());
                    let module_activating: ColoredString = "Activating Hex Module...\n".green();
                    
                    println!("{}", module_activating);
                    thread::sleep(time::Duration::from_secs(1));
                    
                    hex("access_c_lib", &set_hex_dump_file);

                    match var("HOME") {
                        Ok(value) => {
                            let hex_file = format!("{}/maniac_c_lib.hex", value);

                            set_current_dir(reset_dir_buf).expect("Unable to set directory");

                            // Moves maniac_c_lib.hex to current directory (location of the catherine executable on the user's system)
                            if Path::new(&hex_file).exists() {
                                Command::new("mv")
                                        .arg(hex_file)
                                        .arg(".")
                                        .spawn()
                                        .expect("Unable to process request");

                                println!("Data dumped to maniac_c_lib.hex");
                            }
                        },
                        Err(err) => println!("Unable to interpret environment variable. Is your $HOME variable set?\n {}", err),
                    }
                }
            },
    
            "db_analysis" | "DB_Analysis" => {
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

                set_current_dir(reset_dir_buf).expect("Unable to set directory");
            },

            "exe_dump" | "Exe_Dump" => {
                let module_activating: ColoredString = "Activating Executable Dump Module...\n".green();
                let note_for_user: ColoredString = "Only Windows exec format are accepted!\n".red();
                
                println!("{}", module_activating);
                println!("NOTE: {}", note_for_user);
                thread::sleep(time::Duration::from_secs(1));
    
                if env::consts::OS == "linux" {
                    let run_exit: ColoredString = "After entering the location of your file, run the 'dump' command. After that, run the 'exit' command to move the file into your current directory\n".green();
                    println!("{}", run_exit);

                    Command::new(EXE_PATH)
                            .status()
                            .expect("Failed to execute process");

                    match var("HOME") {
                        Ok(value) => {
                            let header_file = format!("{}/header_dump.log", value);

                            set_current_dir(reset_dir_buf).expect("Unable to set directory");

                            // Moves header_dump.log to current directory (location of the catherine executable on the user's system)
                            if Path::new(&header_file).exists() {
                                Command::new("mv")
                                        .arg(header_file)
                                        .arg(".")
                                        .spawn()
                                        .expect("Unable to process request");

                                println!("Data dumped to header_dump.log");
                            }
                        },
                        Err(err) => println!("Unable to interpret environment variable. Is your $HOME variable set?\n {}", err),
                    }
                } else {
                    println!("Unable to run module on this operating system");
                }
            },
    
            "list" | "view" => {
                view_modules();
            },
    
            "" => { },

            "update" => {
                println!("Updating Catherine...");

                git_downloader("https://github.com/cybersuki/catherine-modules");

                Command::new("rm")
                        .arg("-r")
                        .arg(".catherine")
                        .output()
                        .expect("Unable to process request");

                Command::new("mkdir")
                        .arg(".catherine")
                        .output()
                        .expect("Unable to process request");

                Command::new("mv")
                        .arg("catherine-modules")
                        .arg(".catherine")
                        .output()
                        .expect("Unable to process request");
            },
    
            "help" => {
                // JSON file
                let json_file = ".catherine/catherine-modules/modules.json"; // Local
    
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
                set_current_dir(reset_dir_buf).expect("Unable to set directory");
            }
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
    pretty_output("netscan\nparser\nhex\ndb_analysis\nexe_dump", "Collects public network information about a host\nParses web content, extracting external and internal links\nExports a custom hexadecimal dump for most file types (.exe, .toml, .c, etc.)\nReal-time Redis analysis and monitoring\nMulti-format parser built to extract various data points from executables, object binaries, DLLs and more (32-bit & 64-bit)", "Module", "Description");

    println!("\n=== Help ===");
    pretty_output("help\nversion\nexit", "Help menu\nVersion info for Catherine framework\nExit Catherine framework", "Command", "Description");
}