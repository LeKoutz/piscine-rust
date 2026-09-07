pub fn fibonacci(n: u32) -> u32 {
    let mut previous = 0;
    let mut current = 1;
    let mut counter = 0;

    while counter < n {
        let next = previous + current;
        previous = current;
        current = next;
        counter += 1;
    }

    previous
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fibonacci(22);
        assert_eq!(result, 17711);
    }
}
