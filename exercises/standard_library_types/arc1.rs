// arc1.rs
// Make this code compile by filling in a value for `shared_numbers` where the
// TODO comment is and create an initial binding for `child_numbers`
// somewhere. Try not to create any copies of the `numbers` Vec!
// Execute `rustlings hint arc1` for hints :)

// I AM DONE

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u64).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    // Answer 1
    for offset in 0..8 {
        let child_numbers = shared_numbers.clone();
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 8;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }

    //// Answer 2
    //let threads = 10;
    //let shard_size = shared_numbers.len() / threads;
    //if shared_numbers.len() % threads != 0 {
    //    panic!("shard_size isn't a multiple of numbers.len");
    //}
    //for offset in 0..threads {
    //    let child_numbers = shared_numbers.clone();
    //    joinhandles.push(thread::spawn(move || {
    //        let base = shard_size * offset;
    //        let mut sum = 0;
    //        for i in 0..shard_size {
    //            sum += child_numbers[base + i];
    //        }
    //        println!("Sum of offset {} is {}", offset, sum);
    //    }));
    //}

    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
