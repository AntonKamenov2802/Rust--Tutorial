fn main() {

    let my_rect = Rectangle {
        width: 20,
        height: 30,
    };
    
    let my_rect2 = Rectangle {
        width: 10,
        height: 10,
    };

    let my_square = Rectangle::square(4);

    let rect_area = my_rect.area();
    println!("Can hold: {0}", my_rect2.can_hold(&my_rect));
    println!("Rectangle: {:#?}", my_rect);
    println!("Area: {0}", rect_area);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        let area = rect.area();
        let my_area = self.area();

        area <= my_area
    }
    // associete function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}