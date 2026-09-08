pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut result: Vec<String> = Vec::new();
    for x in a.split_ascii_whitespace() {
        let value = x.parse::<f64>().unwrap().exp();
        result.push(value.to_string());
    }
    let b = result.join(" ");
    (a, b)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let c: Vec<f64> = b
        .iter()
        .map(|x| (*x as f64).abs().ln())
        .collect();
    (b, c)
}
