pub fn stars(n: u32) -> String {
    let base: usize = 2;
    "*".repeat(base.pow(n))
}
