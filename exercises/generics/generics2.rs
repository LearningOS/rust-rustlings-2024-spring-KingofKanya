// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


struct Wrapper <T> {
    value: T,
}

// 第一个 <T> 是泛型类型参数的声明，它表明 Wrapper 结构体是一个泛型类型，它可以接受任意类型的值作为其成员。

// 第二个 <T> 出现在 new 方法的返回类型 Self 之前，它表示方法 new 也是一个泛型方法，它接受一个类型
// 参数 T，并返回一个包含 T 类型值的 Wrapper<T> 实例。这个 <T> 的作用域仅限于方法 new 内部。
impl <T> Wrapper <T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
