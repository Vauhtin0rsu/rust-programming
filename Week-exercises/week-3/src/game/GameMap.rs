use super::Country::Country;

pub struct GameMap {
    pub countries: Vec<Country>,
}

impl GameMap {
    pub fn new() -> Self {
        Self {
            countries: vec![
                Country::new(String::from("Finland"), 5600000, 900000, vec![], false),
                Country::new(String::from("Sweden"), 10000000, 200000, vec![], false),
                Country::new(String::from("Norway"), 5500000, 100000, vec![], false),
                Country::new(String::from("Denmark"), 6000000, 50000, vec![], false),
            ]
        }
    }

    pub fn list_countries(&self) {
        let mut i: i32 = 1;
        for country in &self.countries {
            println!("{}) {}", i, country.get_name());
            i = i + 1;
        }
    }

    pub fn get_country_by_index(&self, i: i32) -> &Country {
        let index: usize = (i - 1) as usize;
        &self.countries[index]
    }

}