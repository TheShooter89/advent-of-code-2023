use std::{fs, i32};

use day02::{Cube, Game, Subset};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedGameList {
    pub id_sum: i32,
    pub games: Vec<Game>,
    pub valid_games: Vec<Game>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinimumCombinationGameList {
    pub combinations_pow_sum: i32,
    pub combinations: Vec<Subset>,
}

fn print_title() {
    println!("Advent of Code 2023 - Day 02 [PART 1]\n");
    println!("coded with 💛️💙️ by tanque");
    println!("---------------------------------------");
}

fn print_bag_composition(bag: &Subset) {
    println!("+-----------------------------+");
    println!("| BAG COMPOSITION:");
    println!("+-----------------------------+");
    println!("| RED cubes: {:?}", bag.red.count());
    println!("| GREEN cubes: {:?}", bag.green.count());
    println!("| BLUE cubes: {:?}", bag.blue.count());
    println!("+-----------------------------+\n");
}

fn print_result(list: ValidatedGameList) {
    let valid_games: Vec<i32> = list.valid_games.iter().map(|game| game.id).collect();
    let valid_games_percentage: f32 = list.valid_games.len() as f32 / list.games.len() as f32;
    let percent_label = (valid_games_percentage * 100.0) as u32;
    let all_games: Vec<i32> = list.games.iter().map(|game| game.id).collect();

    println!("+--------------------------------------");
    println!("| RESULTS:");
    println!("+--------------------------------------");
    println!("| total sum of ids: {:?}", list.id_sum);
    println!(
        "| number of valid games: {:?} ({:?} %)",
        list.valid_games.len(),
        percent_label
    );
    println!("| valid games: {:?}", valid_games);

    println!("| number of games: {:?}", list.games.len());

    println!("| all games: {:?}", all_games);
    println!("+--------------------------------------\n");
}

fn check_games(games_lines: &Vec<&str>) -> MinimumCombinationGameList {
    let mut games_list: Vec<Game> = vec![];
    let mut combinations_pow_accumulator = 0;
    let mut combinations_list: Vec<Subset> = vec![];

    for game_str in games_lines {
        let parsed_game = Game::parse_str(game_str);

        match parsed_game {
            Some(game) => {
                //println!("game: {:?}", game);
                let game_minimum_combination = game.minimum_valid_combination();
                combinations_pow_accumulator += game_minimum_combination.pow();
                combinations_list.push(game_minimum_combination);
            }
            None => continue,
        }
    }

    MinimumCombinationGameList {
        combinations_pow_sum: combinations_pow_accumulator,
        combinations: combinations_list,
    }
}

fn main() -> Result<(), std::io::Error> {
    print_title();

    let content = fs::read_to_string("src/bin/input1.txt")?;
    //let content = fs::read_to_string("src/bin/test_input.txt")?;
    let lines: Vec<&str> = content.lines().collect();

    let mut cubes_bag = Subset::new();
    cubes_bag.add(Cube::Red(12));
    cubes_bag.add(Cube::Green(13));
    cubes_bag.add(Cube::Blue(14));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        //let content = fs::read_to_string("src/bin/input1.txt").unwrap();
        let content = fs::read_to_string("src/bin/test_input.txt").unwrap();
        let lines: Vec<&str> = content.lines().collect();

        let minimum_valid_combination_list = check_games(&lines);

        assert_eq!(minimum_valid_combination_list.combinations[0].pow(), 48);
        assert_eq!(minimum_valid_combination_list.combinations[1].pow(), 12);
        assert_eq!(minimum_valid_combination_list.combinations[2].pow(), 1560);
        assert_eq!(minimum_valid_combination_list.combinations[3].pow(), 630);
        assert_eq!(minimum_valid_combination_list.combinations[4].pow(), 36);

        assert_eq!(minimum_valid_combination_list.combinations_pow_sum, 2286);
    }
}
