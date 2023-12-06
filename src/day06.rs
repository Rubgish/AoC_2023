pub fn day06(input_lines: &str) -> (String, String) {
    let times = input_lines
        .lines()
        .nth(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let distance = input_lines
        .lines()
        .nth(1)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut total_beats = 1;
    for i in 0..times.len() {
        let mut beat_count = 0;
        let mut vel = 0;
        let mut remaining_time = times[i];
        for _ in 0..times[i] {
            vel += 1;
            remaining_time -= 1;
            if distance[i] - vel * remaining_time < 0 {
                beat_count += 1;
            }
        }
        total_beats *= beat_count;
    }

    let mut real_time: i64 = 0;
    let mut real_distance: i64 = 0;
    for i in 0..times.len() {
        for _ in 0..(times[i].to_string().len()) {
            real_time *= 10;
        }
        real_time += times[i] as i64;
        for _ in 0..(distance[i].to_string().len()) {
            real_distance *= 10;
        }
        real_distance += distance[i] as i64;
    }
    let mut beat_count = 0;
    let mut vel: i64 = 0;
    let mut remaining_time = real_time;
    for _ in 0..real_time {
        vel += 1;
        remaining_time -= 1;
        if real_distance - vel * remaining_time < 0 {
            beat_count += 1;
        }
    }

    let answer1 = total_beats;
    let answer2 = beat_count;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day06_part1_case1() {
        assert_eq!(day06("").0, "0".to_string())
    }

    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(day06("").1, "0".to_string())
    }

    #[test]
    fn check_day06_both_case1() {
        assert_eq!(day06(""), ("0".to_string(), "0".to_string()))
    }
}
