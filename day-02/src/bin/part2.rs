use crate::CalibrationSet::Coordinates;
use std::{fs, str::FromStr};

#[derive(Debug, Clone)]
enum Digit {
    EMPTY,
    NUMBER(String),
    NaN,
}

impl Digit {
    fn new() -> Digit {
        Digit::EMPTY
    }

    fn from_str(string_slice: &str) -> Digit {
        if string_slice.len() == 0 {
            return Digit::EMPTY;
        }

        match string_slice {
            "1" => Digit::NUMBER("1".to_string()),
            "2" => Digit::NUMBER("2".to_string()),
            "3" => Digit::NUMBER("3".to_string()),
            "4" => Digit::NUMBER("4".to_string()),
            "5" => Digit::NUMBER("5".to_string()),
            "6" => Digit::NUMBER("6".to_string()),
            "7" => Digit::NUMBER("7".to_string()),
            "8" => Digit::NUMBER("8".to_string()),
            "9" => Digit::NUMBER("9".to_string()),
            "one" => Digit::NUMBER("one".to_string()),
            "two" => Digit::NUMBER("two".to_string()),
            "three" => Digit::NUMBER("three".to_string()),
            "four" => Digit::NUMBER("four".to_string()),
            "five" => Digit::NUMBER("five".to_string()),
            "six" => Digit::NUMBER("six".to_string()),
            "seven" => Digit::NUMBER("seven".to_string()),
            "eight" => Digit::NUMBER("eight".to_string()),
            "nine" => Digit::NUMBER("nine".to_string()),
            _ => Digit::NaN,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Digit::EMPTY => 0,
            Digit::NaN => 0,
            Digit::NUMBER(val) => match val.as_str() {
                "1" => 1,
                "2" => 2,
                "3" => 3,
                "4" => 4,
                "5" => 5,
                "6" => 6,
                "7" => 7,
                "8" => 8,
                "9" => 9,
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => 0,
            },
        }
    }

    fn width(&self) -> usize {
        match self {
            Digit::EMPTY => 0,
            Digit::NaN => 0,
            Digit::NUMBER(val) => val.chars().count(),
        }
    }

    fn to_string(&self) -> String {
        match *self {
            Digit::NUMBER(ref val) => val.to_string(),
            Digit::NaN => "NaN".to_string(),
            Digit::EMPTY => "0".to_string(),
        }
    }

    fn is_number(&self) -> bool {
        match *self {
            Digit::NUMBER(_) => true,
            Digit::NaN => false,
            Digit::EMPTY => false,
        }
    }
}

#[derive(Debug)]
struct Line {
    pub content: String,
    pub head: Digit,
    pub tail: Digit,
}

impl Line {
    fn new() -> Line {
        Line {
            content: "".to_string(),
            head: Digit::EMPTY,
            tail: Digit::EMPTY,
        }
    }

    fn is_empty(&self) -> bool {
        match (self.content == "".to_string(), &self.head, &self.tail) {
            (true, Digit::EMPTY, Digit::EMPTY) => true,
            (_, _, _) => false,
        }
    }

    fn parse_head(line: &str) -> Digit {
        if line.len() == 0 {
            return Digit::EMPTY;
        }

        for i in 0..5 {
            if i < line.len() {
                let digit = Digit::from_str(&line[0..i + 1]);
                match digit {
                    Digit::NUMBER(_) => return digit,
                    _ => (),
                }
            }
        }

        Digit::NaN
    }

    fn parse_tail(line: &str) -> Digit {
        if line.len() == 0 {
            return Digit::EMPTY;
        }

        for i in 0..5 {
            if i < line.len() {
                let digit = Digit::from_str(&line[line.len() - i - 1..line.len()]);
                match digit {
                    Digit::NUMBER(_) => return digit,
                    _ => (),
                }
            }
        }

        Digit::NaN
    }

    fn from_str(line: &str) -> Line {
        if line.len() < 1 {
            return Line::new();
        }

        if line.len() == 1 {
            match Digit::from_str(line) {
                Digit::NUMBER(val) => {
                    return Line {
                        content: line.to_string(),
                        head: Digit::NUMBER(val.clone()),
                        tail: Digit::NUMBER(val),
                    }
                }
                Digit::NaN => {
                    return Line {
                        content: line.to_string(),
                        head: Digit::NaN,
                        tail: Digit::NaN,
                    }
                }
                _ => (),
            }
        }

        if line.len() <= 5 {
            match Digit::from_str(line) {
                Digit::NUMBER(val) => {
                    return Line {
                        content: line.to_string(),
                        head: Digit::NUMBER(val.clone()),
                        tail: Digit::NUMBER(val),
                    }
                }
                _ => (),
            }
        }

        match (Line::parse_head(line), Line::parse_tail(line)) {
            (Digit::NaN, Digit::NUMBER(tail)) => Line::from_str(&line[1..line.len()]),
            (Digit::NUMBER(head), Digit::NaN) => Line::from_str(&line[..line.len() - 1]),
            (Digit::NUMBER(head), Digit::NUMBER(tail)) => Line {
                content: line.to_string(),
                head: Digit::NUMBER(head),
                tail: Digit::NUMBER(tail),
            },
            (Digit::NaN, Digit::NaN) => Line::from_str(&line[1..line.len() - 1]),
            _ => Line::new(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Coordinate {
    FULL(i32, i32),
    HALF(i32),
    EMPTY,
}

impl Coordinate {
    fn new() -> Coordinate {
        Coordinate::EMPTY
    }

    fn add(&self, digit: i32) -> Coordinate {
        match *self {
            Coordinate::EMPTY => Coordinate::HALF(digit),
            Coordinate::HALF(first_coord) => Coordinate::FULL(first_coord, digit),
            Coordinate::FULL(first_coord, last_coord) => Coordinate::FULL(first_coord, last_coord),
        }
    }

    fn from_str(line: &str) -> Coordinate {
        //
        let mut coordinates = Coordinate::new();
        let current_line = Line::from_str(line);

        if current_line.is_empty() {
            return Coordinate::EMPTY;
        }

        coordinates = coordinates.add(current_line.head.value());
        coordinates = coordinates.add(current_line.tail.value());

        coordinates
    }

    fn to_int(&self) -> i32 {
        match *self {
            Coordinate::EMPTY => 0,
            Coordinate::HALF(first_coord) => first_coord * 10 + first_coord,
            Coordinate::FULL(first_coord, last_coord) => first_coord * 10 + last_coord,
        }
    }
}

#[derive(Debug, Clone)]
enum CalibrationSet {
    EMPTY,
    Coordinates(Vec<Coordinate>),
}

impl CalibrationSet {
    fn new() -> CalibrationSet {
        CalibrationSet::EMPTY
    }

    fn add(&self, coordinate: Coordinate) -> CalibrationSet {
        match *self {
            CalibrationSet::EMPTY => CalibrationSet::Coordinates(vec![coordinate]),
            CalibrationSet::Coordinates(ref coordinates_list) => {
                let mut new_list = coordinates_list.to_vec();
                new_list.push(coordinate);
                CalibrationSet::Coordinates(new_list)
            }
        }
    }

    fn compute(&self) -> Result<i32, &str> {
        match *self {
            CalibrationSet::EMPTY => Err("compute error: calibration set is empty"),
            CalibrationSet::Coordinates(ref coordinates_list) => {
                let new_list = coordinates_list.to_vec();
                let mut result: i32 = 0;
                for coord in new_list {
                    result += coord.to_int()
                }
                Ok(result)
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Advent of Code 2023!\n");
    let content = fs::read_to_string("src/bin/input1.txt")?;
    //let content = fs::read_to_string("src/bin/test_input.txt")?;
    let lines: Vec<&str> = content.lines().collect();
    println!("total lines found: {}", lines.len());

    let mut calibration = CalibrationSet::new();

    for line in lines {
        let parsed_line = Coordinate::from_str(line);
        calibration = calibration.add(parsed_line);
        println!("-----------");
        println!("line: {:?}", line);
        println!("parsed_line: {:?}", parsed_line);
        println!("-----------");
    }

    println!("calibration: {:?}", calibration);

    let solution = calibration.compute();
    println!("SOLUTION IS: {:?}", solution.unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let content = fs::read_to_string("src/bin/test_input.txt").unwrap();
        let lines: Vec<&str> = content.lines().collect();

        let mut calibration = CalibrationSet::new();

        for line in lines {
            let parsed_line = Coordinate::from_str(line);
            calibration = calibration.add(parsed_line);
        }

        let solution = calibration.compute();
        assert_eq!(solution.unwrap(), 281);
    }
}
