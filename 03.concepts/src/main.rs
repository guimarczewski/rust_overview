fn main() {

    // variable
    let mut x = 5;
    println!("The value is {}", x);
    x = 6;
    println!("The value is {}", x);

    // constant
    const SUBSCRIBER_COUNT: u32 = 100_000;

    // shadowing - new variable with an existing name, replacing
    let mut x = 5;
    println!("The value is {}", x);
    let x = "six";
    println!("The value is {}", x);

    let tup = ("ABC", 100_000);
}
