pub mod game;
use game::GameMap::GameMap;
use game::Player::Player;

fn main() {
    use std::io;

    let gm: GameMap = GameMap::new();

    println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");
    println!("Choose your country: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Virhe syötteen lukemisessa");
    let input = input.trim();

    let mut i: i32 = input.trim().parse().unwrap();
    let player: Player = Player::new(gm.get_country_by_index(i).clone());

    println!("| Inspection on your own nation? | y = yes | n = no |");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Virhe syötteen lukemisessa");
    let input = input.trim();

    if input == "y" {
        println!("An inspection has been completed..");
        player.inspect();
    } else if input == "n" {
        println!("The leader is confident. No inspection needed.");
    }

    loop {
        println!("| 1) Spy on a country | 0) Exit program |");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Virhe syötteen lukemisessa");
        let input = input.trim();

        if input == "1" {
            gm.list_countries();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Virhe syötteen lukemisessa");
            let input = input.trim();

            let i2: i32 = input.trim().parse().unwrap();

            if i2 == i {
                println!("You can't spy on your own nation!")
            } else {
                player.spy(gm.get_country_by_index(i));
            }
        } else if input == "0" {
            break;
        } else {
            println!("Invalid game input. Try again.");
            continue;
        }

        println!("| Inspection on your own nation? | y = yes | n = no |");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Virhe syötteen lukemisessa");
        let input = input.trim();

        if input == "y" {
            println!("An inspection has been completed..");
            player.inspect();
        } else if input == "n" {
            println!("The leader is confident. No inspection needed.");
        }

    }
}
