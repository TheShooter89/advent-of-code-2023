use super::element::Element;
use super::schema::Schema;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Edges {
    LEFT,
    RIGHT,
    TOP,
    TOP_LEFT,
    TOP_RIGHT,
    BOTTOM,
    BOTTOM_LEFT,
    BOTTOM_RIGHT,
}

impl Edges {
    pub fn compute(&self, element: &Element, schema: &Schema) -> Option<Position> {
        let (el_x, el_y) = (element.position().x, element.position().y);
        let vec_schema = schema.schema();
        let x_length = vec_schema.len();
        let y_length = vec_schema[x_length - 1].len();

        match self {
            Edges::TOP_LEFT => {
                let exit_cases: Vec<bool> = vec![
                    el_x == 0,
                    (el_x >= 1 && el_x < x_length - 1 && el_y == 0),
                    (el_x == x_length - 1 && el_y == 0),
                ];

                for case in exit_cases {
                    if case {
                        return None;
                    }
                }

                Some(Position {
                    x: el_x - 1,
                    y: el_y - 1,
                })
            }
            Edges::TOP => {
                let exit_cases: Vec<bool> = vec![el_x == 0];

                for case in exit_cases {
                    if case {
                        return None;
                    }
                }

                Some(Position {
                    x: el_x - 1,
                    y: el_y,
                })
            }
            Edges::TOP_RIGHT => {
                let exit_cases: Vec<bool> = vec![
                    el_x == 0,
                    (el_x >= 1 && el_x < x_length - 1 && el_y == 0),
                    (el_x >= 1 && el_x < x_length - 1 && el_y == y_length - 1),
                    (el_x == x_length - 1 && el_y == y_length - 1),
                ];

                for case in exit_cases {
                    if case {
                        return None;
                    }
                }

                Some(Position {
                    x: el_x - 1,
                    y: el_y + 1,
                })
            }
            Edges::RIGHT => {
                let exit_cases: Vec<bool> = vec![
                    (el_x == 0 && el_y == y_length - 1),
                    (el_x >= 1 && el_x < x_length - 1 && el_y == y_length - 1),
                    (el_x == x_length - 1 && el_y == y_length - 1),
                ];

                for case in exit_cases {
                    if case {
                        return None;
                    }
                }

                Some(Position {
                    x: el_x,
                    y: el_y + 1,
                })
            }
            Edges::BOTTOM_RIGHT => {
                let exit_cases: Vec<bool> = vec![
                    el_x == x_length - 1,
                    (el_x == 0 && el_y == y_length - 1),
                    (el_x >= 1 && el_x < x_length - 1 && el_y == y_length - 1),
                ];

                for case in exit_cases {
                    if case {
                        return None;
                    }
                }

                Some(Position {
                    x: el_x + 1,
                    y: el_y + 1,
                })
            }
            Edges::BOTTOM => {
                let exit_cases: Vec<bool> = vec![el_x == x_length - 1];

                for case in exit_cases {
                    if case {
                        return None;
                    }
                }

                Some(Position {
                    x: el_x + 1,
                    y: el_y,
                })
            }
            Edges::BOTTOM_LEFT => {
                let exit_cases: Vec<bool> = vec![
                    el_x == x_length - 1,
                    (el_x == 0 && el_y == 0),
                    (el_x >= 1 && el_x < x_length - 1 && el_y == 0),
                ];

                for case in exit_cases {
                    if case {
                        return None;
                    }
                }

                Some(Position {
                    x: el_x + 1,
                    y: el_y - 1,
                })
            }
            Edges::LEFT => {
                let exit_cases: Vec<bool> = vec![
                    (el_x == 0 && el_y == 0),
                    (el_x >= 1 && el_x < x_length - 1 && el_y == 0),
                    (el_x == x_length - 1 && el_y == 0),
                ];

                for case in exit_cases {
                    if case {
                        return None;
                    }
                }

                Some(Position {
                    x: el_x,
                    y: el_y - 1,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_single_edges() {
        let testing_schema = Schema::from_file("src/schema/test_schema_input.txt");
        let top_left_element = testing_schema.get(Position { x: 0, y: 0 }).unwrap();

        let tl_position = Edges::TOP_LEFT;
        let left_position = Edges::LEFT;
        let right_position = Edges::RIGHT;

        assert_eq!(
            tl_position.compute(&top_left_element, &testing_schema),
            None
        );
        assert_eq!(
            left_position.compute(&top_left_element, &testing_schema),
            None
        );
        assert_eq!(
            right_position.compute(&top_left_element, &testing_schema),
            Some(Position { x: 0, y: 1 })
        );
    }

    #[test]
    fn test_edges() {
        let testing_schema = Schema::from_file("src/schema/test_schema_input.txt");

        let cases: Vec<(Edges, Vec<((usize, usize), Option<Position>)>)> = vec![
            (
                Edges::TOP_LEFT,
                vec![
                    ((0, 0), None),
                    ((0, 1), None),
                    ((0, 2), None),
                    ((1, 0), None),
                    ((1, 1), Some(Position { x: 0, y: 0 })),
                    ((1, 2), Some(Position { x: 0, y: 1 })),
                    ((2, 0), None),
                    ((2, 1), Some(Position { x: 1, y: 0 })),
                    ((2, 2), Some(Position { x: 1, y: 1 })),
                ],
            ),
            (
                Edges::TOP,
                vec![
                    ((0, 0), None),
                    ((0, 1), None),
                    ((0, 2), None),
                    ((1, 0), Some(Position { x: 0, y: 0 })),
                    ((1, 1), Some(Position { x: 0, y: 1 })),
                    ((1, 2), Some(Position { x: 0, y: 2 })),
                    ((2, 0), Some(Position { x: 1, y: 0 })),
                    ((2, 1), Some(Position { x: 1, y: 1 })),
                    ((2, 2), Some(Position { x: 1, y: 2 })),
                ],
            ),
            (
                Edges::TOP_RIGHT,
                vec![
                    ((0, 0), None),
                    ((0, 1), None),
                    ((0, 2), None),
                    ((1, 0), None),
                    ((1, 1), Some(Position { x: 0, y: 2 })),
                    ((1, 2), None),
                    ((2, 0), Some(Position { x: 1, y: 1 })),
                    ((2, 1), Some(Position { x: 1, y: 2 })),
                    ((2, 2), None),
                ],
            ),
            (
                Edges::RIGHT,
                vec![
                    ((0, 0), Some(Position { x: 0, y: 1 })),
                    ((0, 1), Some(Position { x: 0, y: 2 })),
                    ((0, 2), None),
                    ((1, 0), Some(Position { x: 1, y: 1 })),
                    ((1, 1), Some(Position { x: 1, y: 2 })),
                    ((1, 2), None),
                    ((2, 0), Some(Position { x: 2, y: 1 })),
                    ((2, 1), Some(Position { x: 2, y: 2 })),
                    ((2, 2), None),
                ],
            ),
            (
                Edges::BOTTOM_RIGHT,
                vec![
                    ((0, 0), Some(Position { x: 1, y: 1 })),
                    ((0, 1), Some(Position { x: 1, y: 2 })),
                    ((0, 2), None),
                    ((1, 0), Some(Position { x: 2, y: 1 })),
                    ((1, 1), Some(Position { x: 2, y: 2 })),
                    ((1, 2), None),
                    ((2, 0), None),
                    ((2, 1), None),
                    ((2, 2), None),
                ],
            ),
            (
                Edges::BOTTOM,
                vec![
                    ((0, 0), Some(Position { x: 1, y: 0 })),
                    ((0, 1), Some(Position { x: 1, y: 1 })),
                    ((0, 2), Some(Position { x: 1, y: 2 })),
                    ((1, 0), Some(Position { x: 2, y: 0 })),
                    ((1, 1), Some(Position { x: 2, y: 1 })),
                    ((1, 2), Some(Position { x: 2, y: 2 })),
                    ((2, 0), None),
                    ((2, 1), None),
                    ((2, 2), None),
                ],
            ),
            (
                Edges::BOTTOM_LEFT,
                vec![
                    ((0, 0), None),
                    ((0, 1), Some(Position { x: 1, y: 0 })),
                    ((0, 2), Some(Position { x: 1, y: 1 })),
                    ((1, 0), None),
                    ((1, 1), Some(Position { x: 2, y: 0 })),
                    ((1, 2), Some(Position { x: 2, y: 1 })),
                    ((2, 0), None),
                    ((2, 1), None),
                    ((2, 2), None),
                ],
            ),
            (
                Edges::LEFT,
                vec![
                    ((0, 0), None),
                    ((0, 1), Some(Position { x: 0, y: 0 })),
                    ((0, 2), Some(Position { x: 0, y: 1 })),
                    ((1, 0), None),
                    ((1, 1), Some(Position { x: 1, y: 0 })),
                    ((1, 2), Some(Position { x: 1, y: 1 })),
                    ((2, 0), None),
                    ((2, 1), Some(Position { x: 2, y: 0 })),
                    ((2, 2), Some(Position { x: 2, y: 1 })),
                ],
            ),
        ];

        println!("-----");
        for (case_index, (edge, data)) in cases.iter().enumerate() {
            //
            for (result_index, (target_coordinates, result_element_position)) in
                data.iter().enumerate()
            {
                //
                println!("case_index: {}, result_index: {}", case_index, result_index);
                println!("target_coordinates: {:?}", target_coordinates);
                println!("edge: {:?}", edge);
                let extracted_element = testing_schema
                    .get(Position {
                        x: target_coordinates.0,
                        y: target_coordinates.1,
                    })
                    .unwrap();
                println!("extracted_element: {:?}", extracted_element);

                let computed_position = edge.compute(&extracted_element, &testing_schema);
                println!("computed_position: {:?}", computed_position);
                //
                println!("-----");
                assert_eq!(&computed_position, result_element_position);
            }
        }
    }
}
