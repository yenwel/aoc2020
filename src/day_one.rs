use itertools::Itertools;

pub fn challenge_one (input: &str) -> Option<(u32,u32)>{
    return input.split("\n")
            .cartesian_product(input.split("\n"))
            .map(|(first, second)| (first.parse::<u32>().unwrap(),second.parse::<u32>().unwrap()))
            .find(|(first, second)| first != second && ((first + second) == 2020));

}

pub fn challenge_two (input: &str) -> Option<(u32,u32,u32)>{
    return input.split("\n")
            .cartesian_product(input.split("\n"))
            .cartesian_product(input.split("\n"))
            .map(|((first, second), third)| (first.parse::<u32>().unwrap(),second.parse::<u32>().unwrap(),third.parse::<u32>().unwrap()))
            .find(|(first, second,third)| first != second && second != third && ((first + second + third) == 2020));

}

#[cfg(test)]
mod tests {
    
    use super::*;
    use super::super::utils;

    #[test]
    fn test_challenge_one() {
        let input = utils::read_day(1);
        match input {
            Ok(input) =>{
                let result = challenge_one(&input);
                assert!(result.is_some());
                match result {
                    Some((first, second)) =>  assert_eq!(first+second,2020),
                    None => assert!(false)
                }               
            }
            Err(_) => assert!(false)
        }        
    }
    
     #[test]
    fn test_challenge_two() {
        let input = utils::read_day(1);
        match input {
            Ok(input) =>{
                let result = challenge_two(&input);
                assert!(result.is_some());
                match result {
                    Some((first, second, third)) =>  assert_eq!(first+second+third,2020),
                    None => assert!(false)
                }               
            }
            Err(_) => assert!(false)
        }      
    }
}