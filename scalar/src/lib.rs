pub fn sum(left: u8, right: u8) -> u8 {
    left + right
}

pub fn diff(left: i16, right: i16) -> i16 {
    left - right
}

pub fn pro(left: i8, right: i8) -> i8 {
    left * right
}

pub fn quo(left: f32, right: f32) -> f32 {
    left / right
}

pub fn rem(left: f32, right: f32) -> f32 {
    left % right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn diff_test() {
        let result = diff(2, 2);
        assert_eq!(result, 0);
    }
    #[test]
    fn pro_test() {
        let result = pro(2, 5);
        assert_eq!(result, 10);
    }
    #[test]
    fn quo_test() {
        let result = quo(22.0, 2.0);
        assert_eq!(result, 11.0);
    }
    #[test]
    fn rem_test() {
        let result = rem(-128.23, 2.0);
        assert_eq!(result, -0.22999573);
    }
}
