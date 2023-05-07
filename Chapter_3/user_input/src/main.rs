use std::io;

fn main() {
    
    let my_array: [u8; 4] = [1, 2, 3, 4];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

    let element = my_array[index];

    println!("The value from array is {element}");

}
