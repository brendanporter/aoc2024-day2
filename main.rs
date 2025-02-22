use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("can't open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("cannot read the file");

    // println!("File Contents:\n\n{}", contents);

    println!(
        "is 7 6 4 2 1 safe?: {}",
        check_safety("7 6 4 2 1".to_string())
    );
}

fn check_safety(input: String) -> bool {
    // Split input string into integers
    let parts = input.split(" ");

    let mut increasing = false;
    let mut decreasing = false;
    let mut gradual = true; // if increase/decrease is by 4 or more, gradual = false
    let mut last_number = -1;

    for (index, part) in parts.enumerate() {
        // convert part to int
        let partInt: i32 = part.parse().unwrap();
        if last_number == -1 {
            last_number = partInt;
        } else {
            let mut diff: i32 = 0;
            if index > 0 {
                diff = partInt - last_number
            }

            if diff.abs() > 3 {
                gradual = false;
            }

            if index > 0 {
                if diff < 0 {
                    decreasing = true;
                } else {
                    increasing = true;
                }
            }
        }
        println!("Part: {}, last: {}", part, last_number);
        println!("Gradual: {}", gradual);
        println!("Increasing?: {}", increasing);
        println!("Decreasing?: {}", decreasing);
        last_number = partInt;
    }

    return (decreasing ^ increasing) & gradual;

    return false;
}

#[cfg(test)]
mod tests {
    #[test]
    fn safe() {
        assert_eq!(check_safety("7 6 4 2 1"), true);
        assert_eq!(check_safety("1 2 7 8 9"), false);
    }
}
