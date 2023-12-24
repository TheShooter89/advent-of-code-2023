#[derive(Debug, PartialEq, Eq)]
pub enum Cube {
    Red(i32),
    Green(i32),
    Blue(i32),
}

impl Cube {
    pub fn count(&self) -> i32 {
        match *self {
            Cube::Red(val) => val,
            Cube::Green(val) => val,
            Cube::Blue(val) => val,
        }
    }

    pub fn from_str(line: &str) -> Option<Cube> {
        let mut cube_quantity = 0;
        let mut cube_type = "";

        for i in 0..line.len() {
            if &line[i..i + 1] == " " {
                cube_quantity = line[..i].parse().unwrap();
                cube_type = &line[i + 1..line.len()];
                break;
            }
        }

        match cube_type {
            "red" => Some(Cube::Red(cube_quantity)),
            "green" => Some(Cube::Green(cube_quantity)),
            "blue" => Some(Cube::Blue(cube_quantity)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_from_str() {
        let control_cube = Cube::Green(4);
        let cube_from_str = Cube::from_str("4 green").unwrap();
        assert_eq!(cube_from_str, control_cube);
    }

    #[test]
    #[should_panic]
    fn test_bad_cube_str() {
        let cube_from_str = Cube::from_str("4 ygreen").unwrap();
        let cube_from_str = Cube::from_str("yigi4 green").unwrap();
        let cube_from_str = Cube::from_str("4green").unwrap();
    }
}
