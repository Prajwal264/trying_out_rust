fn main() {
    let _x = 10;
    let x = 100;
    {
        let x = 1000;
        println!("The Value of X is {}", x);
    }
    println!("The Value of X is {}", x);
}
