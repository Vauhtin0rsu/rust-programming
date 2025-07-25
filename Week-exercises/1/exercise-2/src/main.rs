use std::io;
fn main() {
    let string1 = String::from("rust");
    let string2 = String::from("no");

    let mut vastaus = String::new();
    io::stdin().read_line(&mut vastaus).expect("Virhe syötteen lukemisessa");
    let vastaus = vastaus.trim();
    if vastaus.to_lowercase() == string1 {
        println!("So you appreciate Rust? That's great! Thank you!");
    } else if vastaus.to_lowercase() == string2 {
        println!("So you like nothing? Alright then... :)");
    } else {
        println!("It seems that you like {}.", vastaus);
    }
}
