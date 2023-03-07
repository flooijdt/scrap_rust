use reqwest;
use scraper::{Html, Selector};
use tokio;

#[tokio::main]
async fn main() {
    let url: &str = "https://helix-editor.com/";
    let mut req = reqwest::get(url).await;

    let body = Html::parse_document(&req.text().unwrap());
    let matter = Selector::parse(".matter").unwrap();

    for m in body.select(&matter) {
        let matters = m.text().collect::<Vec<_>>();
        println!("{:#?}", &matters[0]);
    }
}
