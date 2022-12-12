use advent_of_code_2022::utils::parse_input::parse_input;
use std::collections::HashMap;

fn main() {
    let input = parse_input("src/calendar/day2/input.txt");
    let games = input.lines().collect::<Vec<&str>>();

    let gesture = HashMap::from([
        ('A', "rock"),
        ('X', "rock"),
        ('B', "paper"),
        ('Y', "paper"),
        ('C', "scissors"),
        ('Z', "scissors"),
    ]);
    let gesture_scores = HashMap::from([("rock", 1), ("paper", 2), ("scissors", 3)]);

    let total_score: i32 = games
        .iter()
        .map(|game| {
            let b = game.as_bytes();
            assert_eq!(b.len(), 3);
            let (opponent, player) = (b[0] as char, b[2] as char);
            let (opponent_gesture, player_gesture) = (
                gesture.get(&opponent).unwrap(),
                gesture.get(&player).unwrap(),
            );
            let player_gesture_score = gesture_scores.get(player_gesture).unwrap();

            // Draw
            if opponent_gesture == player_gesture {
                return 3 + player_gesture_score;
            }

            if opponent_gesture == &"rock" && player_gesture == &"paper"
                || opponent_gesture == &"paper" && player_gesture == &"scissors"
                || opponent_gesture == &"scissors" && player_gesture == &"rock"
            {
                return 6 + player_gesture_score;
            }

            return 0 + player_gesture_score;
        })
        .sum();

    println!("Total score: {}", total_score);
}
