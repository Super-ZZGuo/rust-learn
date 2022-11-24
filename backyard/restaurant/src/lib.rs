
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// pub fn eat_at_restaurant() {
//     // 绝对路径
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 相对路径
//     front_of_house::hosting::add_to_waitlist();
// }

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    fn serve_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("food"),
            }
        }
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 生吐司
    let mut meal = back_of_house::Breakfast::summer("Raw");

    // 访问成员属性，修改为：不生吐司
    meal.toast = String::from("Not raw");

    // 没有访问权限
    // meal.seasonal_fruit = String::from("Not food");
    hosting::add_to_waitlist();

}