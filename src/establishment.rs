use crate::{Error, Establishment, Session, BASE_URL};
use reqwest::{redirect::Policy, ClientBuilder};
use scraper::{Html, Selector};
use url::Url;

pub async fn get_establishments(session: &Session) -> Result<Vec<Establishment>, Error> {
  let mut url = Url::parse(BASE_URL).unwrap();
  url.set_path("/mesmenus");

  let client = ClientBuilder::new().redirect(Policy::default()).build()?;
  let request = client.get(url).headers(session.get_headers());

  let html = request.send().await?.text().await?;
  let document = Html::parse_document(&html);

  let selector = Selector::parse("#select_menu_etablissement>option").unwrap();
  let mut establishements: Vec<Establishment> = Vec::new();

  for menu_option_el in document.select(&selector) {
    let split_url: Vec<&str> = menu_option_el
      .attr("value")
      .unwrap()
      .split('/')
      .collect::<Vec<&str>>()[0..4]
      .to_vec();

    establishements.push(Establishment {
      name: menu_option_el.text().next().unwrap().trim().to_string(),
      url: split_url.join("/"),
      id: split_url[2].parse::<i32>().unwrap(),
    });
  }

  Ok(establishements)
}
