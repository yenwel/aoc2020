#[macro_use] extern crate lazy_static;

mod day_one;
mod day_two;
mod day_three;
mod utils;


fn main(){
    day_one::run_challenges();
    day_two::run_challenges();
    day_three::run_challenge_one();    
    day_three::run_challenge_two();
}


