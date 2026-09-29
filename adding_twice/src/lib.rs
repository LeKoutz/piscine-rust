pub fn add_curry(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

pub fn twice<T>(func: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x| func(func(x))
}
