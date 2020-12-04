
use super::utils;
use std::collections::HashMap;
use itertools::Itertools;

const BYR: &str = "byr";
const IYR: &str = "iyr";
const EYR: &str = "eyr";
const HGT: &str = "hgt";
const HCL: &str = "hcl";
const ECL: &str = "ecl";
const PID: &str = "pid";
const REQUIRED_KEYS: [&str;7] =  [BYR, IYR, EYR, HGT, HCL, ECL, PID];
const VALID_ECL: [&str; 7] = ["amb","blu","brn","gry","grn","hzl","oth"];

fn check_required_fields_valid_passports(passport: &HashMap<&str,&str>) -> bool {
    passport
        .keys()
        .filter(|key| REQUIRED_KEYS.contains(key))
        .count() == 7

}

fn check_byr(byr: &str)-> bool {
    if let Ok(byr) = byr.parse::<u16>() {
        byr >= 1920 && byr <= 2002
    } else {
        false
    }
}

fn check_iyr(iyr: &str)-> bool {
    if let Ok(iyr) = iyr.parse::<u16>() {
        iyr >= 2010 && iyr <= 2020
    } else {
        false
    }
}

fn check_eyr(eyr: &str)-> bool {
    if let Ok(eyr) = eyr.parse::<u16>() {
        eyr >= 2020 && eyr <= 2030
    } else {
        false
    }
}

fn check_hgt(hgt: &str)-> bool {
    if &(hgt[hgt.len()-2..hgt.len()]) == "cm" {
        if let Ok(hgt_cm) = &hgt[..hgt.len()-2].parse::<u16>()
        {
            *hgt_cm >= 150 && *hgt_cm <= 193
        }
        else{
            false
        }
    }
    else if &(hgt[hgt.len()-2..hgt.len()]) == "in" {
        if let Ok(hgt_in) = &hgt[..hgt.len()-2].parse::<u16>()
        {
            *hgt_in >= 59 && *hgt_in <= 76
        }
        else{
            false
        }
    }
    else {
        false
    }
}

fn check_hcl(hcl: &str)-> bool {
    hcl.chars().nth(0).filter( |c| *c == '#').is_some() 
    &&
    u32::from_str_radix(&hcl.chars().skip(1).collect::<String>(),16).is_ok()
    &&
    hcl.len() == 7
}

fn check_pid(pid: &str)-> bool {
    pid.len() == 9 &&
    pid.chars().skip_while(|c| *c == '0').collect::<String>().parse::<u32>().is_ok()
}

fn check_ecl(ecl: &str)-> bool {   
    VALID_ECL.contains(&ecl)
}
fn check_content_valid_passports(passport: &HashMap<&str,&str>) -> bool {
    if !check_byr(passport[BYR]) {
        println!("invalid byr {}", passport[BYR]);
        false
    }
    else if !check_iyr(passport[IYR]) {
        println!("invalid iyr {}", passport[IYR]);
        false
    }
    else if !check_eyr(passport[EYR]) {
        println!("invalid eyr {}", passport[EYR]);
        false
    }
    else if !check_hgt(passport[HGT]) {
        println!("invalid hgt {}", passport[HGT]);
        false
    }
    else if !check_hcl(passport[HCL]) {
        println!("invalid hcl {}", passport[HCL]);
        false
    }
    else if !check_pid(passport[PID]) {
        println!("invalid pid {}", passport[PID]);
        false
    }
    else if !check_ecl(passport[ECL]) {
        println!("invalid ecl {}", passport[ECL]);
        false
    }
    else
    {
        true
    }
}

fn parse(input: &str)-> Vec<HashMap<&str,&str>> {
    input
        .split("\r\n\r\n")
        .map(
            |passport| 
                passport
                    .split_whitespace()
                    .map(|kv| kv.splitn(2,':').collect_tuple())
                    .flat_map(|e| e)
                    .collect())
        .collect()
}

fn check_validity_challenge_one(passports: Vec<HashMap<&str,&str>> ) -> usize{
    passports
        .iter()
        .filter(|passport|check_required_fields_valid_passports(passport))
        .count()
}


fn check_validity_challenge_two(passports: Vec<HashMap<&str,&str>> ) -> usize{
    passports
        .iter()
        .filter(|passport|check_required_fields_valid_passports(passport))
        .filter(|passport|check_content_valid_passports(passport))
        .count()
}


pub fn run_challenge_one()
{
    let input = utils::read_day(4);
    match input {
        Ok(input) => println!("{}",check_validity_challenge_one(parse(&input))),
        Err(_) => println!("error")
    }        
}

pub fn run_challenge_two () 
{
    let input = utils::read_day(4);
   match input {
        Ok(input) => println!("{}",check_validity_challenge_two(parse(&input))),
        Err(_) => println!("error")
    }   

}

#[cfg(test)]
mod tests {
    
    use super::*;
    use super::super::utils;

    #[test]
    fn test_challenge_one() {
        let input = utils::read_test(4, None);
        match input {
            Ok(input) =>{
                let passports = parse(&input);
                assert_eq!(passports.len(),4);
                let count_correct = check_validity_challenge_one(passports);
                assert_eq!(count_correct,2)
            }
            Err(_) => assert!(false)
        }        
    }

    #[test]
    fn test_challenge_two_checks() {
        assert!(check_byr("2002"));        
        assert!(!check_byr("2003"));

        assert!(check_hgt("60in"));        
        assert!(check_hgt("190cm"));        
        assert!(!check_hgt("190in"));        
        assert!(!check_hgt("190"));        

        assert!(check_hcl("#123abc"));        
        assert!(!check_hcl("#123abz"));        
        assert!(!check_hcl("123abc"));        
        
        assert!(check_ecl("brn"));        
        assert!(!check_ecl("wat"));        
        
        assert!(check_pid("000000001"));
        assert!(check_pid("087499704"));      
        assert!(!check_pid("0123456789"));
    
    }

    #[test]
    fn test_challenge_two_valid() {
        let input = utils::read_test(4, Some("valid"));
        match input {
            Ok(input) =>{
                let passports = parse(&input);
                assert_eq!(passports.len(),4);
                println!("{:?}", passports);
                assert!(check_content_valid_passports(&passports[0]));
                assert!(check_content_valid_passports(&passports[1]));
                assert!(check_content_valid_passports(&passports[2]));
                assert!(check_content_valid_passports(&passports[3]));
                let count_correct = check_validity_challenge_two(passports);
                assert_eq!(count_correct,4)
            }
            Err(_) => assert!(false)
        }        
    }

    #[test]
    fn test_challenge_two_invalid() {
        let input = utils::read_test(4, Some("invalid"));
        match input {
            Ok(input) =>{
                let passports = parse(&input);
                assert_eq!(passports.len(),4);
                let count_correct = check_validity_challenge_two(passports);
                assert_eq!(count_correct,0)
            }
            Err(_) => assert!(false)
        }        
    }
}