use regex::Regex;
fn main() {
    fn multiply(m: &str) -> i32 {
        let result = m.replace("mul(", "").replace(')', "");
        let split: Vec<&str> = result.split(',').collect();
        split.iter().map(|s| s.parse::<i32>().unwrap()).product()
    }
    fn first_part() {
        let pattern = Regex::new(r"mul\(\d*,\d*\)").unwrap();
        let input_file = std::fs::read_to_string("./input").unwrap();
        let result: i32 = pattern
            .find_iter(&input_file)
            .map(|m| {
                let m_str = m.as_str();
                multiply(m_str)
            })
            .sum();
        println!("{:}", result);
    }
    first_part();
    fn second_part() {
        let pattern = Regex::new(r"(don't\(\))|(do\(\))|mul\(\d*,\d*\)").unwrap();
        let input_file = std::fs::read_to_string("./input").unwrap();
        let mut authorized_to_push = true;
        let mut good_matches: Vec<&str> = vec![];
        let matches: Vec<&str> = pattern.find_iter(&input_file).map(|m| m.as_str()).collect();
        matches.clone().into_iter().for_each(|m| match m {
            "don't()" => authorized_to_push = false,
            "do()" => authorized_to_push = true,
            _ => {
                if authorized_to_push {
                    good_matches.push(m)
                }
            }
        });
        let result: i32 = good_matches.into_iter().map(multiply).sum();
        println!("{result:}")
    }
    second_part();
}
