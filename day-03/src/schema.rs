use std::{fs, usize};

use super::Scanner;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Schema {
    schema: Vec<Vec<SchemaElement>>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema { schema: Vec::new() }
    }

    pub fn from_file(file_path: &str) -> Schema {
        match fs::read_to_string(file_path) {
            Ok(content) => {
                //
                let mut schema_vec: Vec<Vec<SchemaElement>> = Vec::new();
                let lines: Vec<&str> = content.lines().collect();

                let mut line_number: usize = 0;

                for line in lines {
                    //println!("line nuber: {:?}", line_number);
                    //println!("line: {:?}", line);

                    let mut line_vec: Vec<SchemaElement> = Vec::new();
                    let mut line_scanner = Scanner::new(line);
                    let mut x_line: usize = 0;

                    line_scanner.scan(|character| {
                        let y_line = line_number.clone();

                        if character.eq(&'.') {
                            line_vec.push(SchemaElement::Dot(SchemaElementProps {
                                position: SchemaPosition {
                                    x: x_line,
                                    y: y_line,
                                },
                                value: character.to_string(),
                                width: 1,
                            }))
                        };

                        if character.is_alphabetic() {
                            line_vec.push(SchemaElement::Unknown(SchemaElementProps {
                                position: SchemaPosition {
                                    x: x_line,
                                    y: y_line,
                                },
                                value: character.to_string(),
                                width: 1,
                            }))
                        };

                        if character.is_numeric() {
                            line_vec.push(SchemaElement::Number(SchemaElementProps {
                                position: SchemaPosition {
                                    x: x_line,
                                    y: y_line,
                                },
                                value: character.to_string(),
                                width: 1,
                            }))
                        }

                        x_line += 1;
                        Some(character.clone())
                    });

                    //println!("line_vec: {:?}", line_vec);
                    schema_vec.push(line_vec);
                    line_number += 1;
                }

                Schema { schema: schema_vec }
            }
            Err(err) => {
                eprintln!("Error in reading file '{}': {}", file_path, err);
                Schema { schema: Vec::new() }
            }
        }
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
