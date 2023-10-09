/*
    Project: Catherine Framework (https://github.com/azazelm3dj3d/catherine)
    Author: azazelm3dj3d (https://github.com/azazelm3dj3d)
    License: BSD 2-Clause
*/

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{
  process::exit,
  env
};

use chrono::{
  Timelike,
  Local
};

use mercy::{
  decode,
  extra,
  experimental
};

#[tauri::command]
fn decode_string(method_name: &str, encoded_data: &str) -> String {
  format!("Decoded String: {}", decode(method_name, encoded_data))
}

#[tauri::command]
fn sys_info() -> String {
  if env::consts::OS == "linux" {
    format!("{}Internal IP Address: {}\n", extra("system_info", "all"), extra("internal_ip", ""))
  } else {
    format!("Command not available on your operating system yet!")
  }
}

#[tauri::command]
fn defang_string(defang_value: &str) -> String {
  format!("Defanged: {}", extra("defang", defang_value))
}

#[tauri::command]
fn whois_search(whois_url: &str) -> String {
  format!("WHOIS Data: \n{}", extra("whois", whois_url))
}

// Causes extreme lag. Needs to be reviewed.
// #[tauri::command]
// fn malicious_search(mal_url: &str) -> String {
//   format!("URL Status: {}!", malicious("status", mal_url))
// }

#[tauri::command]
fn id_string(id_str: &str) -> String {
  format!("Guess: {}", extra("identify", id_str))
}

#[tauri::command]
fn crack_hash(hash_cracker: &str) -> String {
  format!("{}", extra("crack", hash_cracker))
}

#[tauri::command]
fn domain_gen(domain_str: &str) -> String {
  format!("{:?}", experimental("domain_gen", domain_str))
}

#[tauri::command]
fn extract_zip(extract_zip_file: &str) {
  experimental("zip", extract_zip_file)
}

#[tauri::command]
fn parse_email(parse_email_file: &str) -> String {
  format!("{}", extra("parse_email", parse_email_file))
}

#[tauri::command]
fn exit_catherine() -> String {
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
  
  exit(0x0100);
}

pub fn launch_gui() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    decode_string,
    sys_info,
    defang_string,
    whois_search,
    id_string,
    crack_hash,
    domain_gen,
    extract_zip,
    parse_email,
    exit_catherine
  ])
  .run(tauri::generate_context!())
  .expect("Unable to launch Catherine GUI");
}
