use std::fs;
use std::time::Instant;

fn find_matching_char(compartments: (&str, &str)) -> Option<char> {
    for character in compartments.0.chars() {
        if compartments.1.find(character) != None {return Some(character);}
    }
    println!("Couldn't find matching character in {:?}", compartments);
    return None
}

fn find_badges(sacks: (&str, &str, &str)) -> Option<char> {
    for character in sacks.0.chars() {
        if sacks.1.find(character) != None && sacks.2.find(character) != None {
            return Some(character);
        }
    }
    println!("Couldn't find matching badges in {:?}", sacks);
    return None;
}

fn main() {
    let instant = Instant::now();
    let priority: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let input_string = fs::read_to_string("input.txt").unwrap();
    let input_vector: Vec<&str> = input_string.split("\r\n").collect();
    let mut points: u32 = 0;
    // println!("Time elapsed: {:?}", instant.elapsed());

    for sack in &input_vector {
        let compartments = sack.split_at((&sack.len()) / 2);
        let matching_char = find_matching_char(compartments);

        match matching_char {
            Some(_) => {
                points += priority.find(matching_char.unwrap()).unwrap() as u32 + 1;       
            }
            None => {
                panic!();
            } 
        }
    }

    println!("Part one Points: {}", points);
    // println!("Time elapsed: {:?}", instant.elapsed());
    points = 0;

    for vector_index in (0..input_vector.len()).step_by(3) {
        let badge = find_badges((&input_vector[vector_index], &input_vector[vector_index + 1], &input_vector[vector_index + 2]));

        match badge {
            Some(_) => points += priority.find(badge.unwrap()).unwrap() as u32 + 1,
            None => panic!()
        }
    }

    println!("Part two Points: {}", points);
    println!("Time elapsed: {:?}", instant.elapsed());
}
