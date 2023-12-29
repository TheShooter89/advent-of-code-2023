use std::error::Error;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subset {
    red: Cube,
    green: Cube,
    blue: Cube,
}

impl Subset {
    pub fn new() -> Subset {
        Subset {
            red: Cube::Red(0),
            green: Cube::Green(0),
            blue: Cube::Blue(0),
        }
    }

    pub fn add(&mut self, element: Cube) {
        match element {
            Cube::Red(count) => self.red = Cube::Red(count),
            Cube::Green(count) => self.green = Cube::Green(count),
            Cube::Blue(count) => self.blue = Cube::Blue(count),
        };
    }

    pub fn contains(&self, subset: Subset) -> bool {
        let fields = [
            (self.red.count(), subset.red.count()),
            (self.green.count(), subset.green.count()),
            (self.blue.count(), subset.blue.count()),
        ];

        for (self_cube, cube_to_compare) in fields {
            if cube_to_compare > self_cube {
                return false;
            }
        }

        true
    }

    pub fn parse_str(line: &str) -> Option<Subset> {
        //
        let elements: Vec<&str> = line.trim().split(",").collect();

        if elements.len() < 1 {
            return None;
        }

        let mut new_subset = Subset::new();

        for elem in elements.iter() {
            let parsed_subset = Cube::parse_str(elem.trim());

            if parsed_subset.is_none() {
                continue;
            }

            new_subset.add(parsed_subset.unwrap());
        }

        Some(new_subset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset() {
        let mut control_subset = Subset::new();
        control_subset.add(Cube::Red(23));
        assert_eq!(control_subset.red, Cube::Red(23));
        assert_eq!(control_subset.red.count(), 23);
    }

    #[test]
    fn test_subset_from_str() {
        let mut control_subset = Subset::new();
        control_subset.add(Cube::Red(23));
        control_subset.add(Cube::Green(33));
        control_subset.add(Cube::Blue(7));

        let test_subset = Subset::parse_str("23 red,33 green,7 blue");

        if let Some(subset) = test_subset {
            assert_eq!(subset.red, control_subset.red);
            assert_eq!(subset.green, control_subset.green);
            assert_eq!(subset.blue, control_subset.blue);
            assert_eq!(subset.red, Cube::Red(23));
            assert_eq!(control_subset.red, Cube::Red(23));
            assert_eq!(subset.green, Cube::Green(33));
            assert_eq!(control_subset.green, Cube::Green(33));
            assert_eq!(subset.blue, Cube::Blue(7));
            assert_eq!(control_subset.blue, Cube::Blue(7));
        }
    }

    #[test]
    fn test_partial_subset_from_str() {
        let mut control_subset = Subset::new();
        control_subset.add(Cube::Green(33));
        control_subset.add(Cube::Blue(7));

        let test_subset = Subset::parse_str("33 green,7 blue");

        if let Some(subset) = test_subset {
            assert_eq!(subset.red, control_subset.red);
            assert_eq!(subset.green, control_subset.green);
            assert_eq!(subset.blue, control_subset.blue);
            assert_eq!(subset.red, Cube::Red(0));
            assert_eq!(control_subset.red, Cube::Red(0));
            assert_eq!(subset.green, Cube::Green(33));
            assert_eq!(control_subset.green, Cube::Green(33));
            assert_eq!(subset.blue, Cube::Blue(7));
            assert_eq!(control_subset.blue, Cube::Blue(7));
        }
    }

    #[test]
    fn test_subset_contains() {
        let mut control_subset = Subset::new();
        control_subset.add(Cube::Red(23));
        control_subset.add(Cube::Green(33));
        control_subset.add(Cube::Blue(7));

        let test_subset_identical = Subset::parse_str("23 red,33 green,7 blue").unwrap();
        let test_subset_valid = Subset::parse_str("6 red,12 green,1 blue").unwrap();
        let test_subset_valid_partial_fields = Subset::parse_str("21 red,3 blue").unwrap();
        let test_subset_invalid = Subset::parse_str("6 red,865 green,1 blue").unwrap();
        let test_subset_invalid_spaced = Subset::parse_str("6 red, 865 green, 1 blue").unwrap();

        assert!(control_subset.contains(test_subset_identical));
        assert!(control_subset.contains(test_subset_valid));
        assert!(control_subset.contains(test_subset_valid_partial_fields));
        assert!(!control_subset.contains(test_subset_invalid));
        assert!(!control_subset.contains(test_subset_invalid_spaced));
    }
}
