// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

// I AM DONE

//use std::num::ParseIntError;
//use std::error;
//use std::fmt;
//
//// This always causes main to return 0 on success 1 on failure
//fn main() -> Result<(), Box<dyn error::Error>> {
//    let mut tokens = 100;
//    //let pretend_user_input = "not a number"; // Error: ParseIntError { kine: InvalidDigit }
//    //let pretend_user_input = "80"; // Error: NotEnoughTokens
//    let pretend_user_input = "8"; // Ok: 59 tokens
//
//    let cost = total_cost(pretend_user_input)?;
//
//    if cost > tokens {
//        // To return multiple errors see errorsn.rs :)
//        return Err(Box::new(CreationError::NotEnoughTokens));
//    } else {
//        tokens -= cost;
//        println!("You now have {} tokens.", tokens);
//    }
//    Ok(())
//}
//#[derive(PartialEq, Debug)]
//enum CreationError {
//    NotEnoughTokens,
//}
//
//impl fmt::Display for CreationError {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        let description = match *self {
//            CreationError::NotEnoughTokens=> "You can't afford that many",
//        };
//        f.write_str(description)
//    }
//}
//
//impl error::Error for CreationError {}

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
