fn main() {
    println!("Hello, world!");

    let s1 = String::from("TEST is This");
    let indx = first_word(&s1);
    println!("{indx}");
    let s2 = &s1[..4];
    println!("{s2}");

    let s3 = first_word_w_slices(&s1);
    println!("{s3}");

    let a = [1,2,3,4,5];
    let slice = &a[1..3];

}

// fucntion without slices
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
                return i;
            }
        }
    
    s.len()
}

// function with slices

fn first_word_w_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
                return &s[..i];
            }
        }
    &s[..]
}
