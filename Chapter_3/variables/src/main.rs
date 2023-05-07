fn main() {
    const MY_CONSTANT: u32 = 4;

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of My Constant is: {MY_CONSTANT}");

    // Shadowing a variable
    let y = 1; // Declare Y as 1
    println!("Declaring y as: {y}");
    let y = y * MY_CONSTANT; // Re-declare y as 1 * 4
    println!("Re-declaring y as: {y}");
    {
        let y = y * 4;
        println!("Y in inner scope: {y}");
    }
    println!("Y in outer scope: {y}");
    
}
