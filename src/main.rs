use reqwest;
use scraper::{Html, Selector};

fn main() {
    let url: &str = "https://helix-editor.com/";
    let mut req = reqwest::get(url).unwrap();

    let body = Html::parse_document(&req.text().unwrap());
    let matter = Selector::parse(".matter").unwrap();

    for m in body.select(&matter) {
        let matters = m.text().collect::<Vec<_>>();
        println!("{:#?}", &matters[0]);
    }
}
