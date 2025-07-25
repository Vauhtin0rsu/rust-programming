use super::Monster::Monster;
use super::Element::Element;

pub struct MonsterMap {
    pub normals: Vec<Monster>,
    pub bosses: Vec<Monster>,
}

impl MonsterMap {
    pub fn new() -> Self {
        Self {
            normals: vec![
                Monster::new("Hiisi".to_string(), 30, 0.8, Element::Tuli, 20),
                Monster::new("Monni".to_string(), 50, 1.0, Element::Luonto, 70),
                Monster::new("Hämis".to_string(), 100, 0.96, Element::Tuli, 50),
                Monster::new("Jättiläinen".to_string(), 150, 1.15, Element::Vesi, 60),
                Monster::new("Pahat possut".to_string(), 80, 1.02, Element::Tuli, 10),
                Monster::new("Kelarotta".to_string(), 25, 0.5, Element::Luonto, 2),
                Monster::new("Röökielementaali".to_string(), 30, 0.78, Element::Vesi, 15),
                Monster::new("Norppavalas".to_string(), 130, 1.07, Element::Luonto, 54),
                Monster::new("Vauvalohari".to_string(), 65, 1.0, Element::Vesi, 40)
            ],
            bosses: vec![
                Monster::new("Lohikäärme".to_string(), 354, 2.5, Element::Vesi, 2000),
                Monster::new("Vesikäärme".to_string(), 354, 2.56, Element::Luonto, 2000),
                Monster::new("Vuorikäärme".to_string(), 444, 2.64, Element::Tuli, 2500)
            ],
        }
    }
}
