use std::collections::HashMap;

use super::both::{Hand, PlayingCard};




pub fn part_one(input: Vec<String>) -> u32 {
    println!("Part one");

    //Maps a hand to the bet for it.
    let hand_map: HashMap<Vec<PlayingCard>, u32> = input
        .iter()
        .map(|line| {
            let both = line.split_whitespace().collect::<Vec<_>>();
            (PlayingCard::parse_vec_from_str(both[0]), both[1].parse::<u32>().unwrap())
            
        })
        .collect();


    0


}


