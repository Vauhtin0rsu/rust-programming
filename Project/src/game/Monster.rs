use super::Element::Element;

#[derive(Clone)]
pub struct Monster {
    pub name: String,
    pub hp: i32,
    pub str: f32,
    pub weakness: Element,
    pub gold: i32,
}

impl Monster {
    pub fn new(name: String, hp: i32, str: f32, weakness: Element, gold: i32) -> Self {
        Self {
            name: name,
            hp: hp,
            str: str,
            weakness: weakness,
            gold: gold,
        }
    }
}
