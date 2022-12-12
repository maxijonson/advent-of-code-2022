use advent_of_code_2022::utils::parse_input::parse_input;

fn main() {
    let datastream = parse_input("src/calendar/day6/input.txt");

    // sopm = start-of-packet marker
    let mut sopm_start_index = 0;
    let sopm_size = 4;

    // Grab 4 chars from sopm_start_index and check if they are unique by converting to a HashSet
    while datastream[sopm_start_index..sopm_start_index + sopm_size]
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .collect::<std::collections::HashSet<&char>>()
        .len()
        != sopm_size
    {
        sopm_start_index += 1;
    }

    println!("Characters processed before start-of-packet marker is detected: {}", sopm_start_index + sopm_size);
}
