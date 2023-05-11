fn main() {

    let mut s1 = String::from("hello");

    let len = calculcate_lenght(&s1);
    println!("{len}");

    change_str(&mut s1);
    println!("{s1}");

}

fn calculcate_lenght(s: &String) -> usize { // Accepts a reference
    s.len()
    // s is borowing the value
}

// fn change_str(s: &String) { // Will not work since the reference is not declared as mutable
//     s.push_str(" , world");
// }

fn change_str(s: &mut String) {
    s.push_str(" , world!");
}

