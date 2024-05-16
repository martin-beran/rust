pub use front_of_house::hosting::NAME;
pub use front_of_house::hosting::NAME as RESTAURANT_NAME;

mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

fn deliver_order() {}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::wait();
    // made possible by use ...
    hosting::seat_at_table();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I would like {} toast, please!", meal.toast);
    //meal.seasonal_fruit =  String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("Eating");
}

pub mod customer {
    use crate::front_of_house::self_service::Order as SelfOrder;
    use crate::front_of_house::serving::Order;
    pub fn eat_at_restaurant() {
        // use does not apply here directly
        //hosting::add_to_waitlist();
        super::hosting::add_to_waitlist();
        let order = Order {};
        let order = SelfOrder {};
        println!("Customer eating");
    }
}
