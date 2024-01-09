use super::position::Position;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ElementProps {
    pub position: Position,
    pub value: String,
    pub width: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Element {
    Dot(ElementProps),
    Symbol(ElementProps),
    Number(ElementProps),
    Unknown(ElementProps),
}

impl Element {
    pub fn position(&self) -> &Position {
        match self {
            Element::Dot(props) => &props.position,
            Element::Symbol(props) => &props.position,
            Element::Number(props) => &props.position,
            Element::Unknown(props) => &props.position,
        }
    }

    pub fn value(&self) -> &String {
        match self {
            Element::Dot(props) => &props.value,
            Element::Symbol(props) => &props.value,
            Element::Number(props) => &props.value,
            Element::Unknown(props) => &props.value,
        }
    }

    pub fn width(&self) -> usize {
        match self {
            Element::Dot(props) => props.width,
            Element::Symbol(props) => props.width,
            Element::Number(props) => props.width,
            Element::Unknown(props) => props.width,
        }
    }
}
