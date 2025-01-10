use fetcher::HeaderMap;

#[cfg(target_arch = "wasm32")]
#[wasm::export]
pub struct Session {
  php_sess_id: String,
  base_url: String,
  fetcher: js_sys::Function,
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(feature = "ffi", derive(uniffi::Object))]
pub struct Session {
  php_sess_id: String,
  base_url: String
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(feature = "ffi", uniffi::export)]
impl Session {
  #[cfg_attr(feature = "ffi", uniffi::constructor)]
  pub fn new(php_sess_id: &str) -> Self {
    Self {
      php_sess_id: php_sess_id.to_string(),
      base_url: "https://www.clicetmiam.fr".to_string()
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm::export]
impl Session {
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
  pub fn new(php_sess_id: &str, fetcher: js_sys::Function) -> Self {
    Self {
      php_sess_id: php_sess_id.to_string(),
      base_url: "https://www.clicetmiam.fr".to_string(),
      fetcher,
    }
  }

  #[wasm_bindgen(getter = phpSessId)]
  pub fn _php_sess_id(&self) -> String {
    self.php_sess_id.clone()
  }

  #[wasm_bindgen(getter = baseUrl)]
  pub fn _base_url(&self) -> String {
    self.base_url.clone()
  }
}

impl Session {
  pub fn php_sess_id(&self) -> &str {
    &self.php_sess_id
  }

  pub fn base_url(&self) -> &String {
    return &self.base_url;
  }

  #[cfg(target_arch = "wasm32")]
  pub fn fetcher(&self) -> &js_sys::Function {
    &self.fetcher
  }

  pub fn get_headers(&self) -> HeaderMap {
    // We only need the PHPSESSID cookie.
    let cookies = "PHPSESSID=".to_string() + &self.php_sess_id;

    // Build the headers with that cookie !
    let mut headers = HeaderMap::new();
    headers.insert("Cookie", cookies.parse().unwrap());
    headers
  }
}