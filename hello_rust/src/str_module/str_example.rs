pub fn play_with_strings() {
    // Creating a new String
    let mut my_string = String::from("Hello, ");
    println!("Initial string: {}", my_string);

    // Concatenation
    my_string.push_str("World!");
    println!("After concatenation: {}", my_string);

    // Slicing
    let slice = &my_string[0..5];
    println!("Slice: {}", slice);

    // Iterating over string characters
    println!("Characters:");
    for c in my_string.chars() {
        println!("{}", c);
    }

    // String length
    println!("Length of string: {}", my_string.len());
}
