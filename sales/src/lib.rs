#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Default::default(),
            receipt: Default::default(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some((_, price)) = s.products.iter().find(|product| product.0 == ele) {
            self.items.push((ele, *price));
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut values: Vec<f32> = self.items.iter().map(|product| product.1).collect();
        values.sort_by(|a, b| a.total_cmp(b));

        let total: f32 = values.iter().sum();
        let free: f32 = values.iter().take(values.len() / 3).sum();
        let ratio = (total - free) / total;

        let receipt: Vec<f32> = values
            .iter()
            .map(|v| (v * ratio * 100.0).round() / 100.0)
            .collect();

        self.receipt = receipt.clone();
        receipt
    }
}
