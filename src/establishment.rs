use fetcher::{fetch, Method, Request, Url};
use scraper::{Html, Selector};

use crate::{Error, Establishment, Session, BASE_URL};

#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm::append_fetcher, wasm::export)]
pub async fn get_establishments(session: &Session) -> Result<Vec<Establishment>, Error> {
  let mut url = Url::parse(BASE_URL).unwrap();
  url.set_path("/mesmenus");

  let request = Request {
    url,
    method: Method::GET,
    headers: session.get_headers(),
    follow: true,
    body: None,
  };

  #[cfg(target_arch = "wasm32")]
  let response = fetch(request, &fetcher).await?;

  #[cfg(not(target_arch = "wasm32"))]
  let response = fetch(request).await?;

  let html = &response.text();
  let document = Html::parse_document(html);

  let selector = Selector::parse("#select_menu_etablissement>option").unwrap();

  let mut establishements: Vec<Establishment> = Vec::new();

  for menu_option_el in document.select(&selector) {
    let split_url: Vec<&str> = menu_option_el
      .attr("value")
      .unwrap()
      .split('/')
      .collect::<Vec<&str>>()[0..4]
      .to_vec();

    establishements.push(Establishment::new(
      menu_option_el.text().next().unwrap().trim().to_string(),
      split_url.join("/"),
      split_url[2].parse::<i32>().unwrap(),
    ));
  }

  Ok(establishements)
}
