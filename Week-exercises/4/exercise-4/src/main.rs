use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn create_threads(money: i64) {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            tx1.send(10000).unwrap();
        }
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(3));
            tx2.send(35000).unwrap();
        }
    });   

    let tx_handle = tx.clone();
    thread::spawn(move || {
        loop {
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "catch" {
                let _ = tx_handle.send(0).unwrap();
                break;
            }
        }
    });

    let mut bank = money;

    for msg in rx {
        bank -= msg;
        if msg == 10000 {
            println!("ALERT!!! Someone stole $10,000 from you!");
        }

        if msg == 35000 {
            println!("ALERT!!! Someone stole $35,000 from you!");
        }

        if msg == 0 || bank <= 0 {
            println!("You lost all your money!");
            break;
        } else {
            println!("Funds left: {}", bank);
        }
    }

}


fn main() {
    println!("Do you have a million dollars? | y = yes, n = no");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    if input == "y" {
        println!("All right then, millionaire.");
        create_threads(1000000);
    } else if input == "n" {
        println!("Let's just assume you have $100,000 then.");
        create_threads(100000);
    } else {
        println!("Invalid input.");
    }
}