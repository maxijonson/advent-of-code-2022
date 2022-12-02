use advent_of_code_2022::parse_input;

fn main() {
    let input = parse_input("src/calendar/day1/input.txt");
    let elves = input.split("\n\n").collect::<Vec<&str>>();

    let mut total_calories = elves
        .iter()
        .map(|elf| {
            elf.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .fold(0, |acc, x| acc + x)
        })
        .collect::<Vec<i32>>();
    total_calories.sort();
    total_calories.reverse();

    let top3_sum = total_calories[0..3]
        .to_vec()
        .iter()
        .fold(0, |acc, x| acc + x);

    println!("Sum of top 3: {}", top3_sum);
}
