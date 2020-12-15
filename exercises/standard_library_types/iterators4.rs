#![feature(iterator_fold_self)]
// iterators4.rs

// I AM DONE

pub fn recursive(num: u64) -> u64 {
    let mut fact = num;
    if num > 1 {
        fact = num * recursive(num - 1)
    }
    println!("recursive: num={} fact={}", num, fact);
    fact
}

pub fn simplest_recursive(num: u64) -> u64 {
    if num > 1 {
        num * simplest_recursive(num - 1)
    } else {
        num
    }
}

pub fn match_recursive(num: u64) -> u64 {
    match num {
        n if n > 1 => n * match_recursive(n - 1),
        n => n,
    }
}

pub fn for_in(num: u64) -> u64 {
    let mut fact = 1;
    for i in 1..num+1 {
        fact *= i;
        println!("for_in: num={} fact={}", num, fact);
    }
    fact
}

// Create iterable version of Factorial

struct Factorial {
    num: u64,
    factorial: u64,
}

impl Factorial {
    fn new(num: u64) -> Factorial {
        Factorial {
            num,
            factorial: 1
        }
    }
}

impl Iterator for Factorial {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        println!("Factorial::next:+ self.num={}", self.num);
        if self.num > 0 {
            self.factorial *= self.num;
            self.num -= 1;
            println!("Factorial::next:- Some self.factorial={}", self.factorial);
            return Some(self.factorial)
        } else {
            println!("Factorial::next:- None self.factorial={}", self.factorial);
            return None
        }
    }
}

pub fn loop_next_iterator(num: u64) -> u64 {
    let mut f = Factorial::new(num);
    loop {
        let n = f.next();
        if n == None {
            break;
        }
        println!("iterator: f.next()={}", n.unwrap());
    }
    println!("iterator:- f.factorial={}", f.factorial);
    f.factorial
}

pub fn for_in_iterator(num: u64) -> u64 {
    println!("iterator:+ num={}", num);
    let f = Factorial::new(num);
    let mut x: u64 = 1;
    for curf in f {
        println!("iterator: curf={}", curf);
        x = curf;
    }
    println!("iterator:- x={}", x);
    x
}

pub fn while_next_iterator(num: u64) -> u64 {
    let mut f = Factorial::new(num);
    while f.next() != None {}
    f.factorial
}

pub fn range_for_each_iterator(num: u64) -> u64 {
    // "rf" is a a reference to mutable fact
    // which is captured by the map closure.
    //let mut fact: u64 = 1;
    //let rf = &mut fact;
    //(1..num+1).map(|n| {
    //    *rf *= n;
    //    println!("n={} *rf={}", n, *rf);
    //} ).for_each(drop);
    //fact

    // Turns out you don't need "rf", I'm guessing because fact
    // is on the left side of "=" and therfore is areference.
    let mut fact: u64 = 1;
    (1..num+1).map(|n| {
        fact *= n;
        //println!("n={} fact={}", n, fact);
    } ).for_each(drop);
    fact
}

pub fn range_fold_iterator(num: u64) -> u64 {
    (1..num+1).fold(1, |fact, n| fact * n)

    //// For debugging add a println
    //(1..num+1).fold(1, |fact, n| {
    //    let f = fact * n;
    //    println!("n={} f={}", n, f);
    //    f
    //} )
}

pub fn range_fold_first_iterator(num: u64) -> u64 {
    (1..num+1).fold_first(|fact, n| fact * n).unwrap()

    //// For debugging add a println
    //(1..num+1).fold_first(|fact, n| {
    //    let f = fact * n;
    //    println!("n={} f={}", n, f);
    //    f
    //} ).unwrap()
}

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // Answer 1 recursive
    //recursive(num)

    // Answer 2 simplest recursive
    //simplest_recursive(num)

    // Answer 3 match recursive
    //match_recursive(num)

    // Answer 4 for_in no recursion
    //for_in(num)

    // Answer 5 iterate using loop
    //loop_next_iterator(num)

    // Answer 6 iterate using for in
    //for_in_iterator(num)

    // Answer 7 iterate using while
    //while_next_iterator(num)

    // Answer 8 Iterate using range and for_each
    //range_for_each_iterator(num)

    // Answer 9 Iterate using range and fold
    // THIS IS THE BEST ANSWER, or maybe fold_first below.
    //range_fold_iterator(num)

    // Answer 10 Iterate using range and fold_first
    // THIS IS THE BEST ANSWER :)
    range_fold_first_iterator(num)
}
 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
