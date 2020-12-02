#[macro_use] extern crate lazy_static;

mod day_one;
mod day_two;
mod utils;

fn main(){
    let input_one = utils::read_day(1);
    match input_one {
        Ok(input_one) => {
            let result = day_one::challenge_one(&input_one);
            println!("{:?}",result);
            match result {
                Some((first, second)) => println!("{:?}",first*second),
                None => {}
            }
            let result_two = day_one::challenge_two(&input_one);
            println!("{:?}", result_two);
            match result_two {
                Some((first, second, third)) => println!("{:?}",first*second*third),
                None => {}
            }
        },
        Err(e) => println!("{}",e)
    }
    day_two::challenge_day_two();
}


