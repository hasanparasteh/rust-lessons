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
}
