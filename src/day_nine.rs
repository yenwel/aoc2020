use super::utils;
use itertools::Itertools;

fn parse(input: &str) -> Vec<u128> {
    input
    .split("\r\n")
    .map(|xmas_val| xmas_val.parse::<u128>())
    .flat_map(|xmas_val| xmas_val)
    .collect()
    

}

fn find_invalid<'a>(input: &str, preamble_size: usize) -> Option<u128> {
    let xmasses = parse(input);
    let found = xmasses
        .iter()
        .enumerate()
        .skip(preamble_size)
        .find(|(xmas_index, xmas)| 
            xmasses
                .iter()
                .skip(xmas_index - preamble_size)
                .take(preamble_size)
                .cartesian_product(xmasses
                    .iter()
                    .skip(xmas_index - preamble_size)
                    .take(preamble_size))
                .find(|(one,two)| {
                    println!("{} + {} =? {} ", one, two, xmas);
                    (*one + *two) == **xmas
                    })
                .is_none()                
        );
    if let Some((_,value)) = found {
        Some(*value)
    }
    else{
        None
    }
}

fn find_contiguous<'a>(input: &str, number: u128) -> Option<u128> {    
    let xmasses = parse(input);
    let found = (0..xmasses.len())
        .cartesian_product(0..xmasses.len())
        .find(|(indexone, indextwo)|
            {
                println!("{} -> {}", indexone, indextwo);
                (indexone != indextwo) && xmasses.iter().skip(*indexone).take(*indextwo).map(|val| *val).sum::<u128>() == number
            }
        );
    if let Some((indexone,indextwo)) = found {
        let min = xmasses.iter().skip(indexone).take(indextwo).min().unwrap();
        let max = xmasses.iter().skip(indexone).take(indextwo).max().unwrap();
        Some(min+max)
    }
    else{
        None
    }
}

pub fn run_challenge_one()
{
    let input = utils::read_day(9);
    match input {
        Ok(input) => println!("{:?}",find_invalid(&input,25)),
        Err(_) => println!("error")
    }    
}

pub fn run_challenge_two () 
{
    let input = utils::read_day(9);
   match input {
        Ok(input) => println!("{:?}",find_contiguous(&input,70639851)),
        Err(_) => println!("error")
    }   

}

#[cfg(test)]
mod tests {
    
    use super::*;
    use super::super::utils;

    #[test]
    fn test_challenge_one() {
        let input = utils::read_test(9, None);
        match input {
            Ok(input) =>{  
                assert_eq!(find_invalid(&input,5),Some(127))
            }
            Err(_) => assert!(false)
        }        
    }

    #[test]
    fn test_challenge_two() {
        let input = utils::read_test(9, None);
        match input {
            Ok(input) =>{  
                assert_eq!(find_contiguous(&input,127),Some(62))
            }
            Err(_) => assert!(false)
        }        
    }
}