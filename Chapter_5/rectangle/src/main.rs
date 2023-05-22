fn main() {

    let my_rect = Rectangle {
        width: 20,
        height: 30,
    };

    let rect_area = area(&my_rect);

    println!("Rectangle: {:#?}", my_rect);
    println!("Area: {0}", rect_area);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}