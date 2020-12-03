// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// I AM DONE

// Put your function here!
fn calculate_apple_price(count: i32) -> i32 {
    // Answer 1
    if count > 40 { count } else { count * 2 }

    // Answer 2
    //let mut cost_per_apple = 2i32;
    //if count > 40 {
    //    cost_per_apple /= 2;
    //}
    //cost_per_apple * count
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
