use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::{fs::OpenOptions, io::Write};

macro_rules! input {
    ($prompt:expr) => {{
        println!("{}", $prompt);
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Failed to get input in macros 'input'.");
        buf.trim().to_string()
    }};
}

fn main() {
    let master_key = input!("Enter the master key");
    loop {
        let mode =
            input!("Press '1' - to show list. Press '2' to add new item. Press 'q' to quit.")
                .to_lowercase();

        if mode == "q" {
            break;
        } else if mode == "1" {
            show(&master_key);
        } else if mode == "2" {
            add(&master_key);
        } else {
            println!("Invalid command.Please try again.");
        }
    }
}

fn add(master_key: &str) {
    let account_name = input!("Account Name: ");
    let password = input!("Password: ");
    let magic_crypt = new_magic_crypt!(master_key, 256);
    let base_64_pwd = magic_crypt.encrypt_bytes_to_base64(&password);

    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open("data.txt")
        .expect("Failed to open file to write data");
    f.write_all(&format!("{account_name}|{base_64_pwd}\n").as_bytes())
        .expect("Failed to save data");
}

fn show(master_key: &str) {
    if !std::path::Path::new("data.txt").exists() {
        println!("You don't have any data yet! Press '2' to add new item ");
        return;
    }
    let data = std::fs::read_to_string("data.txt").expect("Failed to load data");
    let magic_crypt = new_magic_crypt!(master_key, 256);
    for line in data.lines() {
        let (account_name, password) = line.split_once("|").expect("Invalid data was loaded");
        let pwd = magic_crypt.decrypt_base64_to_string(password);
        match pwd {
            Ok(p) => println!("{account_name} {p}"),
            Err(_) => {
                println!("Incorrect master key!");
            }
        };
    }
}
