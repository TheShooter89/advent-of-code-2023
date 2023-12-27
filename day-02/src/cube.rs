use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cube {
    Red(i32),
    Green(i32),
    Blue(i32),
}

impl Cube {
    //pub fn set(&mut self, count: i32) {
    //    match self {
    //        Cube::Red(val) => *val + count,
    //        Cube::Green(val) => *val + count,
    //        Cube::Blue(val) => *val + count,
    //    };
    //}

    pub fn set(&mut self, count: i32) -> Self {
        //
        match *self {
            Cube::Red(val) => *self = Cube::Red(val + count),
            Cube::Green(val) => *self = Cube::Green(val + count),
            Cube::Blue(val) => *self = Cube::Blue(val + count),
            //Cube::Red(val) => Self::Red(*val + count),
            //Cube::Green(val) => Self::Green(*val + count),
            //Cube::Blue(val) => Self::Blue(*val + count),
            //Cube::Red(val) => *val + count,
            //Cube::Green(val) => *val + count,
            //Cube::Blue(val) => *val + count,
        };
        *self
    }

    pub fn count(&self) -> i32 {
        match *self {
            Cube::Red(val) => val,
            Cube::Green(val) => val,
            Cube::Blue(val) => val,
        }
    }

    pub fn parse_str(line: &str) -> Option<Cube> {
        let elements: Vec<&str> = line.trim().split_whitespace().collect();

        if elements.len() != 2 {
            return None;
        }

        if elements[0].parse::<i32>().is_err() {
            return None;
        }

        let cube_number = elements[0].parse::<i32>().unwrap();

        match elements[1] {
            "red" => Some(Cube::Red(cube_number)),
            "green" => Some(Cube::Green(cube_number)),
            "blue" => Some(Cube::Blue(cube_number)),
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
        let cube_from_str = Cube::parse_str("4 green").unwrap();
        assert_eq!(cube_from_str, control_cube);

        let control_cube = Cube::Blue(876);
        let cube_from_str = Cube::parse_str("876 blue").unwrap();
        assert_eq!(cube_from_str, control_cube);

        let control_cube = Cube::Red(56);
        let cube_from_str = Cube::parse_str("56 red").unwrap();
        assert_eq!(cube_from_str, control_cube);
    }

    #[test]
    fn test_cube_from_enum() {
        let control_cube = Cube::Green(4);
        assert_eq!(control_cube, Cube::Green(4));
        assert_eq!(control_cube.count(), 4);

        let control_cube = Cube::Blue(876);
        assert_eq!(control_cube, Cube::Blue(876));
        assert_eq!(control_cube.count(), 876);

        let control_cube = Cube::Red(56);
        assert_eq!(control_cube, Cube::Red(56));
        assert_eq!(control_cube.count(), 56);
    }

    #[test]
    fn test_cube_set() {
        let mut control_cube = Cube::Green(0);
        control_cube.set(4);
        assert_eq!(control_cube, Cube::Green(4));
        assert_eq!(control_cube.count(), 4);

        let mut control_cube = Cube::Blue(0);
        control_cube.set(876);
        assert_eq!(control_cube, Cube::Blue(876));
        assert_eq!(control_cube.count(), 876);

        let mut control_cube = Cube::Red(0);
        control_cube.set(56);
        assert_eq!(control_cube, Cube::Red(56));
        assert_eq!(control_cube.count(), 56);
    }

    #[test]
    #[should_panic]
    fn test_bad_cube_str() {
        let cube_from_str = Cube::parse_str("4 ygreen").unwrap();
        let cube_from_str = Cube::parse_str("yigi4 green").unwrap();
        let cube_from_str = Cube::parse_str("4green").unwrap();
    }
}
