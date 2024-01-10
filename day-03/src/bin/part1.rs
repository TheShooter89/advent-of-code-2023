use std::fs;

use day03::{Element, ElementProps, Engine, EnginePart, Position, Schema};

fn print_title() {
    println!("Advent of Code 2023 - Day 02 [PART 1]\n");
    println!("coded with 💛️💙️ by tanque");
    println!("---------------------------------------");
}

fn main() -> Result<(), std::io::Error> {
    print_title();

    // let source_file = "src/bin/input1.txt";
    let source_file = "src/bin/test_input.txt";

    let mut test_schema = Schema::from_file(source_file);
    println!("\nparsing schema from file \"{}\"...\n", source_file);
    println!("-----------------");
    //println!("test_schema is: {:#?}", test_schema);
    println!("-----------------");

    println!("\n\n# STATISTICS #");
    println!("-----------------\n");

    let parsed_schema = test_schema.schema();
    let schema_width = parsed_schema.len();
    let schema_height = parsed_schema[schema_width - 1].len();

    let parsed_engine_parts = test_schema.parts();
    let mut engine_parts_sum = 0;

    for part in parsed_engine_parts {
        println!("part: {:?}", part);
        println!("part value: {}", part.value());
        engine_parts_sum += part.value();
    }

    println!("engine schema width: {}", schema_width);
    println!("engine schema height: {}", schema_height);
    println!(
        "total engine parts on schema: {}",
        parsed_engine_parts.len()
    );
    println!("\nsum of engine parts values: {}", engine_parts_sum);
    println!("-----------------");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let mut test_schema = Schema::from_file("src/bin/test_input.txt");
        //
    }
}
