fn main() {
    println!("Hello, world!");
    another_function(4, 'A');
    let mut x = five();

    println!("The value of X is: {x}");

    x = plus_four(x);
    println!("The values of X is {x}");

}

fn another_function(x: i32, label: char) {
    println!("The values of {label} is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_four(x: i32) -> i32 {
    x + 4
}