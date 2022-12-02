use advent_of_code_2022::parse_input;

fn main() {
    let input = parse_input("src/calendar/day1/input.txt");
    let elves = input.split("\n\n").collect::<Vec<&str>>();

    let max = elves
        .iter()
        .map(|elf| {
            elf.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .fold(0, |acc, x| acc + x)
        })
        .max()
        .unwrap();

    println!("Max: {}", max);
}
