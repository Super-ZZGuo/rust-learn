
enum Option<T> {
    None,
    Some(T),
}


enum Coin {
    Penny(String),
    Nickel(String),
}

fn value_in_cents(c : &Coin) {
    match c {
        Coin::Penny(wow) => {
            println!("{}! You are so lucky!", wow);
            
        }
        other  => {
            println!("other! You are so lucky!");
            
        }
    }
}
fn main() {
    let mut coin = Coin::Penny(String::from("wow"));
    let mut coin2 = Coin::Nickel(String::from("wowwwwww"));
    value_in_cents(&coin);
    value_in_cents(&coin2);

    println!("Hello, world!");

    // 此时 coin2已经被释放
    // let coin2 = Coin::Nickel(String::from("wooooooooow"));
    if let Coin::Nickel(state) = coin2 {
        println!("{}! You are so lucky!", state);
    } else {
        println!("You are not lucky!");
    }
}
