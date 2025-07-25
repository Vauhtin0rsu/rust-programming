use super::Weapon::Weapon;

pub struct Player {
    pub max_hp: i32,
    pub curnt_hp: i32,
    pub str: f32,
    pub gold: i32,
    pub potions: i32,
    pub weapons: Vec<Weapon>,
    pub bosses_won: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            max_hp: 200,
            curnt_hp: 200,
            str: 1.0,
            gold: 0,
            potions: 5,
            weapons: vec![],
            bosses_won: 0,
        }
    }


}