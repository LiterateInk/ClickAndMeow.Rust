#[cfg(target_arch = "wasm32")]
#[wasm::export]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32
}

#[cfg(target_arch = "wasm32")]
#[wasm::export]
impl Date {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(year: i32, month: i32, day: i32) -> Self {
        Self {
            year,
            month,
            day
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(feature = "ffi", uniffi::export)]
impl Date {
    #[cfg_attr(feature = "ffi", uniffi::constructor)]
    pub fn new(year: i32, month: i32, day: i32) -> Self {
        Self {
            year,
            month,
            day
        }
    }
}