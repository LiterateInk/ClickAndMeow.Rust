#[cfg_attr(target_arch = "wasm32", wasm::export)]
pub struct Menu {
    pub name: String,
    pub url: String
}