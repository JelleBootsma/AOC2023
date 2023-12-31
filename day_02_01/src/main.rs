fn main() {
    let read_file_result = std::fs::read_to_string("input.txt");
    let input = match read_file_result {
        Ok(file) => file,
        Err(error) => {
            println!("Problem opening the file: {:?}", error);
            panic!("Problem opening the file: {:?}", error);
        }
    };

    let games: std::str::Split<'_, char> = input.split('\n');
    let game_id_sum = games
        .map(|x| x.to_string().replace("Game ", ""))
        .map(|game_string| process_game(game_string.as_str()))
        .reduce(|acc, id| acc + id)
        .unwrap_or(0);

    println!("{}", game_id_sum);
}

fn process_game(game_string: &str) -> i32 {
    let split_game: Vec<&str> = game_string.split(':').map(|x| x.trim()).collect();
    let game_id = split_game[0]
        .parse::<i32>()
        .expect("Invalid format in game ID"); // Non-recoverable error, so expect is okay

    // Split the game up into subgames
    let all_valid = split_game[1]
        .split(';')
        .all(|g| validate_sub_game(g.trim()));

    if all_valid {
        return game_id;
    }
    return 0;
} 

fn validate_sub_game(sub_game_string: &str) -> bool {
    const RED_CUBE_COUNT: i32 = 12;
    const GREEN_CUBE_COUNT: i32 = 13;
    const BLUE_CUBE_COUNT: i32 = 14;


    let mut red_remaining = RED_CUBE_COUNT;
    let mut green_remaining = GREEN_CUBE_COUNT;
    let mut blue_remaining = BLUE_CUBE_COUNT;


    // Split on comma
    for draw in sub_game_string.split(',').map(|x| x.trim()) {
        let split: Vec<&str> = draw.split(' ').collect();
        let count = split[0]
            .parse::<i32>()
            .expect("Invalid format in game ID");
        let colour = split[1].trim();

        match colour {
            "red" => red_remaining -= count,
            "green" => green_remaining -= count,
            "blue" => blue_remaining -= count,
            _ => println!("Invalid colour: {colour}"),
        }
        if red_remaining < 0 || green_remaining < 0 || blue_remaining < 0 
        {
            return false;
        }
    }
    return true;
}
