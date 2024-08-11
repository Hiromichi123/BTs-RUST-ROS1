// 定义一个前台模块
mod front_of_house {
    // 定义一个hosting子模块，默认属性是私有
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    // 定义一个serving子模块，pub关键字表示公有
    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::serving::take_order();

    // 相对路径
    front_of_house::serving::take_order();
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
        // 使用super关键字访问父模块，类似于文件系统中的..命令
    }

    fn cook_order() {}
}