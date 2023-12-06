fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Should have been able to read this file without issues");

    let games: std::str::Split<'_, char> = input.split('\n');
    for game in games {
        println!("Hello world!")
    }
}
