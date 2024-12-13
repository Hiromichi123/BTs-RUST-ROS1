fn main() {
    // 函数中定义泛型
    // PartialOrd是一个trait，实现该trait的类型才可以使用比较符比较大小
    // Copy是一个trait，实现该trait的类型可以通过复制的方式传递给函数，而不是通过引用的方式传递
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    
    // Struct中定义泛型
    struct Point<T, U> {
        x: T,
        y: U,
    }
    let integer = Point { x: 5, y: 4.0 }; // 自动类型推断

    // Enum中定义泛型，例如Option、Result
    enum Option<T> {
        Some(T),
        None,
    }

    // Trait中定义泛型
    impl <T> Option<T> { // 表示在类型T上实现方法
        fn unwrap(self) -> T {
            match self {
                Option::Some(value) => value,
                Option::None => panic!("called `Option::unwrap()` on a `None` value"),
            }
        }
    }
}
