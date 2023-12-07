use either::{self, Either};
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

pub fn reverse_itr<'a, Container: DoubleEndedIterator<Item = T>, T>(
    into_itr: Container,
    reverse: bool,
) -> Either<
    std::iter::Rev<<Container as IntoIterator>::IntoIter>,
    <Container as IntoIterator>::IntoIter,
> {
    let itr = into_itr.into_iter();
    if reverse {
        Either::Left(itr.rev())
    } else {
        Either::Right(itr)
    }
}

pub fn get_input() -> Vec<String> {
    let mut lines_in = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines_in.push(match line {
            Ok(ln) => ln,
            Err(err) => {
                println!("Error: {}", err);
                "".to_string()
            }
        });
    }
    lines_in
}


pub fn get_test_input(raw_str: &str) -> Vec<String> {
    raw_str.lines().map(|s| s.to_string()).collect()
}

pub fn reverse_hashmap(map: &HashMap<u32, u32>) -> HashMap<u32, u32> {
    let mut new_map = HashMap::new();

    for (key, value) in map {
        new_map.insert(*value, *key);
    }

    new_map
}

#[no_mangle]
#[inline(never)]
pub fn flatten_vec_of_tuple(data: &[(i32, i32)]) -> Vec<i32> {
    data.iter()
        .fold(Vec::with_capacity(data.len() * 2), |mut acc, p| {
            acc.extend(&[p.0, p.1]);
            acc
        })
}
