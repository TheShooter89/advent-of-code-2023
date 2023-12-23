use std::fs;

#[derive(Debug)]
struct Coordinate {
    pub first: u32,
    pub last: u32,
}

impl Coordinate {
    fn to_int(&self) -> u32 {
        return self.first * 10 + self.last;
    }
}

fn get_coodinate(line: &str) -> Option<Coordinate> {
    if line.len() == 0 {
        return None;
    }
    if line.len() < 2 {
        if line.chars().next().unwrap().is_numeric() {
            return Some(Coordinate {
                first: line.parse().unwrap(),
                last: line.parse().unwrap(),
            });
        }

        return None;
    }

    let mut current_slice = line;

    if current_slice.chars().next().unwrap().is_numeric()
        && current_slice.chars().last().unwrap().is_numeric()
    {
        return Some(Coordinate {
            first: current_slice.chars().next().unwrap().to_digit(10).unwrap(),
            last: current_slice.chars().last().unwrap().to_digit(10).unwrap(),
        });
    }

    if !current_slice.chars().next().unwrap().is_numeric() {
        current_slice = &current_slice[1..];
    }

    if !current_slice.chars().last().unwrap().is_numeric() {
        current_slice = &current_slice[0..current_slice.len() - 1];
    }

    //println!("current_slice is: {:?}", current_slice);

    return get_coodinate(current_slice);
}

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Advent of Code 2023!\n");
    let content = fs::read_to_string("src/bin/input1.txt")?;
    let lines: Vec<&str> = content.lines().collect();
    println!("total lines found: {}", lines.len());
    println!("first line found: {}", lines[1]);
    let test_coord = get_coodinate(lines[1]).unwrap().to_int();
    println!("test_coord is: {:?}", test_coord);

    println!("+++++++++++++");
    println!("parsing coordinates...");

    let mut total = 0;

    for line in 0..lines.len() {
        let current_coordinate = get_coodinate(lines[line]);
        match current_coordinate {
            Some(coord) => {
                println!("current coord: {:?}", coord);
                total += coord.to_int();
            }
            None => println!("No coordinate on this line"),
        }
    }

    println!("sum of all coordinates: {:?}", total);
    Ok(())
}
