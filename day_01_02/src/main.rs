fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Should have been able to read this file without issues");

    let mut sum: i32 = 0;
    for l in input.split('\n'){
        sum += process_single_line(&replace_written_digits_context_aware(l))
    }
    println!("{}", sum);
}

fn replace_written_digits_context_aware(line: &str) -> String {
    let mut result_string = "".to_string();
    let mut has_seen_first_digit = false;
    // We add characters from left to right incrementally, and replace after every increment.
    // This is to ensure replacements happen from left to right
    for c in line.chars() {
        result_string.push(c);
        // The moment we encounter the first digit, we stop replacing after adding every character and do the rest in one go (as we know we can do that now)
        if !has_seen_first_digit {
            result_string = replace_written_digits(result_string);
            has_seen_first_digit = contains_digit(&result_string);
        }
    }
    
    result_string = replace_written_digits(result_string);

    return result_string;
}

fn replace_written_digits(line: String) -> String {
    return line
        .replace("oneight", "on8")      // Replace portmanteau weighted to the end
        .replace("threeight", "thre8")  // Before the first digit is found the characters are
        .replace("fiveight", "fiv8")    // added one by one, so then the replacements is
        .replace("nineight", "nin8")    // already weighted to the front
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");
}

fn contains_digit(line: &str) -> bool {
    return line.chars().any(|c| c as i8 >= 47 && c as i8 <= 57);
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
