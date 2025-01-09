use crate::{Session, definitions::Menu, Error};
use fetcher::{fetch, Method, Request, Url};
use scraper::{Html, Selector};

#[cfg_attr(feature = "ffi", uniffi::export)]
#[cfg_attr(target_arch = "wasm32", wasm::export)]
pub async fn get_menus(session: &Session, establishment_id: u16) -> Result<Vec<Menu>, Error> {
    let mut url = Url::parse(&session.base_url).unwrap();
    url.set_path(&format!("/mesmenus/{}/0/2025/01/08", establishment_id));

    let request = Request {
        url,
        method: Method::GET,
        headers: session.get_headers(),
        follow: false,
        body: None
    };

    #[cfg(target_arch = "wasm32")]
    let response = fetch(request, &fetcher).await?;

    #[cfg(not(target_arch = "wasm32"))]
    let response = fetch(request).await?;

    let html = &response.text();
    let document = Html::parse_document(html);

    let selector = Selector::parse("#select_menu_repas>option").unwrap();
    
    let mut menus: Vec<Menu> = Vec::new();

    for menu_option_el in document.select(&selector) {
        menus.push(Menu {
            name: menu_option_el.text().next().unwrap().trim().to_string(),
            url: menu_option_el.attr("value").unwrap().split('/').collect::<Vec<&str>>()[0..4].join("/")
        });
    }

    Ok(menus)
}