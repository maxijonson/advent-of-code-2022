use advent_of_code_2022::utils::parse_input::parse_input;
use std::vec;

fn main() {
    let input = parse_input("src/calendar/day5/input.txt");
    let instructions = input.lines().collect::<Vec<&str>>();

    /*
    To avoid having to parse this complex structure, the initial stacks are hard-coded, since parsing is not the point of this exercise.
    Initial stacks:
            [H]     [W] [B]
        [D] [B]     [L] [G] [N]
    [P] [J] [T]     [M] [R] [D]
    [V] [F] [V]     [F] [Z] [B]     [C]
    [Z] [V] [S]     [G] [H] [C] [Q] [R]
    [W] [W] [L] [J] [B] [V] [P] [B] [Z]
    [D] [S] [M] [S] [Z] [W] [J] [T] [G]
    [T] [L] [Z] [R] [C] [Q] [V] [P] [H]
    1   2   3   4   5   6   7   8   9
    */
    let mut stacks = vec![
        vec!["T", "D", "W", "Z", "V", "P"],           // 1
        vec!["L", "S", "W", "V", "F", "J", "D"],      // 2
        vec!["Z", "M", "L", "S", "V", "T", "B", "H"], // 3
        vec!["R", "S", "J"],                          // 4
        vec!["C", "Z", "B", "G", "F", "M", "L", "W"], // 5
        vec!["Q", "W", "V", "H", "Z", "R", "G", "B"], // 6
        vec!["V", "J", "P", "C", "B", "D", "N"],      // 7
        vec!["P", "T", "B", "Q"],                     // 8
        vec!["H", "G", "Z", "R", "C"],                // 9
    ];

    for instruction in instructions {
        let (_, instruction_details) = instruction.split_once(" ").unwrap();
        let (amount, instruction_details) = instruction_details.split_once(" from ").unwrap();
        let (src_stack, dest_stack) = instruction_details.split_once(" to ").unwrap();

        let amount = amount.parse::<usize>().unwrap();
        let src_stack = src_stack.parse::<usize>().unwrap() - 1; // Zero-indexed
        let dest_stack = dest_stack.parse::<usize>().unwrap() - 1; // Zero-indexed

        let mut crates_to_move = vec![];
        for _ in 0..amount {
            crates_to_move.push(stacks[src_stack].pop().unwrap());
        }
        crates_to_move.reverse();
        for crate_to_move in crates_to_move {
            stacks[dest_stack].push(crate_to_move);
        }
    }

    print!("Final stacks: ");
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
}
