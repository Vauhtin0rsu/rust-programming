use super::Weapon::Weapon;
use super::Element::Element;

pub struct WeaponMap {
    pub starters: Vec<Weapon>,
    pub for_the_shop: Vec<Weapon>,
    pub legendary: Vec<Weapon>,
}

impl WeaponMap {
    pub fn new() -> Self {
        Self {
            starters: vec![
                Weapon::new("Molotovin cocktailit".to_string(), Element::Tuli, 1.2, 0),
                Weapon::new("Vesi-ilmapallot".to_string(), Element::Vesi, 1.1, 0),
                Weapon::new("Myrkkynuolet".to_string(), Element::Luonto, 1.3, 0)
            ],
            for_the_shop: vec![
                Weapon::new("Tulimiekka".to_string(), Element::Tuli, 1.2, 150),
                Weapon::new("Jääkeihäs".to_string(), Element::Vesi, 1.2, 170),
                Weapon::new("Myrkyllisemmät nuolet".to_string(), Element::Luonto, 1.2, 200)
            ],
            legendary: vec![
                Weapon::new("Liekehtivä moukari".to_string(), Element::Tuli, 2.2, 0),
                Weapon::new("Poseidonin kolmiterä".to_string(), Element::Vesi, 2.14, 0),
                Weapon::new("Tosi myrkylliset nuolet".to_string(), Element::Luonto, 2.0, 0)
            ],
        }
    }
}
