use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameList {
    pub id_sum: i32,
    pub games: Vec<Game>,
    pub valid_games: Vec<Game>,
}

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
}
