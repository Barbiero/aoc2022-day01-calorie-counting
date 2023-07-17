use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

const INPUT_FILENAME: &str = "./inputs/input.txt";

/** from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html */
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(BufReader::new(file).lines())
}

fn read_input(filename: &str) -> Result<Vec<Vec<i32>>> {
    let lines = read_lines(filename)?;

    let mut elf_packs: Vec<Vec<i32>> = vec![vec![]];
    let mut curr_pack = &mut elf_packs[0];

    for line in lines.flatten() {
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

    Ok(elf_packs)
}

fn find_max_calories(elf_packs: Vec<Vec<i32>>) -> i32 {
    elf_packs
        .iter()
        .map(|elf_pack| elf_pack.iter().sum())
        .max()
        .unwrap_or_default()
}

fn find_top3_calories(elf_packs: Vec<Vec<i32>>) -> [i32; 3] {
    let mut calories_heap = elf_packs
        .iter()
        .map(|elf_pack| elf_pack.iter().sum())
        .collect::<BinaryHeap<i32>>();

    [
        calories_heap.pop().unwrap_or_default(),
        calories_heap.pop().unwrap_or_default(),
        calories_heap.pop().unwrap_or_default(),
    ]
}

fn part1() -> Result<()> {
    let input = read_input(INPUT_FILENAME)?;
    println!("Elf with most calories: {}", find_max_calories(input));

    Ok(())
}

fn part2() -> Result<()> {
    let input = read_input(INPUT_FILENAME)?;

    let top3 = find_top3_calories(input);
    println!("Top 3 elves with most calories: {:#?}", top3);
    println!("Total: {}", top3.iter().sum::<i32>());

    Ok(())
}

fn main() -> Result<()> {
    println!("Part1:");
    part1()?;
    println!("======");
    println!("Part2:");
    part2()
}
