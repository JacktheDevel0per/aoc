mod part_one;
mod both;
use common::{get_input, get_test_input};
use part_one::part_one;

#[cfg(test)]
mod tests {
    

    use std::io::BufRead;

    use super::*;

    #[test]
    fn test_part_one() {


        let input = get_test_input(include_str!("test_input.txt"));
        
        let result = part_one(input);


        assert_eq!(result, 6440);
    }
}
