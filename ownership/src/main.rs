fn main() {
    
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
    println!("————————");

    //  二次释放
    let s1 = String::from("String1");

    //  s1此时已经离开作用域，被释放
    let s2 = s1;

    println!("{}", s2);
    println!("————————");

    // 深拷贝，堆内存复制
    let s3 = String::from("String2");

    let s4 = s3.clone();

    println!("{}, {}", s3, s4);
    println!("————————");

    // 基本类型栈上分配， 不会出现 x 被释放的情况
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    println!("————————");

    // 借用
    let mut s5 = String::from("hello fn");

    change(&mut s5);
    println!("s5 = {}", s5);
    println!("————————");

    // 可变变量引用，无法同时被两次以上引用
    let mut s = String::from("hello");
    // 会存在数据竞争问题
    // let r1 = &mut s;
    // let r2 = &mut s;

    // 也不能同时使用可变与不可变引用
    // let r1 = &s; 
    // let r2 = &s; 
    // let r3 = &mut s; 

    let s1 = &s;
    let s2 = &s;
    println!("s1 = {}, s2 = {}", s1,s2);
    // s1, s2作用域结束，此时可以存在可变变量的引用

    let s3 = &mut s;
    println!("s3 = {}", s3);
    println!("————————");


    // 悬挂引用，指针释放问题
    let references = dangle();
    println!("references = {}", references);
    println!("————————");

    // 字符串slice
    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("first_word = {}", word);
    println!("————————");


}

pub fn change(some_string: &mut String) {
    some_string.push_str(" change");
}

// 如果返回的是指针，有关生命周期的问题，指针可能会被释放出去
// 直接交出所有权
pub fn dangle() -> String{
    let s = String::from("hhhhelo!");
    s
}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
