

#[cfg(test)]
mod tests {

    use aoc01::*;
    
    #[test]
    fn sonar_test() {
        assert_eq!(sonar(String::from("./tests/test.input")),7);
    }

    #[test]
    fn sonar_challenge() {
        let num = sonar(String::from("./tests/challenge.input"));
        println!("{}", num)
    }

    #[test]
    fn sonar_window_test() {
        let num = sonar_window(String::from("./tests/test.input"));
        println!("{}", num)
    }

    #[test]
    fn sonar_window_challenge() {
        let num = sonar_window(String::from("./tests/challenge.input"));
        println!("sonar_window_challenge {}", num)
    }
}