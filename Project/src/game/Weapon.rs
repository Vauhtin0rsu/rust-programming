use super::Element::Element;

#[derive(Clone)]
pub struct Weapon {
    pub name: String,
    pub element: Element,
    pub str: f32,
    pub cost: i32,
}

impl Weapon {
    pub fn new(name: String, element: Element, str: f32, cost: i32) -> Self {
        Self {
            name,
            element,
            str,
            cost
        }
    }
}