
use regex::Regex;

use super::utils;

pub fn challenge_day_two(){

    let input = utils::read_day(2).unwrap();

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<char>[[:lower:]]{1}): (?P<password>[[:lower:]]+)").unwrap();
    }
    
    let mut count_correct = 0;
    let mut count_correct_second = 0;
    for s in input.split("\r\n").into_iter(){
        let caps = RE.captures(s).unwrap();
        let min = &caps["min"].parse::<usize>().unwrap();
        let max = &caps["max"].parse::<usize>().unwrap();
        let char =  &caps["char"];
        let password =  &caps["password"];
        let count = password.matches(char).count();
        if count >= *min && count <= *max {
            println!("{:#?} passed first policy", s);
            count_correct += 1;
        }    
        if (password.chars().nth(min-1).unwrap() == char.chars().nth(0).unwrap()) ^ (password.chars().nth(max-1).unwrap() == char.chars().nth(0).unwrap()) {
            println!("{:#?} passed second policy", s);
            count_correct_second += 1;
        }    
    }
    println!("{:#?} passed first policy", count_correct);
    println!("{:#?} passed second policy", count_correct_second);
}