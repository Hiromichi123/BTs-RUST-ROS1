// 定义一个前台模块
mod front_of_house {
    // 定义一个hosting子模块
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    // 定义一个serving子模块
    pub mod serving { // pub关键字表示该模块是公有的
        pub fn take_order() {} // pub关键字表示该函数是公有的
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}