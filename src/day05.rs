pub fn day05(input_lines: &str) -> (String, String) {
    let mut read_seeds = false;
    let mut seeds = Vec::new();
    let mut all_instructions: [[Vec<Instruction>; 1]; 7] = Default::default(); // Initialize all_instructions
    let mut index = 0;
    let mut new_seeds: Vec<(i64, i64)> = Vec::new();
    for line in input_lines.lines() {
        if !read_seeds {
            seeds = line
                .split_once(": ")
                .unwrap()
                .1
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            new_seeds = line
                .split_once(": ")
                .unwrap()
                .1
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
                .chunks(2)
                .map(|chunk| (chunk[0], chunk[1]))
                .collect();
            read_seeds = true;
        } else {
            if line == "" {
                index += 1;
            } else if !line.chars().next().unwrap().is_digit(10) {
                continue;
            } else {
                let vals = line.split(" ").collect::<Vec<&str>>();
                let dest_start = vals[0].parse::<i64>().unwrap();
                let source_start = vals[1].parse::<i64>().unwrap();
                let length = vals[2].parse::<i64>().unwrap();
                let instruction = Instruction::new(dest_start, source_start, length);
                all_instructions[index - 1][0].push(instruction); // -1 cause index is incremented before this line
            }
        }
    }
    let mut end_vals = Vec::new();
    for seed in seeds {
        let mut next_val = seed;
        for i in 0..7 {
            for instruction in &all_instructions[i][0] {
                if instruction.in_source_range(next_val) {
                    next_val = instruction.get_dest(next_val);
                    break;
                }
            }
        }
        end_vals.push(next_val);
    }
    let mut end_vals_2 = Vec::new();
    for (first, second) in &new_seeds {
        println!("Starting from {}", first);
        for i in 0..*second {
            if (i % 10000000) == 0 {
                println!("Poggers: {}/{}", i, second);
            }
            let mut next_val = first + i;
            for j in 0..7 {
                for instruction in &all_instructions[j][0] {
                    if instruction.in_source_range(next_val) {
                        next_val = instruction.get_dest(next_val);
                        break;
                    }
                }
            }
            end_vals_2.push(next_val);
        }
    }
    end_vals.sort();
    println!("Sorting this many numbers: {}", end_vals_2.len());
    let mut i: i64 = 0;
    let mut lowest_val: i64 = 123456789123456789;
    for val in &end_vals_2 {
        i += 1;
        if (i % 100000000) == 0 {
            println!("Poggers: {}/{}", i, end_vals_2.len());
        }
        for (first, second) in &new_seeds {
            if val >= &first && val < &(first + second) {
                if val < &lowest_val {
                    lowest_val = *val;
                }
                break;
            }
        }
    }

    let answer1 = end_vals[0];
    let answer2 = lowest_val;
    (format!("{}", answer1), format!("{}", answer2))
}

struct Instruction {
    dest_start: i64,
    source_start: i64,
    length: i64,
}

impl Instruction {
    fn new(dest_start: i64, source_start: i64, length: i64) -> Instruction {
        Instruction {
            dest_start,
            source_start,
            length,
        }
    }

    fn in_source_range(&self, index: i64) -> bool {
        index >= self.source_start && index < self.source_start + self.length
    }

    fn get_dest(&self, index: i64) -> i64 {
        let offset = index - self.source_start;
        self.dest_start + offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(day05("").0, "0".to_string())
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(day05("").1, "0".to_string())
    }

    #[test]
    fn check_day05_both_case1() {
        assert_eq!(day05(""), ("0".to_string(), "0".to_string()))
    }
}
