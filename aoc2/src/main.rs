use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let path = "numbers.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut vector: Vec<String> = vec![];

    for line in buffered.lines() {
        match line {
            Err(why) => panic!("{:?}", why),
            Ok(str) => vector.push(str),
        }
    }
    solve1(&vector);
    solve2(&vector);
    Ok(())
}

fn solve1(input: &[String]) {
    let count2 = get_count(&input, &2);
    let count3 = get_count(&input, &3);
    println!("{}", count2 * count3);
}

fn get_count(input: &[String], freq: &u32) -> usize {
    input.iter().filter(|str| has_frequency(str, freq)).count()
}

fn has_frequency(str: &String, frequency: &u32) -> bool {
    let mut lookup: HashMap<char, u32> = HashMap::new();

    for chr in str.chars() {
        let count = match lookup.get(&chr) {
            Some(current) => *current,
            None => 0,
        };

        lookup.insert(chr, count + 1);
    }

    lookup.values().any(|cnt| cnt == frequency)
}

fn solve2(input: &[String]) {
    let mut remaining = input;

    let result = loop {
        match remaining.split_first() {
            Some((str1, rest)) => match rest.iter().find(|str2| get_distance(str1, str2) == 1) {
                Some(str2) => break Some(get_common_letters(str1, str2)),
                None => remaining = rest,
            },
            None => break None,
        }
    };

    match result {
        Some(letters) => println!("{}", letters),
        None => println!("No results"),
    }
}

fn get_distance(str1: &String, str2: &String) -> usize {
    str1.chars()
        .zip(str2.chars())
        .filter(|(chr1, chr2)| chr1 != chr2)
        .count()
}

fn get_common_letters(str1: &String, str2: &String) -> String {
    str1.chars()
        .zip(str2.chars())
        .filter(|(chr1, chr2)| chr1 == chr2)
        .map(|(chr1, _chr2)| chr1)
        .collect()
}
