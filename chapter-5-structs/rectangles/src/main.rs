#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// methods associated with the struct
impl Rectangle {
    fn area(&self) -> u32{
        return self.width * self.height
    }

    fn can_fit(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height
    }
}

// "associate function" - a subset of struct methods that do not call &self
impl Rectangle {
    fn square(length: u32) -> Rectangle {
        return Rectangle {width: length, height: length}
    }

}

fn main() {
    // create rectangles
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    let rect2 = Rectangle {
        width: 5,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 30,
        height: 10,
    };

    let rect4 = Rectangle {
        width: 20,
        height: 10,
    };

    let rect5 = Rectangle::square(10);
    
    // print out rectangle info
    println!("rect is {:?} with area {}", rect,  rect.area());
    println!("rect2 is {:?} with area {}", rect, rect2.area());
    println!("rect3 is {:?} with area {}", rect, rect3.area());
    println!("rect4 is {:?} with area {}", rect, rect4.area());
    println!("rect5 is {:?} with area {}", rect, rect5.area());
    
    // print if rect can fit rect2, rect3, rect4, rect5
    println!("rect can fit rect2? {}", rect.can_fit(&rect2));
    println!("rect can fit rect3? {}", rect.can_fit(&rect3));
    println!("rect can fit rect4? {}", rect.can_fit(&rect4));
    println!("rect can fit rect5? {}", rect.can_fit(&rect5));
}

