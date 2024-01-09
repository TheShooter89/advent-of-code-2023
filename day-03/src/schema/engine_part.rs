use super::element::Element;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EnginePart {
    //pub label: String,
    pub elements: Vec<Element>,
}
