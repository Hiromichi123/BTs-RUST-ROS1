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

// use关键字导入路径
// 函数：一般导入父模块
use crate::front_of_house::serving;
pub fn eat_at_restaurant2() {
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
}

// struct，enum：等一般直接导入本身
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// 使用as关键字重命名导入的模块
use std::fmt::Result;
use std::io::Result as IoResult;

// 使用pub use重导出
mod front_of_house2 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house2::hosting; // 重导出，使外部模块可以直接访问hosting模块

// 使用外部包
// 在Cargo.toml中添加rand = "0.5.5"依赖，然后使用use导入
//注：rust中的标准库（std）也被当做外部包，但不需要在Cargo.toml中添加依赖
use rand::Rng;

// 嵌套路径，减少重复use
use std::{cmp::Ordering, io};
//use std::io::{self, Write}; // self表示io本身，Write表示io::Write

// 使用通配符*导入所有公有定义
use std::collections::*;

// 将模块移动到不同文件
mod front_of_house3; // rust会自动寻找front_of_house3.rs或front_of_house3/mod.rs文件