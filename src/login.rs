use reqwest::header::HeaderMap;
use scraper::{Html, Selector};
use url::Url;

use crate::{Error, Session, BASE_URL};

pub async fn login(username: &str, password: &str) -> Result<Session, Error> {
  let mut url = Url::parse(BASE_URL)?;
  url.set_path("/connexion");

  let client = reqwest::Client::new();
  let request = client.get(url).build()?;

  let response = client.execute(request).await?;
  let response_headers = response.headers().clone();

  let html = response.text().await?;
  let document = Html::parse_document(&html);

  let csrf_token = {
    let selector = Selector::parse("input[name='_csrf_token']").unwrap();

    document
      .select(&selector)
      .next()
      .unwrap()
      .attr("value")
      .unwrap()
  };

  let login_session_id = response_headers
    .get_all("Set-Cookie")
    .into_iter()
    .find(|x| x.to_str().unwrap().starts_with("PHPSESSID="))
    .unwrap()
    .to_str()?
    .split(";")
    .next()
    .unwrap()
    .split("=")
    .nth(1)
    .unwrap();

  let mut url = Url::parse(BASE_URL).unwrap();
  url.set_path("/login-mycheck");

  let params = [
    format!("_csrf_token={}", csrf_token),
    format!("_username={}", username),
    format!("_password={}", password),
    format!("_submit={}", "Connexion"),
  ];

  let mut request_headers = HeaderMap::new();
  request_headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);
  request_headers.insert("Cookie", format!("PHPSESSID={}", login_session_id).parse()?);

  let request = client
    .post(url)
    .headers(request_headers)
    .body(params.join("&"))
    .build()?;

  let response = client.execute(request).await?;
  let response_headers = response.headers();

  if let Some(location) = response_headers.get("Location") {
    if location.to_str().unwrap() != BASE_URL.to_owned() + "/" {
      return Err(Error::InvalidCredentials());
    }
  }
  else {
    return Err(Error::InvalidCredentials());
  }

  let php_session_id = response_headers
    .get_all("Set-Cookie")
    .into_iter()
    .find(|x| x.to_str().unwrap().starts_with("PHPSESSID="))
    .unwrap()
    .to_str()?
    .split(";")
    .next()
    .unwrap()
    .split("=")
    .nth(1)
    .unwrap();

  Ok(Session::new(php_session_id))
}
