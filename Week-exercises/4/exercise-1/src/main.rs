use std::io;
use std::env;
use std::fs;
use std::io::Write;

fn read_file () {
    let contents = fs::read_to_string("read.txt").expect("Error with reading the file");
    println!("{}", contents);
}   

fn prank_user () {
    println!("You have received an email.");
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "read" {
            read_file();
        } else if input == "prank" {
            prank_user();
        } else {
            println!("Invalid command. Try again.");
        }
    }
}
