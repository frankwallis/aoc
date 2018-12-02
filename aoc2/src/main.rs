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
    let count2 = get_count(&input, 2);
    let count3 = get_count(&input, 3);
    println!("{}", count2 * count3);
}

fn get_count(vector: &[String], freq: i32) -> i32 {
    vector.iter().fold(0, |result, str| {
        if has_frequency(str, &freq) {
            result + 1
        } else {
            result
        }
    })
}

fn has_frequency(str: &String, frequency: &i32) -> bool {
    let mut lookup: HashMap<char, i32> = HashMap::new();

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

    let answer = loop {
        match remaining.split_first() {
            Some((str1, rest)) => {
                let found = rest.iter().fold(None, |result, str2| match result {
                    None => {
                        if get_distance(str1, str2) == 1 {
                            Some(get_common_letters(str1, str2))
                        } else {
                            None
                        }
                    }
                    Some(letters) => Some(letters),
                });

                match found {
                    Some(letters) => break Some(letters),
                    None => remaining = rest,
                }
            }
            None => break None,
        }
    };

    match answer {
        Some(letters) => println!("{}", letters),
        None => println!("No results"),
    }
}

fn get_distance(str1: &String, str2: &String) -> i32 {
    str1.chars().zip(str2.chars()).fold(
        0,
        |result, (chr1, chr2)| if chr1 == chr2 { result } else { result + 1 },
    )
}

fn get_common_letters(str1: &String, str2: &String) -> String {
    str1.chars()
        .zip(str2.chars())
        .filter(|(chr1, chr2)| chr1 == chr2)
        .map(|(chr1, _chr2)| chr1)
        .collect()
}
