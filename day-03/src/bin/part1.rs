use std::fs;

use day03::{Engine, Schema, SchemaPosition};

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

    let test_schema = Schema::from_file("src/bin/test_input.txt");
    println!("test_schema is: {:?}", test_schema);
    println!(
        "test_schema.schema[0][0] is: {:?}",
        test_schema.get(SchemaPosition { x: 0, y: 0 })
    );

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
