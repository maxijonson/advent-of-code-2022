use advent_of_code_2022::utils::parse_input::parse_input;

fn main() {
    let input = parse_input("src/calendar/day4/input.txt");
    let pairs = input.lines().collect::<Vec<&str>>();

    let total_partially_contained_ranges: i32 = pairs
        .iter()
        .fold(0, |acc, pair| {
            let (range_1, range_2) = pair.split_once(",").unwrap();
            let (range_1_start, range_1_end) = range_1.split_once("-").unwrap();
            let (range_2_start, range_2_end) = range_2.split_once("-").unwrap();

            let range_1_start = range_1_start.parse::<i32>().unwrap();
            let range_1_end = range_1_end.parse::<i32>().unwrap();
            let range_2_start = range_2_start.parse::<i32>().unwrap();
            let range_2_end = range_2_end.parse::<i32>().unwrap();

            if range_1_start <= range_2_start && range_1_end >= range_2_end {
                acc + 1
            } else if range_2_start <= range_1_start && range_2_end >= range_1_end {
                acc + 1
            } else if range_1_start <= range_2_start && range_1_end >= range_2_start {
                acc + 1
            } else if range_2_start <= range_1_start && range_2_end >= range_1_start {
                acc + 1
            } else {
                acc
            }
        });

    println!("Total partially contained ranges: {}", total_partially_contained_ranges);
}
