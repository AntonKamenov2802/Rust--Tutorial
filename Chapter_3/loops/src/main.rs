fn main() {
    let mut count = 0;

    'counting_up_loop: loop {
        println!("count == {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 5 {
                break;
            }
            if count == 2 {
                break 'counting_up_loop;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // loop {
    //      println!("The number is {number}");
    // }
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{result}");

    let mut number = 0;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF");

    let a = [1, 2, 3, 4];

    for element in a {
        println!("The current value is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
}
