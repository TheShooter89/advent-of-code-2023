use std::{fs, usize};

use crate::Scanner;

use super::element::{Element, ElementProps};
use super::engine_part::EnginePart;
use super::position::Position;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Schema {
    schema: Vec<Vec<Element>>,
    parts: Vec<EnginePart>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            schema: Vec::new(),
            parts: Vec::new(),
        }
    }

    pub fn from_file(file_path: &str) -> Schema {
        match fs::read_to_string(file_path) {
            Ok(content) => {
                //
                let mut schema_vec: Vec<Vec<Element>> = Vec::new();
                let lines: Vec<&str> = content.lines().collect();

                let mut all_parts: Vec<EnginePart> = Vec::new();
                let mut new_part: Vec<Element> = Vec::new();

                let mut line_number: usize = 0;

                for line in lines {
                    //println!("line nuber: {:?}", line_number);
                    //println!("line: {:?}", line);

                    let mut line_vec: Vec<Element> = Vec::new();
                    let mut line_scanner = Scanner::new(line);

                    //let mut new_part: Vec<Element> = Vec::new();

                    let mut x_line: usize = 0;

                    line_scanner.scan(|character| {
                        let y_line = line_number.clone();

                        if character.eq(&'.') {
                            if !new_part.is_empty() {
                                all_parts.push(EnginePart {
                                    elements: new_part.clone(),
                                });
                                new_part = Vec::new();
                            }

                            line_vec.push(Element::Dot(ElementProps {
                                position: Position {
                                    x: x_line,
                                    y: y_line,
                                },
                                value: character.to_string(),
                                width: 1,
                            }))
                        };

                        if character.is_alphabetic() {
                            line_vec.push(Element::Unknown(ElementProps {
                                position: Position {
                                    x: x_line,
                                    y: y_line,
                                },
                                value: character.to_string(),
                                width: 1,
                            }))
                        };

                        if character.is_numeric() {
                            let new_element = Element::Number(ElementProps {
                                position: Position {
                                    x: x_line,
                                    y: y_line,
                                },
                                value: character.to_string(),
                                width: 1,
                            });

                            new_part.push(new_element.clone());

                            line_vec.push(new_element)
                        }

                        if !character.eq(&'.')
                            && !character.is_numeric()
                            && !character.is_alphabetic()
                        {
                            line_vec.push(Element::Symbol(ElementProps {
                                position: Position {
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
                    println!("-----------\nall_parts: {:#?}", all_parts);
                    schema_vec.push(line_vec);
                    line_number += 1;
                }

                let mut result = Schema {
                    schema: schema_vec,
                    parts: Vec::new(),
                };
                result.parse_parts(all_parts);

                result
            }
            Err(err) => {
                eprintln!("Error in reading file '{}': {}", file_path, err);
                Schema {
                    schema: Vec::new(),
                    parts: Vec::new(),
                }
            }
        }
    }

    pub fn schema(&self) -> &Vec<Vec<Element>> {
        &self.schema
    }

    pub fn parts(&self) -> &Vec<EnginePart> {
        &self.parts
    }

    pub fn get(&self, position: Position) -> Option<&Element> {
        if self.schema.len() == 0 {
            return None;
        }

        if position.y >= self.schema.len() {
            return None;
        }

        if position.x >= self.schema[0].len() {
            return None;
        }

        Some(&self.schema[position.y][position.x])
    }

    pub fn parse_parts(&mut self, parts_list: Vec<EnginePart>) {
        for part in &parts_list {
            for element in &part.elements {
                if self.collides_with_symbol(element.clone()) {
                    self.parts.push(part.clone())
                }
            }
        }
    }

    pub fn has_symbol(&self, position: Position) -> bool {
        //
        match self.get(position) {
            Some(Element::Symbol(_)) => true,
            _ => false,
        }
    }

    pub fn collides_with_symbol(&self, element: Element) -> bool {
        let TOP_LEFT = if element.position().x < 1 {
            element.position().x - 1
        } else {
            0
        };

        // top-left
        if self.has_symbol(Position {
            x: element.position().x - 1,
            y: element.position().y - 1,
        }) {
            return true;
        }

        // left
        if self.has_symbol(Position {
            x: element.position().x - 1,
            y: element.position().y,
        }) {
            return true;
        }

        // bottom-left
        if self.has_symbol(Position {
            x: element.position().x - 1,
            y: element.position().y + 1,
        }) {
            return true;
        }

        // botton-right
        if self.has_symbol(Position {
            x: element.position().x + 1,
            y: element.position().y - 1,
        }) {
            return true;
        }

        // right
        if self.has_symbol(Position {
            x: element.position().x + 1,
            y: element.position().y,
        }) {
            return true;
        }

        // top-right
        if self.has_symbol(Position {
            x: element.position().x + 1,
            y: element.position().y + 1,
        }) {
            return true;
        }

        // top
        if self.has_symbol(Position {
            x: element.position().x,
            y: element.position().y + 1,
        }) {
            return true;
        }

        // bottom
        if self.has_symbol(Position {
            x: element.position().x,
            y: element.position().y - 1,
        }) {
            return true;
        }

        false
    }
}
