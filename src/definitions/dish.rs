#[cfg_attr(target_arch = "wasm32", wasm::export)]
pub struct Dishes {
    pub appetizers: Vec<String>,
    pub lunchs: Vec<String>,
    pub side_dishes: Vec<String>,
    pub dairy_products: Vec<String>,
    pub desserts: Vec<String>,
    pub unknown: Vec<String>
}