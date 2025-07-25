fn receive_player_attack_dmg() -> f32 {
    rand::random_range(12.5..20.0)
}

fn receive_defense_multiplier() -> f32 {
    let num: f32 = rand::random_range(2.0..4.0);
    1 as f32 / num
}

fn receive_health_potion(num: i16) -> f32 {
    if num > 0 {
        25.0
    } else {
        0.0
    }
}

fn receive_boss_attack_dmg() -> f32 {
    rand::random_range(5.0..25.0)
}

fn main() {
    let mut player_hp: f32 = 100.0;
    let mut boss_hp: f32 = 150.0;

    let mut dmg: f32;
    let mut multiplier: f32 = 1.0;
    let mut potions: i16 = 3;

    use std::io;
    loop {
        println!("| Your HP - {} | Boss HP - {} |", player_hp, boss_hp);
        println!("| 1) Attack | 2) Defend | 3) Heal |");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Virhe syötteen lukemisessa");
        let input = input.trim();

        if input == "1" {
            dmg = receive_player_attack_dmg();
            boss_hp -= dmg;
            println!("Your attack deals {} amount of damage.", dmg);

        } else if input == "2" {
            multiplier = receive_defense_multiplier();
            println!("Defence activated!");

        } else if input == "3" {
            player_hp += receive_health_potion(potions);
            potions -= 1;
            println!("You consume a potion.");

        } else {
            println!("Pick a number from 1 to 3.");
            continue;
        }

        if boss_hp <= 0.0 {
            println!("Boss has been defeated!");
            break;
        } else {
            dmg = receive_boss_attack_dmg()*multiplier;
            player_hp -= dmg;
            multiplier = 1.0;
            println!("You take {} damage", dmg);
        }
        
        if player_hp <= 0.0 {
            println!("You have been defeated!");
            break;
        } else {
            continue;
        }
    }
}
