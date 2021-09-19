use std::io;

fn main() {
    // PART 1
    // Mutable vs Immutable
    // all variables are immutable by default

    // let x = 5; // wont work at all because it's immutable
    let mut x = 5; // works just fine because it's mutable
    println!("the value of x is: {}", x);
    x = 6; // we can change mutable variables
    println!("The value of x is: {}", x);

    // PART 2
    // Immutable vs Constants
    // 1. const must be annotated!
    // 2. constants can be declared in any scope
    // 3. constants may be set only to a constant expression 🤔
    // (not the result of a function call or any other value that could only be computed at runtime.)

    const MAX_POINTS: u32 = 100_000; // underscores can be inserted in numeric literals to improve readability
    println!("Max points is: {}", MAX_POINTS);

    // Part 3
    // Shadowing
    // Caution: We can't use shadowing methods on mutable variables
    let spaces = "    ";
    let spaces = spaces.len();
    println!("There is {} spaces in that string", spaces);

    // Part 3
    // I skiped the types that makes sense if you had any
    // programming background. let's jump into `Tuple`

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1); // it can detect types too...
    let (x, y, z) = tup;
    println!("Wow the values are: {}, {}, {}", x, y, z);

    let second_tuple_value = tup.1; // access tuple values with '.'
    print!("this is the second value of tuple: {}", second_tuple_value);

    // Array
    // All array elements must be the same type!
    let numbers_list: [i32;5] = [1, 2, 3, 4, 5]; // This is an array with i32 type and 5 elements.
    println!("Number array is: {:?}", numbers_list);

    let second_number_of_array = numbers_list[1];
    println!("Second number of array is: {}", second_number_of_array);

    // Invalid Array Element Access
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");
    let index: usize = index // convert type of index that user typed
        .trim()
        .parse()
        .expect("index that you enterd was not a number.");
    let element = numbers_list[index];
    println!("the value of the element at index {} is: {}", index, element);

}
