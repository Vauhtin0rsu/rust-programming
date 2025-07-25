use std::thread;
use std::time::Duration;
use std::io::{self, Write};


fn start_duel() {
    thread::sleep(Duration::from_secs(5));
    println!("Opponent shoots first!");
    std::process::exit(0);
}

fn main() {
    println!("FIRE!!!");
    let handle = thread::spawn(|| {
        start_duel();
    });

    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read input");

    match user_input.trim() {
        "f" => {
            println!("You fire first!");
            return;
        },
        _ => {
            println!("Oh no! You missed!");
        }
    }
    let _ = handle.join();
}