#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(feature = "ffi", derive(uniffi::Object))]
pub struct Dishes {
    appetizers: Vec<String>,
    lunchs: Vec<String>,
    side_dishes: Vec<String>,
    dairy_products: Vec<String>,
    desserts: Vec<String>,
    unknown: Vec<String>
}

#[cfg(target_arch = "wasm32")]
#[wasm::export]
pub struct Dishes {
    appetizers: Vec<String>,
    lunchs: Vec<String>,
    side_dishes: Vec<String>,
    dairy_products: Vec<String>,
    desserts: Vec<String>,
    unknown: Vec<String>
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(feature = "ffi", uniffi::export)]
impl Dishes {
    #[cfg_attr(feature = "ffi", uniffi::constructor)]
    pub fn new(
        appetizers: Vec<String>,
        lunchs: Vec<String>,
        side_dishes: Vec<String>,
        dairy_products: Vec<String>,
        desserts: Vec<String>,
        unknown: Vec<String>
    ) -> Self {
        Self {
            appetizers,
            lunchs,
            side_dishes,
            dairy_products,
            desserts,
            unknown
        }
    }

    pub fn appetizers(&self) -> &Vec<String> {
        return &self.appetizers;
    }

    pub fn lunchs(&self) -> &Vec<String> {
        return &self.lunchs;
    }

    pub fn side_dishes(&self) -> &Vec<String> {
        return &self.side_dishes;
    }

    pub fn dairy_products(&self) -> &Vec<String> {
        return &self.dairy_products;
    }
    
    pub fn desserts(&self) -> &Vec<String> {
        return &self.desserts;
    }

    pub fn unknown(&self) -> &Vec<String> {
        return &self.unknown;
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm::export]
impl Dishes {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(
        appetizers: Vec<String>,
        lunchs: Vec<String>,
        side_dishes: Vec<String>,
        dairy_products: Vec<String>,
        desserts: Vec<String>,
        unknown: Vec<String>
    ) -> Self {
        Self {
            appetizers,
            lunchs,
            side_dishes,
            dairy_products,
            desserts,
            unknown
        }
    }

    #[wasm_bindgen(getter = appetizers)]
    pub fn _appetizers(&self) -> Vec<String> {
        return self.appetizers.clone();
    }

    #[wasm_bindgen(getter = lunchs)]
    pub fn _lunchs(&self) -> Vec<String> {
        return self.lunchs.clone();
    }

    #[wasm_bindgen(getter = sideDishes)]
    pub fn _side_dishes(&self) -> Vec<String> {
        return self.side_dishes.clone();
    }

    #[wasm_bindgen(getter = dairyProducts)]
    pub fn _dairy_products(&self) -> Vec<String> {
        return self.dairy_products.clone();
    }
    
    #[wasm_bindgen(getter = desserts)]
    pub fn _desserts(&self) -> Vec<String> {
        return self.desserts.clone();
    }

    #[wasm_bindgen(getter = unknown)]
    pub fn _unknown(&self) -> Vec<String> {
        return self.unknown.clone();
    }
}