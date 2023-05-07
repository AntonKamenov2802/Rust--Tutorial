fn main() {
    // Tuple
    let tup: (i32, f64, u8) = (444, 4.4, 4);
    
    let (x, _y, z) = tup;
    let x = tup.0;
    println!("The value of z is: {z}");
    println!("The value of x is: {x}");

    // Array

    let my_array = [1, 2, 3, 4]; // Data is collected on the stack, fixed number of elements
    let my_array: [i32; 4] = [1, 2, 3, 4];

    let arr = [3; 5]; // [3, 3, 3, 3, 3]

    let first_el = my_array[0];
    let last_el = arr[4];

    println!("First element: {first_el}");
    println!("Last element: {last_el}");

}
