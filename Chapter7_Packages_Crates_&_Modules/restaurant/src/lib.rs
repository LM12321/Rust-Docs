//library root crate


fn deliver_order(){}
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {} //sibilings

        fn seat_at_table() {}   //siblings
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //would break if not for mod in pub mod housing & add_to_waitlist...
    // as those are private by default

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();


}