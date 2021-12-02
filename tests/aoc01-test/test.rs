#[cfg(test)]
mod tests {

    mod aoc01;

    #[test]
    fn sonar_test() {
        assert_eq!(sonar(String::from("./tests/aoc01-test/test.input")), 7);
    }

    #[test]
    fn sonar_challenge() {
        let num = sonar(String::from("./tests/aoc01-test/challenge.input"));
        println!("{}", num)
    }

    #[test]
    fn sonar_window_test() {
        let num = sonar_window(String::from("./tests/aoc01-test/test.input"));
        println!("{}", num)
    }

    #[test]
    fn sonar_window_challenge() {
        let num = sonar_window(String::from("./tests/aoc01-test/challenge.input"));
        println!("sonar_window_challenge {}", num)
    }
}
 