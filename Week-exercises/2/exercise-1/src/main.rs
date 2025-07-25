const S: &str = "I want to be changed.";

fn create_default() -> String {
    S.to_string()
}

fn remove_latest_word(s1: &mut String) {
    let mut parts: Vec<&str> = s1.split_whitespace().collect();
    parts.pop();
    
    let mut s2: String = "".to_string();
    for part in parts {
        s2.push_str(part);
        s2.push_str(" ");
    }
    *s1 = s2.trim_end().to_string();
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
            remove_latest_word(&mut user_string);
            
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
