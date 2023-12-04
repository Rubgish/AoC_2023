use std::collections::HashMap;

pub fn day03(input_lines: &str) -> (String, String) {
    let mut a: [[char; 142]; 142] = [['.'; 142]; 142];
    let mut lin_num = 0;
    let mut nums: Vec<Num> = Vec::new();
    for line in input_lines.lines() {
        lin_num += 1;
        let mut col_num = 1;
        let mut in_num = false;
        for c in line.chars() {
            a[lin_num][col_num] = c;
            if c.is_digit(10) && in_num == false {
                in_num = true;
                let new_num: i32 = line
                    .chars()
                    .skip(col_num - 1)
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse()
                    .unwrap();
                nums.push(Num {
                    num: new_num,
                    pos: (lin_num as i32, col_num as i32),
                });
            } else if !c.is_digit(10) {
                in_num = false;
            }
            col_num += 1;
        }
    }

    // Look through the numbers and find any not . or digit
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    let mut answer1 = 0;
    for num in nums {
        let len = num.num.to_string().len();
        let mut found = false;
        for x in num.pos.0 - 1..num.pos.0 + 2 {
            for y in num.pos.1 - 1..num.pos.1 + len as i32 + 1 {
                if a[x as usize][y as usize] != '.' && !a[x as usize][y as usize].is_digit(10) {
                    if !found {
                        answer1 += num.num;
                    }
                    found = true;
                    if a[x as usize][y as usize] == '*' {
                        gears
                            .entry((x as usize, y as usize))
                            .or_insert(Vec::new())
                            .push(num.num);
                    }
                }
            }
        }
    }

    let mut answer2 = 0;
    for (_, nums) in gears {
        if nums.len() == 2 {
            let mut a = 1;
            for num in nums {
                a *= num;
            }
            answer2 += a;
        }
    }
    (format!("{}", answer1), format!("{}", answer2))
}

struct Num {
    num: i32,
    pos: (i32, i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(day03("").0, "0".to_string())
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(day03("").1, "0".to_string())
    }

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(day03(""), ("0".to_string(), "0".to_string()))
    }
}
