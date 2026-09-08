pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut skip_next = 0;
    for v in s.chars() {
        if v == '-' {
            result.pop();
        } else if v == '+' {
            skip_next += 1;
        } else if skip_next > 0 {
            skip_next -= 1;
        } else {
            result.push(v);
        }
    }
    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for equation in v.iter_mut() {
        let mut result:i32 = 0;
        if let Some((a, b)) = equation.split_once('+') {
            result = a.parse::<i32>().unwrap() + b.parse::<i32>().unwrap();
        } else if let Some ((a, b)) = equation.split_once('-') {
            result = a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap();
        }
        *equation = result.to_string();
    }
}
