use std::{fs, i32};

use day02::{Cube, Game, MinimumCombinationGameList, Subset, ValidatedGameList};

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
                combinations_list.push(game_minimum_combination);
                combinations_pow_accumulator += game_minimum_combination.pow()
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
