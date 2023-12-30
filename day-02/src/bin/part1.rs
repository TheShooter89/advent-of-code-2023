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

fn check_games(games_lines: &Vec<&str>, cubes_bag: &Subset) -> ValidatedGameList {
    let mut games_list: Vec<Game> = vec![];
    let mut id_accumulator = 0;
    let mut valid_games: Vec<Game> = vec![];

    for game_str in games_lines {
        let parsed_game = Game::parse_str(game_str);

        match parsed_game {
            Some(game) => {
                //println!("game: {:?}", game);
                if game.is_valid_for_subset(cubes_bag) {
                    id_accumulator += game.id;
                    valid_games.push(game.clone());
                    games_list.push(game.clone());
                    continue;
                }
                games_list.push(game.clone());
            }
            None => continue,
        }
    }

    ValidatedGameList {
        id_sum: id_accumulator,
        games: games_list,
        valid_games: valid_games,
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

    print_bag_composition(&cubes_bag);
    println!("### processing...");

    let games_list = check_games(&lines, &cubes_bag);

    print_result(games_list);

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

        let mut cubes_bag = Subset::new();
        cubes_bag.add(Cube::Red(12));
        cubes_bag.add(Cube::Green(13));
        cubes_bag.add(Cube::Blue(14));

        let games_list = check_games(&lines, &cubes_bag);

        //assert_eq!(games_list.id_sum, 2563);
        assert_eq!(games_list.id_sum, 8);
    }
}
