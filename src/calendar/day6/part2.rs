use advent_of_code_2022::utils::parse_input::parse_input;

fn main() {
    let datastream = parse_input("src/calendar/day6/input.txt");

    // somm = start-of-message marker
    let mut somm_start_index = 0;
    let somm_size = 14;

    // Grab 4 chars from somm_start_index and check if they are unique by converting to a HashSet
    while datastream[somm_start_index..somm_start_index + somm_size]
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .collect::<std::collections::HashSet<&char>>()
        .len()
        != somm_size
    {
        somm_start_index += 1;
    }

    println!("Characters processed before start-of-message marker is detected: {}", somm_start_index + somm_size);
}
