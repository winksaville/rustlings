// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// I AM DONE

fn main() {
    //// Answer 1
    //let mut shopping_list: Vec<&str> = Vec::new();
    //shopping_list.push("milk");

    // Answer 2
    let mut shopping_list: Vec<String> = Vec::new();
    shopping_list.push("milk".into());
}
