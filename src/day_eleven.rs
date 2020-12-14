use super::utils;

fn switch_empty(row: usize, col:usize, current : &Vec<Vec<char>>) -> bool{
    let mut switch = true;
    for k in if row == 0 { 0 } else { row -1 } .. if row == current.len()-1 {current.len()} else { row+2 } {
        for l in  if col == 0 { 0 } else { col -1 } .. if col == current[row].len()-1 { current[row].len()} else { col+2 } {
            if !(row == k && col ==l) {
                switch = switch && (current[k][l] == 'L' || current[k][l] == '.');
            }
        }
    }      
    switch
}

fn switch_seated(row: usize, col: usize, current : &Vec<Vec<char>>) -> bool{
    let mut count = 0;
    for k in if row == 0 { 0 } else { row -1 } .. if row == current.len()-1 {current.len()} else { row+2 } {
        for l in  if col == 0 { 0 } else { col -1 } .. if col == current[row].len()-1 { current[row].len()} else { col+2 } {           
            if !(row == k && col ==l) && current[k][l] == '#' { 
                count += 1
            }
        }
    }
    count >= 4
}


fn iterate_game(current : &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_game :  Vec<Vec<char>> = [].to_vec();
    for row in 0..current.len() {
        let mut new_line :Vec<char> = [].to_vec();
        for col in 0..current[row].len(){
            if !(current[row][col] == '.') {
                new_line.push(match current[row][col] {
                    'L' => if switch_empty(row,col, &current) {'#'} else {'L'}
                    '#' =>  if switch_seated(row,col, &current) {'L'} else {'#'}
                    _ => '?'
                })     
            }
            else {
                new_line.push('.');
            }            
        }
        new_game.push(new_line);
    }
    new_game
}

fn game_of_seats(input: &str, iterator: &dyn Fn(&Vec<Vec<char>>) ->  Vec<Vec<char>>) -> usize {
    let mut seats: Vec<Vec<char>> = 
        input
            .split("\r\n")
            .map(|line| line.chars().collect())
            .collect();
    let mut dont_stop = true;
    println!("first iteration:");
    for line in seats.iter().map(|line|line.into_iter().collect::<String>()) {
        println!("{:?}", line)
    }
    while dont_stop {
        let iteration = iterator(&seats);
        println!("next iteration:");
        for line in iteration.iter().map(|line|line.into_iter().collect::<String>()) {
            println!("{:?}", line)
        }
        if iteration == seats {
            dont_stop = false;
        }
        else {
            seats = iteration;
        }
    }
    seats.iter().flat_map(|rows| rows.iter()).filter(|c| **c == '#').count()
}

pub fn run_challenge_one()
{
    let input = utils::read_day(11);
    match input {
        Ok(input) => println!("{:?}",game_of_seats(&input,&self::iterate_game)),
        Err(_) => println!("error")
    }    
}

pub fn run_challenge_two () 
{
    let input = utils::read_day(11);
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
        let input = utils::read_test(11, None);
        match input {
            Ok(input) =>{  
                assert_eq!(game_of_seats(&input,&self::iterate_game),37)
            }
            Err(_) => assert!(false)
        }        
    }

    #[test]
    fn test_challenge_two() {
        let input = utils::read_test(11, None);
        match input {
            Ok(input) =>{  
                assert_eq!(1,1)
            }
            Err(_) => assert!(false)
        }        
    }
}