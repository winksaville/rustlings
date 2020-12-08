// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

// I AM DONE

fn main() {
    //let vec0 = Vec::new();

    // Other ways to initialize to empty with default capacity
    //let vec0 = vec![];
    //let vec0 = vec!();
    //let vec0: Vec<i32> = vec![];
    //let vec0: Vec<i32> = vec!();
    //let vec0 = Vec::<i32>::new();

    // You can also set the capacity explicitly, which might
    // enhance performance or save space.
    //let vec0 = Vec::with_capacity(0);
    let vec0 = Vec::with_capacity(4);

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

//fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//    let mut vec = vec;
//
//    vec.push(22);
//    vec.push(44);
//    vec.push(66);
//
//    vec
//}

// Make the "move" inside fill_vec obvious
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vecx = vec;

    vecx.push(22);
    vecx.push(44);
    vecx.push(66);

    vecx
}
