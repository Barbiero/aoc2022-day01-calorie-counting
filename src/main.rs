use std::{fs, io};

const INPUT_FILENAME: &str = "./inputs/input.txt";

fn read_input(filename: &str) -> Result<Vec<Vec<i32>>, io::Error> {
    let binding = fs::read_to_string(filename)?;
    let lines = binding.lines();
    let mut elf_packs: Vec<Vec<i32>> = vec![];
    elf_packs.push(vec![]);

    let mut curr_pack = &mut elf_packs[0];

    for line in lines {
        let parsed_line = line.parse::<i32>();
        match parsed_line {
            Ok(calories) => {
                curr_pack.push(calories);
            }
            Err(_) => {
                elf_packs.push(vec![]);
                curr_pack = elf_packs.last_mut().unwrap();
            }
        }
    }

    return Ok(elf_packs);
}

fn find_max_calories(elf_packs: Vec<Vec<i32>>) -> i32 {
    let mut max = -1;
    for elf_pack in elf_packs {
        let calories = elf_pack.iter().fold(0, |acc, curr| acc + curr);
        if calories > max {
            max = calories;
        }
    }
    return max;
}

fn find_top3_calories(elf_packs: &Vec<Vec<i32>>) -> [i32; 3] {
    let mut max = -1;
    for elf_pack in elf_packs {
        let calories = elf_pack.iter().fold(0, |acc, curr| acc + curr);
        if calories > max {
            max = calories;
        }
    }

    let mut sorted_calories: Vec<i32> = elf_packs
        .iter()
        .map(|elf_pack| elf_pack.iter().fold(0, |acc, curr| acc + curr))
        .collect();
    sorted_calories.sort();
    sorted_calories.reverse();

    return [sorted_calories[0], sorted_calories[1], sorted_calories[2]];
}

fn part1() {
    let file_lines = read_input(INPUT_FILENAME);
    match file_lines {
        Ok(input) => println!("Elf with most calories: {}", find_max_calories(input)),
        Err(e) => println!("called `Result::unwrap()` on an `Err` value: {}", &e),
    }
}

fn part2() {
    let file_lines = read_input(INPUT_FILENAME);
    match file_lines {
        Ok(input) => {
            let top3 = find_top3_calories(&input);
            println!("Top 3 elves with most calories: {:#?}", top3);
            println!("Total: {}", top3.iter().sum::<i32>())
        }
        Err(e) => println!("called `Result::unwrap()` on an `Err` value: {}", &e),
    }
}

fn main() {
    println!("Part1:");
    part1();
    println!("======");
    println!("Part2:");
    part2();
}
