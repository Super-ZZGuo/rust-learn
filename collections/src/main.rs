use std::collections::{
    HashMap,
};

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.push(6);

    let first = &v[0];

    println!("The first element is: {}", first);

    for i in &v {
        println!("The element is: {}", i);
    }

    for i in &mut v {
        *i += 100;
    }

    for i in &v {
        println!("The element plus is: {}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Hello")),
    ];

    println!("——————————————————————————————————————————————");

    let data = "Hello world!";

    let mut s1 = data.to_string();

    let s2 = "I am fine, thank you!";

    s1.push_str(s2);

    println!("{}", s1);
    
    let s3 = String::from("Hello");

    let s4 = String::from(" world!");

    // 会获取 s1 的所有权，附加上从 s2 中拷贝的内容，并返回结果的所有权
    let s5 = s3 + &s4;

    println!("{}", s5);

    println!("——————————————————————————————————————————————");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = String::from("how a wonderful world!");

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
