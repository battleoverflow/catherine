/*
    Project: Catherine (https://github.com/CatherineFramework)
    Author: azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
*/

use std::{
    process::{
        Child,
        Command,
        Stdio
    },
    path::Path,
    env, thread, time
};

use chrono::{
    Timelike,
    Local
};

use colored::Colorize;

use crate::core::{
    commands::{
        start_server,
        view_modules,
        scan_ports_catherine,
        scan_ports_nmap,
        set_module,
        search_exploit,
        set_windows_module,
        help_menu
    },

    utils::pretty_output,
    x::catherine_shell
};

use mercy::{
    mercy_decode,
    mercy_extra
};

#[cfg(target_os = "windows")]
extern crate ipconfig;

pub(crate) static NAME: &str = "Catherine";
pub(crate) static VERSION: &str = "0.3.51";

pub(crate) static NETSCAN_PATH: &str = "/opt/catherine/modules/net/netscan/dist/netscan";
pub(crate) static LINK_PARSER_PATH: &str = "/opt/catherine/modules/web/parsers/dist/links";
pub(crate) static MERCY_EXT_PATH: &str = "/opt/catherine/modules/mercy/dist/mercy_ext";
pub(crate) static REDIS_ANALYSIS_PATH: &str = "/opt/catherine/modules/db/redis/dist/redis_analysis";
pub(crate) static WIN_EXE_DUMP_PATH: &str = "/opt/catherine/modules/data/exe/dist/exec_dump";

pub fn init(boot_msg: &str) {

    // Cool little boot message
    println!("\n{}", boot_msg);
    thread::sleep(time::Duration::from_millis(1000));

    let booted_time = Local::now();
    let (is_pm, hour) = booted_time.hour12();

    println!(
        "\nCatherine Framework booted up at {:02}:{:02}:{:02} {}\n",
        hour,
        booted_time.minute(),
        booted_time.second(),
        if is_pm { "PM" } else { "AM" }
    );
    
    loop {
        // Accepts user input
        let user_input = catherine_shell(NAME, VERSION, "None".green());
        let check_match: &str = &user_input;

        match check_match {
            "start_server" => {
                let set_addr_bind = catherine_shell(NAME, VERSION, "Enter ip address & port (ex: 127.0.0.1:1337)".blue());
                start_server(&set_addr_bind);
            },

            "view_modules" => {
                view_modules();
            },

            "scan_ports" => {
                println!("\nAvailable options:");
                println!("[0] catherine (experimental)");
                println!("[1] nmap\n");

                let scan_ports_input = catherine_shell(NAME, VERSION, "scan_ports".blue());

                if scan_ports_input == "0" || scan_ports_input == "catherine" {
                    print!("\nall: Scan all 65,535 available ports\n");
                    print!("set: Scan a set range of ports\n\n");

                    let scan_port_len = catherine_shell(NAME, VERSION, "scan_ports/set_level".blue());
                    
                    if scan_port_len == "all" {
                        scan_ports_catherine("all");
                    } else {
                        scan_ports_catherine("set");
                    }

                } else if scan_ports_input == "1" || scan_ports_input == "nmap" {
                    print!("\nall: Scan all 65,535 available ports\n");
                    print!("quick: Runs a quick scan, pinging open ports\n\n");

                    let scan_port_len = catherine_shell(NAME, VERSION, "scan_ports/set_level".blue());
                    
                    if scan_port_len == "all" {
                        scan_ports_nmap("all");
                    } else if scan_port_len == "quick" {
                        scan_ports_nmap("quick");
                    }
                }
            },

            "set_decode" => {
                println!("\nAvailable options:");
                println!("[0] base64");
                println!("[1] rot13\n");

                let decode_method = catherine_shell(NAME, VERSION, "set_decode".blue());
                let set_method: &str = &decode_method;

                match set_method {
                    "0" | "base64" => {
                        let encoded_msg = catherine_shell(NAME, VERSION, "set_decode/base64_input".blue());
                        pretty_output(&encoded_msg, &mercy_decode("base64", &encoded_msg), "Encoded Message", "Decoded Message");
                    },

                    "1" | "rot13" => {
                        let encoded_msg = catherine_shell(NAME, VERSION, "set_decode/rot13_input".blue());
                        pretty_output(&encoded_msg, &mercy_decode("rot13", &encoded_msg), "Encoded Message", "Decoded Message");

                    },
                    
                    _ => { }
                }
            },

            "search_exploit" => {
                search_exploit();
            },

            "set_module" => {
                set_module();
            },

            "set_windows_module" => {
                set_windows_module();
            },

            "sys_info" => {
                println!("{}Internal IP Address: {}\n", mercy_extra("system_info", "all"), mercy_extra("internal_ip", ""));
            },

            "version" => {
                println!("\nCatherine Framework v{}", VERSION);
                println!("Author: azazelm3dj3d (https://github.com/azazelm3dj3d)");
                println!("GitHub Sponsors: https://github.com/sponsors/azazelm3dj3d");
                println!("Patreon: https://www.patreon.com/azazelm3dj3d\n");
            },

            "help" => {
                help_menu();
            },

            "exit" | "quit" => {
                print!("\nThanks for checking out the Catherine Framework!");

                let shutdown_time = Local::now();
                let (is_pm, hour) = shutdown_time.hour12();

                println!(
                    "\nCatherine Framework shutdown at {:02}:{:02}:{:02} {}\n",
                    hour,
                    shutdown_time.minute(),
                    shutdown_time.second(),
                    if is_pm { "PM" } else { "AM" }
                );

                break; 
            },

            // Checks if a command is empty
            "" => {
                println!("No command present");
            },

            _ => {
                // Temporarily putting in an OS check
                // Only the whoami command works on Windows
                if env::consts::OS == "linux" || user_input == "whoami" {
                    unknown_command(user_input);
                } else {
                    println!("Unable to process command");
                }
            }
        }
    }
}

fn unknown_command(shell_cmd: String) {
    let mut commands = shell_cmd.trim().split(" | ").peekable();
    let mut previous_command = None;

    #[allow(unused_variables)]
    while let Some(cmd) = commands.next()  {

        // Trims whitespace for args
        let mut whitespace_trim = shell_cmd.trim().split_whitespace();
        let cmd = whitespace_trim.next().expect("Unable to process system command");
        let arguments = whitespace_trim;

        match cmd {
            "cd" => {
                // cd command for Unix-style systems
                let new_dir = arguments.peekable().peek().map_or("/", |x| *x);
                let set_dir = Path::new(new_dir);
                
                if let Err(err) = env::set_current_dir(&set_dir) {
                    println!("{}", err);
                }

                previous_command = None;
            },
            
            cmd => {
                let std_input = previous_command.map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));

                let std_out = if commands.peek().is_some() {
                    Stdio::piped()
                } else {
                    Stdio::inherit()
                };

                // If no command in the Catherine shell is recognized, it will try to run the command as a system command
                let output = Command::new(cmd)
                            .args(arguments)
                            .stdin(std_input)
                            .stdout(std_out)
                            .spawn();

                match output {
                    Ok(output) => {
                        previous_command = Some(output);
                    },

                    Err(err) => {
                        // If no command is found on the system
                        previous_command = None;
                        // println!("Command not found");
                        // println!("{}", err);
                    },
                };
            }
        }
    }

    if let Some(mut last_command) = previous_command {
        last_command.wait().unwrap();
    }
}

/*
    Master shutdown
    Whenever safety or anything else that could potentially create a vulnerablity on the user's device is noticed, it needs to be resolved on shutdown
*/
pub fn shutdown(shutdown_msg: &str) {

    Command::new("chmod")
            .arg("-x")
            .args([NETSCAN_PATH, LINK_PARSER_PATH, REDIS_ANALYSIS_PATH, WIN_EXE_DUMP_PATH, MERCY_EXT_PATH])
            .output()
            .expect("Unable process module executable loop");

    println!("{}\n", shutdown_msg);
    thread::sleep(time::Duration::from_millis(1000));
}