fn main() {
    let file = std::fs::read_to_string("../../input").unwrap();

    let first_list: Vec<i32> = file
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split("   ").collect();
            let first_number: i32 = parts[0].parse().unwrap();
            first_number
        })
        .collect();

    let second_list: Vec<i32> = file
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split("   ").collect();
            let second_number: i32 = parts[1].parse().unwrap();
            second_number
        })
        .collect();

    let result: i32 = first_list
        .into_iter()
        .map(|first_number| {
            let occurences: i32 = second_list
                .clone()
                .into_iter()
                .filter(|second_number| *second_number == first_number)
                .count()
                .try_into()
                .unwrap();
            first_number * occurences
        })
        .sum();
    println!("{}", result);
}
