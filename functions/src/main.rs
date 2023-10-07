fn main() {
    println!("Inside the main Function");
    another_function();
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.

    // Functions with Return values
    let a = five();
    println!("{}", a);
    let b = plus_one(a);
    println!("{}", b);
}

fn plus_one(value: u8) -> u8 {
    value + 1
}

fn five() -> u8 {
    5
}

fn another_function() {
    println!("Inside another Function");
}
