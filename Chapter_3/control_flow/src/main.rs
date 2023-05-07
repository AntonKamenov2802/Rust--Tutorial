fn main() {
    println!("Hello, world!");

    let number = 4;

    if number <= 4 {
        println!("Number is <= 4");
    } else if number > 4 {
        println!("Number is less than 5");
    } else {
        println!("Number is more than 5");
    }

    let condition = true;
    let number = if condition {4} else {0};

}
