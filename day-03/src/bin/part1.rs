use std::fs;

use day03::{Element, ElementProps, Engine, EnginePart, Position, Schema};

fn print_title() {
    println!("Advent of Code 2023 - Day 02 [PART 1]\n");
    println!("coded with 💛️💙️ by tanque");
    println!("---------------------------------------");
}

fn main() -> Result<(), std::io::Error> {
    print_title();

    let content = fs::read_to_string("src/bin/input1.txt")?;
    //let content = fs::read_to_string("src/bin/test_input.txt")?;
    let lines: Vec<&str> = content.lines().collect();

    let mut test_schema = Schema::from_file("src/bin/test_input.txt");
    println!("-----------------");
    println!("test_schema is: {:?}", test_schema);
    println!("-----------------");
    println!(
        "test_schema.schema[0][0] is: {:?}",
        test_schema.get(Position { x: 0, y: 0 })
    );

    println!(
        "test_schema(x: 5, y: 8) is: {:?}",
        test_schema.get(Position { x: 5, y: 8 }).unwrap()
    );
    println!(
        "test_schema.has_symbol in (x: 5, y: 8) is: {:?}",
        test_schema.has_symbol(Position { x: 5, y: 8 })
    );

    println!(
        "test_schema(x: 3, y: 2) is: {:?}",
        test_schema.get(Position { x: 3, y: 2 })
    );

    println!(
        "test_schema collides with symbol in (x: 3, y: 2) is: {:?}",
        test_schema.collides_with_symbol(Element::Number(ElementProps {
            position: Position { x: 3, y: 2 },
            value: "5".to_string(),
            width: 1
        }))
    );

    println!(
        "test_schema collides with symbol in (x: 3, y: 9) is: {:?}",
        test_schema.collides_with_symbol(Element::Number(ElementProps {
            position: Position { x: 3, y: 9 },
            value: "5".to_string(),
            width: 1
        }))
    );

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
    //test_schema.parse_parts(vec![mock_part]);
    println!("test_schema.parse_parts (x: 9, y: 9) is: {:?}", test_schema);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        //let content = fs::read_to_string("src/bin/input1.txt").unwrap();
        let content = fs::read_to_string("src/bin/test_input.txt").unwrap();
        let lines: Vec<&str> = content.lines().collect();

        //assert_eq!(games_list.id_sum, 2563);
        assert_eq!(3, 8);
    }
}
