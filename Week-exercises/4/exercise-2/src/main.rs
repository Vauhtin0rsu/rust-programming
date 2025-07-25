use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Write};

fn write_in_file(file_name: &String, content: &String) -> io::Result<()> {

    let mut file = File::create(file_name).expect("Failed with creating the file");
    write!(file, "{}", content);
    // read_file(file_name);
        
    Ok(())
}

fn read_file (file_name: &String) {
    println!("The contents of the file:");
    println!("");
    let contents = fs::read_to_string(file_name).expect("Error with reading the file");
    println!("{}", contents);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No arguments were given.");
        return;
    } else if args.len() < 3 {
        println!("Invalid arguments.");
        return;
    }

    if args[2] == "write"  {
        if args.len() > 3  {
            write_in_file(&args[1], &args[3]);
        } else {
          println!("More arguments needed if not reading.");  
        }
    } else if args[2] == "read" {
        if args.len() == 3 {
            read_file(&args[1]);
        } else {
            println!("Only give three arguments when writing.");
        }
    } else {
        println!("Invalid arguments.");
    }
}
