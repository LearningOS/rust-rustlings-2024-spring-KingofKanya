// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    my_macro!();
}

// 将 my_macro 宏导出到当前模块的外部作用域
// 这样 main 函数所在的模块就能够引用这个宏了
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
