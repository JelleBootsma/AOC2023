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

    // Split the game up into subgames
    let game_requirements = split_game[1]
        .split(';')
        .map(|g| count_sub_game_requirements(g.trim()))
        .reduce(|acc,m| 
            (std::cmp::max(acc.0, m.0), 
            std::cmp::max(acc.1, m.1), 
            std::cmp::max(acc.2, m.2)))
        .expect("Failed split the game on ;");

    return game_requirements.0 * game_requirements.1 * game_requirements.2;
} 


fn count_sub_game_requirements(sub_game_string: &str) -> (i32, i32, i32) {

    let mut red_count = 0;
    let mut green_count = 0;
    let mut blue_count = 0;


    // Split on comma
    for draw in sub_game_string.split(',').map(|x| x.trim()) {
        let split: Vec<&str> = draw.split(' ').collect();
        let count = split[0]
            .parse::<i32>()
            .expect("Invalid format in game ID");
        let colour = split[1].trim();

        match colour {
            "red" => red_count += count,
            "green" => green_count += count,
            "blue" => blue_count += count,
            _ => println!("Invalid colour: {colour}"),
        }
    }
    return (red_count, green_count, blue_count);
}
