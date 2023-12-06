fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Should have been able to read this file without issues");

    let mut sum: i32 = 0;
    for l in input.split('\n'){
        sum += process_single_line(l)
    }
    println!("{}", sum);
}

fn process_single_line(line: &str) -> i32{
    let mut first_digit: i8 = -1;
    let mut last_digit: i8 = -1;

    for c in line.chars() {
        let v = (c as i8) - 48;
        if v < 0 || v > 9 {
            continue;
        }
        if first_digit == -1 {
            first_digit = v;
        }
        last_digit = v;
    }
    if first_digit == -1
    {
        return 0;
    }
    return first_digit as i32 * 10 + last_digit as i32;
}
