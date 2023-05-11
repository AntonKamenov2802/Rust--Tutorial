fn main() {
    
    let s = "hello"; // Hardcoded string

    let mut s_new = String::from("hello"); // 
    
    s_new.push_str(", world!");
    println!("{}", s_new);

    {
        let my_string = String::from("The test");
        let my_string_2 = my_string;

        // println!("{}", my_string); // Won't work, my_string is MOVED to my_string_2
        
        let my_string_3 = my_string_2.clone(); // Will deep copy the string
        println!("{my_string_2}, cloned: {my_string_3}");

    }
    let test = String::from("this is a test");
    takes_ownership(test);

    let my_int = 3;
    makes_copy(my_int);

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
