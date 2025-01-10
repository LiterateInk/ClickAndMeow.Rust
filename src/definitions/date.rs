#[cfg_attr(target_arch = "wasm32", wasm::export)]
#[cfg_attr(feature = "ffi", derive(uniffi::Object))]
pub struct Date {
  pub year: i32,
  pub month: i32,
  pub day: i32,
}

#[cfg_attr(target_arch = "wasm32", wasm::export)]
#[cfg_attr(feature = "ffi", uniffi::export)]
impl Date {
  #[cfg_attr(feature = "ffi", uniffi::constructor)]
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
  pub fn new(year: i32, month: i32, day: i32) -> Self {
    Self { year, month, day }
  }
}
