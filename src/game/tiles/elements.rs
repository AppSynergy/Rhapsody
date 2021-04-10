#[derive(Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Debug)]
pub enum Element {
    Air,
    Earth,
    Fire,
    Water,
}

impl Element {
    pub fn to_label(&self) -> &str {
        match self {
            Element::Air => "A",
            Element::Earth => "E",
            Element::Fire => "F",
            Element::Water => "W",
        }
    }

    pub fn to_icon(&self) -> &str {
        match self {
            Element::Air => "🌩 ",
            Element::Earth => "🌏",
            Element::Fire => "🔥",
            Element::Water => "💧",
        }
    }
}
