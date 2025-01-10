#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct Menu {
    pub name: String,
    pub url: String
}

#[cfg(target_arch = "wasm32")]
#[wasm::export]
pub struct Menu {
    name: String,
    url: String
}

impl Menu {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url
        }
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn url(&self) -> &String {
        return &self.url;
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm::export]
impl Menu {
    #[wasm_bindgen(getter = name)]
    pub fn _name(&self) -> String {
        return self.name.clone();
    }

    #[wasm_bindgen(getter = url)]
    pub fn _url(&self) -> String {
        return self.url.clone();
    }
}