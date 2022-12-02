use std::fs;

// A,X = Rock, B,Y = Paper, C,Z= Scissors
// Part 2: X = Lose, Y = Draw, Z = Win

fn choose_move(hand: &&str) -> String {
    let moves: Vec<&str> = hand.split_whitespace().collect();
    
    match *moves.first().unwrap() {
        "A" => {
            match *moves.last().unwrap() {
                "X" => return "Z".to_owned(),
                "Y" => return "X".to_owned(),
                "Z" => return "Y".to_owned(),
                _ => unreachable!()
            }
        },
        "B" => {
            match *moves.last().unwrap() {
                "X" => return "X".to_owned(),
                "Y" => return "Y".to_owned(),
                "Z" => return "Z".to_owned(),
                _ => unreachable!()
            }
        },
        "C" => {
            match *moves.last().unwrap() {
                "X" => return "Y".to_owned(),
                "Y" => return "Z".to_owned(),
                "Z" => return "X".to_owned(),
                _ => unreachable!()
            }
        },
        _ => unreachable!(),
    };
}

fn validate_rps(hand: &&str) -> u32 {
    let mut points: u32 = 0;
    let moves: Vec<&str> = hand.split_whitespace().collect();
    
    match *moves.first().unwrap() {
        "A" => {
            match *moves.last().unwrap() {
                "X" => points += 4,
                "Y" => points += 8,
                "Z" => points += 3,
                _ => unreachable!()
            }
        },
        "B" => {
            match *moves.last().unwrap() {
                "X" => points += 1,
                "Y" => points += 5,
                "Z" => points += 9,
                _ => unreachable!()
            }
        },
        "C" => {
            match *moves.last().unwrap() {
                "X" => points += 7,
                "Y" => points += 2,
                "Z" => points += 6,
                _ => unreachable!()
            }
        },
        _ => unreachable!(),
    };
    return points;
}

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    let input_vector: Vec<&str> = input_string.split("\r\n").collect();
    let mut total_points: u32 = 0;

    for hand in input_vector {
        let chosen_hand = format!("{} {}", hand.split_whitespace().next().unwrap(), choose_move(&hand));
        let chosen_hand = chosen_hand.as_str();

        total_points += validate_rps(&chosen_hand);
    }

    println!("{}", total_points);
}
