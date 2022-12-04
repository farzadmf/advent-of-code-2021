use day04_rust::{covers, get_points, read_input};

fn main() {
    println!("+++++++++++++++++++ PART 1 +++++++++++++++++++");

    let input = read_input("input".to_string());

    let result = input
        .lines()
        .map(|line| get_points(line))
        .map(|coords| covers(coords))
        .sum::<u32>();

    println!("result {}", result);
    println!("------------------- PART 1 -------------------");
}
