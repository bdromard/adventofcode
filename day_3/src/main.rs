use regex::Regex;
fn main() {
    let pattern = Regex::new(r"mul\(\d*,\d*\)").unwrap();
    let input_file = std::fs::read_to_string("./input").unwrap();
    let result: i32 = pattern
        .find_iter(&input_file)
        .map(|m| {
            let result = m.as_str().replace("mul(", "").replace(')', "");
            let split: Vec<&str> = result.split(',').collect();
            let first_number: i32 = str::parse(split[0]).unwrap();
            let second_number: i32 = str::parse(split[1]).unwrap();
            first_number * second_number
        })
        .sum();
    println!("{:?}", result);
}
