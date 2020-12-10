use super::utils;
use itertools::Itertools;

fn parse_program(input : &str) -> Vec<(&str,i128)> {
    input
        .split("\r\n")
        .map(|instruction| instruction.splitn(2,' ').collect_tuple())
        .flat_map(|option| option)
        .map(|(operation, value)|(operation,value.parse::<i128>().unwrap()))
        .collect()
}

fn run(program: &Vec<(&str,i128)>, switch : Option<usize>) -> (i128, bool) {
    println!("program run {:?}",switch);
    const ACC: &str = "acc";
    const JMP: &str = "jmp";
    const NOP: &str = "nop";
    let mut program_counter: usize = 0;
    let mut accumulator: i128  = 0;
    let mut visited_instructions: Vec<usize> = [].to_vec();
    let mut stop_run = true; 
    let mut infinity_run = true;
    while stop_run {  
        if visited_instructions.contains(&program_counter) || program_counter >= program.len()  { 
            stop_run = false;
            infinity_run = visited_instructions.contains(&program_counter);
        }
        else {            
            println!("{:?}  accumulator {} counter {}", program[program_counter],accumulator,program_counter+1);
            let instruction = program[program_counter];
            visited_instructions.push(program_counter);
            match instruction.0 {
                ACC => {
                    accumulator += instruction.1;
                    program_counter += 1;
                }
                JMP => {
                    program_counter = match switch{
                        Some(do_switch) => if do_switch == program_counter {
                            program_counter + 1
                        }
                        else{ 
                            (program_counter as i128 + instruction.1) as usize
                        }
                        None => (program_counter as i128 + instruction.1) as usize
                    }
                }
                NOP => {                
                    program_counter = match switch{
                        Some(do_switch) => if do_switch == program_counter {
                            (program_counter as i128 + instruction.1) as usize
                        }
                        else{ 
                            program_counter + 1                            
                        }
                        None => program_counter + 1
                    }
                }
                _ => {}
            }
        }
    }        
    (accumulator, infinity_run)
}

fn find_fix(program: &Vec<(&str,i128)>) -> (i128, usize) {
    
    if let Some(((accumulator,_),switch))=  (0..(program.len()-1))
        .map(|switch|(run(program, Some(switch)), switch))
        .find(|((_,is_infinity),_)| !is_infinity) {
            (accumulator, switch)
    } else {
        (0,0)
    }

}

pub fn run_challenge_one()
{
    let input = utils::read_day(8);
    match input {
        Ok(input) => println!("{:?}", run(&parse_program(&input),None)),
        Err(_) => println!("error")
    }    
}

pub fn run_challenge_two () 
{
    let input = utils::read_day(8);
   match input {
        Ok(input) => println!("{:?}",find_fix(&parse_program(&input))),
        Err(_) => println!("error")
    }   

}

#[cfg(test)]
mod tests {
    
    use super::*;
    use super::super::utils;

    #[test]
    fn test_parsers() {
        
        let input = utils::read_test(8, None);
        match input {
            Ok(input) =>{  
                assert_eq!(parse_program(&input), [("nop", 0), ("acc", 1), ("jmp", 4), ("acc", 3), ("jmp", -3), ("acc", -99), ("acc", 1), ("jmp", -4), ("acc", 6)])
            }
            Err(_) => assert!(false)
        }        
    }

    #[test]
    fn test_challenge_one() {
        let input = utils::read_test(8, None);
        match input {
            Ok(input) =>{  
                assert_eq!(run(&parse_program(&input), None),(5,true))
            }
            Err(_) => assert!(false)
        }        
    }

    #[test]
    fn test_challenge_two() {
        let input = utils::read_test(8, None);
        match input {
            Ok(input) =>{  
                assert_eq!(find_fix(&parse_program(&input)),(8,7))
            }
            Err(_) => assert!(false)
        }        
    }
}