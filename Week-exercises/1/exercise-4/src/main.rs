
use rand::Rng;
fn receive_random() -> i32 {
    rand::rng().random_range(1..11)
}

fn measure_luck(luku: i32) -> &'static str {
    if luku > 3 {
        "UNLUCKY"
    } else {
        "LUCKY"
    }
}

fn main() {
    let mut kerroin = 0;
    loop {
        kerroin += 1;
        let luku: i32 = receive_random();
        if luku <= 3 {
            println!("Low...");
        } else if luku <= 6 {
            println!("Middle!");
        } else if luku <= 9 {
            println!("High!!");
        } else if luku == 10 {
            println!("Jackpot!!!");
            break;
        } else {
            println!("Un")
        }
    }
    println!("You were {}!", measure_luck(kerroin));
}
