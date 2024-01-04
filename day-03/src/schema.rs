#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Schema {
    schema: Vec<Vec<SchemaElement>>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema { schema: Vec::new() }
    }

    pub fn from_file(file_path: &str) -> Schema {
        Schema { schema: Vec::new() }
    }

    pub fn schema(&self) -> &Vec<Vec<SchemaElement>> {
        &self.schema
    }

    pub fn get(&self, position: SchemaPosition) -> Option<&SchemaElement> {
        if self.schema.len() == 0 {
            return None;
        }

        if position.y >= self.schema.len() {
            return None;
        }

        if position.x >= self.schema[0].len() {
            return None;
        }

        Some(&self.schema[position.x][position.y])
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SchemaPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SchemaElementProps {
    pub position: SchemaPosition,
    pub value: String,
    pub width: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SchemaElement {
    Dot(SchemaElementProps),
    Symbol(SchemaElementProps),
    Number(SchemaElementProps),
    Unknown(SchemaElementProps),
}

impl SchemaElement {
    pub fn position(&self) -> &SchemaPosition {
        match self {
            SchemaElement::Dot(props) => &props.position,
            SchemaElement::Symbol(props) => &props.position,
            SchemaElement::Number(props) => &props.position,
            SchemaElement::Unknown(props) => &props.position,
        }
    }

    pub fn value(&self) -> &String {
        match self {
            SchemaElement::Dot(props) => &props.value,
            SchemaElement::Symbol(props) => &props.value,
            SchemaElement::Number(props) => &props.value,
            SchemaElement::Unknown(props) => &props.value,
        }
    }

    pub fn width(&self) -> usize {
        match self {
            SchemaElement::Dot(props) => props.width,
            SchemaElement::Symbol(props) => props.width,
            SchemaElement::Number(props) => props.width,
            SchemaElement::Unknown(props) => props.width,
        }
    }
}
