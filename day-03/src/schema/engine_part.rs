use super::element::Element;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EnginePart {
    //pub label: String,
    pub elements: Vec<Element>,
}

impl EnginePart {
    pub fn value(&self) -> i32 {
        //
        let mut total_value = 0;

        for element in &self.elements {
            total_value *= 10;
            total_value += element.value().parse().unwrap_or(0);
        }

        total_value
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use std::fs;

    #[test]
    fn test_engine_part() {
        //
        let mock_part = EnginePart {
            elements: vec![
                Element::Number(ElementProps {
                    position: Position { x: 2, y: 2 },
                    value: "3".to_string(),
                    width: 1,
                }),
                Element::Number(ElementProps {
                    position: Position { x: 3, y: 2 },
                    value: "5".to_string(),
                    width: 1,
                }),
                Element::Number(ElementProps {
                    position: Position { x: 8, y: 5 },
                    value: "8".to_string(),
                    width: 1,
                }),
            ],
        };
        assert_eq!(mock_part.value(), 358);
    }
}
