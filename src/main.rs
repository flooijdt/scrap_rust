use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let url: &str = "https://roadmap.sh/blockchain";
    let req = reqwest::get(url).await.unwrap().text().await.unwrap();

    let body = Html::parse_document(&req);
    let selection = Selector::parse("li").unwrap();

    for m in body.select(&selection) {
        let selections = m.text().collect::<Vec<_>>();
        println!("{:#?}", &selections[0]);
    }
}
