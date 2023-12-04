use std::collections::HashSet;

pub fn day04(input_lines: &str) -> (String, String) {
    let mut answer1 = 0;
    let mut card_count: [[u32; 1]; 197] = [[1; 1]; 197];
    let mut row = 0;
    for line in input_lines.lines() {
        let (_, card_results) = line.split_once(": ").unwrap();
        let (win_vals, our_vals) = card_results.split_once(" | ").unwrap();
        let win_vals = win_vals
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let our_vals = our_vals
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let win_vals_set: HashSet<_> = win_vals.iter().collect();
        let our_vals_set: HashSet<_> = our_vals.iter().collect();
        let common_vals: Vec<_> = win_vals_set.intersection(&our_vals_set).collect();
        let mut initial_val = 0;
        let mut tmp_row = row;
        for _ in common_vals {
            tmp_row += 1;
            if initial_val == 0 {
                initial_val = 1;
                card_count[tmp_row][0] += 1 * card_count[row][0];
            } else {
                initial_val = initial_val * 2;
                card_count[tmp_row][0] += 1 * card_count[row][0];
            }
        }
        row += 1;
        answer1 += initial_val;
    }
    let mut answer2 = 0;
    for i in 0..197 {
        answer2 += card_count[i][0];
    }
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(day04("").0, "0".to_string())
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(day04("").1, "0".to_string())
    }

    #[test]
    fn check_day04_both_case1() {
        assert_eq!(day04(""), ("0".to_string(), "0".to_string()))
    }
}
