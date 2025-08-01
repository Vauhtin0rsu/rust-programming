pub mod game;
use game::Monster::Monster;
use game::MonsterMap::MonsterMap;
use game::Player::Player;
use game::Weapon::Weapon;
use game::WeaponMap::WeaponMap;

use rand::Rng;
use std::io;

fn menu(function: &str) -> i8 {
    // Different menus are printed depending on what "function" is called (could have a better name)
    // if function == "main" {
    //     println!("\nMitä tehdään, {}?", name);
    //     println!("1) Seikkaile tyrmissä     2) Käy kaupassa     3) Hakkaa lohikäärme    4) Tarkista pelaajana attribuutit     0) Luovuta");
    // }

    if function == "battle_normal" {
        println!("\n1) Hyökkää    2) Puolustaudu     3) Juo lääkerohto    4) Juokse pakoon");
    }

    if function == "battle_boss" {
        println!("\n1) Hyökkää    2) Väistä hyökkäystä    3) Erityishyökkäys    4) Juo lääkerohto");
    }

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Virhe syötteen lukemisessa");
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

fn battle_normal(player: &mut Player, monsters: &mut Vec<Monster>) {
    println!("\nLähestyt kohti tyrmää...");

    let mut boss_multiplier: f32 = 1.0; // make enemies stronger depending on how many bosses the player has beaten
    if player.bosses_won == 1 {
        boss_multiplier = 1.1;
    } if player.bosses_won == 2 {
        boss_multiplier == 1.2;
    }

    println!("Minkä aseen aiot ottaa mukaan?"); // pick a weapon for the dungeon
    let mut i: i32 = 1;
    for weapon in &player.weapons {
        println!("{}) {}", i, weapon.name);
        i += 1;
    }

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Virhe syötteen lukemisessa");
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

    let curnt_weapon: &Weapon = &player.weapons[i - 1];
    println!("{}! Hyvä valinta!", curnt_weapon.name);

    for monster in monsters {   // loop through the monster fights
        if player.curnt_hp < 1 {
            break;
        }
        println!("\nEteesi hyökkää {}!", monster.name);
        println!("Elämäpisteet: {}", monster.hp);
        if curnt_weapon.element == monster.weakness {
            println!("Aseesi toimii erityisen hyvin tähän hirviöön!");
        }

        loop {         
            let j = menu("battle_normal");
            let mut player_defence_multiplier: f32 = 1.0;
            match j {
                1 => { // normal attack, players and weapons strength affect
                    let mut player_dmg_float: f32 =
                        rand::rng().random_range(15..25) as f32 * player.str;
                    if curnt_weapon.element == monster.weakness {
                        player_dmg_float = player_dmg_float * curnt_weapon.str;
                    }

                    let player_dmg_int: i32 = player_dmg_float.round() as i32;
                    monster.hp -= player_dmg_int;
                    println!("Teet vahinkoa {} elämäpistettä", player_dmg_int);

                    if monster.hp < 1 {
                        println!(
                            "Voitit hirviön!! Saat siitä hyvästä {} kultaa",
                            monster.gold
                        );
                        player.gold += monster.gold;
                        break;
                    }
                }
                2 => { // Possibility to defence, this multiplier is effective when the monster attacks
                    println!("Valmistaudut ottamaan hyökkäyksen vastaan");
                    player_defence_multiplier = rand::rng().random_range(0.45..0.8);
                }
                3 => { // drinking a potion
                    if player.potions > 0 {
                        println!("Juot lääkerohdon");
                        player.curnt_hp += rand::rng().random_range(40..56);
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
                4 => { // Running away from the monster (made easy)
                    println!("Pakenet hirviön kynsistä");
                    break;
                }
                _ => {
                    continue;
                }
            }

            println!("\n{} hyökkää päällesi!!", monster.name); // Monster attacks, monsters strength, players defence and bosses won affect
            let monster_dmg_float: f32 = rand::rng().random_range(10..20) as f32 * monster.str * player_defence_multiplier * boss_multiplier;
            let monster_dmg_int: i32 = monster_dmg_float.round() as i32;
            player.curnt_hp -= monster_dmg_int;

            println!(
                "{} tekee vahinkoa {} elämäpistettä",
                monster.name, monster_dmg_int
            );
            println!("\nElämäpisteitä jäljellä: {}", player.curnt_hp);

            if player.curnt_hp < 1 {
                println!("{} on voittanut sinut!", monster.name);
                break;
            }
        }
    }
}

fn battle_boss(monster: &Monster, player: &mut Player, wm: &WeaponMap) { 
    // This is very similar to the normal battle funcion
    player.curnt_hp = player.max_hp; // full hp for the boss :)
    let mut boss: Monster = monster.clone();
    println!("\nLähestyt urheasti kohti paikkaa, jossa {} on viimeksi nähty...", boss.name);

    println!("Minkä aseen aiot ottaa mukaan?");
    let mut i: i32 = 1;
    for weapon in &player.weapons {
        println!("{}) {}", i, weapon.name);
        i += 1;
    }

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Virhe syötteen lukemisessa");
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

    let mut curnt_weapon: Weapon = player.weapons[i - 1].clone();
    println!("{}! Hyvä valinta!", curnt_weapon.name);
    if curnt_weapon.element == boss.weakness {
        println!("Aseesi tekee ekstravahinkoa lohikäärmeeseen!");
    }

    let mut round_counter: i32 = 0; // Counter because the boss attacks only on every third round
    loop {
        let j: i8 = menu("battle_boss");
        round_counter += 1;
        match j {
            1 => {
                let mut player_dmg_float: f32 =
                    rand::rng().random_range(15..25) as f32 * player.str;
                if curnt_weapon.element == boss.weakness {
                    player_dmg_float = player_dmg_float * curnt_weapon.str;
                }

                let player_dmg_int: i32 = player_dmg_float.round() as i32;
                boss.hp -= player_dmg_int;
                println!("Teet vahinkoa {} elämäpistettä", player_dmg_int);

            }
            2 => { // This completely skips the round (could be something else tho)
                println!("Väistät mahdollisen hyökkäyksen varalta!");
                continue;
            }
            3 => {
                if curnt_weapon.special_attacks > 0 {
                    println!("Teet erityishyökkäyksen!");
                    
                    let mut player_dmg_float: f32 = rand::rng().random_range(50..80) as f32 * player.str;
                    player_dmg_float = player_dmg_float * curnt_weapon.str;

                    let player_dmg_int: i32 = player_dmg_float.round() as i32;
                    boss.hp -= player_dmg_int;
                    println!("Teet vahinkoa {} elämäpistettä", player_dmg_int);   
                    curnt_weapon.special_attacks -= 1;          

                } else {
                    println!("Tällä aseella ei ole erityishyökkäystä tai olet tehnyt niitä liikaa!");
                    round_counter -= 1;
                    continue;
                }
            }
            4 => {
                if player.potions > 0 {
                    println!("Juot lääkerohdon");
                    player.curnt_hp += rand::rng().random_range(45..56);
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
            _ => {
                round_counter -= 1;
                continue;
            }
        }
        if boss.hp <= 0 {
            println!("Olet voittanut lohikäärmeen! Hurraa!!!");
            // append_monsterfile
            player.max_hp += 50;
            player.str += 0.33;
            player.weapons.push(wm.legendary[player.bosses_won as usize].clone());
            println!("Lohikäärmeen voitettuasi se jättää sinulle uuden harvinaisen aseen nimeltään {}", wm.legendary[player.bosses_won as usize].name);
            player.bosses_won += 1;
            break; 
        }

        if round_counter % 3 == 0 {
            println!("\n{} hyökkää sinua kohti!!", boss.name);
            let boss_dmg_float: f32 = rand::rng().random_range(40..65) as f32 * boss.str;
            let boss_dmg_int: i32 = boss_dmg_float.round() as i32;
            player.curnt_hp -= boss_dmg_int;

            println!("{} tekee vahinkoa {} elämäpistettä", boss.name, boss_dmg_int);
            println!("\nElämäpisteitä jäljellä: {}", player.curnt_hp);

            if player.curnt_hp < 1 {
                println!("{} on voittanut sinut!", boss.name);
                break;
            }
        } else {
            println!("{} valmistautuu vielä hyökkäykseensä...", boss.name);
        }
    }
}

fn weapon_market(player: &mut Player, wm: &WeaponMap) {
    println!("\nMyynnissä olevat aseet:");

    let mut i: i32 = 1;
    for weapon in &wm.for_the_shop {
        println!("{}) {}, {} kultaa", i, weapon.name, weapon.cost);
        i += 1;
    }
    println!("0) Ei kiitos");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Virhe syötteen lukemisessa");
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

    if player.gold >= wm.for_the_shop[i - 1].cost {
        println!("\n{}! Saamanne pitää", wm.for_the_shop[i - 1].name);
        player.gold -= wm.for_the_shop[i - 1].cost;
        player.weapons.push(wm.for_the_shop[i - 1].clone());
    } else {
        println!("Valitettavasti sinulla ei ole varaa tähän aseeseen.");
    }
}

fn potion_market(player: &mut Player) {
    println!("\nHaluatko ostaa lääkerohtoja? (k / e)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Virhe syötteen lukemisessa");

    if input.trim() == "k" {
        println!("Kuinka monta? Yksi lääkerohto on 20 kultaa.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Virhe syötteen lukemisessa");
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
        } else {
            println!("Rahasi eivät riitä!");
            potion_market(player);
        }
    }
}

fn main() {
    println!("Hei seikkailija! Mikä on nimesi?");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Virhe syötteen lukemisessa");
    let name = name.trim();

    let mut player: Player = Player::new();
    let mut wm: WeaponMap = WeaponMap::new();
    let mm: MonsterMap = MonsterMap::new();

    // Pick the starting weapon, delete the rest
    let mut rng_max: usize = wm.starters.len();
    let starting_weapon: Weapon = wm.starters[rand::rng().random_range(0..rng_max)].clone();
    println!("Hurraa! Ensimmäinen aseesi on {}", starting_weapon.name);

    player.weapons.push(starting_weapon);
    wm.starters = vec![];
    let mut round_counter: i32 = 0;

    loop {
        if player.curnt_hp < 1 {
            break;
        }

        println!("\nPelaajan {} tämänhetkiset attribuutit:", name);
        println!("Elämäpisteet: {}", player.max_hp);
        println!("Rahat: {} kultaa", player.gold);
        println!("Voimakerroin: {}", player.str);
        println!("Lohikäärmeitä voitettu: {}", player.bosses_won);

        round_counter += 1;
        if round_counter % 5 == 0 {
            battle_boss(&mm.bosses[player.bosses_won as usize], &mut player, &wm);
        } else if round_counter % 3 == 0 {
            println!("\nLöydät tiesi ostoksille");
            println!("Sinulla on {} kultaa.", player.gold);
            weapon_market(&mut player, &wm);
            potion_market(&mut player);          
        } else {
            println!("\nLöydät tiesi ostoksille");
            println!("Sinulla on {} kultaa.", player.gold);
            weapon_market(&mut player, &wm);
            potion_market(&mut player);
            
            let mut dungeon_monsters: Vec<Monster> = vec![];
            rng_max = mm.normals.len();
            for mut _j in 0..3 {
                let rng_monster: Monster =
                    mm.normals[rand::rng().random_range(0..rng_max)].clone();
                dungeon_monsters.push(rng_monster);
            }
            battle_normal(&mut player, &mut dungeon_monsters);    
        }
        
        if player.bosses_won == 3 {
            println!("Hurraa! Olet voittanut pelin! Olet mestari!!!! Jee!!!!!");
            break;
        }
    }

}