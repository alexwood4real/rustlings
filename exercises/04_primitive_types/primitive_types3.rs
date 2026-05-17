fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???

    // this is an array of 100 signed 32-bit integers that are all initialized to 0;
    let a: [i32; 100] = [0; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
