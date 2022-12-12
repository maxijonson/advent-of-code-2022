use advent_of_code_2022::utils::parse_input::parse_input;
use std::collections::HashMap;

fn main() {
    let input = parse_input("src/calendar/day2/input.txt");
    let games = input.lines().collect::<Vec<&str>>();

    let gesture = HashMap::from([('A', "rock"), ('B', "paper"), ('C', "scissors")]);
    let gesture_scores = HashMap::from([("rock", 1), ("paper", 2), ("scissors", 3)]);
    let outcome_scores = HashMap::from([('X', 0), ('Y', 3), ('Z', 6)]);

    let gesture_lose = HashMap::from([
        ("rock", "scissors"),
        ("paper", "rock"),
        ("scissors", "paper"),
    ]);
    let gesture_win = HashMap::from([
        ("rock", "paper"),
        ("paper", "scissors"),
        ("scissors", "rock"),
    ]);

    let total_score: i32 = games
        .iter()
        .map(|game| {
            let b = game.as_bytes();
            assert_eq!(b.len(), 3);
            let (opponent, outcome) = (b[0] as char, b[2] as char);
            let opponent_gesture = gesture.get(&opponent).unwrap();
            let outcome_score = outcome_scores.get(&outcome).unwrap();

            let player_gesture = match outcome {
                'X' => gesture_lose.get(opponent_gesture).unwrap(),
                'Y' => opponent_gesture,
                'Z' => gesture_win.get(opponent_gesture).unwrap(),
                _ => panic!("Unexpected outcome")
            };

            return gesture_scores.get(player_gesture).unwrap() + outcome_score;
        })
        .sum();

    println!("Total score: {}", total_score);
}
