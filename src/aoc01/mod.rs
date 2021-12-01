use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn sonar(filename: String) -> u32{
    // File hosts must exist in current path before this produces output
    let mut counter: u32 = 0;
    
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut lastnum: u32 = u32::MAX;
        for line in lines {
            if let Ok(ip) = line {
                let num: u32 = ip.parse().unwrap();
                if lastnum < num {
                    counter += 1;
                }
                lastnum = num;
            }
        }
    }
    println!("{}",counter);
    return counter;
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
