use crate::{
  definitions::{Date, Dishes, Menu},
  Error, Session, BASE_URL,
};

use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;

pub async fn get_menus(session: Session, establishment_id: u16) -> Result<Vec<Menu>, Error> {
  let mut url = Url::parse(BASE_URL).unwrap();
  url.set_path(&format!("/mesmenus/{}/0/2025/01/08", establishment_id));

  let client = Client::new();
  let request = client
    .get(url.clone())
    .headers(session.get_headers())
    .build()?;

  let html = client.execute(request).await?.text().await?;
  let document = Html::parse_document(&html);

  let selector = Selector::parse("#select_menu_repas>option").unwrap();

  let mut menus = Vec::new();

  for menu_option_el in document.select(&selector) {
    menus.push(Menu {
      name: menu_option_el.text().next().unwrap().trim().to_string(),
      url: menu_option_el
        .attr("value")
        .unwrap()
        .split('/')
        .collect::<Vec<&str>>()[0..4]
        .join("/"),
    });
  }

  Ok(menus)
}

pub async fn get_menu_dishes(session: &Session, menu: &Menu, date: &Date) -> Result<Dishes, Error> {
  let mut url = Url::parse(BASE_URL).unwrap();
  url.set_path(&format!(
    "{}/{}/{:0>2}/{:0>2}",
    menu.url, date.year, date.month, date.day
  ));

  let client = Client::new();
  let request = client
    .get(url.clone())
    .headers(session.get_headers())
    .build()?;

  let html = client.execute(request).await?.text().await?;
  let document = Html::parse_document(&html);

  let dish_container_selector = Selector::parse(".menu_composante_container").unwrap();

  let title_selector = Selector::parse(".menu_composante_title").unwrap();
  let dishes_selector = Selector::parse(".menu-composante-libelle").unwrap();

  let mut appetizers = Vec::new();
  let mut lunchs = Vec::new();
  let mut side_dishes = Vec::new();
  let mut dairy_products = Vec::new();
  let mut desserts = Vec::new();
  let mut unknown = Vec::new();

  for dish_container in document.select(&dish_container_selector) {
    let title_element = dish_container.select(&title_selector).next().unwrap();
    let title_label = title_element
      .text()
      .next()
      .unwrap()
      .strip_suffix(" -")
      .unwrap()
      .strip_prefix("- ")
      .unwrap();

    let mut dishes = Vec::new();

    for dish_label in dish_container.select(&dishes_selector) {
      dishes.push(dish_label.text().next().unwrap().trim().to_string());
    }

    match title_label {
      "Hors d'oeuvre" => appetizers.extend(dishes),
      "Plat" => lunchs.extend(dishes),
      "Garniture" => side_dishes.extend(dishes),
      "Produit Laitier" => dairy_products.extend(dishes),
      "Dessert" => desserts.extend(dishes),
      _ => unknown.extend(dishes),
    };
  }

  Ok(Dishes {
    appetizers,
    lunchs,
    side_dishes,
    dairy_products,
    desserts,
    unknown,
  })
}
