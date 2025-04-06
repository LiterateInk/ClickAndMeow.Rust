use reqwest::header::HeaderMap;

pub struct Session {
  php_sess_id: String,
}

impl Session {
  pub fn new(php_sess_id: &str) -> Self {
    Self {
      php_sess_id: php_sess_id.to_string(),
    }
  }
}

impl Session {
  pub fn php_sess_id(&self) -> &str {
    &self.php_sess_id
  }

  pub fn get_headers(&self) -> HeaderMap {
    let cookies = "PHPSESSID=".to_string() + &self.php_sess_id;

    let mut headers = HeaderMap::new();
    headers.insert("Cookie", cookies.parse().unwrap());
    headers
  }
}
