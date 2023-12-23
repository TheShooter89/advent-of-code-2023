use crate::CalibrationSet::Coordinates;
use std::{fs, str::FromStr};

#[derive(Debug, Copy, Clone)]
enum Digit {
    EMPTY,
    NUMBER(i32),
    NUMBER_SET(i32, i32),
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

        if string_slice.len() == 2 {
            let first = Digit::from_str(&string_slice[..1]);
            let second = Digit::from_str(&string_slice[1..]);

            if first.is_number() && second.is_number() {
                return Digit::NUMBER_SET(first.value(), second.value());
            }

            if first.is_number() {
                return Digit::NUMBER(first.value());
            }

            if second.is_number() {
                return Digit::NUMBER(second.value());
            }
        }

        match string_slice {
            "1" => Digit::NUMBER(1),
            "2" => Digit::NUMBER(2),
            "3" => Digit::NUMBER(3),
            "4" => Digit::NUMBER(4),
            "5" => Digit::NUMBER(5),
            "6" => Digit::NUMBER(6),
            "7" => Digit::NUMBER(7),
            "8" => Digit::NUMBER(8),
            "9" => Digit::NUMBER(9),
            "one" => Digit::NUMBER(1),
            "two" => Digit::NUMBER(2),
            "three" => Digit::NUMBER(3),
            "four" => Digit::NUMBER(4),
            "five" => Digit::NUMBER(5),
            "six" => Digit::NUMBER(6),
            "seven" => Digit::NUMBER(7),
            "eight" => Digit::NUMBER(8),
            "nine" => Digit::NUMBER(9),
            _ => {
                if string_slice.len() <= 5 {
                    let mut sub_result = Digit::NaN;

                    for i in 1..string_slice.len() {
                        let left_edge = Digit::from_str(&string_slice[0..string_slice.len() - i]);
                        let right_edge = Digit::from_str(&string_slice[i..string_slice.len()]);

                        if left_edge.is_number() && right_edge.is_number() {
                            sub_result = Digit::NUMBER_SET(left_edge.value(), right_edge.value())
                        };

                        if left_edge.is_number() {
                            sub_result = Digit::NUMBER(left_edge.value())
                        }

                        if right_edge.is_number() {
                            sub_result = Digit::NUMBER(right_edge.value())
                        }
                    }
                    sub_result
                } else {
                    let mut sub_result = Digit::NaN;
                    let mut sub_slice = &string_slice;

                    let left_edge_str = &sub_slice[..5];
                    let right_edge_str = &sub_slice[sub_slice.len() - 5..string_slice.len()];

                    let left_edge = Digit::from_str(left_edge_str);
                    let right_edge = Digit::from_str(right_edge_str);

                    if left_edge.is_number() && right_edge.is_number() {
                        sub_result = Digit::NUMBER_SET(left_edge.value(), right_edge.value());
                        return sub_result;
                    }

                    if left_edge.is_number() {
                        for i in 1..5 {
                            let resized_slice = &sub_slice[..sub_slice.len() - i];
                            sub_result = Digit::from_str(resized_slice);
                        }
                        return sub_result;
                    }

                    if right_edge.is_number() {
                        for i in 1..5 {
                            let resized_slice = &sub_slice[i..sub_slice.len()];
                            sub_result = Digit::from_str(resized_slice);
                            println!("resized_slice is: {:?}", resized_slice);
                        }
                        return sub_result;
                    }

                    if !left_edge.is_number() && !right_edge.is_number() {
                        let resized_slice = &sub_slice[1..sub_slice.len() - 1];
                        sub_result = Digit::from_str(resized_slice);
                        return sub_result;
                    }

                    sub_result
                }
            }
        }
    }

    fn value(&self) -> i32 {
        match *self {
            Digit::NUMBER(val) => val,
            Digit::NaN => 0,
            Digit::EMPTY => 0,
            Digit::NUMBER_SET(one, two) => one * 10 + two,
        }
    }

    fn width(digit: &str) -> u32 {
        match digit {
            "1" => 1,
            "2" => 1,
            "3" => 1,
            "4" => 1,
            "5" => 1,
            "6" => 1,
            "7" => 1,
            "8" => 1,
            "9" => 1,
            "one" => 3,
            "two" => 3,
            "three" => 5,
            "four" => 4,
            "five" => 4,
            "six" => 3,
            "seven" => 5,
            "eight" => 5,
            "nine" => 4,
            _ => 0,
        }
    }

    fn elements(&self) -> (i32, i32) {
        match *self {
            Digit::NUMBER(val) => (val, val),
            Digit::NaN => (0, 0),
            Digit::EMPTY => (0, 0),
            Digit::NUMBER_SET(one, two) => (one, two),
        }
    }

    fn to_string(&self) -> String {
        match *self {
            Digit::NUMBER(val) => val.to_string(),
            Digit::NUMBER_SET(left, right) => {
                format!("{left}, {right}")
            }
            Digit::NaN => "NaN".to_string(),
            Digit::EMPTY => "0".to_string(),
        }
    }

    fn is_number(&self) -> bool {
        match *self {
            Digit::NUMBER(_) => true,
            Digit::NUMBER_SET(_, _) => false,
            Digit::NaN => false,
            Digit::EMPTY => false,
        }
    }

    fn is_number_set(&self) -> bool {
        match *self {
            Digit::NUMBER(_) => false,
            Digit::NUMBER_SET(_, _) => true,
            Digit::NaN => false,
            Digit::EMPTY => false,
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
        let digit = Digit::from_str(line);

        match digit {
            Digit::NUMBER_SET(first, last) => {
                //let mut coordinates = Coordinate::new();
                coordinates = coordinates.add(first);
                coordinates = coordinates.add(last);

                coordinates
            }
            Digit::NUMBER(val) => {
                //let mut coordinates = Coordinate::new();
                coordinates = coordinates.add(val);

                coordinates
            }
            _ => coordinates,
        }
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
                let mut new_list = coordinates_list.to_vec();
                let mut result: i32 = 0;
                for coord in new_list {
                    result += coord.to_int()
                }
                return Ok(result);
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Advent of Code 2023!\n");
    //  let content = fs::read_to_string("src/bin/input1.txt")?;
    let content = fs::read_to_string("src/bin/test_input.txt")?;
    let lines: Vec<&str> = content.lines().collect();
    println!("total lines found: {}", lines.len());
    println!("first line found: {}", lines[1]);

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
