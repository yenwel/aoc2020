use super::utils;

pub fn run_challenge_one()
{
    let input = utils::read_day(10);
    match input {
        Ok(input) => println!("{:?}",1),
        Err(_) => println!("error")
    }    
}

pub fn run_challenge_two () 
{
    let input = utils::read_day(10);
   match input {
        Ok(input) => println!("{:?}",1),
        Err(_) => println!("error")
    }   

}

#[cfg(test)]
mod tests {
    
    use super::*;
    use super::super::utils;

    #[test]
    fn test_challenge_one() {
        let input = utils::read_test(10, None);
        match input {
            Ok(input) =>{  
                assert_eq!(1,1)
            }
            Err(_) => assert!(false)
        }        
    }

    #[test]
    fn test_challenge_two() {
        let input = utils::read_test(10, None);
        match input {
            Ok(input) =>{  
                assert_eq!(1,1)
            }
            Err(_) => assert!(false)
        }        
    }
}