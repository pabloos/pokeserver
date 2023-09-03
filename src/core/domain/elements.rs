use strum_macros::Display;

#[derive(Copy, Clone, Display, PartialEq)]
pub enum Element {
    Grass, Fire, Water, Electric, Normal,
}

impl Element {
    pub fn weak_to(self, other: Element) -> bool {
        matches!((self, other), 
            (Element::Grass, Element::Fire) |
            (Element::Fire, Element::Water) |
            (Element::Water, Element::Electric) |
            (Element::Electric, Element::Grass))
    }

    pub fn strong_to(self, other: Element) -> bool {
        matches!((self, other), 
            (Element::Water, Element::Fire) | 
            (Element::Electric, Element::Water) |
            (Element::Fire, Element::Grass) |
            (Element::Grass, Element::Electric))
    }
}
