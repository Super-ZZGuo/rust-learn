
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width: width, height: height }
    }

    // Self 指代 impl 后面的结构体名字
    fn square(size : u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn who_is_bigger(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}
fn main() {
    let num = 2;
    let rect = Rectangle {
        width: num * 2,
        height: num,
    };

    println!("rect is {:#?}", rect);
    println!("-————————————————————————————————");

    let rect1 = Rectangle::new(20, 30);
    let rect2 = Rectangle::square(25);

    println!("Is rect1 area bigger than rect2 ? {}", rect1.who_is_bigger(&rect2));


}
