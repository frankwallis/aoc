use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::collections::{HashMap};

fn main() -> Result<()> {
    let path = "numbers.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut vector: Vec<i32> = vec![];

    for line in buffered.lines() {
        match line {
            Err(why)   => panic!("{:?}", why),
            Ok(string) => match string.trim().parse::<i32>() {
                Err(why2)        => panic!("{:?}", why2),
                Ok(number)=> vector.push(number)
            }
        }    
    }
    let res1: i32 = vector.iter().sum();
    println!("{}", res1);

    let res2: i32 = get_first_repeated_sum(vector);
    println!("{}", res2);

    Ok(())
}

fn get_first_repeated_sum(vector: Vec<i32>) -> i32 {
    let mut lookup: HashMap<i32, bool> = HashMap::new();
    let mut current_idx: usize = 0;
    let mut current_sum: i32 = 0;

    loop {
        let current_num = vector[current_idx];
        current_sum = current_sum + current_num;

        if lookup.contains_key(&current_sum) {
            break current_sum;
        } else {
            lookup.insert(current_sum, true);
        }

        current_idx = if current_idx == vector.len() -1 { 0 } else { current_idx + 1 };
    }
}
