/*
    Project: Catherine (https://github.com/CatherineFramework)
    Author: azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
*/

use std::io::Write;

use chrono::{Local, Timelike};
use colored::{ColoredString, Colorize};

// Opens a shell that accepts user input
pub fn catherine_shell(framework: &str, version: &str, active: ColoredString) -> String {
    let mut line = String::new();
    
    let current_time = Local::now();
    let (time_period, _hour) = current_time.hour12();
    let prompt_select: ColoredString = " 〉".green();

    // Displays a moon/sun depending on the time
    if time_period {
        print!("🦀 {} [v{}] ({}) 🌑 {}", framework, version, active, prompt_select);
    } else {
        print!("🦀 {} [v{}] ({}) ☀️ {}", framework, version, active, prompt_select);
    }
    
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("[ERROR] Unable to process input");
 
    return line.trim().to_string()
}