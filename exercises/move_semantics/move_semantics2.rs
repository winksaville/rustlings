// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// I AM DONE

fn main() {
    // hint 1: clone vec0 (I figured this out :)
    // but thought there were other ways so looked at the hint
    {
        let vec0 = Vec::new();
        let mut vec1 = fill_vec(vec0.clone());
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
        vec1.push(88);
        println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    }

    // hint2
    {
        let vec0 = Vec::new();
        let mut vec1 = dup_fill_vec(&vec0);
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
        vec1.push(88);
        println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    }

    // hint 3a: pass mutable vec0 and don't return anything
    {
        let mut vec0 = Vec::new();
        append_vec(&mut vec0);
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
        vec0.push(88);
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    }

    // hint 3b
    {
        let vec0 = &mut Vec::new();
        append_vec(vec0);
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
        vec0.push(88);
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    }
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn dup_fill_vec(cur_vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = cur_vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn append_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
