use super::utils;

fn get_index(line: &str, index: usize, right: usize, down: usize) -> usize
{
    (index * right) / down % line.len()
}

fn traverse_slope(input: &str, right: usize, down: usize) -> usize{
    return input.split("\r\n")
            .enumerate()
            .filter(|(index,_)| (index) % down == 0)
            .map(|(index, line)| {
                println!("y {} x {}",index, get_index(line,index,right,down));
                line.chars().nth(get_index(line, index, right, down))
            })
            .filter(|result| result.is_some() && result.unwrap() == '#')
            .count()

}

pub fn run_challenge_one()
{
    let input = utils::read_day(3);
    match input {
        Ok(input) => println!("{}",traverse_slope(&input, 1,3)),
        Err(_) => println!("error")
    }        
}

pub fn run_challenge_two () 
{
    let input = utils::read_day(3);
   match input {
        Ok(input) => 
        {
            println!("{}",
            traverse_slope(&input, 1,1)*
            traverse_slope(&input, 3,1)*
            traverse_slope(&input, 5,1)*
            traverse_slope(&input, 7,1)*
            traverse_slope(&input, 1,2))
        }
        Err(_) => println!("error")
    }   

}

#[cfg(test)]
mod tests {
    
    use super::*;
    use super::super::utils;

    #[test]
    fn test_challenge_one() {
        let input = utils::read_test(3, None);
        match input {
            Ok(input) =>{
                let count1 = traverse_slope(&input,1,1);
                assert_eq!(count1,2);                
                let count2 = traverse_slope(&input,3,1);
                assert_eq!(count2,7);                
                let count3 = traverse_slope(&input,5,1);
                assert_eq!(count3,3);                
                let count4 = traverse_slope(&input,7,1);
                assert_eq!(count4,4);
                let count5 = traverse_slope(&input,1,2);
                assert_eq!(count5,2)
            }
            Err(_) => assert!(false)
        }        
    }
}