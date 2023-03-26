fn deliver_order() {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
pub fn find_seat_at_restaurant() {
    front_of_house::hosting::seat_at_table();
}

pub fn serving_at_restaurant() {
    front_of_house::serving::take_order();

    front_of_house::serving::serve_order();

    front_of_house::serving::take_payment();
}

pub fn fix_order() {
    back_of_house::fix_incorrect_order();
}