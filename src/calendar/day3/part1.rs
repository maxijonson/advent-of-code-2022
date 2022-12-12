use advent_of_code_2022::utils::parse_input::parse_input;

fn main() {
    let input = parse_input("src/calendar/day3/input.txt");
    let rucksacks = input.lines().collect::<Vec<&str>>();

    let total_priority: i32 = rucksacks
        .iter()
        .map(|rucksack| {
            let size = rucksack.chars().count();
            let compartment_size = size / 2;
            let (compartment_1, compartment_2) = rucksack.split_at(compartment_size);

            let common_item = compartment_1
                .chars()
                .find(|letter| compartment_2.contains(*letter))
                .unwrap();

            match common_item {
                'a'..='z' => common_item as i32 - 96,
                'A'..='Z' => common_item as i32 - 38,
                _ => panic!("Invalid letter"),
            }
        })
        .sum();

    println!("Total priority: {}", total_priority);
}
