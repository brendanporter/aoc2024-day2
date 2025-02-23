use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("can't open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("cannot read the file");

    // println!("File Contents:\n\n{}", contents);

    let mut safe_lines: i32 = 0;
    let mut total_lines: i32 = 0;

    // dbg!(check_safety("7 6 4 2 1".to_string(), -1));

    for line in contents.lines() {
        if check_safety(line.to_string()) {
            safe_lines += 1
        }
        total_lines += 1
    }

    println!("Safe lines: {} of {}", safe_lines, total_lines)

    // dbg!(check_safety("1 2 7 8 9".to_string()));
}

fn check_safety(input: String) -> bool {
    // Split input string into integers
    let binding = input.clone();
    let parts = binding.split(" ");

    let mut minimum_diff_met = true;
    let mut increasing = false;
    let mut decreasing = false;
    let mut gradual = true; // if increase/decrease is by 4 or more, gradual = false
    let mut last_number = -1;
    let mut dampener_index = -1;

    for (index, part) in parts.clone().enumerate() {
        // convert part to int
        let part: i32 = part.parse().unwrap();

        if last_number == -1 {
            last_number = part;
            continue;
        }
        let diff = part - last_number;

        if diff.abs() > 3 {
            gradual = false;
        }

        if diff == 0 {
            minimum_diff_met = false;
        }
        if diff < 0 {
            decreasing = true;
        } else {
            increasing = true;
        }

        last_number = part;

        if dampener_index == -1 && !((decreasing ^ increasing) & gradual & minimum_diff_met) {
            dampener_index = index as i32
        }
    }

    if (decreasing ^ increasing) & gradual & minimum_diff_met {
        return true;
    }

    minimum_diff_met = true;
    increasing = false;
    decreasing = false;
    gradual = true; // if increase/decrease is by 4 or more, gradual = false
    last_number = -1;

    for (index, part) in parts.clone().enumerate() {
        // convert part to int
        let part: i32 = part.parse().unwrap();

        if index as i32 == dampener_index {
            continue;
        }

        if last_number == -1 {
            last_number = part;
            continue;
        }
        let diff = part - last_number;

        if diff.abs() > 3 {
            gradual = false;
        }

        if diff == 0 {
            minimum_diff_met = false;
        }
        if diff < 0 {
            decreasing = true;
        } else {
            increasing = true;
        }

        last_number = part;
    }

    return (decreasing ^ increasing) & gradual & minimum_diff_met;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn safe() {
        assert_eq!(check_safety("7 6 4 2 1".to_string()), true);
        assert_eq!(check_safety("1 2 7 8 9".to_string()), false);
        assert_eq!(check_safety("9 7 6 2 1".to_string()), false);
        assert_eq!(check_safety("1 3 2 4 5".to_string()), true);
        assert_eq!(check_safety("8 6 4 4 1".to_string()), true);
        assert_eq!(check_safety("1 3 6 7 9".to_string()), true);

        // assert_eq!(check_safety("7 6 4 2 1".to_string()), true);
        // assert_eq!(check_safety("1 2 7 8 9".to_string()), false);
        // assert_eq!(check_safety("9 7 6 2 1".to_string()), false);
        // assert_eq!(check_safety("1 3 2 4 5".to_string()), false);
        // assert_eq!(check_safety("8 6 4 4 1".to_string()), false);
        // assert_eq!(check_safety("1 3 6 7 9".to_string()), true);
    }
}
