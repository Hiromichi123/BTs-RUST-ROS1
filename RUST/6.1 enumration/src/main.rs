enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 为枚举定义方法
impl Message {
    fn call(&self) {
        // 方法体
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1")); 

    let q = Message::Write(String::from("hello"));
    let m = Message::Move { x: 1, y: 2 };
    let w = Message::ChangeColor(0, 0, 0);
    let c = Message::Quit;

    // 调用枚举值的方法
    q.call();
}
