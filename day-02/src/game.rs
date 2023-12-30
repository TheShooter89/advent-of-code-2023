use std::result;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub id: i32,
    pub subsets: Vec<Subset>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: 0,
            subsets: vec![],
        }
    }

    pub fn add(&mut self, element: Subset) {
        self.subsets.push(element);
    }

    pub fn parse_id(line: &str) -> Option<i32> {
        //
        let elements: Vec<&str> = line.trim().split_whitespace().collect();

        if elements.len() != 2 {
            return None;
        }

        let parsed_id = elements[1].parse::<i32>();
        if parsed_id.is_err() {
            return None;
        }

        Some(parsed_id.unwrap())
    }

    pub fn parse_subset_list(line: &str) -> Vec<Subset> {
        //
        let elements: Vec<&str> = line.trim().split(";").collect();
        let mut subset_list: Vec<Subset> = Vec::new();

        for elem in elements {
            let subset_str = elem.trim();
            match Subset::parse_str(subset_str) {
                Some(subset) => subset_list.push(subset),
                None => continue,
            }
        }

        subset_list
    }

    pub fn parse_str(line: &str) -> Option<Game> {
        //
        let elements: Vec<&str> = line.trim().split(":").collect();

        if elements.len() != 2 {
            return None;
        }

        let new_id = Game::parse_id(elements[0]);

        if new_id.is_none() {
            return None;
        }

        let new_subset_list = Game::parse_subset_list(elements[1]);

        Some(Game {
            id: new_id.unwrap(),
            subsets: new_subset_list,
        })
    }

    pub fn is_valid_for_subset(&self, subset: &Subset) -> bool {
        for current_subset in self.subsets.iter() {
            if !subset.contains(current_subset) {
                return false;
            }
        }
        true
    }

    pub fn minimum_valid_combination(&self) -> Subset {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        let mut result = Subset::new();

        for sub in &self.subsets {
            if sub.red.count() > min_red {
                min_red = sub.red.count();
            }

            if sub.green.count() > min_green {
                min_green = sub.green.count();
            }

            if sub.blue.count() > min_blue {
                min_blue = sub.blue.count();
            }
        }

        result.add(Cube::Red(min_red));
        result.add(Cube::Green(min_green));
        result.add(Cube::Blue(min_blue));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let mut test_subset1 = Subset::new();
        test_subset1.add(Cube::Red(23));
        test_subset1.add(Cube::Green(5));
        test_subset1.add(Cube::Blue(7));

        let mut test_subset2 = Subset::new();
        test_subset2.add(Cube::Red(0));
        test_subset2.add(Cube::Green(69));
        test_subset2.add(Cube::Blue(420));

        let mut test_subset3 = Subset::new();
        test_subset3.add(Cube::Red(96));
        test_subset3.add(Cube::Green(0));
        test_subset3.add(Cube::Blue(1080));

        let control_game = Game {
            id: 13,
            subsets: vec![test_subset1, test_subset2, test_subset3],
        };
        assert_eq!(
            control_game,
            Game::parse_str(
                "game 13: 23 red, 5 green, 7 blue; 69 green, 420 blue; 96 red, 1080 blue"
            )
            .unwrap()
        );

        let reference_subset = Subset::parse_str("96 red, 69 green, 1080 blue").unwrap();
        assert!(control_game.is_valid_for_subset(&reference_subset));
    }

    #[test]
    fn test_parse_id() {
        let control_id = 69;

        assert_eq!(Game::parse_id("game 69").unwrap(), control_id)
    }

    #[test]
    fn test_parse_subset_list() {
        let mut test_subset1 = Subset::new();
        test_subset1.add(Cube::Red(23));
        test_subset1.add(Cube::Green(5));
        test_subset1.add(Cube::Blue(7));

        let mut test_subset2 = Subset::new();
        test_subset2.add(Cube::Red(0));
        test_subset2.add(Cube::Green(69));
        test_subset2.add(Cube::Blue(420));

        let mut test_subset3 = Subset::new();
        test_subset3.add(Cube::Red(96));
        test_subset3.add(Cube::Green(0));
        test_subset3.add(Cube::Blue(1080));

        assert_eq!(
            Game::parse_subset_list(
                "23 red, 5 green, 7 blue; 69 green, 420 blue; 96 red, 1080 blue"
            ),
            vec![test_subset1, test_subset2, test_subset3]
        )
    }

    #[test]
    fn test_minimum_valid_combination() {
        let mut control_subset = Subset::new();
        control_subset.add(Cube::Red(4));
        control_subset.add(Cube::Green(2));
        control_subset.add(Cube::Blue(6));

        let mut test_subset_1 = Subset::new();
        test_subset_1.add(Cube::Red(4));
        test_subset_1.add(Cube::Green(0));
        test_subset_1.add(Cube::Blue(3));

        let mut test_subset_2 = Subset::new();
        test_subset_2.add(Cube::Red(1));
        test_subset_2.add(Cube::Green(2));
        test_subset_2.add(Cube::Blue(6));

        let mut test_subset_3 = Subset::new();
        test_subset_3.add(Cube::Red(0));
        test_subset_3.add(Cube::Green(2));
        test_subset_3.add(Cube::Blue(0));

        let mut test_game = Game::new();
        test_game.add(test_subset_1);
        test_game.add(test_subset_2);
        test_game.add(test_subset_3);

        let test_minimum_valid_combination = test_game.minimum_valid_combination();

        assert_eq!(test_minimum_valid_combination, control_subset)
    }
}
