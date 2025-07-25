fn addition(num: &mut i32) -> i32 {
    use std::io;
    println!("What number?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading the input");
    
    let add: i32 = match input.trim().parse() {
        Ok(luku) => luku,
        Err(_) => {
            println!("Parsing error. Was the input a number?");
            return *num
        } 
    }; 
    *num + add
}

fn retraction(num: &mut i32) -> i32 {
    use std::io;
    println!("What number?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading the input");
    
    let ret: i32 = match input.trim().parse() {
        Ok(luku) => luku,
        Err(_) => {
            println!("Parsing error. Was the input a number?");
            return *num
        } 
    }; 
    *num - ret
}

fn multiplication(num: &mut i32) -> i32 {
    use std::io;
    println!("What number?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading the input");
    
    let mult: i32 = match input.trim().parse() {
        Ok(luku) => luku,
        Err(_) => {
            println!("Parsing error. Was the input a number?");
            return *num
        } 
    }; 
    *num * mult
}

fn division(num: &mut i32) -> i32 {
    use std::io;
    println!("What number?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading the input");
    
    let div: i32 = match input.trim().parse() {
        Ok(luku) => luku,
        Err(_) => {
            println!("Parsing error. Was the input a number?");
            return *num
        } 
    }; 
    *num / div
}

fn main() {
    let mut result: i32 = 0;

    use std::io; 
    loop {
        println!("| 1) Reset | 2) Addition | 3) Retraction | 4) Multiplication | 5) Division | 6) Print | 0) End program |");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading the input");
        let input = input.trim();

        let i: i32 = input.parse().expect("Parsing error. Was the input a number?");
        
        if i == 1 {
            result = 0;

        } else if i == 2 {
            result = addition(&mut result);

        } else if i == 3 {
            result = retraction(&mut result);

        } else if i == 4 {
            result = multiplication(&mut result);

        } else if i == 5 {
            result = division(&mut result); 

        } else if i == 6 {
            println!("Current number: {}", result);
            
        } else if i == 0 {
            break;

        } else {
            println!("Invalid input. Try again.")
        }
    }
    println!("Ending the program.");
}
