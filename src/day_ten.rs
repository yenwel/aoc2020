use super::utils;

fn parse(input: &str) -> Vec<u128>{
    let mut joltages_adaptors: Vec<u128> = input
    .split("\r\n")
    .map(|joltage| joltage.parse::<u128>())
    .flat_map(|joltage_opt| joltage_opt)
    .collect();
    joltages_adaptors.sort();
    joltages_adaptors
}

fn joltages(input :&str) ->  u128 {    
    let (_, diff_one, diff_three) = 
        parse(input)
            .iter()
            .fold((0,0,0),|(prev_joltage,diff_one,diff_three), joltage| {
                println!("{:?}",(prev_joltage, diff_one, diff_three));
                match joltage - prev_joltage {
                    1 => (*joltage,diff_one+1,diff_three),
                    3 => (*joltage,diff_one,diff_three+1),
                    _ => (*joltage,diff_one,diff_three)
                }
            } );
    diff_one * (diff_three+1)
}

fn find_removable(input :&str) ->  u128 {   
    let (_,_,_,_,result) = 
        parse(input)
            .iter()
            .fold((0,3,0,0,1),|(prev_joltage,prev_diff, count_removable, prev_removable,count_permutations), joltage| {
                if prev_diff + (joltage - prev_joltage) < 3 {
                    println!("{:?}",(prev_joltage, prev_diff, count_removable));
                    (*joltage,joltage - prev_joltage, count_removable + 1, *joltage,count_permutations*2)
                }
                else {
                    (*joltage,joltage - prev_joltage, count_removable, prev_removable,count_permutations)
                }
            } );
    result
}


pub fn run_challenge_one()
{
    let input = utils::read_day(10);
    match input {
        Ok(input) => println!("{:?}",joltages(&input)),
        Err(_) => println!("error")
    }    
}

pub fn run_challenge_two () 
{
    let input = utils::read_day(10);
   match input {
        Ok(input) => println!("{:?}",find_removable(&input)),
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
                assert_eq!(joltages(&input),220)
            }
            Err(_) => assert!(false)
        }        
    }

    #[test]
    fn test_challenge_two_simple() {
        let input = utils::read_test(10, Some("simple"));
        match input {
            Ok(input) =>{  
                assert_eq!(find_removable(&input),8)
            }
            Err(_) => assert!(false)
        }        
    }

    #[test]
    fn test_challenge_two() {
        let input = utils::read_test(10, None);
        match input {
            Ok(input) =>{  
                assert_eq!(find_removable(&input),32768)
            }
            Err(_) => assert!(false)
        }        
    }
}