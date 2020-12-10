// errors2.rs
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy,
// and the `total_cost` function will calculate the total number of tokens.
// Since the player typed in the quantity, though, we get it as a string-- and
// they might have typed anything, not just numbers!

// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is:
// if we call the `parse` function on a string that is not a number, that
// function will return a `ParseIntError`, and in that case, we want to
// immediately return that error from our function and not try to multiply
// and add.

// There are at least two ways to implement this that are both correct-- but
// one is a lot shorter! Execute `rustlings hint errors2` for hints to both ways.

// I AM DONE

use std::num::ParseIntError;
use std::mem::discriminant;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // Answer 1
    //let qty = item_quantity.parse::<i32>()?;
    //Ok(qty * cost_per_item + processing_fee)

    // Answer 2
    //let qty = match item_quantity.parse::<i32>() {
    //    Ok(qty) => return Ok(qty * cost_per_item + processing_fee),
    //    Err(e) => return Err(e),
    //};

    // Answer 3 How would you use an if/else ladder to instead of a match?
    // Note: This doesn't compile!
    let r = item_quantity.parse::<i32>();
    let rd = discriminant(&r);
    let d = discriminant(&Result::<i32, String>::Ok(123));
    println!("rd={:?} d={:?}", rd, d);

    // Verify that changing the "values" (i32 or string)
    // doesn't change the discriminant
    let ds = discriminant(&Result::<i32, String>::Ok(123));
    assert_eq!(format!("{:?}", ds), "Discriminant(0)");
    let ds = discriminant(&Result::<i32, String>::Ok(124));
    assert_eq!(format!("{:?}", ds), "Discriminant(0)");
    let ds = discriminant(&Result::<i32, String>::Err("bad news".into()));
    assert_eq!(format!("{:?}", ds), "Discriminant(1)");
    let ds = discriminant(&Result::<i32, String>::Err("really bad news".into()));
    assert_eq!(format!("{:?}", ds), "Discriminant(1)");

    
    // Seems you can't compare discriminant's directly :(
    //if discriminant(&r) == discriminant(&Result::<i32, String>::Ok(0)) { // fails
    //if rd == d { // fails
    
    // But you can compare their Display'd valued
    if format!("{:?}", rd) == format!("{:?}", d).as_str() {
        println!("rd == d");
        let qty = r.unwrap();
        Ok(qty * cost_per_item + processing_fee)
    } else {
        println!("rd != d");
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
