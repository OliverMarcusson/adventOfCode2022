use std::fs;

fn fully_contains(sections: &str) -> bool {
    let split_sections: Vec<&str> = sections.split(",").collect();
    let elf_1_sections: Vec<&str> = split_sections[0].split("-").collect();
    let elf_2_sections: Vec<&str> = split_sections[1].split("-").collect();

    if elf_1_sections[0].parse::<u32>().unwrap() <= elf_2_sections[0].parse::<u32>().unwrap() && elf_1_sections[1].parse::<u32>().unwrap() >= elf_2_sections[1].parse::<u32>().unwrap() {
        return true;
    }
    
    if elf_2_sections[0].parse::<u32>().unwrap() <= elf_1_sections[0].parse::<u32>().unwrap() && elf_2_sections[1].parse::<u32>().unwrap() >= elf_1_sections[1].parse::<u32>().unwrap() {
        return true;
    }     
    
    false
}

fn partly_contains(sections: &str) -> bool {
    let split_sections: Vec<&str> = sections.split(",").collect();
    let elf_1_sections: Vec<&str> = split_sections[0].split("-").collect();
    let elf_2_sections: Vec<&str> = split_sections[1].split("-").collect();

    if elf_1_sections[0].parse::<u32>().unwrap() <= elf_2_sections[0].parse::<u32>().unwrap() && elf_1_sections[1].parse::<u32>().unwrap() >= elf_2_sections[1].parse::<u32>().unwrap() {
        return true;
    }
    
    if elf_2_sections[0].parse::<u32>().unwrap() <= elf_1_sections[0].parse::<u32>().unwrap() && elf_2_sections[1].parse::<u32>().unwrap() >= elf_1_sections[1].parse::<u32>().unwrap() {
        return true;
    }
    
    if elf_1_sections[0].parse::<u32>().unwrap() <= elf_2_sections[0].parse::<u32>().unwrap() && elf_2_sections[1].parse::<u32>().unwrap() >= elf_1_sections[1].parse::<u32>().unwrap() && elf_1_sections[1].parse::<u32>().unwrap() >= elf_2_sections[0].parse::<u32>().unwrap() {
        println!("Elf 1 Partly contain Elf 2: {:?}", split_sections);
        return true;
    }

    if elf_2_sections[0].parse::<u32>().unwrap() <= elf_1_sections[0].parse::<u32>().unwrap() && elf_1_sections[1].parse::<u32>().unwrap() >= elf_2_sections[1].parse::<u32>().unwrap() && elf_2_sections[1].parse::<u32>().unwrap() >= elf_1_sections[0].parse::<u32>().unwrap() {
        println!("Elf 2 Partly contain Elf 1: {:?}", split_sections);
        return true;
    }
    
    false
}


fn main() {
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
}
