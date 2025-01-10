#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct Dishes {
  pub appetizers: Vec<String>,
  pub lunchs: Vec<String>,
  pub side_dishes: Vec<String>,
  pub dairy_products: Vec<String>,
  pub desserts: Vec<String>,
  pub unknown: Vec<String>,
}

#[cfg(target_arch = "wasm32")]
#[wasm::export]
pub struct Dishes {
  appetizers: Vec<String>,
  lunchs: Vec<String>,
  side_dishes: Vec<String>,
  dairy_products: Vec<String>,
  desserts: Vec<String>,
  unknown: Vec<String>,
}

impl Dishes {
  pub fn new(
    appetizers: Vec<String>,
    lunchs: Vec<String>,
    side_dishes: Vec<String>,
    dairy_products: Vec<String>,
    desserts: Vec<String>,
    unknown: Vec<String>,
  ) -> Self {
    Self {
      appetizers,
      lunchs,
      side_dishes,
      dairy_products,
      desserts,
      unknown,
    }
  }

  pub fn appetizers(&self) -> &Vec<String> {
    &self.appetizers
  }

  pub fn lunchs(&self) -> &Vec<String> {
    &self.lunchs
  }

  pub fn side_dishes(&self) -> &Vec<String> {
    &self.side_dishes
  }

  pub fn dairy_products(&self) -> &Vec<String> {
    &self.dairy_products
  }

  pub fn desserts(&self) -> &Vec<String> {
    &self.desserts
  }

  pub fn unknown(&self) -> &Vec<String> {
    &self.unknown
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm::export]
impl Dishes {
  #[wasm_bindgen(getter = appetizers)]
  pub fn _appetizers(&self) -> Vec<String> {
    self.appetizers.clone()
  }

  #[wasm_bindgen(getter = lunchs)]
  pub fn _lunchs(&self) -> Vec<String> {
    self.lunchs.clone()
  }

  #[wasm_bindgen(getter = sideDishes)]
  pub fn _side_dishes(&self) -> Vec<String> {
    self.side_dishes.clone()
  }

  #[wasm_bindgen(getter = dairyProducts)]
  pub fn _dairy_products(&self) -> Vec<String> {
    self.dairy_products.clone()
  }

  #[wasm_bindgen(getter = desserts)]
  pub fn _desserts(&self) -> Vec<String> {
    self.desserts.clone()
  }

  #[wasm_bindgen(getter = unknown)]
  pub fn _unknown(&self) -> Vec<String> {
    self.unknown.clone()
  }
}
