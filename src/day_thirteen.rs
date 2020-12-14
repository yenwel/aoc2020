use super::utils;

fn find_delay_and_bus(input: &str) -> (usize, usize) {
    let bus_schedule = input.split("\r\n").collect::<Vec<&str>>();
    let earliest = bus_schedule[0].parse::<usize>().unwrap();
    bus_schedule[1]
        .split(",")
        .filter(|bus| *bus != "x")
        .map(|bus| bus.parse::<usize>())
        .flat_map(|bus| bus)
        .map(|bus| (((((earliest / bus ) * bus) + bus) % earliest) , bus))
        .min_by_key(|(delay, _)| *delay)
        .unwrap()
}

pub fn run_challenge_one()
{
    let input = utils::read_day(13);
    match input {
        Ok(input) => {
            let (bus, delay) = find_delay_and_bus(&input); 
            println!("{:?}",bus * delay)
        }
        Err(_) => println!("error")
    }    
}

pub fn run_challenge_two () 
{
    let input = utils::read_day(13);
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
        let input = utils::read_test(13, None);
        match input {
            Ok(input) =>{ 
                let (bus, delay) = find_delay_and_bus(&input); 
                assert_eq!(bus * delay,295)
            }
            Err(_) => assert!(false)
        }        
    }

    #[test]
    fn test_challenge_two() {
        let input = utils::read_test(13, None);
        match input {
            Ok(input) =>{  
                assert_eq!(1,1)
            }
            Err(_) => assert!(false)
        }        
    }
}