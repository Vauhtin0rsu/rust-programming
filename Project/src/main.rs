pub mod game;
use game::Player::Player;
use game::WeaponMap::WeaponMap;
use game::MonsterMap::MonsterMap;
use game::Weapon::Weapon;
use game::Monster::Monster;

use std::io;
use rand::Rng;

fn menu(name: &str, function: &str) -> i8 {
    if function == "main" {
        println!("\nMitä tehdään, {}?", name);
        println!("1) Seikkaile tyrmissä     2) Käy kaupassa     3) Hakkaa lohikäärme     0) Luovuta");
    }

    if function == "battle_normal" {
        println!("1) Hyökkää    2) Puolustaudu     3) Juo lääkerohto    4) Juokse pakoon");
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Virhe syötteen lukemisessa");
    let input = input.trim();

    let i: i8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Virhe syötteen lukemisessa");
            -1
        } 
    };
    i
}

fn battle_normal(player: &mut Player, monsters: &mut Vec<Monster> ) {
    // pick a weapon for the dungeon
    println!("Minkä aseen aiot ottaa mukaan?");
    let mut i: i32 = 1;
    for weapon in &player.weapons {
        println!("{}) {}", i, weapon.name);
        i += 1;
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Virhe syötteen lukemisessa");
    let input = input.trim();

    let i: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Virhe syötteen lukemisessa");
            return;
        } 
    };

    if i > player.weapons.len() {
        println!("Virhe syötteen lukemisessa, palataan päävalikkoon.");
        return;
    }

    let curnt_weapon: &Weapon = &player.weapons[i-1];
    println!("{}! Hyvä valinta!", curnt_weapon.name);
    
    // loop through the monster fights
    println!("Lähestyt kohti tyrmää...");
    for monster in monsters {
        if player.curnt_hp < 1 {
            break;
        }
        println!("\nEteesi hyökkää {}!", monster.name);
        println!("Elämäpisteet: {}", monster.hp);
        if curnt_weapon.element == monster.weakness {
            println!("Aseesi toimii erityisen hyvin tähän hirviöön!");
        }

        loop {
            let j = menu("","battle_normal");
            match j {
                1 => {
                    let mut player_dmg_float: f32 = rand::rng().random_range(15..25) as f32 * player.str;
                    if curnt_weapon.element == monster.weakness {
                        player_dmg_float = player_dmg_float * curnt_weapon.str;
                    }

                    let player_dmg_int: i32 = player_dmg_float.round() as i32;
                    monster.hp -= player_dmg_int;
                    println!("Teet vahinkoa {} elämäpistettä", player_dmg_int);

                    if monster.hp < 1 {
                        println!("Voitit hirviön!! Saat siitä hyvästä {} kultaa", monster.gold);
                        player.gold += monster.gold;
                        // append_monsterfile(monster.name);
                        break;
                    }
                } 
                2 => {

                }
                3 => {
                    if player.potions > 0 {
                        println!("Juot lääkerohdon");
                        player.curnt_hp += rand::rng().random_range(30..40);
                        player.potions -= 1;

                        if player.curnt_hp > player.max_hp {
                            player.curnt_hp = player.max_hp;
                        }

                        println!("Elämäpisteet: {}", player.curnt_hp);
                        println!("Lääkerohtoja jäljellä: {}", player.potions);
                        

                    } else {
                        println!("Lääkerohtoja ei ole jäljellä!");
                        continue;
                    }
                }
                4 => {
                    println!("Pakenet hirviön kynsistä");
                    break;
                }
                _ => {
                    continue;
                }
            }
            
            println!("\n{} hyökkää päällesi!!", monster.name);
            let monster_dmg_float: f32 = rand::rng().random_range(10..20) as f32 * monster.str;
            let monster_dmg_int: i32 = monster_dmg_float.round() as i32;
            player.curnt_hp -= monster_dmg_int;

            println!("{} tekee vahinkoa {} elämäpistettä", monster.name, monster_dmg_int);
            println!("\nElämäpisteitä jäljellä: {}", player.curnt_hp);

            if player.curnt_hp < 1 {
                println!("{} on voittanut sinut!", monster.name);
                break;
            }
        }
    }
}

fn battle_boss(boss: &Monster, player: &mut Player, wm: &WeaponMap) {
    
}

fn weapon_market(player: &mut Player, wm: &WeaponMap) {
    println!("Myynnissä olevat aseet:");

    let mut i: i32 = 1;
    for weapon in &wm.for_the_shop {
        println!("{}) {}, {} kultaa", i, weapon.name, weapon.cost);
        i += 1;
    }
    println!("0) Ei kiitos");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Virhe syötteen lukemisessa");
    let input = input.trim();

    let i: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Virhe syötteen lukemisessa");
            return;
        } 
    };

    if i > wm.for_the_shop.len() {
        println!("Virhe syötteen lukemisessa, palataan päävalikkoon.");
        return;
    }

    if i == 0 {
        println!("Ensi kerralla sitten!");
        return;
    }

    if player.gold >= wm.for_the_shop[i-1].cost {
        println!("\n{}! Saamanne pitää", wm.for_the_shop[i-1].name);
        player.gold -= wm.for_the_shop[i-1].cost;
        player.weapons.push(wm.for_the_shop[i-1].clone());
    } else {
        println!("Valitettavasti sinulla ei ole varaa tähän aseeseen.");
    }
} 

fn potion_market(player: &mut Player) {
    println!("\nHaluatko ostaa lääkerohtoja? (k / e)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Virhe syötteen lukemisessa");

    if input.trim() == "k" {
        println!("Kuinka monta? Yksi lääkerohto on 20 kultaa.");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Virhe syötteen lukemisessa");
        let i: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Virhe syötteen lukemisessa");
                return;
            }
        };

        let sum: i32 = i as i32 * 20;
        if sum <= player.gold {
            player.gold -= sum;
            player.potions += i as i32;
            println!("Ostit juuri {} lääkerohtoa", i as i32);
        }
    }
}

// fn append_monsterfile(name: &str) {

// }

// fn analyze_monsterfile() {

// }

fn main() {
   println!("Hei seikkailija! Mikä on nimesi?");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Virhe syötteen lukemisessa");
    let name = name.trim();

    let mut player: Player = Player::new();
    let mut wm: WeaponMap = WeaponMap::new();
    let mm: MonsterMap = MonsterMap::new();

    // Pick the starting weapon, delete the rest
    let mut rng_max: usize = wm.starters.len();
    println!("{}", rng_max);
    let starting_weapon: Weapon = wm.starters[rand::rng().random_range(0..rng_max)].clone();
    println!("Hurraa! Ensimmäinen aseesi on {}", starting_weapon.name);
    
    player.weapons.push(starting_weapon);
    wm.starters = vec![];

    loop {
        if player.curnt_hp < 1 {
            break;
        }

        let i = menu(name, "main");
        match i {
            1 => {
                let mut dungeon_monsters: Vec<Monster> = vec![];
                rng_max = mm.normals.len() - 1;
                for mut _j in 0..3 {
                    let rng_monster: Monster = mm.normals[rand::rng().random_range(0..rng_max)].clone();
                    dungeon_monsters.push(rng_monster);
                }
                battle_normal(&mut player, &mut dungeon_monsters);        

            }
            2  => {
                println!("Sinulla on {} kultaa.", player.gold);
                weapon_market(&mut player, &wm);
                potion_market(&mut player);
            }
            3 => {
                battle_boss(&mm.bosses[player.bosses_won as usize], &mut player, &wm);

            }

            0 => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
    //analyze_monsterfile()
}
