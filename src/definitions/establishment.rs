#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct Establishment {
  pub name: String,
  pub url: String,
  pub id: i32,
}

#[cfg(target_arch = "wasm32")]
#[wasm::export]
pub struct Establishment {
  name: String,
  url: String,
  id: i32,
}

impl Establishment {
  pub fn new(name: String, url: String, id: i32) -> Self {
    Self { name, url, id }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn id(&self) -> &i32 {
    &self.id
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm::export]
impl Establishment {
  #[wasm_bindgen(getter = name)]
  pub fn _name(&self) -> String {
    self.name.clone()
  }

  #[wasm_bindgen(getter = url)]
  pub fn _url(&self) -> String {
    self.url.clone()
  }

  #[wasm_bindgen(getter = id)]
  pub fn _id(&self) -> i32 {
    self.id.clone()
  }
}
