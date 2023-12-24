use std::fs;

use day02::Cube;

fn main() -> Result<(), std::io::Error> {
    println!("Advent of Code 2023 - Day 02 [PART 1]\n");
    let content = fs::read_to_string("src/bin/input1.txt")?;
    //let content = fs::read_to_string("src/bin/test_input.txt")?;
    let lines: Vec<&str> = content.lines().collect();
    println!("total lines found: {}", lines.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let content = fs::read_to_string("src/bin/test_input.txt").unwrap();
        let lines: Vec<&str> = content.lines().collect();

        assert_eq!("3", "3");
    }
}
