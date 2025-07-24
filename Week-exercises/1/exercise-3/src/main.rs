use std::io;
fn main() {
    let mut summa: i16 = 0;
    println!("By how much do you want to increment the number?");
    loop {
        println!("Current: {}. Increment by: ", summa);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Virhe syötteen lukemisessa");
        let luku: i32 = match input.trim().parse() {
            Ok(luku) => luku,
            Err(_) => {
                println!("Parsing failed. Was the number too long for a 16-bit variable?");
                continue;
            }
        }; 
        if luku < 0 { 
            println!("The given value is lower than 0.");
            continue; 
        }
        if luku == 0 { 
            println!("The given value is 0. Ending the program.");
            break;
        }
        
        if summa - i16::MAX + luku as i16 > 0 {
            println!("Enough incrementations.");
            break;
        } else {
            summa += luku as i16;
        }
    } 
}
