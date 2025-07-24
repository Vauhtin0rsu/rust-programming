const S: &str = "I want to be changed.";

fn create_default() -> String {
    S.to_string()
}

fn remove_latest_word(str: &mut String) -> String {
    let mut parts: Vec<&str> = str.split_whitespace().collect();
    parts.pop();

    let mut result = String::new();
    for part in parts {
        result.push_str(part);
        result.push_str(" ");
    }
    result.trim().to_string() // delete the last whitespace
}

fn main() {
    let mut user_string = create_default();

    use std::io;
    loop {
        println!("| 1) Reset | 2) Remove a word | 3) Add a word | 4) Print string | 0) End program |");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading the input");

        let i: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parsing error. Was the input a number?");
                continue;
            } 
        };

        if i == 1 {
            user_string = create_default(); 
      
        } else if i == 2 {
            user_string = remove_latest_word(&mut user_string);
            
        } else if i == 3 {
            println!("The new word: ");
            let mut new_word = String::new();
            io::stdin().read_line(&mut new_word).expect("Error reading the input");
            new_word = new_word.trim().to_string();
            user_string.push_str(" ");
            user_string.push_str(&new_word);

        } else if i == 4 {
            println!("{}", user_string);

        } else if i == 0 {
            break;

        } else {
            println!("Invalid input. Try again.")
        }
    }
}
