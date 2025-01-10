#[cfg_attr(target_arch = "wasm32", wasm::export)]
pub struct Menu {
    name: String,
    url: String
}

#[cfg(not(target_arch = "wasm32"))]
impl Menu {
    #[cfg_attr(feature = "ffi", uniffi::constructor)]
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url
        }
    }

    pub fn _name(&self) -> &String {
        return &self.name;
    }

    pub fn _url(&self) -> &String {
        return &self.url;
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm::export]
impl Menu {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url
        }
    }

    #[wasm_bindgen(getter = name)]
    pub fn _name(&self) -> String {
        return self.name.clone();
    }

    #[wasm_bindgen(getter = url)]
    pub fn _url(&self) -> String {
        return self.url.clone();
    }
}