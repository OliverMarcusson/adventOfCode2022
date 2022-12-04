use std::fs;
use std::time::Instant;

fn fully_contains(sections: &str) -> bool {
    let split_sections: Vec<&str> = sections.split(",").collect();
    let elf_1_sections: Vec<&str> = split_sections[0].split("-").collect();
    let elf_2_sections: Vec<&str> = split_sections[1].split("-").collect();
    let elf_2_sections = parse_sections(&elf_2_sections);
    let elf_1_sections = parse_sections(&elf_1_sections);

    if elf_1_sections[0] <= elf_2_sections[0] && elf_1_sections[1] >= elf_2_sections[1] {
        return true;
    }
    
    if elf_2_sections[0] <= elf_1_sections[0] && elf_2_sections[1] >= elf_1_sections[1] {
        return true;
    }     
    
    false
}

fn parse_sections(sections: &Vec<&str>) -> Vec<u32> {
    let parsed_vec = vec![sections[0].parse::<u32>().unwrap(), sections[1].parse::<u32>().unwrap()];
    parsed_vec
}

fn partly_contains(sections: &str) -> bool {
    let split_sections: Vec<&str> = sections.split(",").collect();
    let elf_1_sections: Vec<&str> = split_sections[0].split("-").collect();
    let elf_2_sections: Vec<&str> = split_sections[1].split("-").collect();
    let elf_2_sections = parse_sections(&elf_2_sections);
    let elf_1_sections = parse_sections(&elf_1_sections);

    if elf_1_sections[0] <= elf_2_sections[0] && elf_1_sections[1] >= elf_2_sections[1] {
        return true;
    }
    
    if elf_2_sections[0] <= elf_1_sections[0] && elf_2_sections[1] >= elf_1_sections[1] {
        return true;
    }
    
    if elf_1_sections[0] <= elf_2_sections[0] && elf_2_sections[1] >= elf_1_sections[1] && elf_1_sections[1] >= elf_2_sections[0] {
        return true;
    }

    if elf_2_sections[0] <= elf_1_sections[0] && elf_1_sections[1] >= elf_2_sections[1] && elf_2_sections[1] >= elf_1_sections[0] {
        return true;
    }
    
    false
}

fn main() {
    let time = Instant::now();
    let input_string = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input_string.split("\n").collect();
    let mut amount: u32 = 0;
    
    for item in input.iter() {
        if fully_contains(item) == true {
            amount += 1
        };
    }
    println!("Amount Part 1: {}", amount);

    amount = 0;
    for item in input.iter() {
        if partly_contains(item) == true {
            amount += 1
        };
    }
    println!("Amount Part 2: {}", amount);
    println!("Time elapsed: {:?}", time.elapsed());
}
