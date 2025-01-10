use fetcher::{fetch, HeaderMap, Method, Request, Url};
use scraper::{Html, Selector};

use crate::{Error, Session, BASE_URL};

#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm::append_fetcher, wasm::export)]
pub async fn login(username: &str, password: &str) -> Result<Session, Error> {
  let headers = HeaderMap::new();

  let mut url = Url::parse(BASE_URL).unwrap();
  url.set_path("/connexion");

  let request = Request {
    url,
    method: Method::GET,
    body: None,
    headers,
    follow: false,
  };

  #[cfg(target_arch = "wasm32")]
  let response = fetch(request, &fetcher).await?;

  #[cfg(not(target_arch = "wasm32"))]
  let response = fetch(request).await?;

  let html = response.text();

  let document = Html::parse_document(html.as_str());

  let csrf_token = {
    let selector = Selector::parse("input[name='_csrf_token']").unwrap();

    document
      .select(&selector)
      .next()
      .unwrap()
      .attr("value")
      .unwrap()
  };

  let login_session_id = response
    .headers
    .get_all("Set-Cookie")
    .into_iter()
    .find(|x| x.to_str().unwrap().starts_with("PHPSESSID="))
    .unwrap()
    .to_str()
    .unwrap()
    .split(";")
    .next()
    .unwrap()
    .split("=")
    .nth(1)
    .unwrap();

  let mut url = Url::parse(BASE_URL).unwrap();
  url.set_path("/login-mycheck");

  let mut params: Vec<&str> = Vec::new();

  let csrf_param = format!("_csrf_token={}", csrf_token);
  let username_param = format!("_username={}", username);
  let password_param = format!("_password={}", password);
  let submit_param = format!("_submit={}", "Connexion");

  params.push(&csrf_param);
  params.push(&username_param);
  params.push(&password_param);
  params.push(&submit_param);

  let mut headers = HeaderMap::new();
  headers.insert(
    "Content-Type",
    "application/x-www-form-urlencoded".parse().unwrap(),
  );
  headers.insert(
    "Cookie",
    format!("PHPSESSID={}", login_session_id).parse().unwrap(),
  );

  let request = Request {
    url,
    method: Method::POST,
    body: Some(params.join("&")),
    headers,
    follow: false,
  };

  #[cfg(target_arch = "wasm32")]
  let response = fetch(request, &fetcher).await?;

  #[cfg(not(target_arch = "wasm32"))]
  let response = fetch(request).await?;

  if response.headers["Location"] != BASE_URL.to_owned() + "/" {
    return Err(Error::InvalidCredentials());
  }

  let php_session_id = response
    .headers
    .get_all("Set-Cookie")
    .into_iter()
    .find(|x| x.to_str().unwrap().starts_with("PHPSESSID="))
    .unwrap()
    .to_str()
    .unwrap()
    .split(";")
    .next()
    .unwrap()
    .split("=")
    .nth(1)
    .unwrap();

  #[cfg(target_arch = "wasm32")]
  let session = Session::new(php_session_id, fetcher);

  #[cfg(not(target_arch = "wasm32"))]
  let session = Session::new(php_session_id);

  Ok(session)
}
