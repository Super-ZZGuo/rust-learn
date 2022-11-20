fn main() {
    let mut x = 5;
    println!("nums is {}", x);

    x = 6;
    println!("nums is {}", x);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("while -> the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("for -> the value is: {}", element);
    }

    for number in (1..10).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
