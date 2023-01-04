/*
    Project: Catherine (https://github.com/CatherineFramework)
    Author: azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
*/

use std::{
    io::prelude::*,
    net::TcpStream,
    ops::Range,
    process::Command,
    path::Path,
    sync::{
        mpsc, Arc, Mutex,
    },
    env::{ self },
    fs, thread, time
};

use colored::Colorize;

use super::x::catherine_shell;

use crate::catherine::{ NAME, VERSION };

use prettytable::{
    Cell,
    Row,
    Table
};

use os_type::OSType::{
    Kali, Ubuntu, Unknown
};

/*
    Helper Functions
*/

pub fn connection_handler(mut stream: TcpStream) {
    
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_response = b"GET / HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(get_response) {
        // Render the index page here
        ("HTTP/1.1 200 OK", "server/public/index.html")
    } else {
        // 404 - Missing Resource/Page Error Code
        ("HTTP/1.1 404 NOT FOUND", "server/public/err/404.html")
    };

    let return_content = fs::read_to_string(filename).unwrap();
    
    // Handles our response information
    let response = format! (
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        return_content.len(),
        return_content
    );

    if status_line == "HTTP/1.1 200 OK" {
        println!("{} [{}]", filename, status_line);
    } else {
        println!("{} [{}]", filename, status_line);
    }

    // This is where our request information is rendered
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // println!("Request: {}\n", String::from_utf8_lossy(&buffer[..]));
    // println!("Response: {}", response);
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Destroy,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, {
            let job = Box::new(f);
            self.sender.send(Message::NewJob(job)).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending termination request");

        for _ in &self.workers {
            self.sender.send(Message::Destroy).unwrap();
        }

        println!("Shutting down all active workers");

        for worker in &mut self.workers {

            // println!("Worker {} shutting down", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    // println!("Worker {} executing", id);
                    
                    job();
                }

                Message::Destroy => {
                    // println!("Worker {} Terminating", id);
                    
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn is_port_open(check_port: u16, ip_addr: &str) -> bool {
    // Change the ip address for port scanning here
    match TcpStream::connect((ip_addr, check_port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Scans the chosen port range to locate all open ports
pub fn find_open_ports(port_range: Range<u16>) -> Vec<u16> {
    let mut open_ports = Vec::new();

    let set_ip_bind = catherine_shell(NAME, VERSION, "Enter ip address (ex: 127.0.0.1)".blue());

    println!("Running port scan against {}\n", &set_ip_bind);
    
    for check_port in port_range {
        if is_port_open(check_port, &set_ip_bind) {
            open_ports.push(check_port);
        }
    }
    
    open_ports
}

pub fn nmap_scanner(scan_all: &str) {
    let set_ip_bind = catherine_shell(NAME, VERSION, "Enter ip address (ex: 127.0.0.1)".blue());

    println!("Running port scan against {}\n", &set_ip_bind);

    if scan_all == "full" {
        Command::new("nmap")
                .arg("-sC")
                .arg("-sV")
                .arg("-v")
                .arg("-p-")
                .arg(&set_ip_bind)
                .status()
                .expect("Failed to execute process");
    } else if scan_all == "quick" {
        Command::new("nmap")
                .arg("-sC")
                .arg("-sV")
                .arg("-v")
                .arg(&set_ip_bind)
                .status()
                .expect("Failed to execute process");
    }
}

pub fn existence(dir_name: &str) -> bool {
    // Checks to see if the directory already exists
    let exploit_dir: bool = Path::new(dir_name).is_dir();
    return exploit_dir;
}

pub fn db_search(exploit_info: String) {
    // *Should* work for most Linux distributions and WSL
    if env::consts::OS == "linux" {
        let os = os_type::current_platform();

        // TODO: Add more Linux distributions
        match os.os_type {
            Kali | Unknown | Ubuntu => {
                match Command::new("searchsploit").output() {
                    Ok(_) => {
                        println!("Searching DB...\n");

                        Command::new("searchsploit")
                                .arg(exploit_info)
                                .status()
                                .expect("Failed to execute process");
                    },
                    Err(err) => {
                        #[allow(irrefutable_let_patterns)]
                        if let _unable_to_locate = err.kind() {
                            
                            // Checks if ExploitDB already exists
                            if existence("exploitdb") != true {
                                println!("\nYou're missing ExploitDB/Searchsploit. Would you like to download it now? (this could take awhile)\n");
                                
                                let download_db = catherine_shell("Catherine", VERSION, "search_exploit/db_search/download_db".red());
                
                                if download_db == "y" || download_db == "yes" {
                                    // Downloads ExploitDB from GitHub
                                    git_downloader("https://github.com/offensive-security/exploitdb.git");
    
                                    println!("\nDownload complete. Please enter your query below\n");
                                    
                                    let db_query = catherine_shell("Catherine", VERSION, "search_exploit/db_search".red());

                                    println!("Searching DB...\n");
                                    thread::sleep(time::Duration::from_secs(1));
            
                                    Command::new("./exploitdb/searchsploit")
                                            .arg(db_query)
                                            .status()
                                            .expect("Failed to execute process");
                                }
                            } else {
                                let db_query = catherine_shell("Catherine", VERSION, "search_exploit/db_search".red());

                                println!("Searching DB...\n");
                                thread::sleep(time::Duration::from_secs(1));
        
                                Command::new("./exploitdb/searchsploit")
                                        .arg(db_query)
                                        .status()
                                        .expect("Failed to execute process");
                            }
                        } else {
                            println!("An error occurred");
                        }
                    }, 
                }
            },

            _ => {
              println!("Your distribution is not implemented yet!");
            }
        }
    } else {
        // TODO: Add Windows support (works - WIP)
        // TODO: Add macOS support (works - WIP)
        println!("Your Operating System is not implemented yet!");
    }
}

pub fn git_downloader(url_path: &str) {
    println!("Downloading...this may take awhile");
    println!("Downloading path: {}", url_path);

    Command::new("git")
            .arg("clone")
            .arg(url_path)
            .output()
            .expect("Failed to execute process");
}

pub fn pretty_output(input: &str, output: &str, left_col: &str, right_col: &str) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new(left_col),
        Cell::new(right_col)
    ]));

    table.add_row(Row::new(vec![
        Cell::new(input),
        Cell::new(output)
    ]));

    table.printstd();
}

// Sets the path to the $HOME directory
// pub fn set_home() {
//     let home_env = "HOME";
    
//     match var(home_env) {
//         Ok(value) => set_current_dir(value).expect("Unable to set directory"),
//         Err(err) => println!("Unable to interpret environment variable. Is your $HOME variable set?\n {}", err),
//     }
// }