#[macro_use] extern crate lazy_static;
use std::env;

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_seven;
mod day_eight;
mod utils;


fn main(){
    if let Some(day) = env::args().nth(1) {
        match day.as_str() {
           "one" => day_one::run_challenges(),
           "two" => day_two::run_challenges(),
           "three" => {
                day_three::run_challenge_one();    
                day_three::run_challenge_two();
           }
           "four" => {
               day_four::run_challenge_one();
               day_four::run_challenge_two();
           } 
           "seven" => {
                day_seven::run_challenge_one();
                day_seven::run_challenge_two();
           } 
           "eight" => {
                day_eight::run_challenge_one();
                day_eight::run_challenge_two();
           }
           &_ => println!("day not iplemented")
        }
    }
}


