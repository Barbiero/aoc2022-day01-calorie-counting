use std::{collections::BinaryHeap, fs, io};

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
    let result = elf_packs
        .iter()
        .map(|elf_pack| elf_pack.iter().sum())
        .max()
        .unwrap_or_default();
    return result;
}

fn find_top3_calories(elf_packs: Vec<Vec<i32>>) -> [i32; 3] {
    let mut calories_heap = elf_packs
        .iter()
        .map(|elf_pack| elf_pack.iter().sum())
        .collect::<BinaryHeap<i32>>();

    return [
        calories_heap.pop().unwrap_or_default(),
        calories_heap.pop().unwrap_or_default(),
        calories_heap.pop().unwrap_or_default(),
    ];
}

fn part1() -> Result<(), io::Error> {
    let input = read_input(INPUT_FILENAME)?;
    println!("Elf with most calories: {}", find_max_calories(input));
    return Ok(());
}

fn part2() -> Result<(), io::Error> {
    let input = read_input(INPUT_FILENAME)?;

    let top3 = find_top3_calories(input);
    println!("Top 3 elves with most calories: {:#?}", top3);
    println!("Total: {}", top3.iter().sum::<i32>());

    return Ok(());
}

fn main() -> Result<(), io::Error> {
    println!("Part1:");
    part1()?;
    println!("======");
    println!("Part2:");
    part2()?;
    return Ok(());
}
