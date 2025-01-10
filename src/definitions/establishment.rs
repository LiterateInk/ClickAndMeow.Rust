#[cfg_attr(target_arch = "wasm32", wasm::export)]
pub struct Establishment {
    name: String,
    url: String,
    id: i32
}

#[cfg(not(target_arch = "wasm32"))]
impl Establishment {
    #[cfg_attr(feature = "ffi", uniffi::constructor)]
    pub fn new(name: String, url: String, id: i32) -> Self {
        Self {
            name,
            url,
            id
        }
    }

    pub fn _name(&self) -> &String {
        return &self.name;
    }

    pub fn _url(&self) -> &String {
        return &self.url;
    }

    pub fn _id(&self) -> &i32 {
        return &self.id;
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm::export]
impl Establishment {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(name: String, url: String, id: i32) -> Self {
        Self {
            name,
            url,
            id
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

    #[wasm_bindgen(getter = id)]
    pub fn _id(&self) -> i32 {
        return self.id.clone();
    }
}