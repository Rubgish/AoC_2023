use std::collections::HashMap;

// create a class consisting of some number of red, green and blue cubes
struct ColouredCube {
    red: i32,
    green: i32,
    blue: i32,
}

pub fn day02(input_lines: &str) -> (String, String) {
    // A line is fomatted as "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    let mut cubes: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in input_lines
        .lines()
        .map(|line| line.split(": ").collect::<Vec<&str>>())
    {
        let (_, game_num_str2) = line[0].split_once(" ").unwrap();
        let game_number = game_num_str2.parse::<i32>().unwrap();
        let mut cube = ColouredCube {
            red: 0,
            green: 0,
            blue: 0,
        };
        // Read the number of cubes of each colour and then see if we need to update our highest values
        for cube_list in line[1].split("; ").collect::<Vec<&str>>() {
            let cube_grab = cube_list.split(", ").collect::<Vec<&str>>();
            for colour_pair in cube_grab {
                let colour_split = colour_pair.split(" ").collect::<Vec<&str>>();
                let colour_number = colour_split[0].parse::<i32>().unwrap();
                match colour_split[1] {
                    "red" => cube.red = colour_number,
                    "green" => cube.green = colour_number,
                    "blue" => cube.blue = colour_number,
                    _ => panic!("Invalid colour"),
                }
            }
            // If cubes does not have an entry for this game_number, add the cube
            if !cubes.contains_key(&game_number) {
                cubes.insert(game_number, vec![cube.red, cube.green, cube.blue]);
            } else {
                // If cubes does have an entry for this game_number, check if the value is larger and add it if it is
                if cubes[&game_number][0] < cube.red {
                    cubes.get_mut(&game_number).unwrap()[0] = cube.red;
                }
                if cubes[&game_number][1] < cube.green {
                    cubes.get_mut(&game_number).unwrap()[1] = cube.green;
                }
                if cubes[&game_number][2] < cube.blue {
                    cubes.get_mut(&game_number).unwrap()[2] = cube.blue;
                }
            }
        }
    }
    // For each cube in cubes, check if it has fewer than 12 red cubes, 13 green cubes and 14 blue cubes
    let mut answer1 = 0;
    let mut answer2 = 0;
    for cube in cubes {
        if cube.1[0] <= 12 && cube.1[1] <= 13 && cube.1[2] <= 14 {
            answer1 += cube.0;
        }
        answer2 += cube.1[0] * cube.1[1] * cube.1[2];
    }
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(day02("").0, "0".to_string())
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(day02("").1, "0".to_string())
    }

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(day02(""), ("0".to_string(), "0".to_string()))
    }
}
