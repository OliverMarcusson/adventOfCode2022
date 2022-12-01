use std::fs;

fn main() {
    let input_content = fs::read_to_string("input.txt").unwrap();
    let input_vector: Vec<&str> = input_content.split("\r\n").collect();

    let mut combined_calories: Vec<u64> = Vec::new();
    combined_calories.push(0);

    'adding: for calories in input_vector {
        if calories == "" {
            combined_calories.push(0);
            continue 'adding;
        }
        *combined_calories.last_mut().unwrap() += calories.parse::<u64>().unwrap();
    }
    combined_calories.sort();

    let mut top_three_calories: u64 = 0;
    for i in 1..=3 {
        top_three_calories += combined_calories[combined_calories.len() - i];
    }
    
    println!("{:?}", top_three_calories);
}
