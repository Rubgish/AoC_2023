pub fn day01(input_lines: &str) -> (String, String) {
    let mut numbers: Vec<i32> = Vec::new();
    for line in input_lines.lines() {
        let mut found_first_number = false;
        for character in line.chars() {
            if character.is_digit(10) {
                if !found_first_number {
                    let new_num = character.to_digit(10).unwrap() as i32;
                    numbers.push(new_num * 10);
                    found_first_number = true;
                } else {
                    break;
                }
            }
        }
    }
    let mut line_num = 0;
    for line in input_lines.lines() {
        let mut found_second_number = false;
        for character in line.chars().rev() {
            if character.is_digit(10) {
                if !found_second_number {
                    let new_num = character.to_digit(10).unwrap() as i32;
                    numbers[line_num] += new_num;
                    found_second_number = true;
                } else {
                    line_num += 1;
                    break;
                }
            }
        }
    }
    let answer1 = numbers.iter().sum::<i32>();

    let mut number_strings: Vec<String> = Vec::new();
    for line in input_lines.lines() {
        let mut number_string = String::new();
        let mut index = 0;
        for character in line.chars() {
            if character.is_digit(10) {
                for character in line.chars() {
                    if character.is_digit(10) {
                        number_string.push(character);
                    }
                }
            } else if starts_with_substring(&line[index..], "one") {
                number_string.push('1');
            } else if starts_with_substring(&line[index..], "two") {
                number_string.push('2');
            } else if starts_with_substring(&line[index..], "three") {
                number_string.push('3');
            } else if starts_with_substring(&line[index..], "four") {
                number_string.push('4');
            } else if starts_with_substring(&line[index..], "five") {
                number_string.push('5');
            } else if starts_with_substring(&line[index..], "six") {
                number_string.push('6');
            } else if starts_with_substring(&line[index..], "seven") {
                number_string.push('7');
            } else if starts_with_substring(&line[index..], "eight") {
                number_string.push('8');
            } else if starts_with_substring(&line[index..], "nine") {
                number_string.push('9');
            }
            index += 1;
        }
        number_strings.push(number_string);
    }
    let mut part_two_numbers: Vec<i32> = Vec::new();
    for string in number_strings {
        let first_digit = string.chars().next().unwrap().to_digit(10).unwrap() as i32;
        let last_digit = string.chars().last().unwrap().to_digit(10).unwrap() as i32;
        part_two_numbers.push(first_digit * 10 + last_digit);
    }

    let answer2 = part_two_numbers.iter().sum::<i32>();
    (format!("{}", answer1), format!("{}", answer2))
}

// Function that checks if a string starts with a substring
fn starts_with_substring(string: &str, substring: &str) -> bool {
    // check the string is long enough to contain the substring
    if string.len() < substring.len() {
        return false;
    }
    let mut starts_with = true;
    for (index, character) in substring.chars().enumerate() {
        if string.chars().nth(index).unwrap() != character {
            starts_with = false;
            break;
        }
    }
    starts_with
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(day01("").0, "0".to_string())
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(day01("").1, "0".to_string())
    }

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(day01(""), ("0".to_string(), "0".to_string()))
    }
}
