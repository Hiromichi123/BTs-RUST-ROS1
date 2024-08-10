struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // 结构体返回值
    User {
        email, // Rust支持的简写，等价于 email: email
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        email: String::from("abc@126.com"),
        username: String::from("abc"),
        active: true,
        sign_in_count: 556,
    };

    // 更新语法
    let user2 = User {
        email: String::from("def@126.com"),
        username: String::from("def"),
        ..user1 // 使用其他实例的值
    };

    // Tuple Struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    // 可以模式匹配访问成员，也可以点索引

    // Unit-Like Struct
    struct UnitLikeStruct;
    let unit_like_struct = UnitLikeStruct;
    // 用于在某个类型上实现trait，而不需要存储数据
}
