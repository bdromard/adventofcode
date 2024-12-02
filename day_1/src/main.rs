use std::cmp::Ordering;

fn main() {
    let file = std::fs::read_to_string("../../input").unwrap();

    let mut first_list: Vec<i32> = file
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split("   ").collect();
            let first_number: i32 = parts[0].parse().unwrap();
            first_number
        })
        .collect();

    let mut second_list: Vec<i32> = file
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split("   ").collect();
            let second_number: i32 = parts[1].parse().unwrap();
            second_number
        })
        .collect();

    first_list.sort();
    second_list.sort();

    let result: i32 = first_list
        .into_iter()
        .enumerate()
        .map(|(index, first_number)| {
            let second_number = second_list[index];
            match first_number.cmp(&second_number) {
                Ordering::Less => second_number - first_number,
                Ordering::Greater => first_number - second_number,
                Ordering::Equal => 0,
            }
        })
        .sum();
    println!("{}", result);
}
