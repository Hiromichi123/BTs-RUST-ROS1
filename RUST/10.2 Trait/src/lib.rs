use std::fmt::Display;
use std::fmt::Debug;

pub trait Summary {
    // 一般只写方法签名，不实现
    // fn summarize(&self) -> String;

    // 默认实现
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // 默认实现可以调用其他trait中的方法
    }

    fn summarize_author(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 为 Tweet 实现 Summary trait
impl Summary for Tweet {
    // 当存在默认实现时，这里就可以选择不写或重载
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Trait作为参数
// impl Trait语法，本质是Trait Bound的语法糖
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound语法
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 使用 + 指定多个Trait Bound（impl Trait写法）
pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

// 使用 + 指定多个Trait Bound（Trait Bound写法）
pub fn notify4<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// where语法
pub fn notify5<T, U>(t: &T, u: &U) -> String 
    where T: Summary + Display,
          U: Display + Debug
{
    format!("{}", t.summarize())
}

// Trait作为返回值
fn returns_summarizable() -> impl Summary {
    // 注意只能返回一个类型，不能有多种可能的返回值
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 使用Trait Bound有条件的实现方法，即覆盖实现（blanket implementation）
pub struct Pair<T> {
    x: T,
    y: T,
}
// 为Pair<T>中只有实现了Display和PartialOrd的T实现cmp_display方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
