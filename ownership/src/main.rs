fn main() {
    let str = String::from("HELLO");
    pass_ownership(str);
    // println!("{}", str); // usage isn't allowed here since str is already borrowed.
}

fn pass_ownership(str: String) {
    println!("{}", str);
}
