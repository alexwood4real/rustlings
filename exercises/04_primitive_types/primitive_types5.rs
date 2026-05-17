fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;

    // this is a tuple. extracts the objects from the other tuple into this one
    let (name, age) = (cat.0, cat.1);

    println!("{name} is {age} years old");
}
