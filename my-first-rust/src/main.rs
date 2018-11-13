use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    //println!("{}", load_file("input.txt"));
    let raw_votes = load_file("input.txt");

    let mut vote_counts = HashMap::new();
    for vote in &raw_votes {
        *vote_counts.entry(&vote.fruit).or_insert(0) += vote.votes;
        /*match vote_counts.get(&vote.fruit) {
            None => vote_counts.insert(vote.fruit, vote.votes),
            Some(v) => *vote_counts.get_mut(&vote.fruit).unwrap() += vote.votes
        }*/
    }

    //println!("{}", &vote_counts["apple"])
    for (key, value) in vote_counts.iter() {
        println!("{}, {}", key, value);
    }
    // TODO: Sort by value

    //vote_counts.sort_by
}

struct Vote {
    fruit: String,
    votes: i32,
}


fn load_file(file_name: &str) -> Vec<Vote> {
    println!("Loading file {}...", file_name);
    let mut votes = Vec::new();
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't open {}: {}", file_name, why.description()),
    };
    let file = BufReader::new(file);

    for line in file.lines() {
        //let &'static str: line_str = line.unwrap();
        let line_str = line.unwrap().to_string();
        let line_content: Vec<String> = line_str.split(',').map(|line|line.to_string()).collect();
        println!("Debug: ..{}..{}..", line_content[0].clone(), line_content[1].clone().trim().parse::<i32>().unwrap());
        votes.push(Vote {
            fruit: line_content[0].clone(),
            votes: line_content[1].clone().trim().parse::<i32>().unwrap()
        });
    }
    votes
}
