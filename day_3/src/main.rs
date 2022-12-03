use std::fs;

fn find_matching_char(compartments: (&str, &str)) -> Option<char> {
    // println!("Looking for matching character in: {:?}", compartments);
    for c1_character in compartments.0.chars() {
        for c2_character in compartments.1.chars() {
            if c1_character == c2_character {return Some(c1_character)}
        }
    }
    println!("Couldn't find matching character in {:?}", compartments);
    return None
}

fn main() {
    let priority: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let input_string = fs::read_to_string("input.txt").unwrap();
    let input_vector: Vec<&str> = input_string.split("\r\n").collect();
    let mut points: u32 = 0;

    for sack in input_vector {
        let compartments = sack.split_at((&sack.len()) / 2);
        let matching_char = find_matching_char(compartments);

        match matching_char {
            Some(_) => {
                // println!("Found matching character: {}", matching_char.unwrap());
                points += priority.find(matching_char.unwrap()).unwrap() as u32 + 1;       
            }
            None => {
                panic!();
            } 
        }
        // break;
    }

    println!("Points: {}", points);
}
