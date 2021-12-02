use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn dive(filename: String) -> u32 {
    // File hosts must exist in current path before this produces output
    let mut depth = 0;
    let mut position = 0;

    for line in read_lines(filename).unwrap() {
        let line = line.unwrap();

        let mut iter = line.split_whitespace();
        let action = iter.next().unwrap();
        let num = u32::from_str(iter.next().unwrap()).unwrap();

        match action {
            _ if action == "forward" => position += num,
            _ if action == "down" => depth += num,
            _ if action == "up" => depth -= num,
            _ => println!("nothing"),
        }
    }

    return position * depth;
}

pub fn dive_aim(filename: String) -> u32 {
    // File hosts must exist in current path before this produces output
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    let mut movement = 0;

    for line in read_lines(filename).unwrap() {
        let line = line.unwrap();

        let mut iter = line.split_whitespace();
        let action = iter.next().unwrap();
        let num = u32::from_str(iter.next().unwrap()).unwrap();

        match action {
            _ if action == "forward" => {
                position += num;
                movement = num;
                depth += aim * movement;
            }
            _ if action == "down" => aim += num,
            _ if action == "up" => aim -= num,
            _ => println!("nothing"),
        }
    }

    return depth * position;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn dive_test() {
    assert_eq!(dive(String::from("./src/aoc02/test.input")), 150);
}

#[test]
fn dive_challenge() {
    let num = dive(String::from("./src/aoc02/challenge.input"));
    println!("{}", num)
}

#[test]
fn dive_aim_test() {
    assert_eq!(dive_aim(String::from("./src/aoc02/test.input")), 900);
}

#[test]
fn dive_aim_challenge() {
    let num = dive_aim(String::from("./src/aoc02/challenge.input"));
    println!("{}", num)
}
