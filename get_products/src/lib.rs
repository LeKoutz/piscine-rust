pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut result = Vec::new();
    if arr.len() < 2 {
        return result
    }
    for i in 0..arr.len() {
        let product: usize = arr.iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, value)| *value)
            .product();
        result.push(product);
    }
    result
}