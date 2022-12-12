use advent_of_code_2022::utils::parse_input::parse_input;

fn main() {
    let input = parse_input("src/calendar/day3/input.txt");
    let rucksacks = input.lines().collect::<Vec<&str>>();
    let groups = rucksacks.chunks(3).collect::<Vec<&[&str]>>();

    let total_priority: i32 = groups
        .iter()
        .map(|group| {
            let (rucksack_1, rucksack_2, rucksack_3) = (group[0], group[1], group[2]);

            // Get the common item between the three rucksacks
            let common_item = rucksack_1
                .chars()
                .find(|letter| rucksack_2.contains(*letter) && rucksack_3.contains(*letter))
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
